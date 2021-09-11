# csv-to-json

A simple CSV to JSON command line serializer tool.

## Arguments

`-H` `-help`

- Show help

`-I` `-input`

- Path to the input CSV file

`-O` `-output`

- Path to the output JSON file

`-C` `-config`

- Path to the config JSON file

## Config

A config JSON file

```json
{
  "input": "Path to the input CSV",
  "output": "Path to the output CSV",
  "schema": {
    "<JSON_KEY>": {
      "type": "<string|array>",
      "alias": "CSV header",
      "regex": "Regular expression to match CSV header",
      "header": "Boolean flag to use CSV header as value",
      "properties": {
        // Only valid for type "array"
        "<JSON_KEY>": {
          "type": "<string>"
          // ... Same values as above
        }
      }
    }
  }
}
```

## Development

Run with arguments

`cargo run -- -I ./input.csv -O ./output.json`
