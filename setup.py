#!/usr/bin/env python

from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="hashcore",
    version="1.0",
    rust_extensions=[RustExtension(
        "hashcore.hashcore", binding=Binding.PyO3)],
    packages=["hashcore"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)
