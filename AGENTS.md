# AI-translate-cmd

給之後接手的 AI（Grok、Cursor 或其他 agent）。以這份和目前原始碼為準。2026-11-15 刪掉的舊版寫的是過期交接：結果印到 stdout、還能動態加語言、下一步才接 OpenAI。那些都不是現在的程式。

## 這是什麼

單一 Cargo package，不是 workspace。二進位名稱是 `ai_translate`。

命令列工具。讀一個 UTF-8 文字檔，透過本機 Ollama 的 OpenAI 相容介面翻成台灣繁體中文，把結果寫進另一個檔案。翻譯正文不印到 stdout。stdout 只有進度、狀態與錯誤。

用法必須剛好兩個參數：

```bash
cargo run -- <input_lang> <input_file>
cargo run -- en sample-en.txt
cargo run -- ja sample-ja.txt
```

允許的 `input_lang` 只有 `en` 與 `ja`。程式把它們對成提示詞裡的 `English`、`Japanese`。其他代碼會印錯誤與用法後結束。

輸出寫在輸入檔同一目錄：`原檔名-out.副檔名`。已存在就改成 `原檔名-out-1.副檔名`、`原檔名-out-2.副檔名`。

參數數量不對、語言不允許、翻譯失敗時，`main` 都是印訊息後正常返回。結束碼目前是 0。不要主動改成非 0。

## 編譯期寫入

提示詞與本機設定在編譯時嵌進二進位，執行時不再讀這兩個檔。不要改回 `read_to_string`，也不要加 `config.example` 後援，除非使用者明確要求。

- `src/main.rs` 的 `include_str!("../config.local")`。解析時忽略空白行與 `#` 開頭。只認得 `ollama_url` 與 `model`。值兩端的 `"` 或 `'` 會去掉。
- `src/ollama_translator.rs` 的 `include_str!("AI-commands/translate.md")`。
- `config.example` 裡的 `num_ctx` 沒有被讀，也沒有放進 API 請求。不要自行接上。
- `config.local` 在 `.gitignore`。沒有這個檔就編不過。不要 commit 它，不要在 log、測試輸出或文件裡印出它的 URL 或模型名稱。
- 改 `config.local` 或提示詞後必須重編。`cargo run` 會因為 `include_str!` 自動重編。已經複製走的二進位不會再看旁邊的檔案。

兩個鍵都有值才用 `OllamaTranslator`。否則用 `DummyTranslator`，輸出 `[Dummy Translation] ` 加原文。那是沒有設定時的退路，不是正常翻譯路徑。

## 目錄與職責

- `src/main.rs`：參數、編譯期設定、輸出檔名、選擇翻譯器。
- `src/ai_translate.rs`：語言白名單、逐行讀檔、分段、呼叫翻譯器、寫出結果。
- `src/translator.rs`：`Translator` trait 與 `DummyTranslator`。
- `src/ollama_translator.rs`：對 `{ollama_url}/chat/completions` 送 JSON。`ollama_url` 預期已含 `/v1`，尾端斜線會去掉。
- `src/AI-commands/translate.md`：系統提示詞。目錄名有連字號，裡面沒有 `.rs`，不是 Rust 模組。不要改名。
- `config.example`：可提交的範本。`config.local`：本機檔，由範本複製而來。
- `README.md`、`README-en.md`、`README-ja.md`：給人看的說明。使用者看得到的行為有變時，三份一起改。
- `sample-en.txt`、`sample-ja.txt`：範例輸入。

## 翻譯怎麼跑

1. 目標語言在程式裡固定是 `zh-TW`。`OllamaTranslator` 沒有使用這個參數。真正的目標語寫死在提示詞：Traditional Chinese used in Taiwan。
2. 提示詞的 `{{source_language}}` 換成上面的英文語言名，`{{passage}}` 換成這一段原文。兩個佔位符要留著。
3. 請求只有 `model` 與 `messages`。系統訊息是填好的提示詞。使用者訊息固定是 `Please perform the translation as instructed.`
4. 取第一個 choice 的 `content`，去掉前後空白，寫入輸出檔，後面再加一個空白行。
5. 分段：空行結束一段。一段累積到 2000 個 byte（`String::len`）時，往回切在最後的 `.` `!` `?` `。` `！` `？`；找不到就整段送出。輸入必須是有效 UTF-8。

## 驗證與提交

測試指令是 `cargo test`。現有測試只確認兩個嵌入檔不是空白，不會連到 Ollama。`src/ai_translate.rs` 裡的 `MockTranslator` 沒有被呼叫，編譯測試時會有 dead_code 警告。不要為了消警告或補測試而擴大改動，除非使用者要求。

沒有使用者明確要求時，不要對使用者的 Ollama 發翻譯請求，也不要產生 `*-out.txt`。

這個倉庫的 commit 主旨是 `feat: ...` 這種短句，並加上 `Signed-off-by`。不要 push，除非使用者這次說要 push。不要提交 `config.local`、`target/` 或翻譯輸出檔。
