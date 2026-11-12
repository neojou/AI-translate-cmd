# 📖 語言切換 / Language Switching / 言語切替え

[繁體中文 (Default)](./README.md) | [English](./README-en.md) | [日本語](./README-ja.md)

---

<a name="readme-english"></a>
# 📖 Description (English)

`AI-translate-cmd` is a Rust-based CLI tool that leverages AI models (like Ollama) to automatically translate text into Traditional Chinese.

## ✨ Key Features

- **Efficient Processing**: Supports chunk-based reading and translation for large files, minimizing memory usage.
- **Smart Segmenting**: Automatically detects sentence endings to ensure translations are never cut off mid-sentence.
- **Progress Tracking**: Displays real-time translation percentage in the terminal.
- **Multi-language Support**: Supports various input languages (e.g., English, Japanese).

## 🚀 Quick Start

### 1. Setup

Create your local configuration file:
```bash
cp config.example config.local
```

Edit `config.local` with your Ollama URL and model name.
**Recommended Model**: `translategemma:4b` (optimized for translation).

### 2. Prepare Test Files

Prepare English or Japanese sample files:
```bash
# English sample
echo "Hello, how are you?" > sample-en.txt

# Japanese sample
echo "こんにちは、元気ですか？" > sample-ja.txt
```

### 3. Run Translation

Use the following command:
```bash
# Translate English sample
cargo run -- en sample-en.txt

# Translate Japanese sample
cargo run -- ja sample-ja.txt
```

## 📂 Output Specification

Outputs are written to files instead of stdout:
- Default: `original-out.txt` (e.g., `sample-en-out.txt`).
- If the file exists, it increments: `original-out-1.txt`, `original-out-2.txt` ...

