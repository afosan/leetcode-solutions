struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut even_zero_odd_one = 0;
        let mut even_one_odd_zero = 0;
        s.chars().enumerate().for_each(|(i, c)| match c {
            '0' if i % 2 == 0 => {
                even_one_odd_zero += 1;
            },
            '1' if i % 2 == 0 => {
                even_zero_odd_one += 1;
            },
            '0' => {
                even_zero_odd_one += 1;
            },
            '1' => {
                even_one_odd_zero += 1;
            },
            _ => panic!("unexpected char")
        });

        even_zero_odd_one.min(even_one_odd_zero) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "0100".to_string();
        let expected = 1;
        let output = Solution::min_operations(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let s = "10".to_string();
        let expected = 0;
        let output = Solution::min_operations(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let s = "1111".to_string();
        let expected = 2;
        let output = Solution::min_operations(s);
        assert_eq!(expected, output);
    }
}
