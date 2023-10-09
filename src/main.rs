#![warn(clippy::pedantic)]

mod resolver;

use resolver::Resolver;

use anyhow::Result;
use clap::Parser;
use prettytable::{format, row, Table};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    name: String,
}

async fn dug_host(hostname: &str) -> Vec<(String, String)> {
    let resolver = Resolver::new(hostname);

    let (resolv, all_resolve, dns, dig, drill) = tokio::join!(
        resolver.resolv_conf(),
        resolver.exhaustive_resolv_conf(),
        resolver.dns(),
        resolver.dig(),
        resolver.drill(),
    );
    let os = resolver.os();

    let all = dns
        .into_iter()
        .chain([os, resolv])
        .chain(all_resolve)
        .chain(dig)
        .chain(drill);

    all.collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let name = args.name;
    let pairs = dug_host(&name).await;

    let mut tab = Table::new();
    tab.set_format(*format::consts::FORMAT_CLEAN);
    tab.set_titles(row![bH2c->name]);

    for (source, output) in pairs {
        tab.add_row(row![source, output]);
    }

    tab.printstd();

    Ok(())
}
