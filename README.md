# subsee

CLI to inspect substrate metadata.

```shell
cargo install --git https://github.com/ascjones/subsee
```

```
Usage: subsee [--url <url>] [-p <pallet>] [-f <format>]

Inspect substrate metadata

Options:
  --url             the url of the substrate node to query for metadata
  -p, --pallet      the name of a pallet to display metadata for, otherwise
                    displays all
  -f, --format      the format of the metadata to display: `json`, `hex` or
                    `bytes`
  --help            display usage information
```

For, example:

```shell
subsee --format json --url https://rpc.polkadot.io/
```
