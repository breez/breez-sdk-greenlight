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

Once the CLI is started, the first thing we need to do is create a node and there are three ways to do it:
* **With an invite code** - You can use a one-time invite code to register a new node. Use the command `connect -i <invite_code>` with your invite code.
* **With a partner certificate** - You can use a Greenlight Partner Certificate to register a new node. Use the command `connect -c <partner_cert> -k <partner_key>` with your Greenlight Partner Certificate and Key. This Partner Cerificate can be reused to register multiple nodes.
* **With a mnemonic phrase** - Create a `phrase` file in the data directory containing the BIP39 mnemonic phase of an existing node. Use the command `connect` to recover the node. 

The node credentials and the BIP39 mnemonic seed are saved in the data directory (`creds` and `phrase` files). Once the node is created we can start to send commands to the node. Press `Enter` to see a list of available commands. When restarting the CLI, use `connect` to reconnect to the node and start sending commands to it.

Please note that the CLI is very simple and only intends to demonstrate the usage and investigate issues that are hard to debug on mobile platforms.
