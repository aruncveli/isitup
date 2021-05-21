mod lib;
use std::io::{self, stdin, stdout, Write};

use dyn_fmt::AsStrFormatExt;
use lib::isitup;
use structopt::StructOpt;

#[derive(StructOpt)]
struct InputDomains {
    domains: Vec<String>,
}

const ENTER_DOMAINS: &str = "Enter domains, space separated: ";
const UP_MESSAGE: &str =
    "{} is up! 🎉\nIt took {} seconds time to get a {} status code from an IP address of {}.";
const DOWN_MESSAGE: &str = "{} seems to be down! 💥";
const CONNECTION_ERROR: &str = "Seems like you're not connected to the internet. ❌📶";
const INVALID_DOMAIN_MESSAGE: &str = "{} does not seem to be a valid domain. ❌🌐";

fn main() {
    let domains = InputDomains::from_args().domains;
    if domains.len() == 0 {
        loop {
            print!("{}", ENTER_DOMAINS);
            stdout().flush().unwrap_or_default();
            for domain in read_line_from_console()
                .unwrap_or_default()
                .split_whitespace()
            {
                print_response(domain.to_string());
            }
        }
    } else {
        println!();
        for domain in domains.into_iter() {
            print_response(domain);
        }
    }
}

fn read_line_from_console() -> io::Result<String> {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;
    println!();
    Ok(buffer)
}

fn print_response(domain: String) {
    match isitup(domain.clone()) {
        Ok(valid_response) => {
            if valid_response.status_code == 1 {
                println!(
                    "{}",
                    UP_MESSAGE.format(&[
                        valid_response.domain,
                        valid_response.response_time.to_string(),
                        valid_response.response_code.unwrap_or_default().to_string(),
                        valid_response.response_ip.unwrap_or_default()
                    ])
                );
            } else {
                println!("{}", DOWN_MESSAGE.format(&[domain]));
            }
        }
        Err(error_response) => {
            if error_response.is_connect() {
                println!("{}", CONNECTION_ERROR);
            } else if error_response.is_decode() {
                println!("{}", INVALID_DOMAIN_MESSAGE.format(&[domain]));
            }
        }
    }
    println!();
}
