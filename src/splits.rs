/// Split on multiple delimiters - this will handle multiple delimiters,
/// splitting the input string on any of the delimiters or a combination of them.
/// This method uses lifetime parameters which means that the returned vector will have a lifetime
/// that is the same as the input string.
///
/// # Example:
/// ```
/// use rust_strings::splits::split_on_delimiters;
/// let input = "big,!a brown,cow!";
/// let delimiters = [',','!','a',' '];
/// let expected_output: Vec<&str> = ["big", "brown", "cow"].to_vec();
/// assert_eq!(expected_output, split_on_delimiters(&input, &delimiters));
pub fn split_on_delimiters<'a>(s: &'a str, delimiters: &[char]) -> Vec<&'a str> {
    let mut output = Vec::new();
    let mut start = 0;
    for (i, c) in s.char_indices() {
        for d in delimiters {
            if c == *d {
                output.push(&s[start..i]);
                start = i + c.len_utf8();
            }
        }
    }
    if start < s.len() {
        output.push(&s[start..]);
    }
    output.retain(|item| !item.is_empty());
    output
}

// Split while preserving the delimiters
//pub fn split_keeping_delimiter(s: &str, delimiter: char) -> Vec<&str>

// Split into exactly N parts, padding with empty strings if needed
//pub fn split_into_n_parts(s: &str, delimiter: char, n: usize) -> Vec<&str>

// Remove common prefixes/suffixes
//pub fn trim_common_prefix(strings: &[&str]) -> Vec<&str>
