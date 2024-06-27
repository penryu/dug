# dug

Name resolution aggregator


# Synopsis

Usage: `dug [OPTIONS] <HOSTNAMES>...`



# Options

The following options are available:

Arguments:
  <HOSTNAMES>...

Options:
  -a, --ascii    Format results as simple ASCII text
  -j, --json     Format results as structured JSON text
  -h, --help     Print help
  -V, --version  Print version


# Description

`dug` is designed to be an _exhaustive_ name lookup tool, looking up the given hostname(s) using any
method available in the tool or on the system.

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

`dug` will also use external utilities such as `dig` (from [BIND9][bind9]) or `drill` (from
[ldns][drill]) if found on the `$PATH`.

Resolvers are tried concurrently where possible, and results are aggregated in either a
pretty-printed ASCII view, or as JSON output suitable for consumption by [jq][jq]. See
the [Options] for more info.

[bind9]: https://www.isc.org/bind/
[drill]: https://www.nlnetlabs.nl/projects/ldns/about/
[jq]: https://jqlang.github.io/jq/
[options]: #options


# Installation


## Build from source

If you already have a Rust toolchain installed, you can simply:

```bash
cargo install dug
```

The simplest way to install a Rust toolchain is with [rustup][rustup].


## github

_Coming soon._

[rustup]: https://rust-lang.github.io/rustup/installation/other.html


# Examples


## Example: Text

```
$ dug wikipedia.org www.kame.net

┌wikipedia.org─────────────────┬────────────────────┐
│ Quad9 DNS                    │ 198.35.26.96       │
├──────────────────────────────┼────────────────────┤
│ Google DNS                   │ 198.35.26.96       │
├──────────────────────────────┼────────────────────┤
│ Cloudflare DNS               │ 198.35.26.96       │
├──────────────────────────────┼────────────────────┤
│ OS resolution                │ 198.35.26.96       │
├──────────────────────────────┼────────────────────┤
│ simulated nslookup           │ 198.35.26.96       │
├──────────────────────────────┼────────────────────┤
│ resolv.conf server[10.0.0.1] │ 198.35.26.96       │
├──────────────────────────────┼────────────────────┤
│ dig                          │ 198.35.26.96       │
│                              │ 2620:0:863:ed1a::1 │
├──────────────────────────────┼────────────────────┤
│ drill                        │ 198.35.26.96       │
│                              │ 2620:0:863:ed1a::1 │
└──────────────────────────────┴────────────────────┘
┌www.kame.net──────────────────┬────────────────────────────────────┐
│ Google DNS                   │ 210.155.141.200                    │
├──────────────────────────────┼────────────────────────────────────┤
│ Quad9 DNS                    │ 210.155.141.200                    │
├──────────────────────────────┼────────────────────────────────────┤
│ Cloudflare DNS               │ 210.155.141.200                    │
├──────────────────────────────┼────────────────────────────────────┤
│ OS resolution                │ 210.155.141.200                    │
├──────────────────────────────┼────────────────────────────────────┤
│ simulated nslookup           │ 210.155.141.200                    │
├──────────────────────────────┼────────────────────────────────────┤
│ resolv.conf server[10.0.0.1] │ 210.155.141.200                    │
├──────────────────────────────┼────────────────────────────────────┤
│ dig                          │ mango.itojun.org.                  │
│                              │ 210.155.141.200                    │
│                              │ mango.itojun.org.                  │
│                              │ 2001:2f0:0:8800:226:2dff:fe0b:4311 │
│                              │ 2001:2f0:0:8800::1:1               │
├──────────────────────────────┼────────────────────────────────────┤
│ drill                        │ mango.itojun.org.                  │
│                              │ 210.155.141.200                    │
│                              │ mango.itojun.org.                  │
│                              │ 2001:2f0:0:8800::1:1               │
│                              │ 2001:2f0:0:8800:226:2dff:fe0b:4311 │
└──────────────────────────────┴────────────────────────────────────┘
```

## Example: JSON

```
$ dug --json wikipedia.org
[
  {
    "name": "aws.amazon.com",
    "source": "Quad9 DNS",
    "records": [
      "18.155.190.47"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "Cloudflare DNS",
    "records": [
      "18.155.190.47"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "Google DNS",
    "records": [
      "18.155.190.47"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "OS resolution",
    "records": [
      "18.155.190.47"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "simulated nslookup",
    "records": [
      "18.155.190.47"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "resolv.conf server[10.0.0.1]",
    "records": [
      "18.155.190.47"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "resolv.conf server[127.0.0.1]",
    "failure": "Timed out after 7s"
  },
  {
    "name": "aws.amazon.com",
    "source": "A (dig)",
    "records": [
      "tp.8e49140c2-frontier.amazon.com.",
      "dr49lng3n1n2s.cloudfront.net.",
      "18.155.190.47"
    ]
  }
]

```
## Example: Get only the Google DNS results for a set of hostnames

```
$ dug -j abc.com cbs.com nbc.com | jq -cr \
    'map(select(.source | contains("Google")))[] | "\(.name)\t\(.records | join(" "))"'
abc.com 65.8.161.63 65.8.161.31 65.8.161.4 65.8.161.47
cbs.com 34.149.41.86
nbc.com 23.67.33.102 23.67.33.74
```

## Example: Extract only the resolved IPs with `jq`

```
$ dug -j google.com | jq -r 'map(select(.records).records) | flatten | unique .[]'
142.250.189.174
142.250.191.78
142.251.46.206
172.217.164.110
2607:f8b0:4005:814::200e
```

This looks busy, but here's what `jq` is doing:
1. `map(...)` &mdash; for each result in the array...
2. `select(.records)` &mdash; select only objects where the `records` property is truthy (stripping `null`)
3. `.records` &mdash; ...and extract the `records` property from the selected objects
4. `flatten` the nested of arrays into a single array of all resolved IPs
5. `unique` &mdash; remove all duplicate values
6. `.[]` &mdash; convert the array into a sequence of strings, one per line
