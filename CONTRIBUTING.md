# Contribution guide

Contributions are always welcome. The codebase is maintained using the "contributor workflow" where anyone can contribute proposals using [pull requests](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).

The contribution workflow is as follows:

1. Fork the repository.
2. Create a topic branch.
3. Commit changes to the branch.
4. Push changes to the fork.
5. Create a pull request to merge the branch of the fork into this repository.
6. If you had someone specifically in mind, ask them to review the pull request. 
Otherwise, just wait for a code review: most members with merge permissions receive notifications for newly created pull requests.
7. Address review comments, if any.
8. Merge and submit the pull request. 
If you don't have merge permissions, a reviewer will do it for you.

> **NOTE:** Before starting a particular feature, please make sure to check if a related [issue](https://github.com/breez/breez-sdk-greenlight/issues) already exists and is not already assigned to somebody. If there is no issue created, open up a [new issue](https://github.com/breez/breez-sdk-greenlight/issues/new) if you want to first discuss and to agree on the optimal way of solving a particular problem.
>
> If you don't know where to start, look for issues labeled with [good first issue](https://github.com/breez/breez-sdk-greenlight/labels/good%20first%20issue).

## Formatting, generating code and testing

The Rust source code should be formatted according to `cargo fmt` and have no linting errors from `cargo clippy`. Any changes to public facing functions or structs should be adequately documented according to [rustdoc](https://doc.rust-lang.org/rustdoc/index.html#using-rustdoc-with-cargo). Comments on code should be written clearly and concisely, and written in English.

If there are any changes to the SDK's interface, they also need to be updated in the `sdk-bindings` crate. The SDK uses [UniFFI](https://github.com/mozilla/uniffi-rs) to generate the bindings code for several different languages. Add the changes needed to the [uniffi_bindings.rs](libs/sdk-bindings/src/uniffi_binding.rs) Rust interface and also update the [UDL interface](libs/sdk-bindings/src/breez_sdk.udl) file. Whenever changes are made to the `sdk-bindings`, the [Flutter](libs/sdk-flutter) and [React Native](libs/sdk-react-native) bindings code needs to be regenerated and included in your pull request. You can regenerate these from the project root using `make codegen`. Please see the prerequisites for [Flutter](libs/sdk-flutter/README.md) and [React Native](libs/sdk-bindings/bindings-react-native/README.md) if this is your first time generating code.

Please adequately test your code using the existing tests and write additional tests for new features. You can run the tests from the project root using `make test`. You can also make use of the [CLI](tools/sdk-cli) to test changes while developing your feature.

## Pull requests

A pull request contains one or more related git commits. Please, do not bundle independent and unrelated commits into a single pull request.

Just like a git commit message, a pull request consists of a subject and a body. If a pull request contains only one git commit, set its title and description to the commit's subject and the body. Otherwise, make an overall summary of what all the commits accomplish together, in a way similar to a commit message. If you are addressing an existing issue, please reference it in the pull request body.

See the following docs on creating [github pull requests](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/creating-a-pull-request).

Before creating a pull request, please make sure all tests and lints pass locally on your machine. In a case where a pull request isn't ready for a regular code review and you're just looking for some early feedback,
it's ok to let some tests fail but please mention it explicitly in the request description. 

#### Checklist
You can run all the following commands in the project root directory to comply to the contribution guide:
- Format and lint the source code: `make fmt clippy`
- Generate code Flutter and React Native bindings: `make codegen`
- Test the code: `make test`

You can run all these in one command: `make all`
