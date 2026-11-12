# 📖 語言切換 / Language Switching / 言語切替え

[繁體中文 (Default)](./README.md) | [English](./README-en.md) | [日本語](./README-ja.md)

---

# 📝 說明 (Traditional Chinese)

`AI-translate-cmd` 是一個基於 Rust 開發的命令列工具，利用 AI 模型（如 Ollama）將文本自動翻譯為繁體中文。

## ✨ 功能亮點

- **高效處理**：支援大型檔案的分段讀取與翻譯，不會佔用過多記憶體。
- **智能分段**：自動偵測句末標點符號，確保翻譯不會在句子中間切斷。
- **進度顯示**：在終端機即時顯示翻譯百分比。
- **多語言支援**：支援多種輸入語言（如英文、日文等）。

## 🚀 快速開始

### 1. 設定環境

建立您的本地設定檔：
```bash
cp config.example config.local
```

編輯 `config.local` 並填入您的 Ollama URL 與模型名稱。
**推薦使用模型**：`translategemma:4b` (針對翻譯任務優化)。

### 2. 準備測試檔案

準備英文或日文的測試樣本：
```bash
# 英文樣本
echo "Hello, how are you?" > sample-en.txt

# 日文樣本
echo "こんにちは、元気ですか？" > sample-ja.txt
```

### 3. 執行翻譯

使用以下指令進行翻譯：
```bash
# 翻譯英文樣本
cargo run -- en sample-en.txt

# 翻譯日文樣本
cargo run -- ja sample-ja.txt
```

## 📂 輸出說明

程式不會將結果印在螢幕上，而是會自動寫入檔案：
- 預設輸出：`原檔名-out.txt` (例如 `sample-en-out.txt`)。
- 若檔案已存在，會自動遞增編號：`原檔名-out-1.txt`, `原檔名-out-2.txt` ...

