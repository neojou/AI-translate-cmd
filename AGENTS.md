# Handover Context: AI-translate-cmd

## 📋 Project Summary
This project is a Rust-based command-line tool intended for AI-powered translation (`AI-translate-cmd`). The tool takes an input language and a file path, translates the content to Traditional Chinese using an AI model, and outputs the result to stdout.

## ✅ Current Progress
- [x] Set up Rust project structure.
- [x] Implemented a core module `AITranslate` in `src/ai_translate.rs`.
- [x] `AITranslate` supports:
    - Initializing with a default language (`en`).
    - Dynamic language validation using an internal list (`allowed_langs`).
    - Dynamically adding new allowed languages at runtime.
- [x] Implemented idiomatic error handling using `thiserror`.
- [x] Implemented a CLI interface in `src/main.rs` using `std::env::args` with:
    - Robust argument count validation.
    - A `usage()` function for user guidance when arguments are incorrect.
- [x] Added unit tests for `AITranslate` language management in `src/ai_translate.rs`.

## 🧠 Knowledge & Implementation Details
- **Module System**: Used `mod ai_translate;` and `pub` keywords for cross-module access.
- **Error Handling**: Leveraged `thiserror` to define `TranslateError`, enabling better error propagation and semantic error messages.
- **CLI Logic**: Manual argument parsing via `std::env::args()` with strict checks for exactly 2 arguments (plus the executable name) to ensure correct usage.
- **Testing**: Comprehensive unit tests covering initialization, successful language setting, and failed language setting.

## 🚀 Next Steps (Handover Instructions)
The next focus is **Integration of Translation Logic**.

1.  **Define `Translator` Trait**: Define a generic translation interface to allow easy switching between different AI providers (e.g., OpenAI, Anthropic, or local models).
2.  **Implement OpenAI Provider**: Implement the first real translator using the OpenAI API. This will require adding `reqwest` and `serde` to `Cargo.toml`.
3.  **Extend `AITranslate` struct**: Update `AITranslate` to hold and utilize a specific `Translator` implementation.
4.  **Refine CLI**: Implement a way for users to provide their API Key, either via environment variables or command-line arguments.

**Environment Note**: The project is a standard Cargo workspace. Use `cargo run -- <lang> <file>` for testing.
