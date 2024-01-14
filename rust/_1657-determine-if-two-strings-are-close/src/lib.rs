use std::collections::{HashMap, HashSet};

fn char_counts(w: &str) -> HashMap<char, usize> {
    let mut ccs = HashMap::<char, usize>::new();
    w.chars().for_each(|c| *ccs.entry(c).or_insert(0) += 1);

    ccs
}

struct Solution;

impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let ccs1 = char_counts(&word1);
        let ccs2 = char_counts(&word2);

        let keys1 = ccs1.keys().collect::<HashSet<_>>();
        let keys2 = ccs2.keys().collect::<HashSet<_>>();
        let mut values1 = ccs1.values().collect::<Vec<_>>();
        let mut values2 = ccs2.values().collect::<Vec<_>>();

        values1.sort_unstable();
        values2.sort_unstable();

        keys1 == keys2 && values1 == values2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let word1 = "abc".to_string();
        let word2 = "bca".to_string();
        let expected = true;
        let output = Solution::close_strings(word1, word2);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let word1 = "a".to_string();
        let word2 = "aa".to_string();
        let expected = false;
        let output = Solution::close_strings(word1, word2);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let word1 = "cabbba".to_string();
        let word2 = "abbccc".to_string();
        let expected = true;
        let output = Solution::close_strings(word1, word2);
        assert_eq!(expected, output);
    }
}
