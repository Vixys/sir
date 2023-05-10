# `sir` (Source Input Reader)

`sir` is a command line application written in Rust that aims to mimic the functionality of the `cat` command. It is an educational project and serves as a learning exercise for the Rust programming language.

## Features

- Displays the content of one or multiple files on the command line.
- Supports reading from standard input as well.
- Outputs the content of the files in sequential order.

## Requirements

- Rust (version 1.69.0 or higher)

## Installation

1. Clone the repository:
    ```shell
    git clone https://github.com/vixys/sir.git
    ```

2. Build the project:
    ```shell
    cd `sir`
    cargo build --release
    ```

3. Run `sir`:
    ```shell
    ./target/release/sir [options] [file1] [file2] ...
    ```

## Usage

- Run `sir` with one or multiple file arguments:
    ```shell
    sir file1.txt file2.txt
    ```

- To read from standard input:
    ```shell
    echo "Hello, sir!" | sir -
    ```

For more options and details, refer to the documentation (link_to_your_documentation).

## License

`sir` is licensed under the MIT License. See LICENSE file for more information.
