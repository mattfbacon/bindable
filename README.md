# Bindable

This crate provides the `BindableAddr` type, which is an abstraction over an address that a server can bind to: either an IP address for TCP or a path for a Unix socket.

There is also a `BindBindableExt` trait, that is used for server integrations.

The crate also includes some integrations with other common crates, gated by features.

## Features

### `with-serde` (enabled by default)

Implements `Serialize` and `Deserialize` for `BindableAddr`. The serialized representation is a string with an optional (defaults to TCP) protocol prefix, such as `tcp://` or `unix://`.

### `with-actix`

Implements `BindBindableExt` for `HttpServer`.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
