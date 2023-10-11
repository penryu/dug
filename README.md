# dug

Name resolution aggregator

# Sample Output

## Text

```
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

## JSON

```
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
  },
  {
    "name": "aws.amazon.com",
    "source": "AAAA (dig)",
    "records": [
      "tp.8e49140c2-frontier.amazon.com.",
      "dr49lng3n1n2s.cloudfront.net.",
      "2600:9000:24bb:d800:1c:a813:8512:c241",
      "2600:9000:24bb:6200:1c:a813:8512:c241",
      "2600:9000:24bb:da00:1c:a813:8512:c241",
      "2600:9000:24bb:e400:1c:a813:8512:c241",
      "2600:9000:24bb:b400:1c:a813:8512:c241",
      "2600:9000:24bb:1200:1c:a813:8512:c241",
      "2600:9000:24bb:7c00:1c:a813:8512:c241",
      "2600:9000:24bb:a600:1c:a813:8512:c241"
    ]
  },
  {
    "name": "aws.amazon.com",
    "source": "drill",
    "failure": "deadline has elapsed"
  },
  {
    "name": "www.kame.net",
    "source": "Quad9 DNS",
    "records": [
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "Cloudflare DNS",
    "records": [
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "Google DNS",
    "records": [
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "OS resolution",
    "records": [
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "simulated nslookup",
    "records": [
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "resolv.conf server[127.0.0.1]",
    "failure": "Timed out after 7s"
  },
  {
    "name": "www.kame.net",
    "source": "resolv.conf server[10.7.0.1]",
    "records": [
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "A (dig)",
    "records": [
      "mango.itojun.org.",
      "210.155.141.200"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "AAAA (dig)",
    "records": [
      "mango.itojun.org.",
      "2001:2f0:0:8800:226:2dff:fe0b:4311",
      "2001:2f0:0:8800::1:1"
    ]
  },
  {
    "name": "www.kame.net",
    "source": "drill",
    "failure": "deadline has elapsed"
  }
]
```
