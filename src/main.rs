mod lib;

use dyn_fmt::AsStrFormatExt;
use exitcode::{NOHOST, OK, UNAVAILABLE, USAGE};
use indicatif::{MultiProgress, ProgressBar};
use lib::{isitup, ping_isitup, DomainStatus};
use std::process::exit;
use std::thread::spawn;
use structopt::StructOpt;

#[derive(StructOpt)]
/// CLI utility to check if a website/domain is up or down, leveraging isitup.org API
struct InputDomains {
    domains: Vec<String>,
}

const PROGRESS_TICK_PERIOD: u64 = 50;

const CONN_CHECK: &str = "Checking connectivity";
const CHECKING_DOMAIN: &str = "Checking {}";
const CAN_ACCESS_ISITUP: &str = "Can access isitup.org! ✅\n";

const UP_MESSAGE: &str = "{} is up! 🎉";
const DOWN_MESSAGE: &str = "{} seems to be down! 💥";
const INVALID_DOMAIN_MESSAGE: &str = "{} does not seem to be a valid domain! ❌";

const NO_ARGS_ERROR: &str = "No domains specified. 🧐";
const CONNECTION_ERROR: &str = "Seems like you are not connected to the internet. 📶";
const PROXY_ERROR: &str = "Cannot access isitup.org. Please check your firewall/proxy settings. 🖧";
const UNEXPECTED_ERROR: &str = "Unexpected error occured! 🤯";

fn main() {
    let domains = InputDomains::from_args().domains;
    let dlen = domains.len();
    if dlen == 0 {
        eprintln!("{}", NO_ARGS_ERROR);
        exit(USAGE);
    } else {
        let conn_check_pb = ProgressBar::new_spinner();
        conn_check_pb.enable_steady_tick(PROGRESS_TICK_PERIOD);
        conn_check_pb.set_message(CONN_CHECK);
        match ping_isitup() {
            Ok(can_connect) => {
                if !can_connect {
                    conn_check_pb.finish_with_message(PROXY_ERROR);
                    exit(NOHOST);
                }
            }
            Err(_) => {
                conn_check_pb.finish_with_message(CONNECTION_ERROR);
                exit(UNAVAILABLE);
            }
        }
        conn_check_pb.finish_with_message(CAN_ACCESS_ISITUP);
        let multi_progress = MultiProgress::new();
        for domain in domains.into_iter() {
            let domain_pb = multi_progress.add(ProgressBar::new_spinner());
            domain_pb.enable_steady_tick(PROGRESS_TICK_PERIOD);
            domain_pb.set_message(CHECKING_DOMAIN.format(&[domain.clone()]));
            spawn(move || print_result(domain, domain_pb));
        }
        multi_progress.join().unwrap_or_default();
        exit(OK);
    }
}

fn print_result(domain: String, pb: ProgressBar) {
    match isitup(domain.clone()) {
        Ok(domain_status) => match domain_status {
            DomainStatus::Up => pb.finish_with_message(UP_MESSAGE.format(&[domain])),
            DomainStatus::Down => pb.finish_with_message(DOWN_MESSAGE.format(&[domain])),
            DomainStatus::DoesNotExist => {
                pb.finish_with_message(INVALID_DOMAIN_MESSAGE.format(&[domain]))
            }
        },
        Err(error) => {
            eprintln!("{}", UNEXPECTED_ERROR);
            eprintln!("{}", error);
            exit(UNAVAILABLE);
        }
    }
}
