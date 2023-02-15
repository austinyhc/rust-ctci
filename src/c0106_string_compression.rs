
pub fn string_compression(s: String) -> String {

    let length = s.len();
    if length <= 1 { return s; }

    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut count = 1;
    let mut comparee = chars[i];
    i += 1;

    let mut compressed_stream = String::new();

    while i < length {
        if chars[i] == comparee {
            count += 1
        } else {
            let partial_stream = format!("{}{}", comparee, count);
            compressed_stream.push_str(&partial_stream);
            if compressed_stream.len() >= length { return s; }
            comparee = chars[i];
            count = 1;
        }
        i += 1;
    }
    
    let partial_stream = format!("{}{}", comparee, count);
    compressed_stream.push_str(&partial_stream);
    if compressed_stream.len() >= length { return s; }
    compressed_stream
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!("a2b1c5a3", string_compression(format!("aabcccccaaa")));
        assert_eq!("abcdefg", string_compression(format!("abcdefg")));
        assert_eq!("abcccc", string_compression(format!("abcccc")));
    }

    #[test]
    fn test_double_digit() {
        assert_eq!("a18", string_compression(format!("aaaaaaaaaaaaaaaaaa")));
        assert_eq!("a10c10", string_compression(format!("aaaaaaaaaacccccccccc")));
    }
    
    #[test]
    fn test_extreme() {
        assert_eq!("aa", string_compression(format!("aa")));
        assert_eq!("a", string_compression(format!("a")));
        assert_eq!("", string_compression(format!("")));
        assert_eq!(" ", string_compression(format!(" ")));
    }
}
