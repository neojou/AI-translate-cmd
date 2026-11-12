mod ai_translate;
mod translator;
mod ollama_translator;

use ai_translate::AITranslate;
use ollama_translator::OllamaTranslator;
use translator::DummyTranslator;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
struct Config {
    ollama_url: Option<String>,
    model: Option<String>,
}

impl Config {
    fn from_file(path: &str) -> Self {
        let mut config = Config::default();
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim();
                    let value = value.trim().trim_matches('"').trim_matches('\'');
                    match key {
                        "ollama_url" => config.ollama_url = Some(value.to_string()),
                        "model" => config.model = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }
        config
    }
}

/// 產生不重複的輸出檔名
fn get_output_filename(original_path: &str) -> PathBuf {
    let path = Path::new(original_path);
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("txt");

    // 初始嘗試： 原檔名-out.ext
    let mut candidate = parent.join(format!("{}-out.{}", stem, extension));

    if !candidate.exists() {
        return candidate;
    }

    // 如果已存在，嘗試增加數字： 原檔名-out-1.ext, 原檔名-out-2.ext ...
    let mut counter = 1;
    loop {
        candidate = parent.join(format!("{}-out-{}.{}", stem, counter, extension));
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

/// 顯示使用說明
fn usage(prog_name: &str) {
    println!("用法： {} <input_lang> <input_file>", prog_name);
    println!("範例： {} en sample.txt", prog_name);
    println!("\n提示：若要使用 Ollama，請確保 config.local 已正確設定");
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = &args[0];

    // 參數判斷：預期剛好是 prog_name + 2 個參數 (共 3 個 elements)
    if args.len() != 3 {
        usage(prog_name);
        return;
    }

    let target_lang = &args[1];
    let input_file = &args[2];

    // 讀取設定檔
    let config = Config::from_file("config.local");

    // 決定使用哪種翻譯器
    let translator: Box<dyn translator::Translator> = if let (Some(url), Some(model)) = (config.ollama_url, config.model) {
        println!("🚀 偵測到 Ollama 設定，正在初始化 OllamaTranslator (模型: {})...", model);
        match OllamaTranslator::new(url, model) {
            Ok(ot) => Box::new(ot),
            Err(e) => {
                println!("❌ 初始化 OllamaTranslator 失敗: {}", e);
                return;
            }
        }
    } else {
        println!("⚠️ 未偵測到有效配置，將使用 DummyTranslator (僅供測試用)...");
        Box::new(DummyTranslator)
    };

    // 建立翻譯器實例
    let mut ai_translator = AITranslate::new(translator);
    
    println!("嘗試設定語言為: {}", target_lang);

    // 嘗試設定語言
    match ai_translator.set_input_lang(target_lang.clone()) {
        Ok(_) => {
            println!("✅ 設定成功！目前的語言是: {}", ai_translator.get_input_lang());
            println!("檔案路徑: {}", input_file);

            // 嘗試讀取檔案並進行翻譯 (分段模式)
            println!("正在進行翻譯...");
            let output_path = get_output_filename(input_file);
            match ai_translator.translate_file(input_file, &output_path.to_string_lossy()).await {
                Ok(_) => {
                    println!("✨ 翻譯任務已完成！");
                    println!("📂 結果已寫入: {:?}", output_path);
                }
                Err(e) => {
                    println!("❌ 翻譯失敗！原因: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 設定失敗！原因: {}", e);
            usage(prog_name);
        }
    }
}
