# objq

## Depndencies

* rust-nightly (for syntax extension)
* cargo

## Build

`cargo build --release`

Static linked executable binary is available at https://github.com/rail44/objq/releases

## Usage

```sh
Converter/Querier for data format

Usage: objq [options]
       objq (-h | --help)

Options:
    -i <format>    (json | yaml | msgpack | ini | properties)
    -o <format>    (json | json:pretty | yaml | msgpack)
    -q <query>     Query for data, with format like '.foo.bar[0]'
    -f <file>      Read from file instead of STDIN
    -h, --help     Display this message
```

## Example

```sh
echo '{"hoge": [3, [{"foo": "bar"}, 4]]}' | objq -o json:pretty -q .hoge[1][0]
{
  "foo": "bar"
}⏎
```
