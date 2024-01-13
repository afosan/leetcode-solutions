use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let s_cs = char_counts(&s);
        let t_cs = char_counts(&t);
        let chars = HashSet::<char>::from_iter(s.chars().chain(t.chars()));
        let mut diff = 0;

        for c in chars {
            let sc = *s_cs.get(&c).unwrap_or(&0usize) as i32;
            let tc = *t_cs.get(&c).unwrap_or(&0usize) as i32;

            diff += (sc - tc).abs();
        }

        diff / 2
    }
}

fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut cs = HashMap::<char, usize>::new();
    s.chars().for_each(|c| *cs.entry(c).or_insert(0) += 1);
    cs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "bab".to_string();
        let t = "aba".to_string();
        let expected = 1;
        let output = Solution::min_steps(s, t);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let s = "leetcode".to_string();
        let t = "practive".to_string();
        let expected = 5;
        let output = Solution::min_steps(s, t);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let s = "anagram".to_string();
        let t = "mangaar".to_string();
        let expected = 0;
        let output = Solution::min_steps(s, t);
        assert_eq!(expected, output);
    }
}
