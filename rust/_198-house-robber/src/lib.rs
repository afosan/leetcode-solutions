struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let l = nums.len();

        if l == 0 {
            return 0;
        }

        let mut a = nums[0];

        if l == 1 {
            return a;
        }

        let mut b = nums[1].max(a);

        for i in 2..nums.len() {
            (a, b) = (b, b.max(a + nums[i]))
        }

        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 2, 3, 1];
        let expected = 4;
        let output = Solution::rob(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![2, 7, 9, 3, 1];
        let expected = 12;
        let output = Solution::rob(nums);
        assert_eq!(expected, output);
    }
}
