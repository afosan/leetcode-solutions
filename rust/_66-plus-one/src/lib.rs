struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut acc = 1;
        let mut out = digits.iter().rev().map(|i| {
            let s = *i + acc;
            acc = s / 10;
            s % 10
        }).collect::<Vec<_>>();
        if acc > 0 {
            out.push(acc);
        }
        out.into_iter().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let digits = vec![1, 2, 3];
        let expected = vec![1, 2, 4];
        let output = Solution::plus_one(digits);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let digits = vec![4, 3, 2, 1];
        let expected = vec![4, 3, 2, 2];
        let output = Solution::plus_one(digits);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let digits = vec![9];
        let expected = vec![1, 0];
        let output = Solution::plus_one(digits);
        assert_eq!(expected, output);
    }
}
