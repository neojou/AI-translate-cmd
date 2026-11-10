mod ai_translate;

use std::env;
use ai_translate::AITranslate;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 建立翻譯器實例
    let mut translator = AITranslate::new();

    // 預先展示：增加一個新的允許語言 "zh"
    println!("--- 預先設定：新增 'zh' 到允許清單 ---");
    translator.add_allowed_lang("zh".to_string());
    println!("目前允許的語言: {:?}", translator.get_allowed_langs());
    println!("------------------------------------");

    // 如果有輸入參數，則嘗試設定該語言
    if args.len() >= 2 {
        let target_lang = args[1].clone();
        println!("嘗試設定語言為: {}", target_lang);

        if translator.set_input_lang(target_lang) {
            println!("✅ 設定成功！目前的語言是: {}", translator.get_input_lang());
        } else {
            println!("❌ 設定失敗！該語言不在允許清單中。");
        }
    } else {
        println!("用法: cargo run -- <lang>");
        println!("(目前已動態新增 'zh'，你可以嘗試輸入 'zh' 來測試)");
    }
}
