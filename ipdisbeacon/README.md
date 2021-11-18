```
██╗██████╗ ██████╗ ██╗███████╗██████╗ ███████╗ █████╗  ██████╗ ██████╗ ███╗   ██╗
██║██╔══██╗██╔══██╗██║██╔════╝██╔══██╗██╔════╝██╔══██╗██╔════╝██╔═══██╗████╗  ██║
██║██████╔╝██║  ██║██║███████╗██████╔╝█████╗  ███████║██║     ██║   ██║██╔██╗ ██║
██║██╔═══╝ ██║  ██║██║╚════██║██╔══██╗██╔══╝  ██╔══██║██║     ██║   ██║██║╚██╗██║
██║██║     ██████╔╝██║███████║██████╔╝███████╗██║  ██║╚██████╗╚██████╔╝██║ ╚████║
╚═╝╚═╝     ╚═════╝ ╚═╝╚══════╝╚═════╝ ╚══════╝╚═╝  ╚═╝ ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝
```

<!--Font: ANSI Shadow-->

ipdisbeacon: answer to [ipdisscan](../ipdisscan/README.md) with informations
useful to identify the system.

## About

ipdisbeacon is a service listening (by default) on `0.0.0.0:1901` for
requests sent by ipdisscan.

Requests are UDP packets containing an UTF-8 string used as signature.

If the received signature matches with the expected one (by default
`ipdisbeacon`), an answer is sent back to the client.

The answer contains informations about the system running ipdisbeacon (e.g.
hostname, IP addresses...), useful for identification.

Answers to a same client are subject to a rate limiting of one every 3s.

## Usage

Run `ipdisbeacon --help` for the CLI documentation.

### Environment variables:

[`RUST_LOG`](https://docs.rs/env_logger/0.9.0/env_logger/#enabling-logging)
changes logs verbosity.
E.g.:
`export RUST_LOG=debug`,
`export RUST_LOG=trace`
