use clap::{arg, command, ArgMatches, Command};
use std::{env, process, str::FromStr};

use open_digger_cli::{Result, UrlBuilder, Metric};

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
                .arg(arg!(--repo <REPO>).help("The github repo name, for example: X-lab2017/open-digger.").required(true))
                .arg(
                    arg!(--metric <METRIC>)
                        .help("The metric you want to query.")
                        .required(false)
                        .default_value("all"),
                )
                .arg(
                    arg!(--month <MONTH>)
                        .help("The month you want to query.")
                        .required(false),
                )
                .arg(
                    arg!(--download <DOWNLOAD>)
                        .help("The output file path if you want to download")
                        .required(false)
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
            let metric = Metric::from_str(sub_matches.get_one::<String>("metric").unwrap())?;
            let month = sub_matches.get_one::<String>("month");
            let download = sub_matches.get_one::<String>("download");
            dbg!(repo, metric.to_string(), month, download);

            let url = UrlBuilder::new(&repo).with_metric(metric).build()?;
            let body = reqwest::blocking::get(&url)?.text()?;
            println!("repo.name: {:}", repo);
            println!("request url: {:}", url);
            match month {
                Some(m) => {
                    let value: serde_json::Value = serde_json::from_str(&body)?;
                    println!("month: {:}", m);
                    println!("data: {:}", value.get(m).unwrap());
                }
                None => {
                    println!("data: {:}", body);
                }
            }
        }
        _ => unreachable!(""),
    }
    Ok(())
}
