use std::collections::HashMap;

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

#[cfg(test)]
mod  tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true,  is_unique("aple"));
        assert_eq!(false, is_unique("apple"));
    }
}
