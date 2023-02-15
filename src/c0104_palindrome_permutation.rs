use std::collections::HashMap;

fn convert_ascii_to_u8(c: &char) -> Option<u8> {
    if c.is_ascii() {
        return Some(*c as u8);
    }
    None
}

pub fn palindrome_permutation(s: &str) -> bool {

    let mut frequencies = [0u32; 128];
    let bytes = s.bytes();

    for byte in bytes {
        if byte == b' ' { continue; }
        let byte = byte & !(1 << 5);
        frequencies[byte as usize] += 1;
    }

    let mut num_of_odd_freq = 0;
    for freq in frequencies {
        if freq % 2 == 1 { num_of_odd_freq += 1; }
        if num_of_odd_freq > 1 { return false; }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_permutation() {
        assert_eq!(true, palindrome_permutation("Tact Coa"));
        assert_eq!(true, palindrome_permutation("taco cat"));
        assert_eq!(true, palindrome_permutation("atco cta"));
        assert_eq!(true, palindrome_permutation("app"));
        assert_eq!(false, palindrome_permutation("Austinchen"));
        assert_eq!(true, palindrome_permutation("Do geese see God"));
        assert_eq!(true, palindrome_permutation("Was it a car or a cat I saw"));
    }
}
