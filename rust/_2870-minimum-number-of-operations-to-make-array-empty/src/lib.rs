struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::<i32, usize>::new();

        nums.iter().for_each(|n| *counts.entry(*n).or_insert(0) += 1);

        if counts.values().any(|n| *n == 1) {
            return -1;
        }

        counts.values().filter(|n| **n != 1).map(|n| {
            let d = n / 6 * 2;
            let c = n % 6;
            match c {
                0 => d,
                1 | 2 | 3 => 1 + d,
                4 | 5 => 2 + d,
                _ => unreachable!("mod 6"),
            }
        }).sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
        let expected = 4;
        let output = Solution::min_operations(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![2, 1, 2, 2, 3, 3];
        let expected = -1;
        let output = Solution::min_operations(nums);
        assert_eq!(expected, output);
    }
}
