# dug

Name resolution aggregator

# Synopsis

`dug HOSTNAME [HOSTNAME [HOSTNAME ...]]`

`dug --json HOSTNAME [HOSTNAME [HOSTNAME ...]]`

# Options

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
pretty-printed ASCII view, or as JSON output suitable for consumption by [jq][jq].

[bind9]: https://www.isc.org/bind/
[drill]: https://www.nlnetlabs.nl/projects/ldns/about/
[jq]: https://jqlang.github.io/jq/


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
$ dug aws.amazon.com www.kame.net

┌aws.amazon.com─────────────────┬───────────────────────────────────────┐
│ Cloudflare DNS                │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ Quad9 DNS                     │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ Google DNS                    │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ OS resolution                 │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ simulated nslookup            │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ resolv.conf server[10.7.0.1]  │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ resolv.conf server[127.0.0.1] │ Timed out after 7s                    │
├───────────────────────────────┼───────────────────────────────────────┤
│ A (dig)                       │ tp.8e49140c2-frontier.amazon.com.     │
│                               │ dr49lng3n1n2s.cloudfront.net.         │
│                               │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ AAAA (dig)                    │ tp.8e49140c2-frontier.amazon.com.     │
│                               │ dr49lng3n1n2s.cloudfront.net.         │
│                               │ 2600:9000:24bb:9200:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:e200:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:b000:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:4600:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:e600:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:200:1c:a813:8512:c241  │
│                               │ 2600:9000:24bb:5e00:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:be00:1c:a813:8512:c241 │
├───────────────────────────────┼───────────────────────────────────────┤
│ A (drill)                     │ tp.8e49140c2-frontier.amazon.com.     │
│                               │ dr49lng3n1n2s.cloudfront.net.         │
│                               │ 18.155.190.47                         │
├───────────────────────────────┼───────────────────────────────────────┤
│ AAAA (drill)                  │ tp.8e49140c2-frontier.amazon.com.     │
│                               │ dr49lng3n1n2s.cloudfront.net.         │
│                               │ 2600:9000:24bb:6800:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:2200:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:bc00:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:200:1c:a813:8512:c241  │
│                               │ 2600:9000:24bb:dc00:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:cc00:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:5200:1c:a813:8512:c241 │
│                               │ 2600:9000:24bb:7000:1c:a813:8512:c241 │
└───────────────────────────────┴───────────────────────────────────────┘
┌www.kame.net───────────────────┬────────────────────────────────────┐
│ Google DNS                    │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ Quad9 DNS                     │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ Cloudflare DNS                │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ OS resolution                 │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ simulated nslookup            │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ resolv.conf server[127.0.0.1] │ Timed out after 7s                 │
├───────────────────────────────┼────────────────────────────────────┤
│ resolv.conf server[10.7.0.1]  │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ A (dig)                       │ mango.itojun.org.                  │
│                               │ 210.155.141.200                    │
├───────────────────────────────┼────────────────────────────────────┤
│ AAAA (dig)                    │ mango.itojun.org.                  │
│                               │ 2001:2f0:0:8800::1:1               │
│                               │ 2001:2f0:0:8800:226:2dff:fe0b:4311 │
├───────────────────────────────┼────────────────────────────────────┤
│ drill                         │ deadline has elapsed               │
└───────────────────────────────┴────────────────────────────────────┘
```

## Example: JSON

```
$ dug --json aws.amazon.com
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
    "source": "resolv.conf server[10.7.0.1]",
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
## Example: use with `jq`

Collect all unique records returned by all resolvers.
```
$ dug -j aws.amazon.com | jq -r '[.[].records | select(. != null) .[]] | unique .[]'
13.35.127.119
2600:9000:21c4:3000:1c:a813:8513:e1c1
2600:9000:21c4:6e00:1c:a813:8513:e1c1
2600:9000:21c4:9200:1c:a813:8513:e1c1
2600:9000:21c4:b400:1c:a813:8513:e1c1
2600:9000:21c4:b600:1c:a813:8513:e1c1
2600:9000:21c4:d600:1c:a813:8513:e1c1
2600:9000:21c4:e400:1c:a813:8513:e1c1
2600:9000:21c4:e600:1c:a813:8513:e1c1
dr49lng3n1n2s.cloudfront.net.
tp.8e49140c2-frontier.amazon.com.
```

This looks busy, but here's the process:
1. `dug` outputs an array of JSON objects
2. `jq` extracts the `records` property of each, which is an array of IPs
3. strip the `null` values; ie, ignore unsuccessful resolutions
4. collect all the results into a single array
5. strip duplicate values
