# AI-translate-cmd

`AI-translate-cmd` is a lightweight, Rust-based command-line tool designed for AI-powered translation. It allows users to translate files from a specified source language into Traditional Chinese using advanced AI models.

## ✨ Features

- **AI-Powered**: Leverages large language models (LLMs) for high-quality translations.
- **Dynamic Language Support**: Easily add and validate supported source languages.
- **Extensible Architecture**: Designed to support multiple AI providers (e.g., OpenAI, Anthropic) through a unified trait system.
- **CLI-Friendly**: Simple and intuitive command-line interface.

## 🚀 Quick Start

### Prerequisites

- [Rust and Cargo](https://rustup.rs/)

### Installation

Clone the repository and install locally:

```bash
git clone https://github.com/your-username/AI-translate-cmd.git
cd AI-translate-cmd
cargo install --path .
```

### Usage

To translate a file, specify the source language and the file path:

```bash
# Syntax: ai-translate-cmd <input_lang> <input_file>
# Example:
ai-translate-cmd en sample.txt
```

## 🛠️ Development

To run the project without installing:

```bash
cargo run -- <lang> <file>
```

To run tests:

```bash
cargo test
```

## 📄 License

This project is licensed under the [MIT License](LICENSE).
