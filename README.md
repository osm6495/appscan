<a name="readme-top"></a>


<!-- PROJECT SHIELDS -->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]
[![LinkedIn][linkedin-shield]][linkedin-url]



<!-- PROJECT LOGO -->
<br />
<div align="center">
  <a href="https://github.com/osm6495/appscan/">
  </a>

  <h3 align="center">AppScan</h3>

  <p align="center">
    A CLI tool to automate DNS querying and subdomain enumeration for bug bounty hunting
    <br />
    <a href="https://github.com/osm6495/appscan/">View Demo</a>
    ·
    <a href="https://github.com/osm6495/appscan/issues">Report Bug</a>
    ·
    <a href="https://github.com/osm6495/appscan/issues">Request Feature</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#usage">Usage</a>
      <ul>
        <li><a href="#examples">Examples</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#installing-the-latest-version">Installing the latest version</a></li>
        <li><a href="#installing-from-source">Installing from source</a></li>
      </ul>
    </li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT -->
## Usage
```
A CLI tool to automate DNS querying and subdomain enumeration for bug bounty hunting

Usage: appscan <COMMAND>

Commands:
  dns   Scan for DNS records
  http  Scan for HTTP responses
  help  Print this message or the help of the given subcommand(s)
```

Use the `dns` subcommand to use massdns to find all of the DNS records for the provided urls, then use the `http` subcommand to enumerate possible other subdomains on those domains and query them for a response.

### DNS
```
Scan for DNS records

Usage: appscan dns [OPTIONS] --output-file <OUTPUT_FILE> [FILE_PATH]
Example: appscan dns -o ./dns.txt ./in_scope_domains.txt

Arguments:
  [FILE_PATH]  Path to the URL file

Options:
  -u, --url <URL>                  Specify a single URL, rather than a filepath to a list of URLs
      --no-spinner                 Disable loading spinner
  -o, --output-file <OUTPUT_FILE>  Specify the txt file to output the generated DNS records to
  -h, --help                       Print help
  -V, --version                    Print version
```

### HTTP
```
Scan for HTTP responses

Usage: appscan http [OPTIONS] --output-file <OUTPUT_FILE> [FILE_PATH]
Example: appscan http -m common -o ./responses.json ./dns.txt

Arguments:
  [FILE_PATH]  Path to the URL file

Options:
  -u, --url <URL>                  Specify a single URL, rather than a filepath to a list of URLs
  -m, --method <METHOD>            Specify the json file to output the generated http responses to [default: get]
  -v, --verbose                    Include all responses, including 400 errors
      --no-spinner                 Disable loading spinner
  -o, --output-file <OUTPUT_FILE>  Specify the json file to output the generated http responses to
  -h, --help                       Print help
  -V, --version                    Print version
```
The HTTP subcommand can use the results from the DNS scanning, or a separate list of URLS. 

By default, only GET requests are made, but you can specify other methods with the `-m` flag like:
```
appscan http -m get,post,patch ./dns.txt
```
HTTP methods are case-insensitive and there is also an "all" and a "common" option for methods. `-m all` will use all possible http methods, which is not likely to be as useful as `-m common`, which automatically uses `GET`, `POST`, `PUT`, `PATCH`, and `DELETE`.

<!-- GETTING STARTED -->
## Getting Started

### Installing the latest version
You can use download a pre-built binary directly from the latest release: https://github.com/osm6495/appscan/releases

1. Select the latest version at the top of the page and open the `Assets` section
2. Download the file that applies for your system
3. (Optional) Move the binary to your `/usr/bin` directory for Linux and Mac or `C:\Program Files` for Windows. This will allow you to use the `appscan` command without directly calling the binary or having the source code.


### Installing from Source

_Below is an example of how you can instruct your audience on installing and setting up your app. This template doesn't rely on any external dependencies or services._

1. Install Rust: [http://rust-lang.org/](http://rust-lang.org/)
2. Clone the repo
  ```sh
  git clone https://github.com/osm6495/appscan
  cd appscan
  ```
3. Build the binary
  ```sh
  cargo build --release
  ```
4. Run the program
  ```sh
  ./target/release/sbom -h
  ```
5. (Optional) Move the binary to your `/usr/bin` directory for Linux and Mac or `C:\Program Files` for Windows. This will allow you to use the `appscan` command without directly calling the binary or having the source code.
  ```sh
  sudo mv ./target/release/appscan /usr/bin/appscan
  ```

<!-- ROADMAP -->
## Roadmap

- [ ] Allow other http methods to be included along with "common" in the `-m` flag for the http subcommand, to allow for something like `-m common, options` 

See the [open issues](https://github.com/osm6495/appscan/issues) for a full list of proposed features (and known issues).




<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request




<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.




<!-- CONTACT -->
## Contact

Owen McCarthy - contact@owen.biz

<!-- ACKNOWLEDGEMENT -->
## Acknowledgements

<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/osm6495/appscan.svg?color=orange
[contributors-url]: https://github.com/osm6495/appscan/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/osm6495/appscan.svg?style=flat&color=orange
[forks-url]: https://github.com/osm6495/appscan/network/members
[stars-shield]: https://img.shields.io/github/stars/osm6495/appscan.svg?style=flat&color=orange
[stars-url]: https://github.com/osm6495/appscan/stargazers
[issues-shield]: https://img.shields.io/github/issues/osm6495/appscan.svg?color=orange
[issues-url]: https://github.com/osm6495/appscan/issues
[license-shield]: https://img.shields.io/github/license/osm6495/appscan.svg?color=orange
[license-url]: https://github.com/osm6495/appscan/blob/master/LICENSE.txt
[linkedin-shield]: https://img.shields.io/badge/-LinkedIn-black.svg?color=blue&logo=linkedin&colorB=555
[linkedin-url]: https://www.linkedin.com/in/owen-mccarthy-060827192/
