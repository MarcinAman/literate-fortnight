use std::{vec, collections::HashMap};

//https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut passed: HashMap<i32, usize> = HashMap::new();

    for (i, x) in nums.iter().enumerate() {
        let remains = target - *x;
        if passed.contains_key(&remains) {
            return vec![*passed.get(&remains).unwrap() as i32, i as i32]
        }
        passed.insert(*x, i);
    }

    panic!("No solution found")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(vec![0, 1], two_sum(vec![2, 7, 11, 15], 9))
    }

    #[test]
    fn case_2() {
        assert_eq!(vec![1, 2], two_sum(vec![3, 2, 4], 6))
    }

    #[test]
    fn case_3() {
        assert_eq!(vec![0, 1], two_sum(vec![3, 3], 6))
    }
}
