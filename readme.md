## Description

chia-node-rs is a simple Rust library to interact with the Chia RPC API (fullnode/wallet/harvester). Currently, most of the RPC endpoints are available and some utility functions to encode and decode puzzle hashes.

## Installation

To use chia-node-rs in your Rust project, add the following to your Cargo.toml:

```toml
[dependencies]
chia-node-rs = { git = "https://github.com/mpwsh/chia-node-rs" }
```

## Usage

Please checkout a basic example at [./examples/get_balance.rs](examples/get_balance.rs)

Run the example:

```bash
cargo run --example get_balance
#output
Balance: 0.000000000001 XCH
```

There's another example to create a simple `CLI` called `chiactl` to get balance as well, that you can expand with more useful commands.

## Using example project `chiactl`

Update the contents of [./examples/ctlconfig.yaml](./examples/ctlconfig.yaml) to suit your needs, and then run the following to fetch the balance for a wallet address:

```bash
cargo run --example chiactl -- get balance <wallet_address> --config examples/ctlconfig.yaml
```

> You could also provide these values using command arguments, or specify the path of the config file in an arg as well. Use --help to get all available arguments.

More commands:

```bash
cargo run --example chiactl -- get blockchain --config examples/ctlconfig.yaml
cargo run --example chiactl -- get network --config examples/ctlconfig.yaml
cargo run --example chiactl -- get blockmetrics --config examples/ctlconfig.yaml
```

# Contributing

If you'd like to contribute to the development of `chia-node-rs`, feel free to submit a pull request or create an issue on the GitHub repository.
License

> This project is licensed under the MIT License. See the LICENSE file for more information.

# Credits

This is a fork of crate `chia-node-rs` from Mike Cronce: https://gitlab.cronce.io/foss/chia-node-rs

# Disclaimer

Please note that this library is provided "as-is" and the author is not responsible for any harm or misuse that may arise from the use of this library. Users are advised to use this library at their own risk and are encouraged to review the source code and understand the workings of the library before using it in any application or project.
