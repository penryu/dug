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
//! [bind9]: https://www.isc.org/bind/
//! [drill]: https://www.nlnetlabs.nl/projects/ldns/about/
//! [jq]: https://jqlang.github.io/jq/
//!
//! # Examples
//!
//! ```
//! $ dug archive.org
//! ┌archive.org───────────────────┬───────────────┐
//! │ Cloudflare DNS               │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ Google DNS                   │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ Quad9 DNS                    │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ OS resolution                │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ simulated nslookup           │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ resolv.conf server[10.7.0.1] │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ A (dig)                      │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ AAAA (dig)                   │               │
//! ├──────────────────────────────┼───────────────┤
//! │ A (drill)                    │ 207.241.224.2 │
//! ├──────────────────────────────┼───────────────┤
//! │ AAAA (drill)                 │               │
//! └──────────────────────────────┴───────────────┘
//!
//! $ dug --json archive.org
//! [
//!   {
//!     "name": "archive.org",
//!     "source": "Quad9 DNS",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "Cloudflare DNS",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "Google DNS",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "OS resolution",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "simulated nslookup",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "resolv.conf server[10.7.0.1]",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "A (dig)",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "AAAA (dig)",
//!     "records": []
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "A (drill)",
//!     "records": [
//!       "207.241.224.2"
//!     ]
//!   },
//!   {
//!     "name": "archive.org",
//!     "source": "AAAA (drill)",
//!     "records": []
//!   }
//! ]
//! ```

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

/// Command line arguments.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
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

    let request_set = args.hostnames.iter().map(|name| dug_host(name));
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
