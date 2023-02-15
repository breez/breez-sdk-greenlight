import { NativeModules, Platform } from 'react-native';

const LINKING_ERROR =
  `The package 'react-native-breez-sdk' doesn't seem to be linked. Make sure: \n\n` +
  Platform.select({ ios: "- You have run 'pod install'\n", default: '' }) +
  '- You rebuilt the app after installing the package\n' +
  '- You are not using Expo managed workflow\n';

const BreezSDK = NativeModules.BreezSDK
  ? NativeModules.BreezSDK
  : new Proxy(
      {},
      {
        get() {
          throw new Error(LINKING_ERROR);
        },
      }
    );

export async function mnemonicToSeed(phrase: string): Promise<Uint8Array> {
  const response = await BreezSDK.mnemonicToSeed(phrase);
  console.log(JSON.stringify(response))

  return response;
}
