# Combina

Combina is a simple command-line utility written in Rust that combines the lines of two files in every possible combination, with an optional separator between the lines.

## Usage

```sh
combina [OPTIONS] <prefixesfile> <suffixesfile>
```

### Options

- `-s`, `--separador <str>`: Separator between the prefix and the suffix.

### Arguments

- `<prefixesfile>`: Path to the file containing prefix lines.
- `<suffixesfile>`: Path to the file containing suffix lines.

## Examples

### Simple usage

Combine lines from two files without a separator:

```sh
./combina file1.txt file2.txt
```

### Using a separator

Combine lines from two files with a hyphen as a separator:

```sh
./combina -s "-" file1.txt file2.txt
```

Or you can provide the separator option at the end:

```sh
./combina file1.txt file2.txt -s "-"
```

## Installation

### Building from Source

1. Clone the repository:

    ```sh
    git clone https://github.com/jordiroca/combina-rust.git
    cd combina-rust
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

3. The binary will be located in the `target/release` directory. You can run it directly:

    ```sh
    ./target/release/combina
    ```

## License

See the [LICENSE](LICENSE) file for details.
