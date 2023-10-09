use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::string::ToString;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use futures_util::future;
use itertools::Itertools;
use tokio::{process::Command, task::JoinHandle, time::timeout};
use trust_dns_resolver::{
    config::{NameServerConfig, ResolverConfig, ResolverOpts},
    system_conf, TokioAsyncResolver,
};

pub const LOOKUP_TIMEOUT: Duration = Duration::from_millis(2_000);

pub type Resolution = (String, String);

#[derive(Debug)]
pub struct Resolver {
    name: String,
}

impl Resolver {
    pub fn new(name: &str) -> Self {
        Resolver { name: name.into() }
    }

    /// Uses the system default resolution. (Unix only)
    pub fn os(&self) -> Resolution {
        let value = match format!("{}:0", self.name).to_socket_addrs() {
            Ok(addrs) => addrs.map(|a| a.ip().to_string()).join(" "),
            Err(e) => e.to_string(),
        };
        ("OS".into(), value)
    }

    /// Simulates a resolv.conf lookup by parsing the file.
    pub async fn resolv_conf(&self) -> Resolution {
        let label = "nslookup [simulated]".to_string();

        let resolver = match TokioAsyncResolver::tokio_from_system_conf() {
            Err(e) => return (label, e.to_string()),
            Ok(resolver) => resolver,
        };

        let value = match timeout(LOOKUP_TIMEOUT, resolver.lookup_ip(&self.name)).await {
            Ok(Ok(addrs)) => addrs.into_iter().map(|ip| ip.to_string()).collect(),
            Ok(Err(e)) => e.to_string(),
            Err(e) => e.to_string(),
        };
        (label, value)
    }

    /// Looks up name with every address listed in the system resolv.conf individually.
    pub async fn exhaustive_resolv_conf(&self) -> Vec<Resolution> {
        let label = "exhaustive resolve".to_string();

        let (conf, opts) = match system_conf::read_system_conf() {
            Ok(pair) => pair,
            Err(e) => return vec![(label, e.to_string())],
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
                let label = format!("resolv.conf server[{}]", ip_from_socket_addr(&sock_addr));
                (label, config)
            })
            .collect::<HashMap<_, _>>();

        resolve_with_configs(&self.name, configs, opts).await
    }

    /// Conducts a DNS lookup.
    pub async fn dns(&self) -> Vec<Resolution> {
        let resolver_opts = ResolverOpts::default();

        let resolvers: HashMap<String, ResolverConfig> = HashMap::from([
            ("Cloudflare".to_string(), ResolverConfig::cloudflare()),
            ("Google".to_string(), ResolverConfig::google()),
            ("Quad9".to_string(), ResolverConfig::quad9()),
        ]);

        resolve_with_configs(&self.name, resolvers, resolver_opts).await
    }

    /// Looks up a name using bind-tools dig
    pub async fn dig(&self) -> Vec<Resolution> {
        let ip4_args = &["+short", &self.name, "A"];
        let ip6_args = &["+short", &self.name, "AAAA"];
        let (ip4_lines, ip6_lines) = match tokio::try_join!(
            timeout(LOOKUP_TIMEOUT, command_output("dig", ip4_args)),
            timeout(LOOKUP_TIMEOUT, command_output("dig", ip6_args)),
        ) {
            Ok(pair) => pair,
            Err(e) => return vec![("dig".into(), e.to_string())],
        };

        vec![
            (
                "dig: A".into(),
                ip4_lines.unwrap_or_else(|e| vec![e.to_string()]).join(" "),
            ),
            (
                "dig: AAAA".into(),
                ip6_lines.unwrap_or_else(|e| vec![e.to_string()]).join(" "),
            ),
        ]
    }

    /// Looks up a name using ldns drill
    pub async fn drill(&self) -> Vec<Resolution> {
        let ip4_args = &["-Q", &self.name, "A"];
        let ip6_args = &["-Q", &self.name, "AAAA"];
        let (ip4_lines, ip6_lines) = match tokio::try_join!(
            timeout(LOOKUP_TIMEOUT, command_output("drill", ip4_args)),
            timeout(LOOKUP_TIMEOUT, command_output("drill", ip6_args)),
        ) {
            Ok(pair) => pair,
            Err(e) => return vec![("drill".into(), e.to_string())],
        };

        vec![
            (
                "drill: A".into(),
                ip4_lines.unwrap_or_else(|e| vec![e.to_string()]).join(" "),
            ),
            (
                "drill: AAAA".into(),
                ip6_lines.unwrap_or_else(|e| vec![e.to_string()]).join(" "),
            ),
        ]
    }
}

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

            match timeout(LOOKUP_TIMEOUT, resolver.lookup_ip(name)).await {
                Ok(Ok(lookup)) => (src, lookup.into_iter().map(|ip| ip.to_string()).join(" ")),
                Ok(Err(e)) => (src, e.to_string()),
                Err(_) => (src, format!("Timed out after {LOOKUP_TIMEOUT:?}")),
            }
        });
        handle
    });

    future::try_join_all(handles)
        .await
        .expect("shouldn't happen")
}
