```
██╗██████╗ ██████╗ ██╗███████╗███████╗███████╗██████╗ ██╗   ██╗███████╗██████╗
██║██╔══██╗██╔══██╗██║██╔════╝██╔════╝██╔════╝██╔══██╗██║   ██║██╔════╝██╔══██╗
██║██████╔╝██║  ██║██║███████╗███████╗█████╗  ██████╔╝██║   ██║█████╗  ██████╔╝
██║██╔═══╝ ██║  ██║██║╚════██║╚════██║██╔══╝  ██╔══██╗╚██╗ ██╔╝██╔══╝  ██╔══██╗
██║██║     ██████╔╝██║███████║███████║███████╗██║  ██║ ╚████╔╝ ███████╗██║  ██║
╚═╝╚═╝     ╚═════╝ ╚═╝╚══════╝╚══════╝╚══════╝╚═╝  ╚═╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝
```

<!--Font: ANSI Shadow-->

ipdisserver: answer to [ipdisscan](../ipdisscan/README.md) with informations
useful to identify the system.

## About

ipdisserver is a service listening (by default) on `0.0.0.0:1901` for
requests sent by ipdisscan.

Requests are UDP packets containing an UTF-8 string used as signature.

If the received signature matches with the expected one (by default
`ipdisbeacon`), an answer is sent back to the client.

The answer contains informations about the system running ipdisserver (e.g.
hostname, IP addresses...), useful for identification.

Answers to a same client are subject to a rate limiting of one every 3s.

## Usage

Run `ipdisserver --help` for the CLI documentation.

### Environment variables

`RUST_LOG` changes logs verbosity.
E.g.:
`export RUST_LOG=debug`,
`export RUST_LOG=trace`.
[Full documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives).
