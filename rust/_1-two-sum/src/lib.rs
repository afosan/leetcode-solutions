use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            match seen.get(&(target - n)) {
                Some(v) => {
                    return vec![*v as i32, i as i32];
                },
                None => {
                    seen.insert(*n, i);
                },
            }
        }

        panic!("there must be a solution");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        let output = Solution::two_sum(nums, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let expected = vec![1, 2];
        let output = Solution::two_sum(nums, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let nums = vec![3, 3];
        let target = 6;
        let expected = vec![0, 1];
        let output = Solution::two_sum(nums, target);
        assert_eq!(expected, output);
    }
}
