struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let ss = format!("{s}{s}");
        ss[1..ss.len() - 1].contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "abab".to_string();
        let expected = true;
        let output = Solution::repeated_substring_pattern(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let s = "aba".to_string();
        let expected = false;
        let output = Solution::repeated_substring_pattern(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let s = "abcabcabcabc".to_string()        ;
        let expected = true;
        let output = Solution::repeated_substring_pattern(s);
        assert_eq!(expected, output);
    }
}
