# Generic Rust crate template

This is my personalized template for Rust library crates.

It uses [porteurbars](https://github.com/softprops/porteurbars), so a `cargo
install porteurbars` is needed. Then run this to instantiate the template:

```
porteurbars jonas-schievink/crate-template my-crate
```

It will ask for the crate name as well as the "repo slug", which is the GitHub
username and repo name of a GitHub URL (eg. `jonas-schievink/xml-rpc-rs`).

There are still a few TODOs in `README.md`, `Cargo.toml` and `lib.rs` that need
to be filled out before releasing the crate.

## Features

The crates created from this template will start out with the following
features:

* CC-0 license
* Release automation for maximum laziness with
  [`cargo-release`](https://github.com/sunng87/cargo-release)
  * Changelog will be bumped to the next version
  * Version in `Cargo.toml` will be bumped
  * Usage example in readme will be updated to refer to the new crate version
  * `html_root_url` attribute in `lib.rs` will also be updated
* Test that makes sure crate version numbers in examples, `html_root_url` and
  `Cargo.toml` are in sync
* Changelog
* Readme featuring a crates.io, docs.rs, and Travis CI badge, as well as a basic
  usage example
* Travis CI configuration

Feel free to fork this and adapt to your needs.
