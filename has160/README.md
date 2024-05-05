# RustCrypto: HAS-160

[![crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
![Apache2/MIT licensed][license-image]
![Rust Version][rustc-image]
[![Project Chat][chat-image]][chat-link]
[![Build Status][build-image]][build-link]

Pure Rust implementation of the [HAS-160] cryptographic hash algorithm.

## 🚨 Warning: Cryptographically Broken! 🚨

The SHA-1 hash function should be considered cryptographically broken and
unsuitable for further use in any security critical capacity, as it is practically vulnerable to chosen-prefix collisions.

We provide this crate for legacy interoperability purposes only.

## Examples

### One-shot API

```rust
use hex_literal::hex;
use has160::{Has160, Digest};

let result = Has160::digest(b"hello world");
assert_eq!(result, hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
```

### Incremental API

```rust
use hex_literal::hex;
use has160::{Has160, Digest};

let mut hasher = Has160::new();
hasher.update(b"hello world");
let hash = hasher.finalize();

assert_eq!(hash, hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"));
```

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[//]: # (general links)

[SHA-1]: https://en.wikipedia.org/wiki/HAS-160