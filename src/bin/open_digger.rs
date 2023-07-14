use clap::{arg, command, ArgMatches, Command, value_parser};
use reqwest::Client;
use tokio::sync::mpsc;
use std::{env, process};

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
                        .value_parser(value_parser!(Metric)),
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
            let metric = sub_matches.get_one::<Metric>("metric");
            let month = sub_matches.get_one::<String>("month");
            let download = sub_matches.get_one::<String>("download");
            dbg!(repo, month, download);

            let output = match metric {
                Some(m) => request_with_metric(m, repo, month)?,
                None => request_month_report(repo, month)?,
            };

            match download {
                Some(path) => todo!(),
                None => println!("{:}", output),
            }
        }
        _ => unreachable!(""),
    }
    Ok(())
}

#[inline]
fn request_with_metric(metric: &Metric, repo: &String, month: Option<&String>) -> Result<String> {
    let url = UrlBuilder::new(&repo).with_metric(metric.clone()).build()?;
    let body = reqwest::blocking::get(&url)?.text()?;
    println!("repo.name: {:}", repo);
    println!("request url: {:}", url);
    match month {
        Some(m) => {
            let value: serde_json::Value = serde_json::from_str(&body)?;
            Ok(format!("month: {:}\n data: {:}", m, value.get(m).unwrap()))
        }
        None => {
            Ok(format!("data: {:}", body))
        }
    }
}

#[inline]
fn request_month_report(repo: &String, month: Option<&String>) -> Result<String> {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()?;
    let month = month.cloned().expect("Should give the month when you query month report");
    let metrics = Metric::valid_metrics();
    let urls = metrics.iter()
        .map(|u| UrlBuilder::new(repo).with_metric(u.clone()).build())
        .collect::<Result<Vec<_>>>()?;
    rt.block_on(async move {
        let mut body = String::new();
        let (sender, mut receiver) = mpsc::channel::<Option<String>>(urls.len());
        for (url, metric) in urls.into_iter().zip(metrics.into_iter()) {
            let sender_clone = sender.clone();
            let month = month.clone();
            tokio::spawn(async move {
                fetch_url(&metric, url, &month, sender_clone).await.unwrap();
            });
        }
        drop(sender);
        while let Some(recieve_data) = receiver.recv().await {
            if let Some(s) = recieve_data {
                body.push_str(&s);
            }
        }
        Ok(body)
    })
}

async fn fetch_url(metric: &Metric, url: String, month: &String, sender: mpsc::Sender<Option<String>>) -> Result<()>{
    let response = Client::new()
        .get(&url)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let value: serde_json::Value = serde_json::from_str(&response)?;
    let month_value = value.get(month)
        .map(|v| format!("{:}: {:}\n", metric.to_string(), v));
    sender.send(month_value).await.unwrap();
    Ok(())
}
