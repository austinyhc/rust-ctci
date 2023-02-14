
pub fn urlify(s: String, true_length: u32) -> String {

    s.rsplit
    let mut s_arr = s.as_bytes();
    let mut num_of_space = 0;

    for i in 0..true_length {
        if s_arr[i as usize] == b' ' { num_of_space += 1; }
    }

    let ext_length = true_length + 2*num_of_space;
    let result_vec = Vec::with_capacity(ext_length as usize);
    let mut i = true_length - 1;
    let mut j = extended_length - 1;

    while j > 0 {
        if s_arr[j as usize] == b' ' {


        } else {
            result_vec[j] = s_arr[i];
            i -= 1;
        }
        j -= 1;
    }

    String::from_utf8(result_vec).unwrap()
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
