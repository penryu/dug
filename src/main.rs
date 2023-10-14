#![warn(clippy::pedantic)]

mod resolvers;
mod types;

use types::{DugResult, Resolution};

use anyhow::{ensure, Result};
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

/// Resolves a given `name` using all the listed resolvers.
///
/// Returns a vector of all resolutions, successful or failed.
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

/// Display resolutions for host in a pretty-printed table
///
/// Renders a table with the given `name` as its header, and a row for each `Resolution`.
fn render_resolution_table(name: &str, resolutions: Vec<Resolution>) -> Result<String> {
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
        let all_resolutions: Vec<Resolution> = resolution_set.into_iter().flatten().collect();
        println!("{}", serde_json::to_string_pretty(&all_resolutions)?);
    } else {
        for resolutions in resolution_set {
            ensure!(!resolutions.is_empty(), "Resolution set unexpectedly empty");

            let name = resolutions[0].name.clone();
            println!("{}", render_resolution_table(&name, resolutions)?);
        }
    }

    Ok(())
}
