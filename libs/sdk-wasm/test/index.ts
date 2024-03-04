const { bindings } = require("@breez/breez-sdk");

async function main() {
  console.log("Start")
  const sdk = await bindings.breez_sdk();
  const seed = sdk.mnemonicToSeed(
    "cruise clever syrup coil cute execute laundry general cover prevent law sheriff"
  );
  console.log(`seed: ${seed}`);
}

main();
