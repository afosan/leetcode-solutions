struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counts = HashMap::<i32, usize>::new();
        let mut out: Vec<Vec<i32>> = vec![Vec::with_capacity(nums.len()); nums.len()];

        for n in nums {
            let count = counts.get(&n).copied().unwrap_or(0);
            out[count].push(n);
            *counts.entry(n).or_insert(0) += 1;
        }

        out.into_iter().filter(|line| line.len() > 0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3, 4, 1, 2, 3, 1];
        let expected = vec![vec![1, 3, 4, 2], vec![1, 3], vec![1]];
        let output = Solution::find_matrix(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![2, 1, 1];
        let expected = vec![vec![2, 1], vec![1]];
        let output = Solution::find_matrix(nums);
        assert_eq!(expected, output);
    }
}
