#!/usr/bin/env python

from setuptools import setup

LONG_DESCRIPTION = """# Breez SDK
Python language bindings for the [Breez SDK](https://github.com/breez/breez-sdk).

## Installing

```shell
pip install breez_sdk
```

## Docs

See [sdk-doc.breez.technology](https://sdk-doc.breez.technology).
"""

setup(
    name="breez_sdk",
    version="0.2.7.dev9",
    description="Python language bindings for the Breez SDK",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    packages=["breez_sdk"],
    package_dir={"breez_sdk": "./src/breez_sdk"},
    include_package_data=True,
    package_data={"breez_sdk": ["*.dylib", "*.so"]},
    url="https://github.com/breez/breez-sdk",
    author="Breez <contact@breez.technology>",
    license="MIT",
    has_ext_modules=lambda: True,
)
