//! An exhaustive name resolution aggregator
//!
//! `dug` is designed to be an _exhaustive_ name lookup tool, looking up the given hostname(s)
//! using any method available in the tool or on the system.
//!
//! Some methods/sources used are:
//!
//! - The local host's configured resolver
//!     - e.g., gethostname(3), gethostbyname(3), getnameinfo(3), etc.
//! - Major public DNS resolvers:
//!     - Cloudflare
//!     - Google
//!     - Quad9
//! - A simulated nslookup
//!     - Works by parsing `/etc/resolv.conf` (if present) and querying the hosts found.
//!     - May be significantly different from OS-based resolution.
//!
//! `dug` will also use external utilities such as `dig` (from [BIND9][bind9]) or `drill` (from
//! [ldns][drill]) if found on the `$PATH`.
//!
//! Resolvers are tried concurrently where possible, and results are aggregated in either a
//! pretty-printed ASCII view, or as JSON output suitable for consumption by [jq][jq].
//!
//! See the [README][readme] for examples.
//!
//! [bind9]: https://www.isc.org/bind/
//! [drill]: https://www.nlnetlabs.nl/projects/ldns/about/
//! [jq]: https://jqlang.github.io/jq/
//! [readme]: README.md
//! ```

#![deny(clippy::all)]
#![warn(clippy::pedantic)]

mod resolvers;
mod types;

use std::iter::once;

use anyhow::{ensure, Result};
use clap::Parser;
use futures_util::future::join_all;
use tabled::{
    builder::Builder,
    row,
    settings::{themes::ColumnNames, Style},
};

use types::{DugResult, Resolution};

/// Command line arguments.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    #[arg(short, long, help = "Format results as simple ASCII text")]
    ascii: bool,

    #[arg(short, long, help = "Format results as structured JSON text")]
    json: bool,

    #[arg(required = true)]
    hostnames: Vec<String>,
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

    dns.into_iter()
        .chain(local)
        .chain(all_resolve)
        .chain(once(dig))
        .chain(once(drill))
        .collect()
}

/// Display resolutions for host in a plain text table
///
/// Renders one table of results in ASCII text suitable for use with `grep` or `awk`.
fn render_resolution_set_ascii(resolution_set: Vec<Vec<Resolution>>) {
    resolution_set
        .into_iter()
        .flatten()
        .for_each(|res| println!("{res}"));
}

/// Display resolutions in JSON format
///
/// Renders a resolution set as an array of JSON objects suitable for use with `jq`.
fn render_resolution_set_json(resolution_set: Vec<Vec<Resolution>>) -> Result<()> {
    let all_resolutions = resolution_set.into_iter().flatten().collect::<Vec<_>>();
    let output = serde_json::to_string_pretty(&all_resolutions)?;
    println!("{output}");
    Ok(())
}

/// Display resolutions for host in a pretty-printed table
///
/// Renders a table with the given `name` as its header, and a row for each `Resolution`.
fn render_resolution_set_pretty(resolution_set: Vec<Vec<Resolution>>) {
    for resolutions in resolution_set {
        // set hostname as header
        let hostname = &resolutions[0].name;
        let mut builder = Builder::from(row![hostname]);

        for r in resolutions {
            let results = match r.result {
                DugResult::Records(recs) => recs.join("\n"),
                DugResult::Failure(fail) => fail,
            };
            builder.push_record(vec![r.source, results]);
        }

        let mut table = builder.build();
        table.with(Style::modern()).with(ColumnNames::default());

        println!("{table}");
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    ensure!(!(args.ascii & args.json),);

    let request_set = args.hostnames.iter().map(|name| dug_host(name));
    let resolution_set = join_all(request_set).await;
    ensure!(resolution_set.iter().all(|res| !res.is_empty()));

    match (args.ascii, args.json) {
        (false, false) => render_resolution_set_pretty(resolution_set),
        (true, false) => render_resolution_set_ascii(resolution_set),
        (false, true) => render_resolution_set_json(resolution_set)?,
        _ => eprintln!("Only one of --ascii or --json can be selected."),
    }

    Ok(())
}
