# sdk-cli

A simple cli tool that sends commands to the sdk.

## Run

To use the current directory to save node data:
```
cargo run
```

or to specify a custom data directory:
```
cargo run -- --data_dir <data directory>
```

Once the CLI is started, the first thing we need to do is create a node and there are three ways (commands) to do it:
* `register_node` - Registers a new node in the cloud. The node credentials and the BIP39 mnemonic seed are saved in the data directory (`creds` and `phrase` files). Use this as the first command if you have no seed and no wallet credentials.
* `recover_node` - Recovers a node from BIP39 mnemonics. Use this if you already have a seed (`phrase` file) and you wish to start using the node associated with that seed.
* `init` - Initializes an existing node from credentials. Use this if you already have the node credentials (`creds` file) and wish to start using the associated node.

Once the node is created we can start to send commands to the node. Press `Enter` to see a list of available commands.

Typically, on first run one would use `register_node` as the first command, and on subsequent runs one would start with `init`.

Please note that the CLI is very simple and only intends to demonstrate the usage and investigate issues that are hard to debug on mobile platforms.
