//! A library providing string manipulation functions.
//! Includes functions to check for palindromes, count character occurrences,
//! and reverse strings.

///
/// Checks if the given string is a palindrome, ignoring case and non-alphanumeric characters.
/// # Examples
/// ```
/// use rust_strings::check_for_palindrome;
/// assert!(check_for_palindrome("A man, a plan, a canal: Panama"));
/// assert!(!check_for_palindrome("race a car"));
/// ```
pub fn check_for_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = string_reverse(&cleaned);
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

/// Reverses the given string.
/// # Examples
/// ```
/// use rust_strings::string_reverse;
/// assert_eq!(string_reverse("hello"), "olleh");
/// assert_eq!(string_reverse(""), "");
/// ```
pub fn string_reverse(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_reverse() {
        assert_eq!(string_reverse("hello"), "olleh");
    }

    #[test]
    fn test_string_reverse_empty() {
        assert_eq!(string_reverse(""), "");
    }
}
