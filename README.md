# subsee

CLI to inspect substrate metadata.

`cargo install --git https://github.com/ascjones/subsee`

```
Usage: subsee [--url <url>]

Inspect substrate metadata

Options:
  --url             the url of the substrate node to query for metadata
  --help            display usage information
```

Currently just outputs the metadata of a substrate node as json.

Expects a substrate node to be running locally with the default endpoint `"http://localhost:9933"`.


