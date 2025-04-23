#[cfg(test)]
use super::*;

#[test]
fn adding_word_object() {
    let cotoba = Cotoba::new(
        "梅雨".to_string(),
        vec!["つゆ".to_string(), "ばいう".to_string()],
        "日本の6月ごろにある、雨がたくさん降る期間".to_string(),
    );

    println!("{:?}", cotoba);
}
