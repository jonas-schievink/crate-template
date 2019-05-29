//! TODO: Write crate docs

#![doc(html_root_url = "https://docs.rs/{{CRATE}}/0.0.0")]

// Deny a few warnings in doctests, since rustdoc `allow`s many warnings by default
#![doc(test(attr(deny(unused_imports, unused_must_use))))]

#![warn(missing_debug_implementations, rust_2018_idioms)]
