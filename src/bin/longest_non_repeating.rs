// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashSet;

pub fn length_of_longest_substring(s: String) -> i32 {
    let chars = s.as_bytes();
    let size = chars.len();
    let mut results =  vec![0; size];

    for i in 0..size {
        let mut letters = HashSet::new();
        for j in i..size {
            let letter = chars[j];

            if letters.contains(&letter) {
                break;
            }
            letters.insert(letter);
        }

        results[i] = letters.len() as i32;
    }

    return *results.iter().max().unwrap_or(&0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(3, length_of_longest_substring("abcabcbb".to_owned()))
    }

    #[test]
    fn case_2() {
        assert_eq!(1, length_of_longest_substring("bbbbb".to_owned()))
    }

    #[test]
    fn case_3() {
        assert_eq!(3, length_of_longest_substring("pwwkew".to_owned()))
    }

    #[test]
    fn case_4() {
        assert_eq!(0, length_of_longest_substring("".to_owned()))
    }

    #[test]
    fn case_5() {
        assert_eq!(7, length_of_longest_substring("1234567".to_owned()))
    }

    #[test]
    fn case_6() {
        assert_eq!(1, length_of_longest_substring(" ".to_owned()))
    }
}