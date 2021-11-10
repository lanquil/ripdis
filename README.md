```
██████╗ ██╗██████╗ ██████╗ ██╗███████╗
██╔══██╗██║██╔══██╗██╔══██╗██║██╔════╝
██████╔╝██║██████╔╝██║  ██║██║███████╗
██╔══██╗██║██╔═══╝ ██║  ██║██║╚════██║
██║  ██║██║██║     ██████╔╝██║███████║
╚═╝  ╚═╝╚═╝╚═╝     ╚═════╝ ╚═╝╚══════╝
```

<!--Font: ANSI Shadow-->

Use UDP broadcasting to find devices and report system informations.

This project include a server part, `ipdisbeacon` and a client part,
`ipdisscan`.

`ipdisbeacon` listen by default on port 1901,
`ipdisscan` broadcast by default from port 1902.

Both tools have CLI options to change the default behaviour.

See [ipdisbeacon README](ipdisbeacon/README.md)
and [ipdisscan README](ipdisscan/README.md)
for more informations.
