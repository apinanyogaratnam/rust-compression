# Compression

This is a simple compression cli tool written in Rust.

## Build

After this command is executed, the binary will be located in `target/release/compression`.

```sh
cargo build --release
```


## Usage

### Compress

Compress input file to output file.

```sh
cargo run <input_file_path> <output_file_path> --compress
```

Compress input file to output file with compression level [--fast, --best]

```sh
cargo run <input_file_path> <output_file_path> --fast
```

### Decompress

```sh
cargo run <input_file_path> <output_file_path> --decompress
```

### Help

```sh
cargo run _ _ --help
```

<!--
TODO: error handling inputted commands properly
-->
