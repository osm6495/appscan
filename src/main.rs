use anyhow::Error;
use clap::{ArgAction, Parser, Subcommand};
use reqwest::Client;
use spinoff::{spinners, Color, Spinner};
use std::path::PathBuf;
use std::sync::Arc;

mod dns;
mod http;

#[derive(Parser)]
#[command(author = "Owen McCarthy", version = clap::crate_version!(), about = "A CLI tool to automate DNS querying and subdomain enumeration for bug bounty hunting", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for DNS records
    #[command(arg_required_else_help = true)]
    Dns {
        /// Specify a single URL, rather than a filepath to a list of URLs.
        #[arg(short, long, value_parser, conflicts_with("file_path"))]
        url: Option<String>,

        /// Disable loading spinner.
        #[arg(long, action = ArgAction::SetTrue)]
        no_spinner: bool,

        /// Specify the txt file to output the generated DNS records to.
        #[arg(short = 'o', long, value_parser, required = true)]
        output_file: PathBuf,

        /// Path to the URL file.
        #[arg(value_parser, required_unless_present = "url")]
        file_path: Option<PathBuf>,
    },

    /// Scan for HTTP responses
    #[command(arg_required_else_help = true)]
    Http {
        /// Specify a single URL, rather than a filepath to a list of URLs.
        #[arg(short, long, value_parser, conflicts_with("file_path"))]
        url: Option<String>,

        /// Specify the json file to output the generated http responses to.
        #[arg(short = 'm', long, value_parser, default_value_t = String::from("get"))]
        method: String,

        /// Include all responses, including 400 errors.
        #[arg(short = 'v', long, action = ArgAction::SetTrue)]
        verbose: bool,

        /// Disable loading spinner.
        #[arg(long, action = ArgAction::SetTrue)]
        no_spinner: bool,

        /// Specify the json file to output the generated http responses to.
        #[arg(short = 'o', long, value_parser, required = true)]
        output_file: PathBuf,

        /// Path to the URL file.
        #[arg(value_parser, required_unless_present = "url")]
        file_path: Option<PathBuf>,
    },
}

async fn http(
    url: Option<String>,
    method: String,
    no_spinner: bool,
    output_file: PathBuf,
    file_path: Option<PathBuf>,
    verbose: bool,
) -> anyhow::Result<()> {
    let mut methods: Vec<String> = method.split(',').map(|s| s.trim().to_uppercase()).collect();

    const INVALID_METHOD_ERR: &str = "Invalid method: {}
     Example method uses (methods are case insensitive): 
      -m get -> Send GET requests
      -m get,post,delete -> Send GET, POST, and DELETE requests
      -m all -> Send requests with all http methods
      -m common -> Send requests with common http methods (GET, POST, PUT, DELETE, PATCH)";

    let methods_clone = methods.clone();
    for method in methods_clone {
        // Check if the method is valid or "all" or "common"
        const ALLOWED_METHODS: [&str; 11] = [
            "GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD", "TRACE", "CONNECT", "ALL",
            "COMMON",
        ];

        if !ALLOWED_METHODS.contains(&method.as_str()) {
            return Err(Error::msg(INVALID_METHOD_ERR.to_owned() + &method));
        }
    }

    if methods.contains(&"ALL".to_string()) {
        methods.clear();
        methods = vec![
            "GET".to_string(),
            "POST".to_string(),
            "PUT".to_string(),
            "DELETE".to_string(),
            "PATCH".to_string(),
            "OPTIONS".to_string(),
            "HEAD".to_string(),
            "TRACE".to_string(),
            "CONNECT".to_string(),
        ];
    } else if methods.contains(&"COMMON".to_string()) {
        methods.clear();
        methods = vec![
            "GET".to_string(),
            "POST".to_string(),
            "PUT".to_string(),
            "DELETE".to_string(),
            "PATCH".to_string(),
        ];
    }

    let client = Arc::new(Client::new());
    let sp = if no_spinner {
        None
    } else {
        Some(Spinner::new(spinners::Arc, "Scanning...", Color::White))
    };

    let mut urls: Vec<String> = Vec::new();
    if let Some(url) = url {
        urls.push(url.to_string());
    } else if let Some(file_path) = file_path {
        urls = dns::input_file(&file_path).unwrap();
    } else {
        unreachable!()
    };

    let mut links = Vec::new();
    for link in urls {
        let parts: Vec<&str> = link.split_whitespace().collect();
        match parts.get(0).map(|s| {
            s.trim_end_matches('.')
                .trim_start_matches("https://")
                .trim_start_matches("http://")
        }) {
            Some(url) => {
                links.push(format!("https://{}", url.to_string()));
                links.push(format!("http://{}", url.to_string()));
            }
            None => (),
        }
    }
    let mut responses = http::request(links, client, methods).await;

    if verbose {
        responses = responses.into_iter().collect();
    } else {
        responses = responses
            .into_iter()
            .filter(|res| !res.is_client_error())
            .collect();
    }

    http::json_to_file(responses, output_file).await?;

    if let Some(mut spinner) = sp {
        spinner.stop();
    }

    Ok(())
}

async fn dns(
    url: Option<String>,
    no_spinner: bool,
    dns_output_file: PathBuf,
    file_path: Option<PathBuf>,
) {
    const SUBDOMAIN_WORDLIST_PATH: &str = "./resources/subdomain-wordlist.txt";
    const WORDLIST_OUTPUT_PATH: &str = "/tmp/domains.txt";
    const RESOLVERS_FILE: &str = "./resources/resolvers.txt";

    let mut input: Vec<String> = Vec::new();
    if let Some(url) = url {
        input.push(url.to_string());
    } else if let Some(file_path) = file_path {
        input = dns::input_file(&file_path).unwrap();
    } else {
        unreachable!()
    };

    let sp = if no_spinner {
        None
    } else {
        Some(Spinner::new(spinners::Arc, "Scanning...", Color::White))
    };

    dns::gen_wordlist(
        input,
        SUBDOMAIN_WORDLIST_PATH,
        WORDLIST_OUTPUT_PATH,
        RESOLVERS_FILE,
        &dns_output_file,
    )
    .await
    .unwrap();

    if let Some(mut spinner) = sp {
        spinner.stop();
    }
}

#[tokio::main]
pub async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Dns {
            url,
            no_spinner,
            output_file,
            file_path,
        } => dns(url, no_spinner, output_file, file_path).await,
        Commands::Http {
            url,
            method,
            no_spinner,
            output_file,
            file_path,
            verbose,
        } => http(url, method, no_spinner, output_file, file_path, verbose)
            .await
            .unwrap(),
    }
}
