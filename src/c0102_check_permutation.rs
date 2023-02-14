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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(false, check_permutation("aple", "apple"));
        assert_eq!(true, check_permutation("aple", "leap"));
        assert_eq!(true, check_permutation("aple", "leap"));
    }
}
