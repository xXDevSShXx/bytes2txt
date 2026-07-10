# bytes2txt

A Rust library for encoding and decoding byte arrays into human-readable, copiable text.

## Overview

`bytes2txt` converts arbitrary byte sequences into strings composed entirely of printable ASCII characters (space through tilde, characters 32–126), and decodes them back. This makes encoded data easy to copy, paste, and share across different systems without worrying about encoding issues.

The library uses a base-95 encoding scheme where each group of 4 input bytes is mapped to 5 printable characters, with a single padding digit appended to the end to indicate how many trailing zero bytes were added.

## Usage

Add `bytes2txt` to your `Cargo.toml`:

```toml
[dependencies]
bytes2txt = {git = "https://github.com/xXDevSShXx/bytes2txt.git"}
```

### Encoding

```rust
use bytes2txt::encode;

let data = b"Hello, world!";
let encoded = encode(data);
println!("{}", encoded); // Printable text string
```

### Decoding

```rust
use bytes2txt::decode;

let decoded = decode(encoded);
assert_eq!(decoded, Some(b"Hello, world!".to_vec()));
```

## How It Works

1. The input byte array is split into 4-byte chunks. If the length isn't a multiple of 4, zero-padding bytes are appended.
2. Each 4-byte chunk is converted into a `u32` and then mapped to a 5-character string using a 95-character reference set (all printable ASCII characters).
3. A final character is appended to indicate the number of padding bytes added during encoding.

Decoding reverses this process.

## Note

The comments, documentation, and tests for this project were written with the assistance of AI.

## Contributing

Contributions are welcome! If you'd like to improve `bytes2txt`, feel free to open an issue or submit a pull request. Here are some ways you can help:

- Report bugs or request features by opening an issue
- Improve documentation or add examples
- Add more test coverage
- Optimize the encoding/decoding algorithms
- Suggest new functionality (e.g., streaming support, different character sets)

When contributing, please keep the code style consistent with the existing codebase and make sure all tests pass before submitting.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
