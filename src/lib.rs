//! A library providing string manipulation functions.
//! Includes functions to check for palindromes, count character occurrences,
//! and reverse strings.

pub mod splits;

///
/// Checks if the given string is a palindrome, ignoring the case and non-alphanumeric characters.
/// # Examples
/// ```
/// use rust_strings::check_for_palindrome;
/// assert!(check_for_palindrome("A man, a plan, a canal: Panama"));
/// assert!(!check_for_palindrome("race a car"));
/// ```
pub fn check_for_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned.eq_ignore_ascii_case(&reversed)
}

/// Counts the occurrences of a specific character in a string.
/// # Examples
/// ```
/// use rust_strings::count_chars;
/// assert_eq!(count_chars('a', "banana"), 3);
/// assert_eq!(count_chars('z', "hello"), 0);
/// ```
pub fn count_chars(c: char, s: &str) -> usize {
    s.chars().filter(|&x| x == c).count()
}
