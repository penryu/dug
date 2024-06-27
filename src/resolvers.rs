use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use futures_util::future;
use tokio::{process::Command, task::JoinHandle, time::timeout};
use trust_dns_resolver::{
    config::{NameServerConfig, ResolverConfig, ResolverOpts},
    system_conf, TokioAsyncResolver,
};

use crate::types::{DugResult, Resolution};

pub const LOOKUP_TIMEOUT: Duration = Duration::from_millis(7_000);

/// Uses the system default resolution. (Unix only)
pub fn os(name: &str) -> Resolution {
    let desc = "OS resolution";
    match format!("{name}:0").to_socket_addrs() {
        Ok(addrs) => Resolution::with_records(name, desc, addrs.map(|a| a.ip().to_string())),
        Err(e) => Resolution::with_err(name, desc, e.into()),
    }
}

/// Simulates a resolv.conf lookup by parsing the file.
pub async fn resolv_conf(name: &str) -> Resolution {
    let desc = "simulated nslookup";

    let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
        Ok(resolver) => resolver,
        Err(e) => return Resolution::with_err(name, desc, e.into()),
    };

    let dug_res = match timeout(LOOKUP_TIMEOUT, resolver.lookup_ip(name)).await {
        Ok(Ok(addrs)) => DugResult::from_records(addrs.into_iter().map(|ip| ip.to_string())),
        Ok(Err(e)) => DugResult::from_err(e.into()),
        Err(e) => DugResult::from_err(e.into()),
    };
    Resolution::new(name, desc, dug_res)
}

pub async fn local(name: &str) -> Vec<Resolution> {
    vec![os(name), resolv_conf(name).await]
}

/// Looks up name with every address listed in the system resolv.conf individually.
pub async fn exhaustive_resolv_conf(name: &str) -> Vec<Resolution> {
    let desc = "DNS resolution with all servers in resolv.conf";

    let (conf, opts) = match system_conf::read_system_conf() {
        Ok(pair) => pair,
        Err(e) => return vec![Resolution::with_err(name, desc, e.into())],
    };

    let mut servers: HashMap<SocketAddr, Vec<NameServerConfig>> = HashMap::new();

    for ns in conf.name_servers() {
        servers.entry(ns.socket_addr).or_default().push(ns.clone());
    }

    let configs = servers
        .into_iter()
        .map(|(sock_addr, name_servers)| {
            let config = ResolverConfig::from_parts(
                conf.domain().cloned(),
                conf.search().to_vec(),
                name_servers,
            );
            let desc = format!("resolv.conf server[{}]", ip_from_socket_addr(&sock_addr));
            (desc, config)
        })
        .collect::<HashMap<_, _>>();

    resolve_with_configs(name, configs, opts).await
}

/// Conducts a DNS lookup.
pub async fn dns(name: &str) -> Vec<Resolution> {
    let resolver_opts = ResolverOpts::default();

    let resolvers: HashMap<String, ResolverConfig> = HashMap::from([
        ("Cloudflare DNS".to_string(), ResolverConfig::cloudflare()),
        ("Google DNS".to_string(), ResolverConfig::google()),
        ("Quad9 DNS".to_string(), ResolverConfig::quad9()),
    ]);

    resolve_with_configs(name, resolvers, resolver_opts).await
}

/// Looks up a name using bind-tools dig
pub async fn dig(name: &str) -> Resolution {
    let args_v4 = &["+short", name, "A"];
    let args_v6 = &["+short", name, "AAAA"];

    let result = match tokio::try_join!(
        timeout(LOOKUP_TIMEOUT, command_output("dig", args_v4)),
        timeout(LOOKUP_TIMEOUT, command_output("dig", args_v6)),
    ) {
        Ok((Ok(mut recs), Ok(recs_v6))) => {
            recs.extend(recs_v6);
            DugResult::from_records(recs)
        }
        Ok((Ok(recs), _) | (_, Ok(recs))) => DugResult::from_records(recs),
        Ok((Err(e4), Err(_))) => DugResult::from_err(e4),
        Err(e) => DugResult::from_err(e.into()),
    };

    Resolution::new(name, "dig", result)
}

/// Looks up a name using ldns drill
pub async fn drill(name: &str) -> Resolution {
    let args_v4 = &["-Q", name, "A"];
    let args_v6 = &["-Q", name, "AAAA"];

    let result = match tokio::try_join!(
        timeout(LOOKUP_TIMEOUT, command_output("drill", args_v4)),
        timeout(LOOKUP_TIMEOUT, command_output("drill", args_v6)),
    ) {
        Ok((Ok(mut recs), Ok(recs_v6))) => {
            recs.extend(recs_v6);
            DugResult::from_records(recs)
        }
        Ok((Ok(recs), _) | (_, Ok(recs))) => DugResult::from_records(recs),
        Ok((Err(e4), Err(_))) => DugResult::from_err(e4),
        Err(e) => DugResult::from_err(e.into()),
    };

    Resolution::new(name, "drill", result)
}

// Private utility functions

async fn command_output(cmd: &str, args: &[&str]) -> Result<Vec<String>> {
    let output = Command::new(cmd).args(args).output().await?;

    match output.status.code() {
        None => bail!("process terminated by signal"),
        Some(code) if code != 0 => bail!("process failed with exit code {code}"),
        Some(code) => code,
    };

    let stdout = String::from_utf8(output.stdout).context("output not valid utf8")?;

    Ok(stdout.lines().map(ToString::to_string).collect())
}

fn ip_from_socket_addr(sock_addr: &SocketAddr) -> String {
    match sock_addr {
        SocketAddr::V4(addr4) => addr4.ip().to_string(),
        SocketAddr::V6(addr6) => addr6.ip().to_string(),
    }
    .to_string()
}

async fn resolve_with_configs(
    name: &str,
    configs: HashMap<String, ResolverConfig>,
    opts: ResolverOpts,
) -> Vec<Resolution> {
    let handles = configs.into_iter().map(|(src, cfg)| {
        let name = name.to_string();
        let handle: JoinHandle<Resolution> = tokio::spawn(async move {
            let resolver = TokioAsyncResolver::tokio(cfg, opts);

            let resolve_result = timeout(LOOKUP_TIMEOUT, resolver.lookup_ip(&name)).await;
            let dug_res: DugResult = match resolve_result {
                Ok(Ok(lookup)) => {
                    DugResult::from_records(lookup.into_iter().map(|ip| ip.to_string()))
                }
                Ok(Err(e)) => DugResult::from_err(e.into()),
                Err(_) => DugResult::from_err(anyhow!("Timed out after {LOOKUP_TIMEOUT:?}")),
            };
            Resolution::new(&name, &src, dug_res)
        });
        handle
    });

    future::try_join_all(handles)
        .await
        .expect("shouldn't happen")
}
