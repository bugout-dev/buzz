from setuptools import find_packages, setup

long_description = ""
with open("../README.md") as ifp:
    long_description = ifp.read()

setup(
    name="buzz",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[],
        description="Buzz: Tags pattern matching language.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    author="Bugout.dev",
    author_email="engineering@bugout.dev",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Programming Language :: Python",
        "License :: OSI Approved :: Apache Software License",
        "Topic :: Software Development :: Libraries",
    ],
    url="https://github.com/bugout-dev/buzz",
}