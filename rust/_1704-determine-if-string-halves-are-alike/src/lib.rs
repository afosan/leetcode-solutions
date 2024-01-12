struct Solution;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (first, second) = split_half(s.to_lowercase());
        count_vowels(first) == count_vowels(second)
    }
}

fn split_half(s: String) -> (String, String) {
    let t = s.len() / 2;
    (s[..t].to_string(), s[t..].to_string())
}

fn count_vowels(s: String) -> usize {
    s.chars().filter(|c| match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "book".to_string();
        let expected = true;
        let output = Solution::halves_are_alike(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let s = "textbook".to_string();
        let expected = false;
        let output = Solution::halves_are_alike(s);
        assert_eq!(expected, output);
    }
}
