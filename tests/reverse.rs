use rust_strings::string_reverse;

#[test]
fn test_string_reverse() {
    assert_eq!(string_reverse("hello"), "olleh");
}

#[test]
fn test_string_reverse_empty() {
    assert_eq!(string_reverse(""), "");
}

#[test]
fn test_string_reverse_unicode() {
    assert_eq!(string_reverse("こんにちは"), "はちにんこ");
}

#[test]
fn test_string_reverse_palindrome() {
    assert_eq!(string_reverse("madam"), "madam");
}

#[test]
fn test_string_reverse_long() {
    let long_string = "a".repeat(10000000) + "b" + &"c".repeat(100000000);
    assert_eq!(
        string_reverse(&long_string),
        "c".repeat(100000000) + "b" + &"a".repeat(10000000)
    );
}

#[test]
fn test_string_reverse_special_characters() {
    assert_eq!(string_reverse("!@#$%^&*()"), ")(*&^%$#@!");
}

#[test]
fn test_string_reverse_mixed() {
    assert_eq!(string_reverse("Hello, 世界!"), "!界世 ,olleH");
}
