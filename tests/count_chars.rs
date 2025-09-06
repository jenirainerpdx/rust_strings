use rust_strings::count_chars;

#[test]
fn test_count_chars() {
    assert_eq!(count_chars('a', "banana"), 3);
}

#[test]
fn test_count_chars_empty_string() {
    assert_eq!(count_chars('a', ""), 0);
}

#[test]
fn test_count_chars_no_occurrences() {
    assert_eq!(count_chars('z', "hello"), 0);
}

#[test]
fn test_count_chars_case_sensitive() {
    assert_eq!(count_chars('A', "banana"), 0);
    assert_eq!(count_chars('a', "banana"), 3);
}

#[test]
fn test_count_chars_special_characters() {
    assert_eq!(count_chars('!', "Hello! How are you!"), 2);
}

#[test]
fn test_count_chars_unicode() {
    assert_eq!(count_chars('😊', "😊😊😊"), 3);
}

#[test]
fn test_count_chars_numeric() {
    assert_eq!(count_chars('1', "123321"), 2);
}

#[test]
fn test_count_chars_whitespace() {
    assert_eq!(count_chars(' ', "a b c d e f"), 5);
}

#[test]
fn test_count_chars_long_string() {
    let long_string = "a".repeat(5000) + "b" + &"a".repeat(500000);
    assert_eq!(count_chars('b', &long_string), 1);
    assert_eq!(count_chars('a', &long_string), 505000);
}
