#![warn(clippy::pedantic)]

mod resolvers;
mod types;

use types::{DugResult, Resolution};

use anyhow::{ensure, Context, Result};
use clap::Parser;
use futures_util::future::join_all;
use tabled::{
    builder::Builder,
    row,
    settings::{themes::ColumnNames, Style},
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    json: bool,

    names: Vec<String>,
}

async fn dug_host(name: &str) -> Vec<Resolution> {
    let (local, all_resolve, dns, dig, drill) = tokio::join!(
        resolvers::local(name),
        resolvers::exhaustive_resolv_conf(name),
        resolvers::dns(name),
        resolvers::dig(name),
        resolvers::drill(name),
    );

    let all = dns
        .into_iter()
        .chain(local)
        .chain(all_resolve)
        .chain(dig)
        .chain(drill);

    all.collect()
}

/// Display resolutions for host as a list of JSON objects
fn render_json(resolutions: Vec<Resolution>) -> Result<String> {
    let obj = resolutions.into_iter().collect::<Vec<_>>();
    Ok(serde_json::to_string_pretty(&obj)?)
}

/// Display resolutions for host in a pretty-printed table
fn render_text(name: &str, resolutions: Vec<Resolution>) -> Result<String> {
    ensure!(!resolutions.is_empty(), "Resolution set is empty");

    // set hostname as header
    let mut builder = Builder::from(row![name]);

    for r in resolutions {
        let results = match r.result {
            DugResult::Records(recs) => recs.join("\n"),
            DugResult::Failure(fail) => fail,
        };
        builder.push_record(vec![r.source, results]);
    }

    let mut table = builder.build();
    table.with(Style::modern()).with(ColumnNames::default());

    Ok(table.to_string())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let request_set = args.names.iter().map(|name| dug_host(name));
    let resolution_set = join_all(request_set).await;

    if args.json {
        let all_resolutions: Vec<Resolution> = resolution_set
            .into_iter()
            .reduce(|mut all, res| {
                all.extend(res);
                all
            })
            .context("Resolution set unexpectedly empty")?;
        let output = render_json(all_resolutions)?;
        println!("{output}");
    } else {
        for resolutions in resolution_set {
            ensure!(!resolutions.is_empty(), "Resolution set unexpectedly empty");
            let name = resolutions[0].name.clone();
            println!("{}", render_text(&name, resolutions)?);
        }
    }

    Ok(())
}
