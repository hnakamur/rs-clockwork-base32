rs-clockwork-base32
===================

Another Rust implementation of [Clockwork Base32](https://gist.github.com/szktty/228f85794e4187882a77734c89c384a8) which is a Base32 variant inspired by Crockford's Base32.

# Install

Adding the following to the Cargo.toml in your project:

```
[dependencies]
clockwork_base32 = { git = "https://github.com/hnakamur/rs-clockwork-base32", tag = "0.1.0" }
```

# Usage

Encode bytes to a String:

```
use clockwork_base32 as base32;
let encoded = base32::encode_to_string(b"Hello, world!");
assert_eq!(&encoded, "91JPRV3F5GG7EVVJDHJ22");
```

Decode bytes to a String:

```
use clockwork_base32 as base32;
let decoded = base32::decode_to_string(b"91JPRV3F5GG7EVVJDHJ22")?;
assert_eq!(&decoded, "Hello, world!");
```

See [API documents](https://hnakamur.github.io/rs-clockwork-base32/) for details.

# License

Copyright (c) 2020 Hiroaki Nakamura

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# Prior work

* [woxtu/rust-clockwork-base32: Clockwork Base32 encoding/decoding](https://github.com/woxtu/rust-clockwork-base32)