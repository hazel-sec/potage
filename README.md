Potage
======

a crate for hashdigest calculation

* support sha256, sha1, md5
* from file / reader
* async support

usage
-----

```
[dependencies]
potage = "0.1"
```

### from file

```rust
use potage::HashDigest;

// from file
let hd = HashDigest::from_file("/path/to/target")?;

// from file async
let hd = HashDigest::from_file_async("/path/to/target").await?;

println!("sha256: {}\nsha1: {}\nmd5: {}", hd.sha256(), hd.sha1(), hd.md5());
```

### from reader

```rust
use std::io::copy;

let mut w = potage::Writer::new();
let resp = reqwest::get("http://example.com").await?;

//  you can use tokio::io::copy if tokio::AsyncReader implemented for the reader.
std::io::copy(resp, w)?;

let hd = w.digest();

println!("sha256: {}\nsha1: {}\nmd5: {}", hd.sha256(), hd.sha1(), hd.md5());
```
