//! # Splits module
//!
//! * Purpose: To offer some functionality similar to string split but slightly different.
//!
//! This module includes multiple functions which offer variations on splitting an
//! input String.
//!
//! * Note: For each use case, the module includes one function which returns string slices with lifetime parameters
//! (allowing borrowed elements for memory efficiency), and a separate, parallel function which returns fully owned Strings
//! for those use cases which need longer lived return types.
//!

/// Split on multiple delimiters - this will handle multiple delimiters,
/// splitting the input string on any of the delimiters or a combination of them.
/// This function uses lifetime parameters meaning that the returned vector will have a lifetime
/// that is the same as the input string.
///
/// # Parameters:
/// * `input_string`: A string slice to be split.
/// * `delimiters`: an array slice of chars
///
/// # Returns:
/// * A `Vec<&str>` containing substrings of the input string `s`. The Vector is annotated with a lifetime parameter
/// so that it will have a lifetime that is the same as the input string.
///
/// # Example:
/// ```
/// use rust_strings::splits::split_on_delimiters;
/// let input = "big,!a brown,cow!";
/// let delimiters = [',','!','a',' '];
/// let expected_output: Vec<&str> = ["big", "brown", "cow"].to_vec();
/// assert_eq!(expected_output, split_on_delimiters(&input, &delimiters));
/// ```
///
/// ```
/// use rust_strings::splits::split_on_delimiters;
/// let input = "<html><body><h1>Heading</h1></body></html>";
/// let delimiters = ['<', '>', '/'];
/// let expected_output: Vec<&str> = ["html", "body", "h1", "Heading", "h1", "body", "html"].to_vec();
/// assert_eq!(expected_output, split_on_delimiters(&input, &delimiters));
///```
pub fn split_on_delimiters<'a>(input_string: &'a str, delimiters: &[char]) -> Vec<&'a str> {
    let mut output = Vec::new();
    let mut start = 0;
    for (i, c) in input_string.char_indices() {
        for d in delimiters {
            if c == *d {
                output.push(&input_string[start..i]);
                start = i + c.len_utf8();
            }
        }
    }
    if start < input_string.len() {
        output.push(&input_string[start..]);
    }
    output.retain(|item| !item.is_empty());
    output
}

/// Split on multiple delimiters - this will handle multiple delimiters,
/// splitting the input string on any of the delimiters or a combination of them.
/// This method returns a Vec of owned Strings rather than string slices.
///
/// # Parameters:
/// * `input_string`: A string slice to be split.
/// * `delimiters`: an array slice of chars
///
/// # Returns:
/// * A `Vec<String>` containing substrings of the input string `s`.
///
/// # Examples:
/// ```
/// use rust_strings::splits::split_on_delimiters_returns_owned;
/// let input = "big,!a brown,cow!";
/// let delimiters = [',','!','a',' '];
/// let expected_output: Vec<String> = ["big".to_string(), "brown".to_string(), "cow".to_string()].to_vec();
/// assert_eq!(expected_output, split_on_delimiters_returns_owned(&input, &delimiters));
/// ```
pub fn split_on_delimiters_returns_owned(s: &str, delimiters: &[char]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        for d in delimiters {
            if c == *d {
                output.push(s[start..i].to_owned().parse().unwrap());
                start = i + c.len_utf8();
            }
        }
    }
    if start < s.len() {
        output.push(s[start..].parse().unwrap());
    }
    output.retain(|item| !item.is_empty());
    output
}

/// Splits a string into substrings while keeping the delimiter at the end of each substring.
/// The function uses lifetime parameters meaning that the returned vector will have a lifetime
/// that is the same as the input string.
///
/// # Parameters
/// * `s`: A string slice to be split.
/// * `delimiter`: The character used as the delimiter to split the string.
///
/// # Returns
/// * A `Vec<&str>` containing substrings of the input string `s`. Each substring includes the
/// delimiter at the end (except for the last substring, if the delimiter is the final character
/// in the string).
///
/// # Examples
/// ```
/// use rust_strings::splits::split_keeping_delimiter;
/// let s = "hello,world,here";
/// let result = split_keeping_delimiter(s, ',');
/// assert_eq!(result, vec!["hello,", "world,"]);
/// ```
///
/// ```
/// use rust_strings::splits::split_keeping_delimiter;
/// let s = "hello,world,here,";
/// let result = split_keeping_delimiter(s, ',');
/// assert_eq!(result, vec!["hello,", "world,","here,"]);
/// ```
///
/// ```
/// use rust_strings::splits::split_keeping_delimiter;
/// let s = "no_delimiters_here";
/// let result = split_keeping_delimiter(s, ',');
/// assert_eq!(result, Vec::<&str>::new());
/// ```
///
/// ## Note
/// - The function does not include substrings after the last delimiter.
/// - If the input string does not include the delimiter, an empty vector will be returned.
///
pub fn split_keeping_delimiter<'a>(s: &'a str, delimiter: char) -> Vec<&'a str> {
    let mut output = Vec::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        if c == delimiter {
            output.push(&s[start..=i]);
            start = i + 1;
        }
    }
    output
}

/// Splits a given input string by the specified delimiter while retaining the delimiter
/// at the end of each split substring. Returns the results as a vector of owned strings.
///
/// # Arguments
/// * `input_string` - A reference to the string that will be split.
/// * `delimiter` - A character used as the delimiter to split the string.
///
/// # Returns
/// * `Vec<String>` - A vector of owned strings resulting from the split,
///   each containing the delimiter at the end.
///
/// # Example
/// ```
/// use rust_strings::splits::split_keeping_delimiter_returns_owned;
/// let input = "hello,world,here";
/// let delimiter = ',';
/// let result = split_keeping_delimiter_returns_owned(input, delimiter);
/// assert_eq!(result, vec!["hello,", "world,"]);
/// ```
pub fn split_keeping_delimiter_returns_owned(input_string: &str, delimiter: char) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut start = 0;
    for (i, c) in input_string.char_indices() {
        if c == delimiter {
            output.push(input_string[start..=i].to_owned());
            start = i + 1;
        }
    }
    output
}

/// Splits a string slice into exactly N parts, padding with empty strings if needed. It uses
/// an input delimiter and an input size to determine how many string slices are wanted for output.
/// If the slice cannot be parsed into n sub-strings, then empty strings are used to fill the vector.
///
/// # Arguments:
/// * `input_string`: a string slice to be split into parts.
/// * `delimiter`: a char used to determine how/where to split the input_string
/// * `n`: usize to determine how many pieces to split input_string into.
///
/// # Returns
/// * `Vec<&str>` - resulting from the split, potentially including empty
///  string slices if the input_string did not contain n-1 delimiter characters.
///
/// # Examples:
/// ```
/// use rust_strings::splits::split_into_n_parts;
/// let input_string = "This is a string.";
/// let delimiter = " ";
/// let n = 3;
/// let expected_outcome = vec!["This", "is", "a string."];
/// ```
///
/// ```
/// use rust_strings::splits::split_into_n_parts;
/// let input_string = "This is a string.";
/// let delimiter = " ";
/// let n = 4;
/// let expected_outcome = vec!["This", "is", "a", "string."];
/// ```
///
/// ```
/// use rust_strings::splits::split_into_n_parts;
/// let input_string = "This is a string.";
/// let delimiter = " ";
/// let n = 5;
/// let expected_outcome = vec!["This", "is", "a", "string.", ""];
/// ```
pub fn split_into_n_parts<'a>(input_string: &'a str, delimiter: char, n: usize) -> Vec<&'a str> {
    let mut output: Vec<&str> = Vec::new();
    let count_expect_segments = input_string.chars().filter(|&x| x == delimiter).count() + 1;
    if count_expect_segments == n {
        output = input_string.split(delimiter).collect();
    } else if count_expect_segments < n {
        output = input_string.split(delimiter).collect();
        for mut i in count_expect_segments..n {
            output.push("");
            i += 1;
        }
    } else {
        output = input_string.splitn(n, delimiter).collect();
    }
    output
}

/// Splits a string slice into exactly N parts, padding with empty strings if needed. It uses
/// an input delimiter and an input size to determine how many string slices are wanted for output.
/// If the slice cannot be parsed into n sub-strings, then empty strings are used to fill the vector.
/// This fn returns owned strings in the vector
///
/// # Arguments:
/// * `input_string`: a string slice to be split into parts.
/// * `delimiter`: a char used to determine how/where to split the input_string
/// * `n`: usize to determine how many pieces to split input_string into.
///
/// # Returns
/// * `Vec<String>` - resulting from the split, potentially including empty
///  strings if the input_string did not contain n-1 delimiter characters.
///
/// # Examples:
/// ```
/// use rust_strings::splits::split_into_n_parts_returns_owned;
/// let input_string = "This is a string.";
/// let delimiter = ' ';
/// let n = 3;
/// let expected_outcome = vec!["This".to_string(), "is".to_string(), "a string.".to_string()];
/// assert_eq!(expected_outcome, split_into_n_parts_returns_owned(input_string, delimiter, n));
/// ```
///
/// ```
/// use rust_strings::splits::split_into_n_parts_returns_owned;
/// let input_string = "This is a string.";
/// let delimiter = ' ';
/// let n = 4;
/// let expected_outcome = vec!["This".to_string(), "is".to_string(), "a".to_string(), "string.".to_string()];
/// assert_eq!(expected_outcome, split_into_n_parts_returns_owned(input_string, delimiter, n));
/// ```
///
/// ```
/// use rust_strings::splits::split_into_n_parts_returns_owned;
/// let input_string = "This is a string.";
/// let delimiter = ' ';
/// let n = 5;
/// let expected_outcome = vec!["This".to_string(),
///                             "is".to_string(),
///                             "a".to_string(),
///                             "string.".to_string(),
///                             "".to_string()];
/// assert_eq!(expected_outcome, split_into_n_parts_returns_owned(input_string, delimiter, n));
/// ```
pub fn split_into_n_parts_returns_owned(
    input_string: &str,
    delimiter: char,
    n: usize,
) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let count_expect_segments = input_string.chars().filter(|&x| x == delimiter).count() + 1;
    if count_expect_segments == n {
        output = input_string.split(delimiter).map(String::from).collect();
    } else if count_expect_segments < n {
        output = input_string.split(delimiter).map(String::from).collect();
        for mut i in count_expect_segments..n {
            output.push(String::from(""));
            i += 1;
        }
    } else {
        output = input_string
            .splitn(n, delimiter)
            .map(String::from)
            .collect();
    }
    output
}

// Remove common prefixes/suffixes
//pub fn trim_common_prefix(strings: &[&str]) -> Vec<&str>
