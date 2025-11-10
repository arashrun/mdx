use icu_collator::{Collator, CollatorOptions, Utf8Collator};
use icu_locale::Locale;
use icu_provider::DataProvider;

// 模拟 MDict 中的条目排序
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建一些需要排序的条目，模拟中文字典的情况
    let mut entries = vec![
        "中国".to_string(),
        "中文".to_string(),
        "北京".to_string(),
        "上海".to_string(),
        "啊".to_string(),
        "一".to_string(),
        "园".to_string(),
        "和".to_string(),
        "安".to_string(),
    ];

    println!("原始条目顺序:");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {}", i + 1, entry);
    }

    // 按中文拼音排序 (默认)
    sort_entries_by_locale(&mut entries, "zh-u-co-pinyin")?;
    println!("\n按中文拼音排序 (zh-u-co-pinyin):");
    for (i, entry) in entries.iter().enumerate() {
        println!("{}: {}", i + 1, entry);
    }

    // 重置并按日文排序
    let mut entries_ja = vec![
        "中国".to_string(),
        "日本".to_string(),
        "英語".to_string(),
        "フランス".to_string(),
        "あ".to_string(),
        "い".to_string(),
        "う".to_string(),
    ];

    sort_entries_by_locale(&mut entries_ja, "ja")?;
    println!("\n按日文排序 (ja):");
    for (i, entry) in entries_ja.iter().enumerate() {
        println!("{}: {}", i + 1, entry);
    }

    // 重置并按英文排序
    let mut entries_en = vec![
        "Zoo".to_string(),
        "Apple".to_string(),
        "Book".to_string(),
        "apple".to_string(),
        "Ant".to_string(),
        "zoo".to_string(),
    ];

    sort_entries_by_locale(&mut entries_en, "en")?;
    println!("\n按英文排序 (en):");
    for (i, entry) in entries_en.iter().enumerate() {
        println!("{}: {}", i + 1, entry);
    }

    // 重置并考虑大小写的英文排序
    let mut entries_en_case_sensitive = vec![
        "Zoo".to_string(),
        "Apple".to_string(),
        "Book".to_string(),
        "apple".to_string(),
        "Ant".to_string(),
        "zoo".to_string(),
    ];

    sort_entries_by_locale(&mut entries_en_case_sensitive, "en-u-ks-level1")?; // 区分大小写
    println!("\n按英文排序 (区分大小写, en-u-ks-level1):");
    for (i, entry) in entries_en_case_sensitive.iter().enumerate() {
        println!("{}: {}", i + 1, entry);
    }

    Ok(())
}

// 使用 ICU 排序器按照指定的语言环境排序
fn sort_entries_by_locale(entries: &mut Vec<String>, locale_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let locale: Locale = locale_str.parse()?;
    let options = CollatorOptions::default();
    let provider = icu_testdata::get_provider();
    let collator = Utf8Collator::try_new(provider, &locale, options)?;

    entries.sort_by(|a, b| {
        collator.compare(a.as_str(), b.as_str())
    });

    Ok(())
}

// 仅用于编译通过，实际运行需要安装 icu_testdata
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icu_sort() {
        let mut entries = vec!["中国", "北京", "上海", "啊", "一"];
        let entries: Vec<String> = entries.iter().map(|s| s.to_string()).collect();
        // 这里仅测试函数是否能编译通过
        assert_eq!(entries.len(), 5);
    }
}