# dug

An exhaustive name resolution aggregator

## Synopsis

Usage: `dug [OPTIONS] <HOSTNAMES>...`

## Description

`dug` is designed to be an _exhaustive_ name lookup tool, looking up the given hostname(s)
using any method available in the tool or on the system.

Resolvers are tried concurrently where possible.

Some methods/sources used are:

- The local host's configured resolver
    - e.g., gethostname(3), gethostbyname(3), getnameinfo(3), etc.
- Major public DNS resolvers:
    - Cloudflare
    - Google
    - Quad9
- A simulated nslookup
    - Works by parsing `/etc/resolv.conf` (if present) and querying the hosts found.
    - May be significantly different from OS-based resolution.

`dug` will also use external utilities such as `dig` (from [BIND9][dig]) or `drill` (from
[ldns][drill]) if found on the `$PATH`.

## Output formats

The following output formats are available:

- Table - a pretty-printed table of results
- ASCII - `grep` and `awk`-friendly ASCII text
- JSON  - JSON array of results suitable for use with `jq`

See the [examples][examples] for more information.

[dig]: https://www.isc.org/bind/
[drill]: https://www.nlnetlabs.nl/projects/ldns/about/
[examples]: examples.md
[jq]: https://jqlang.github.io/jq/

## Installation

### Build from source

If you already have a Rust toolchain installed, you can simply:

```bash
cargo install dug
```

The simplest way to install a Rust toolchain is with [rustup][rustup].

### github

_Coming soon._

[rustup]: https://rust-lang.github.io/rustup/installation/other.html
