#!/usr/bin/env python

from setuptools import setup

setup(
    name="breez_sdk",
    version="0.2.7.dev0",
    description="Python language bindings for the Breez SDK",
    include_package_data=True,
    packages=["breez_sdk"],
    package_dir={"breez_sdk": "./src/breez_sdk"},
    url="https://github.com/breez/breez-sdk",
    author="Breez <contact@breez.technology>",
    license="MIT",
    has_ext_modules=lambda: True,
)
