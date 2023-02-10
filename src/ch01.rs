use std::collections::HashMap;

fn convert_ascii_to_u8(c: &char) -> Option<u8> {
    if c.is_ascii() {
        return Some(*c as u8);
    }
    None
}

pub fn is_unique(string: &str) -> bool {

    let mut hm: HashMap<char, bool> = HashMap::new();

    for k in string.chars() {

        match hm.get(&k) {
            None => { hm.insert(k, true); },
            Some(_) => { return false; }
        }
    }
    true
}

#[allow(dead_code)]
fn but_set(value: &mut u32, nbit: u8) {
    *value |= 1 << nbit
}

#[allow(dead_code)]
fn bit_reset(value: &mut u32, nbit: u8) {
    *value &= 1 << nbit
}

#[allow(dead_code)]
fn bit_check(value: &u32, nbit: u8) -> bool {
    let v = *value & 1 << nbit;
    v > 0
}

pub fn check_permutation(s0: &str, s1: &str) -> bool {

    if s0.len() != s1.len() { return false; }

    let mut arr = [0u32; 26];

    for b in s0.as_bytes() {
        let position = b - b'a';
        arr[position as usize] += 1;
    }

    for b in s1.as_bytes() {
        let position = b - b'a';

        if arr[position as usize] <= 0 {
            return false;
        } else {
            arr[position as usize] -= 1;
        }
    }
    true
}

// pub fn urlify(s: &str, true_length: u32) -> &str {
//     let original_len = s.len();
//     let mut num_of_space = 0;
//
//     for c in s.chars() {
//         if c == ' ' { num_of_space += 1; }
//     }
//     let extended_len = original_len + 2*num_of_space;
//     dbg!(extended_len);
// }


pub fn palindrome_permutation(s: &str) -> bool {

    let mut frequencies = [0u32; 128];

    for c in s.chars() {
        let lower = c.to_ascii_lowercase();
        if lower == ' ' { continue; }

        if let Some(index) = convert_ascii_to_u8(&lower) {
            frequencies[index as usize] += 1;
        }
    }

    let mut num_of_odd = 0;
    for freq in frequencies {
        if freq % 2  == 0 { continue; }
        else {
            num_of_odd += 1;
            if num_of_odd > 1 { return false; }
        }
    }
    true
}

pub fn string_compression(s: String) -> String {

    let s_vec: Vec<_> = s.chars().collect();
    let mut result = String::new();

    let mut previous: char = ' ';
    let mut k_count = 0;
    let k_max = s.len() / 2;
    let mut v_count = 0;

    for c in s_vec {
        if previous != c {
            result += &format!("{}{}", previous, v_count);
            previous = c;
            v_count = 0;
            k_count += 1;
            if k_count >= k_max { return s; }
        }
        v_count += 1;
    }
    result += &format!("{}{}", previous, v_count);
    result[2..].to_string()
}

pub fn rotate_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    matrix
}

pub fn zero_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    matrix
}

pub fn string_rotation(s: &str) -> &str {
    s
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(true,  is_unique("aple"));
        assert_eq!(false, is_unique("apple"));
    }

    #[test]
    fn test_check_permutation() {
        assert_eq!(false, check_permutation("aple", "apple"));
        assert_eq!(true, check_permutation("aple", "leap"));
        assert_eq!(true, check_permutation("aple", "leap"));
    }

    // #[test]
    // fn test_urlify() {
    //     assert_eq!("Austin%20Chen", urlify("Austin Chen"));
    //     assert_eq!(urlify("Mr John Smith    ", 13), "Mr%20John%20Smith");
    //     assert_eq!(urlify("Mr John Smith       ", 14), "Mr%20John%20Smith%20");
    //     assert_eq!(urlify_reverse("Mr John Smith    ", 13), "Mr%20John%20Smith");
    // }

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

    #[test]
    fn test_string_compression() {
        assert_eq!("a2b1c5a3", string_compression(format!("aabcccccaaa")));
        assert_eq!("abcdefg", string_compression(format!("abcdefg")));
        assert_eq!("abcccc", string_compression(format!("abcccc")));
    }
}
