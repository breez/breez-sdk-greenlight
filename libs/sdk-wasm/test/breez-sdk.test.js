const { bindings } = require("@breez/breez-sdk");

test("Test", async () => {
  const sdk = await bindings.breez_sdk({
    imports: {
      wasi_snapshot_preview1: {
        fd_write: function () {},
        environ_get: function () {},
        environ_sizes_get: function () {},
        proc_exit: function () {},
      },
    },
  });
  const seed = sdk.mnemonicToSeed("cruise clever syrup coil cute execute laundry general cover prevent law sheriff");
  console.log(`seed: ${seed}`)
  expect(1 + 2).toBe(3);
});
