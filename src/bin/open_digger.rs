use clap::{arg, command, value_parser, ArgMatches, Command};
use reqwest::Client;
use tokio::sync::mpsc;
#[macro_use]
extern crate prettytable;
use open_digger_cli::CliError;
use open_digger_cli::{Metric, Result, UrlBuilder};
use prettytable::format::consts::FORMAT_BOX_CHARS;
use prettytable::Table;
use std::io::Write;

fn main() -> Result<()> {
    let matches = command!()
        .version(std::env!("CARGO_PKG_VERSION"))
        .about(std::env!("CARGO_PKG_DESCRIPTION"))
        .author(std::env!("CARGO_PKG_AUTHORS"))
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
                        .help("The file save path if you want to download, for example: /User/test/download")
                        .required(false)
                ),
        )
        .get_matches();

    if let Err(err) = handle(matches) {
        eprintln!("{:?}", err);
        std::process::exit(-1);
    }
    Ok(())
}

fn handle(matches: ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("repo", sub_matches)) => {
            let repo = sub_matches.get_one::<String>("repo").unwrap();
            let metric = sub_matches.get_one::<Metric>("metric");
            let month = sub_matches.get_one::<String>("month");
            let download_path = sub_matches.get_one::<String>("download");

            let data = request(&repo, metric, month)?;
            match download_path {
                Some(download_path) => download(&repo, data, download_path, month)?,
                None => print_table(data, month),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn request(
    repo: &String,
    metric: Option<&Metric>,
    month: Option<&String>,
) -> Result<Vec<(Metric, String)>> {
    let metrics = match metric {
        Some(metric) => vec![metric.clone()],
        None => Metric::valid_metrics(),
    };
    let month = match month {
        Some(month) => Some(month.clone()),
        None => None,
    };

    let url_builder = UrlBuilder::new(&repo);
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .enable_io()
        .build()?;

    rt.block_on(async move {
        let (sender, mut receiver) = mpsc::channel::<Result<(Metric, String)>>(metrics.len());
        for metric in metrics.into_iter() {
            let url = url_builder.build_url_with_metric(&metric);
            let cloned_sender = sender.clone();
            let cloned_month = month.clone();
            tokio::spawn(async move {
                // safe to unwrap().
                cloned_sender
                    .send(request_and_filter_data(url, &metric, cloned_month).await)
                    .await
                    .unwrap();
            });
        }
        drop(sender);

        let mut result = vec![];
        while let Some(data) = receiver.recv().await {
            result.push(data?);
        }
        Ok(result)
    })
}

async fn request_and_filter_data(
    url: String,
    metric: &Metric,
    month: Option<String>,
) -> Result<(Metric, String)> {
    let data = Client::new().get(&url).send().await?.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&data)?;
    match month {
        Some(month) => {
            if let Some(month_data) = json_data.get(month) {
                Ok((metric.clone(), month_data.to_string()))
            } else {
                Ok((metric.clone(), String::from("NULL")))
            }
        }
        None => Ok((metric.clone(), json_data.to_string())),
    }
}

fn download(
    repo: &String,
    data: Vec<(Metric, String)>,
    download_path: &String,
    month: Option<&String>,
) -> Result<()> {
    let month = match month {
        Some(month) => month.clone(),
        None => String::from("alltime"),
    };

    let dir_path = std::path::PathBuf::from(download_path);
    std::fs::create_dir_all(&dir_path)?;
    for (metric, data) in data.iter() {
        let mut file_tag = 0;
        let mut file_name = format!(
            "{}-{}_{}-{}.json",
            repo,
            metric.to_string(),
            file_tag,
            month
        );
        let mut file_path = dir_path.join(file_name);
        if !file_path.is_absolute() {
            return Err(CliError::String(String::from("invalid download path")));
        }
        // de-duplicate
        loop {
            match std::fs::metadata(&file_path) {
                Ok(_) => (), // file exists.
                Err(_) => break,
            }
            file_tag += 1;
            file_name = format!(
                "{}-{}_{}-{}.json",
                repo,
                metric.to_string(),
                file_tag,
                month
            );
            file_path = dir_path.join(file_name);
        }
        // safe to unwrap().
        let parent_dir = std::path::Path::new(&file_path).parent().unwrap();
        std::fs::create_dir_all(parent_dir)?;
        let mut file = std::fs::File::create(&file_path)?;
        file.write_all(data.as_bytes())?;
        println!("Downloaded file: {}", file_path.to_string_lossy());
    }
    Ok(())
}

fn print_table(data: Vec<(Metric, String)>, month: Option<&String>) {
    let month = match month {
        Some(month) => month.clone(),
        None => String::from("ALL MONTH"),
    };
    let mut table = Table::new();
    table.set_format(*FORMAT_BOX_CHARS);
    table.add_row(row!["Month", "Metric", "Data"]);
    for (metric, data) in data.iter() {
        table.add_row(row![month, metric.to_string(), data]);
    }
    table.printstd();
}
