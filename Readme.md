# rifle

<div align="center">
  <img src="public/rifle_banner.png" height="128">
</div>

---

Rifle is a blocktime estimator for Starknet written in Rust. Made by learning purpose. This project is inspired by ([snipe](https://github.com/0xcacti/snipe)).

## Features

### Convert blocknumber to time.

This feature relies on the `starknet_getBlockWithTxHashes` endpoint. Get timestamp from blocknumber through this endpoint. If you want to return in unix format, add `-f unix` command.

### Convert time to blocknumber.

This feature is using binary search algorithm to search for the nearest blocknumber with given target timestamp.

### Select network want to query.

You can use options to choose. `-n mainnet` will allow you to query from Starkent mainnet, `-n goerli` will allow you to query from Starkent goerli, `-n goerli2` will allow you to query from Starkent goerli2.

## Usage

```
Usage: rifle [OPTIONS] [COMMAND]

Commands:
  --to-time   Convert blocknumber to time. [aliases: btt]
  --to-block  Convert time to blocknumber. [aliases: ttb]
  help        Print this message or the help of the given subcommand(s)

Options:
  -n, --network <NETWORK>     Network: [mainnet/goerli/goerli2]
  -f, --format <TIME_FORMAT>  The format to use time
  -h, --help                  Print help
```

### `--to-block`

```
Convert time to blocknumber.

Usage: rifle --to-block <TIME>

Arguments:
  <TIME>  The time to convert

Options:
  -h, --help  Print help
```

### `--to-time`

```
Convert blocknumber to time.

Usage: rifle --to-time <BLOCK_NUMBER>

Arguments:
  <BLOCK_NUMBER>  The blocknumber to convert

Options:
  -h, --help  Print help
```

## Improvement

- [ ] timezone support
- [ ] RPC support ( gateway will be deprecated )
- [ ] algorithm upgrade?
