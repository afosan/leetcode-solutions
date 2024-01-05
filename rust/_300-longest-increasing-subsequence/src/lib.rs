struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut acc: Vec<i32> = vec![];

        for n in &nums {
            match bs(&acc, *n) {
                Some(index) => {
                    if index == 0 || acc[index - 1] < *n {
                        acc[index] = *n;
                    }
                },
                None => {
                    if acc.len() == 0 || acc[acc.len() - 1] < *n {
                        acc.push(*n);
                    }
                },
            }
        }

        acc.len() as i32
    }
}

fn bs(nums: &Vec<i32>, target: i32) -> Option<usize> {
    let length = nums.len();
    let mut l = 0;
    let mut h = length;
    let mut m;

    while l != h {
        m = (l + h) / 2;
        if nums[m] <= target {
            l = m + 1;
        }
        else {
            h = m;
        }
    }

    if h >= length {
        None
    } else {
        Some(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let expected = 4;
        let output = Solution::length_of_lis(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        let expected = 4;
        let output = Solution::length_of_lis(nums);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        let expected = 1;
        let output = Solution::length_of_lis(nums);
        assert_eq!(expected, output);
    }
}
