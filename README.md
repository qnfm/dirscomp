# Directory Comparison Tool

This tool compares files between two directories, identifying unique files in each directory and files that have identical content but are located in different directories. It uses the BLAKE3 hashing algorithm for efficient and secure file content comparison.

## Prerequisites

To use this tool, you must have Rust and Cargo installed on your machine. If you haven't installed Rust and Cargo yet, you can follow the instructions [here](https://www.rust-lang.org/tools/install).

## Installation

1. Clone this repository to your local machine using git:

    ```sh
    git clone https://github.com/yourusername/dirscomp.git
    cd dirscomp
    ```

2. Build the project with Cargo:

    ```sh
    cargo build --release
    ```

   The `--release` flag builds the project in release mode, which optimizes the binary for performance.

## Usage

After building the project, you can run the tool using Cargo or directly from the target directory. To use Cargo, navigate to the project root directory and run:

```sh
cargo run -- <path_to_directory1> <path_to_directory2>
```

Alternatively, you can run the compiled binary in target/release directly:

```sh
dirscomp <path_to_directory1> <path_to_directory2>
```


Replace `<path_to_directory1>` and `<path_to_directory2>` with the paths to the directories you wish to compare.

## Features

- **Fast Comparison**: Utilizes the BLAKE3 hashing algorithm for fast and secure content comparison.
- **Cross-platform**: Runs on any platform that Rust supports, including Windows, macOS, and Linux.
- **Easy to Use**: Simple command-line interface for straightforward operation.

## Contributing

Contributions are welcome! If you have suggestions for improvements or encounter any issues, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
