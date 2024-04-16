# Code Formatter for LLM Input 🛠️

The Code Formatter is a powerful command-line tool developed in Rust, designed to parse and format code from specific programming languages. This formatted code is then ready to be fed into Large Language Models (LLMs) for further processing. The tool emphasizes accuracy and efficiency in preparing code data for machine learning applications.

## Features 🌟

- **Targeted Language Parsing:** Focuses on specific programming languages, ensuring that the parsing rules are optimized for accuracy.
- **Formatting for LLMs:** Prepares code in a format that is ideal for ingestion by language models, facilitating better understanding and processing.
- **Command-Line Flexibility:** Offers customizable options to handle different formats and requirements directly from the command line.

## Prerequisites 📋

Before you install and run this project, make sure you have the following installed:
- Rust Programming Language ([official installation guide](https://www.rust-lang.org/tools/install))
- Cargo (Rust's package manager, included with Rust installations)
- Tokio (for asynchronous operations, included in the project's dependencies)

## Installation 🔧

To install the project parser, follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/charslee013/code_parser.git
   cd code_parser
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. The executable will be located in `./target/release/`. You can run it directly or add it to your PATH for easier access.

## Usage 🚀

To operate the Code Formatter, use the following command format in the directory containing the executable:

```bash
cargo run -- [OPTIONS]
```

### Full Help Context

Detailed command-line options are provided to customize the tool's operation:

```
❯ cargo run -- --help
Parses code files based on specified languages and formats output.

USAGE:
    code_parser [OPTIONS]

OPTIONS:
    -h, --help                   Print help information 📖
    -l, --language <language>    Specifies the programming language file extension to parse 🛠️
    -o, --output <output>        Optional output file to store the results 🗂️
    -p, --path <path>            The root path to start scanning for code files 📁
    -V, --version                Print version information ℹ️
        --with-test              Includes files and folders containing 'test' in their names 🔍
```

### Examples

1. Parse Python files in the current directory and print formatted output:
   ```bash
   cargo run -- -l py
   ```

2. Parse JavaScript files in a specified directory and save output to a file:
   ```bash
   cargo run -- -l js -p /path/to/directory/ -o output.txt
   ```

## Configuration 🔧

You can configure the project parser by editing the source code:

- Modify `src/main.rs` to change default settings or add new command-line options.
- Update `src/content_formatter.rs` to adjust the formatting style based on language.
- Edit `src/file_walker.rs` to change the rules for file inclusion.

## Contributing 🤝

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License 📄

Distributed under the MIT License. See `LICENSE` for more information.

## Contact 📧

charslee013@gmail.com 


---

