# objq

## Depndencies

* rust-nightly (for syntax extension)
* cargo

## Build

`cargo build`

It will produce executable within `./target/debug` .

or,

`cargo build --release`

It will produce executable within `./target/release` .


## Usage

```sh
Usage:
  objq [--input=<format>] [--output=<format>] [--query=<query>]
```

```sh
echo '{"hoge": [3, [{"foo": "bar"}, 4]]}' | objq --query='.hoge[1][0]'
{
  "foo": "bar"
}⏎
```
