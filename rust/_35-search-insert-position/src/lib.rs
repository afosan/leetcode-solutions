struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut s = 0;
        let mut e = nums.len();
        let mut m;

        while s < e {
            m = (s + e) / 2;
            if nums[m] >= target {
                e = m;
            } else {
                s = m + 1;
            }
        }

        s as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let expected = 2;
        let output = Solution::search_insert(nums, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let expected = 1;
        let output = Solution::search_insert(nums, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let expected = 4;
        let output = Solution::search_insert(nums, target);
        assert_eq!(expected, output);
    }
}
