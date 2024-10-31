## byte_converter
byte_converter is a Rust library for converting between bytes, kilobytes, and megabytes.

## Installation
Add byte_converter to your Cargo.toml:
[dependencies]
byte_converter = "0.1.0"

## Usage
Then, you can use the conversion functions:
```rust
use byte_converter::{bytes_to_kb, kb_to_mb};

fn main() {
    let kb = bytes_to_kb(2048);
    println!("2048 bytes is {} KB", kb);

    let mb = kb_to_mb(2048.0);
    println!("2048 KB is {} MB", mb);
}
```

## License
This project is licensed under the MIT License.

## Author
Ben Santora (<bensatlantik@gmail.com>)