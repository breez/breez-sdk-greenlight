# Breez SDK React Native Example

## Prerequisites
You need to set your Breez API key in the relevant places before running the example.
Either Find and Replace `INSERT_YOUR_BREEZ_API_KEY` with your Breez API key, 
or replace `INSERT_YOUR_BREEZ_API_KEY` in the following files:
* `android/app/gradle.properties`
* `ios/Secrets.xconfig`

## Build and Run

### Android

```bash
# In directory sdk-react-native
make android
yarn

# In the example directory
cd example
rm -rf node_modules
yarn
yarn android
```

#### Android Troubleshooting

* Before running `yarn android`, stop any `Metro` instances that may be running.
* If you get the error
  ```
  Failed to load dynamic library 'libbreez_sdk_core.so': dlopen failed: cannot locate symbol "__extenddftf2"
  ```
  that is likely due to a dependency issue affecting x86_64 images. Try to run the app on a physical Android device or on a x86 image.

### iOS

```bash
# In directory sdk-react-native
make ios
yarn

# In the example directory
cd example
rm -rf node_modules
yarn
yarn pods
yarn ios
```

## Development

To develop locally along with the SDK, you need to switch the podspec from the cocoapods dependencies to the local dependencies.
```
yarn install
mv node_modules/@breeztech/react-native-breez-sdk/breez_sdk.podspec node_modules/@breeztech/react-native-breez-sdk/breez_sdk.podspec.prod
mv node_modules/@breeztech/react-native-breez-sdk/BreezSDK.podspec.dev node_modules/@breeztech/react-native-breez-sdk/BreezSDK.podspec
yarn pods
yarn ios
```
