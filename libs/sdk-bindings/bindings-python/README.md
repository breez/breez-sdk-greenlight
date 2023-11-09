# SDK bindings for Python

## Usage

```
pip install breez_sdk
```

``` python
import breez_sdk

# see https://sdk-doc.breez.technology
```

## Python Package

Run the GitHub workflow 'Publish Python Bindings' when creating a new release of Breez SDK.
It will create wheels for the following Python versions and Platforms and upload them to [PyPi.org](https://pypi.org/project/breez-sdk/).

### Supported Wheels

|                 | GNU/Linux amd64 | GNU/Linux arm64v8 | macOS x86_64 | macos aarch64 |
|-----------------|-----------------|-------------------|--------------|---------------|
| **Python 3.8**  | ✅               | ✅                 | ✅            | ✅             |
| **Python 3.9**  | ✅               | ✅                 | ✅            | ✅             |
| **Python 3.10** | ✅               | ✅                 | ✅            | ✅             |

## Building Manually

To build the package manually inside this directory use the supplied `makefile`:

``` shell
make darwin # builds the package for macos
make linux # builds the package for linux
```

The artifacts will be placed in `src/breez_sdk/`.
