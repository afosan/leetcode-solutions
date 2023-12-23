struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let s = &strs[0];
        let l = (0usize..s.len()).take_while(|i| strs[1..].iter().map(|w| w.chars().nth(*i)).all(|c| c == s.chars().nth(*i))).count();
        s[..l].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let strs = vec!["flower", "flow", "flight"].into_iter().map(|w| String::from(w)).collect::<Vec<String>>();
        let expected = "fl".to_string();
        let output = Solution::longest_common_prefix(strs);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let strs = vec!["dog", "racecar", "car"].into_iter().map(|w| String::from(w)).collect::<Vec<String>>();
        let expected = "".to_string();
        let output = Solution::longest_common_prefix(strs);
        assert_eq!(expected, output);
    }
}
