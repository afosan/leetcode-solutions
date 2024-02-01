struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let out = nums.chunks(3).map(|triplet| triplet.to_vec()).collect::<Vec<_>>();

        println!("{:?}", out);

        if out.iter().any(|v| v[2] - v[0] > k) {
            vec![]
        } else {
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let k = 2;
        let expected = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];
        let output = Solution::divide_array(nums, k);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![1, 3, 3, 2, 7, 3];
        let k = 3;
        let expected: Vec<Vec<i32>> = vec![];
        let output = Solution::divide_array(nums, k);
        assert_eq!(expected, output);
    }
}
