
pub fn urlify(s: String, true_length: u32) -> String {

    let bytes = s.into_bytes();

    let mut num_of_space = 0u32;

    for i in 0..true_length {
        if bytes[i as usize] == b' ' {
            num_of_space += 1;
        }
    }

    let extended_length = true_length + 2 * num_of_space;
    let mut results = vec![0 ;extended_length as usize];
    let mut i: i32 = (true_length - 1) as i32;
    let mut j: i32 = (extended_length - 1) as i32;

    while i >= 0 {
        if bytes[i as usize] == b' ' {
            results[j as usize] = b'0';
            results[(j-1) as usize] = b'2';
            results[(j-2) as usize] = b'%';
            j -= 3; 
        } else {
            results[j as usize] = bytes[i as usize];
            j -= 1;
        }
        i -= 1;
    }

    let results = match String::from_utf8(results) {
        Ok(v) => v,
        Err(e) => panic!("Invalid utf-8 sequence: {}", e),
    };
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(format!("Austin%20Chen"), urlify(format!("Austin Chen  "), 11));
        assert_eq!(format!("Mr%20John%20Smith"), urlify(format!("Mr John Smith    "), 13));
        assert_eq!(format!("Mr%20John%20Smith%20"), urlify(format!("Mr John Smith       "), 14));
    }
}
