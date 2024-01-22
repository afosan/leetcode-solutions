use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut numbers = HashSet::<i32>::from_iter(1..=nums.len() as i32);
        let mut twice = 0;

        nums.iter().for_each(|n| {
            if !numbers.contains(n) {
                twice = *n;
            } else {
                numbers.remove(n);
            }
        });

        let missing = *numbers.iter().next().unwrap();

        vec![twice, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 2, 4];
        let expected = vec![2, 3];
        let output = Solution::find_error_nums(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 1];
        let expected = vec![1, 2];
        let output = Solution::find_error_nums(nums);
        assert_eq!(expected, output);
    }
}
