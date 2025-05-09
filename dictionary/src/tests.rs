use super::*;
#[test]
fn line_greater_than() {
    let text = "second text";

    assert!(text.chars().count() > 10)
}

#[test]

fn identify_line_to_be_wrapped() {
    let wrapped = vec![
        "This is a ",
        "long test ",
        "that needsaaaaaaaaaa",
        " to be wra",
        "pped",
    ];

    let mut index = 0;
    let mut count = 0;
    wrapped.iter().for_each(|line| {
        if line.chars().count() > 10 {
            index = count;
        }
        count = count + 1;
    });

    assert_eq!(2, index)
}

#[test]
fn word_wrap_at_specific_line() {
    let wrapped = vec![
        "This is a ",
        "long test ",
        "that needsaaaaaaaaaa",
        " to be wra",
        "pped",
    ];

    let expected = vec![
        "This is a ",
        "long test ",
        "that needs",
        "aaaaaaaaaa",
        " to be wra",
        "pped",
    ];

    assert_eq!(expected, wrapped);
}

#[test]
fn word_wrap() {
    let text_to_wrap = "This is a long test that needs to be wrapped";
    let wrapped = vec![
        "This is a ",
        "long test ",
        "that needs",
        " to be wra",
        "pped",
    ];

    let chunk_size = 10;
    let mut result = Vec::new();

    let mut remaining = text_to_wrap;
    while !remaining.is_empty() {
        let split_point = remaining.len().min(chunk_size);
        let (chunk, rest) = remaining.split_at(split_point);
        result.push(chunk);
        remaining = rest;
    }

    assert_eq!(wrapped, result)
}
