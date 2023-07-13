use clap::{arg, command, ArgMatches, Command};
use std::{env, process};

use open_digger_cli::{CliError, Result};

fn main() -> Result<()> {
    let matches = command!()
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("repo")
                .about("Query a github repo.")
                .arg(arg!(--repo <REPO>).help("The github repo, for example: X-lab2017/open-digger.").required(true))
                .arg(
                    arg!(--metric <METRIC>)
                        .help("The metric you want to query.")
                        .required(true),
                )
                .arg(
                    arg!(--month <MONTH>)
                        .help("The month you want to query.")
                        .required(false)
                        .default_value("")
                )
                .arg(
                    arg!(--download <DOWNLOAD>)
                        .help("If you want to download: y/n")
                        .required(false)
                        .default_value("n")
                ),
        )
        .get_matches();

    if let Err(err) = request(matches) {
        eprintln!("{:?}", err);
        process::exit(-1);
    }
    Ok(())
}

fn request(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("repo", sub_matches)) => {
            let repo = sub_matches.get_one::<String>("repo").unwrap();
            let metric = sub_matches.get_one::<String>("metric").unwrap();
            let month = sub_matches.get_one::<String>("month").unwrap();
            let download = sub_matches.get_one::<String>("download").unwrap();
            dbg!(&repo, metric, &month, &download);
        }
        _ => unreachable!(""),
    }
    Ok(())
}
