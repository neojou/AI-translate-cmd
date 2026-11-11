mod ai_translate;

use ai_translate::AITranslate;
use std::env;

/// 顯示使用說明
fn usage(prog_name: &str) {
    println!("用法： {} <input_lang> <input_file>", prog_name);
    println!("範例： {} en sample.txt", prog_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = &args[0];

    // 建立翻譯器實例
    let mut translator = AITranslate::new();
    // 模擬動態增加語言，讓使用者可以測試 zh 以外的語言
    translator.add_allowed_lang("zh".to_string());

    // 參數判斷：預期剛好是 prog_name + 2 個參數 (共 3 個 elements)
    // 但根據使用者需求，我們檢查是否「不足」或「太多」
    // 這裡我們定義「正確參數數量」為 3 (包含程式名稱本身)
    if args.len() != 3 {
        usage(prog_name);
        return;
    }

    let target_lang = &args[1];
    let _input_file = &args[2]; // 目前先拿著，尚未實作檔案讀取邏輯

    println!("嘗試設定語言為: {}", target_lang);

    // 嘗試設定語言
    match translator.set_input_lang(target_lang.clone()) {
        Ok(_) => {
            println!("✅ 設定成功！目前的語言是: {}", translator.get_input_lang());
            println!("檔案路徑: {}", _input_file);
            println!("(接下來將會實作讀取 {} 並翻譯成繁體中文的功能)", _input_file);
        }
        Err(e) => {
            println!("❌ 設定失敗！原因: {}", e);
            // 如果是因為語言不對，也可以提示用法
            usage(prog_name);
        }
    }
}
