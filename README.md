# Patronus
Patronus is library that unifies APIs of various grammar checkers to simplify their integration into applications.

## Structure
* `patronus` – main codebase, implemented as a Rust library
* `patronus-capi` – C API, intended to be used by applications written in other languages
* `patronus-provider` – Rust library providing data types for implementing custom providers
* `providers` – default providers

## Installation
Before starting the build `PROVIDER_LOCATION` environment variable must be set to the path where providers will be looked up; on Linux it will probably be `/usr/lib/patronus`. This path will be compiled into the library.
