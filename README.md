# csv2dynamodb

[![Rust](https://github.com/poad/csv2dynamodb/actions/workflows/build.yml/badge.svg)](https://github.com/poad/csv2dynamodb/actions/workflows/build.yml)

import csv to AWS DynamoDB.

## usage

```sh
csv2dynamodb --file-path <file-path> --table <table>
```

```sh
   --file-path value    file to import e.g ./tablename.csv (required)
   --table value        target dynamo db tabe name (required)
```

### example

```sh
csv2dynamodb --file-path ./test_data.csv --table sample-table
```

## How to specify data types in DynamoDB

It is specified in the CSV header by writing (). e.g. price(N)
If not specified, it will be imported as S (string).

```csv
name(S),price(N),visible(BOOL)
"abc",100,true
```

## License

[The MIT License](LICENSE)
