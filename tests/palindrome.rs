use rust_strings::check_for_palindrome; // Replace with your crate name

#[test]
fn test_palindrome_with_spaces() {
    assert!(check_for_palindrome("A man, a plan, a canal: Panama"));
    assert!(!check_for_palindrome("race a car"));
}

#[test]
fn test_palindrome_empty_string() {
    assert!(check_for_palindrome(""));
}

#[test]
fn test_palindrome_single_character() {
    assert!(check_for_palindrome("a"));
    assert!(check_for_palindrome("A"));
}

#[test]
fn test_palindrome_mixed_case() {
    assert!(check_for_palindrome("Madam"));
}

#[test]
fn test_palindrome_with_punctuation() {
    assert!(check_for_palindrome("No 'x' in Nixon"));
}
