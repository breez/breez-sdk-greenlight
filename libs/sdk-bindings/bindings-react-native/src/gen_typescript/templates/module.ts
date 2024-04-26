import { NativeModules, Platform, EmitterSubscription, NativeEventEmitter } from "react-native"

const LINKING_ERROR =
    `The package 'react-native-breez-sdk' doesn't seem to be linked. Make sure: \n\n` +
    Platform.select({ ios: "- You have run 'pod install'\n", default: "" }) +
    "- You rebuilt the app after installing the package\n" +
    "- You are not using Expo managed workflow\n"

const BreezSDK = NativeModules.RNBreezSDK
    ? NativeModules.RNBreezSDK
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR)
              }
          }
      )

const BreezSDKEmitter = new NativeEventEmitter(BreezSDK)
{%- import "macros.ts" as ts %}
{%- include "Types.ts" %}
{% include "Helpers.ts" %}
{% for func in ci.function_definitions() %}
{%- if func.name()|ignored_function == false -%}
{%- include "TopLevelFunctionTemplate.ts" %}
{% endif -%}
{% endfor -%}
{%- include "Objects.ts" %}
