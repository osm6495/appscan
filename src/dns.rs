use anyhow::anyhow;
use anyhow::Result;
use duct::cmd;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn input_file(path: &std::path::PathBuf) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut urls = Vec::new();
    let mut ok = true;
    for raw_line in buf_reader.lines() {
        match raw_line {
            Ok(line) => {
                urls.push(line);
            }
            Err(_) => {
                ok = false;
            }
        }
    }

    if !ok {
        return Err(anyhow::anyhow!("failed to parse input file"));
    }

    return Ok(urls);
}

async fn run_async(command: duct::Expression) -> Result<()> {
    let _ = tokio::task::spawn_blocking(move || command.run())
        .await?
        .map_err(|e| anyhow!("Command failed: {}", e));

    Ok(())
}

pub async fn gen_wordlist(
    urls: Vec<String>,
    wordlist_path: &str,
    tmp_path: &str,
    resolvers_path: &str,
    output_path: &std::path::PathBuf,
) -> Result<()> {
    let mut output = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(tmp_path)?;

    // Loop over URLs to append them to subdomains and collect in a temp file
    for url in urls {
        let line = cmd!("cat", wordlist_path)
            .pipe(cmd!("sed", format!("s/$/.{}/", url)))
            .stdout_null()
            .read()?;
        writeln!(output, "{}", line)?;
    }

    // Sort and remove duplicates, and output to a new temp file
    let sorted_temp_path = "/tmp/sorted_domains.txt";
    cmd!("sort", tmp_path)
        .pipe(cmd!("uniq"))
        .pipe(cmd!("tee", sorted_temp_path))
        .stdout_null()
        .run()?;

    // Clean up and move sorted data back to tmp_path
    fs::remove_file(tmp_path)?;
    fs::rename(sorted_temp_path, tmp_path)?;

    // DNS lookups and filtering, appending results to output
    let dns_types = ["A", "AAAA", "CNAME"];
    for t in dns_types.iter() {
        let child = cmd!("cat", tmp_path)
            .pipe(
                cmd!("massdns", "-r", resolvers_path, "-t", t, "-o", "S", "-q").stderr_to_stdout(),
            )
            .pipe(cmd!("grep", "-v", "TCP written too few bytes for qname"))
            .pipe(cmd!("tee", "-a", output_path))
            .stdout_null();
        run_async(child).await?
    }

    // Final sort and remove duplicates at the output path
    let sorted_output_path = "/tmp/sorted_records.txt";
    cmd!("sort", output_path)
        .pipe(cmd!("uniq"))
        .pipe(cmd!("grep", "-v", "salesforce.edgekey.net"))
        .pipe(cmd!("grep", "-E", dns_types.join("|")))
        .pipe(cmd!("tee", sorted_output_path))
        .stdout_null()
        .run()?;

    // Final cleanup and move
    fs::remove_file(output_path)?;
    fs::rename(sorted_output_path, output_path)?;

    Ok(())
}
