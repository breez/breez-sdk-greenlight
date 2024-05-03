# Breez SDK - CLI

A simple cli tool that sends commands to the sdk. It is intended to demonstrate the usage and investigate issues that are hard to debug on mobile platforms.

## Run

To use the current directory to save node data:
```
cargo run
```

or to specify a custom data directory:
```
cargo run -- --data_dir <data directory>
```

Once the CLI is started, the **first thing** we need to do is to set the API key `set_api_key <api_key>`. You only have to set the API key one time per data directory used.

### Recovering an existing node

In the data directory create a `phrase` file and paste inside the BIP39 mnemonic seed of the existing node. Now you can use `connect` to recover the node and start sending commands to it.

### Registering a new node

There are two ways to create a new node:
* **With an invite code** - You can use a one-time invite code to register a new node. Use the command `connect -i <invite_code>` with your invite code.
* **With a partner certificate** - You can use a Greenlight Partner Certificate to register a new node. Use the command `connect -c <partner_cert> -k <partner_key>` with your Greenlight Partner Certificate and Key. This Partner Certificate can be reused to register multiple nodes.

The BIP39 mnemonic seed is saved in the data directory in a `phrase` file. Once the node is created we can start to send commands to the node. Press `Enter` to see a list of available commands. 

When restarting the CLI, use `connect` to reconnect to the node and start sending commands to it.

## Debug

You can debug the current state of the node / SDK in several ways:

* **Node state** - Check the node and LSP information using the commands `node_info` and `lsp_info`.
* **Health check** - Check the current service status of SDK service providers using the command `service_health_check`.
* **Diagnostic data** - Check the result of `generate_diagnostic_data` to view the state of payments and swaps.
* **Log file** - Check the log entries for the node and SDK in the `sdk.log` file in the data directory.
