struct Solution;

use std::collections::HashMap;
use std::hash::Hash;

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let cc = value_counts(value_counts(arr));
        *cc.iter().max().expect("expects non-empty arr") == 1
    }
}

fn value_counts<T: Eq + Hash>(arr: Vec<T>) -> Vec<usize> {
    let mut vcs = HashMap::<T, usize>::new();
    arr.into_iter().for_each(|a| *vcs.entry(a).or_insert(0) += 1);

    vcs.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let expected = true;
        let output = Solution::unique_occurrences(arr);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let arr = vec![1, 2];
        let expected = false;
        let output = Solution::unique_occurrences(arr);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let expected = true;
        let output = Solution::unique_occurrences(arr);
        assert_eq!(expected, output);
    }
}
