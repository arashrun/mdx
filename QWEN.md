# LibMdx - MDict Dictionary File Reader and Writer

## Project Overview

LibMdx is a comprehensive Rust library for reading, writing, and converting MDict dictionary files (.mdx, .mdd, .zdb formats). It provides high-performance parsing, full-text search capabilities, multiple compression/encryption methods, and support for various dictionary formats.

The library is organized into several key modules:
- **readers**: For reading MDX, MDD, and ZDB dictionary files
- **builder**: For creating and converting dictionary files
- **storage**: For core data structures and storage management
- **crypto**: For encryption operations
- **utils**: For helper functions and common operations

## Key Features

### Reading & Parsing
- **MDX Files**: Parse MDict dictionary files with HTML/Text/Binary content
- **MDD Files**: Extract resources (images, audio, fonts) from resource files
- **ZDB Files**: Read optimized MDict database format
- **Multiple Locales**: Support for various character encodings and locales
- **HTML Processing**: Automatic link rewriting and resource resolution

### Writing & Building
- **ZDB Creation**: Build optimized dictionary databases
- **Format Conversion**: Convert between MDX, MDD, and other formats
- **Custom Compression**: Choose from multiple compression algorithms (LZ4, LZMA, Bzip2, LZO, Zlib)
- **Encryption Support**: Secure dictionaries with various encryption methods

### Search & Indexing
- **Full-Text Search**: Build and query Tantivy-based search indexes
- **Fuzzy Matching**: Support for approximate keyword matching
- **Locale-Aware**: ICU collation for proper Unicode sorting

### Performance
- **Streaming I/O**: Efficient memory usage for large files
- **LRU Caching**: Smart caching for frequently accessed blocks
- **Parallel Processing**: Multi-threaded operations support
- **Optimized Indexing**: Fast binary search for key lookups

## Building and Running

### Prerequisites
- Rust 2024 edition (edition 2024)
- Cargo package manager

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mdx = { version = "0.5.0", features = ["icu"] }
```

### Feature Flags

- **`icu` (default)**: Use ICU4X for Unicode collation (pure Rust, recommended)
- **`rust-icu`**: Use rust_icu for Unicode collation (requires system ICU library)

```toml
# Use rust_icu instead of icu
mdx = { version = "0.5.0", features = ["rust-icu"] }

# Both features can be enabled
mdx = { version = "0.5.0", features = ["icu", "rust-icu"] }
```

### Key Commands

To build the project:
```bash
cargo build
```

To run tests:
```bash
cargo test
```

To build documentation:
```bash
cargo doc --open
```

## Development Conventions

### Error Handling
The library uses the `snafu` crate for error handling, providing automatic backtrace capture. All fallible operations return a `Result<T>` type where errors are represented by `ZdbError`.

Common error variants include:
- `ZdbError::Io`: I/O errors from file operations
- `ZdbError::CrcMismatch`: Data integrity check failures
- `ZdbError::InvalidDataFormat`: Malformed dictionary file data
- `ZdbError::InvalidParameter`: Invalid function parameters
- `ZdbError::KeyNotFound`: Dictionary key lookup failures
- `ZdbError::CompressionError`: Compression/decompression failures
- `ZdbError::ParserError`: XML/JSON parsing errors

### API Usage Examples

#### Reading an MDX Dictionary
```rust
use mdx::readers::MdxReader;
use url::Url;

fn main() -> mdx::Result<()> {
    // Open an MDX dictionary file
    let url = Url::parse("file:///path/to/dictionary.mdx")?;
    let mut reader = MdxReader::from_url(&url, "device_id")?;

    // Look up a word
    let key_index = reader.lookup("hello")?;
    let definition = reader.get_html(&key_index)?;
    println!("Definition: {}", definition);
    Ok(())
}
```

#### Building a ZDB Dictionary
```rust
use mdx::builder::{ZDBBuilder, BuilderConfig, SourceType};
use std::path::PathBuf;

fn main() -> mdx::Result<()> {
    let config = BuilderConfig::default();
    let mut builder = ZDBBuilder::from_config(config);

    // Build from MDX source
    builder.build(
        &PathBuf::from("source.mdx"),
        &PathBuf::from("output.zdb"),
        SourceType::Mdx,
        None,
    )?;
    Ok(())
}
```

### Code Structure

The library follows a modular design:
- **Main Entry Points**: Exposed through `lib.rs` which re-exports common types
- **Error Handling**: Centralized in `error.rs` using the snafu crate
- **Reading Operations**: In the `readers/` directory with dedicated modules
- **Building Operations**: In the `builder/` directory with various builder components
- **Storage Layer**: In the `storage/` directory with data structures for dictionary content
- **Cryptography**: In the `crypto/` directory for encryption/decryption
- **Utilities**: In the `utils/` directory for helper functions

### Versioning and Licensing

- **Version**: 0.5.0
- **License**: GNU Affero General Public License v3.0 (AGPL-3.0)
- **Keywords**: mdict, mdx, mdd, dictionary, reader, writer, parser, serialization
- **Categories**: encoding, parser-implementations

### Dependencies

The library uses many Rust crates for various functionality:
- **Parsing**: serde, serde-xml-rs, quick-xml, serde_json
- **Compression**: lz4, lzma-rs, bzip2, rust-lzo, flate2
- **Cryptography**: Various hash functions and encoding libraries
- **Text Processing**: ICU libraries for locale-aware operations
- **Search**: tantivy for full-text search capabilities
- **Utilities**: url, regex, uuid, chrono, lru cache, etc.

### Documentation

Documentation is available in the source code using Rustdoc format and can be generated with `cargo doc --open`.