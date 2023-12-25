struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        num_decodings_char_vec(&s.chars().collect::<Vec<char>>(), 0usize, &mut vec![-1; s.len()])
    }
}

fn num_decodings_char_vec(chars: &[char], index: usize, mem: &mut Vec<i32>) -> i32 {
    if index == chars.len() {
        return 1;
    }

    if chars[index] == '0' {
        return 0;
    }

    if mem[index] != -1 {
        return mem[index];
    }

    let mut count = 0;

    count += num_decodings_char_vec(chars, index + 1, mem);
    if index + 1 < chars.len() && format!("{}{}", chars[index], chars[index + 1]) < "27".to_string() {
        count += num_decodings_char_vec(chars, index + 2, mem);
    }

    mem[index] = count;
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "12".to_string();
        let expected = 2;
        let output = Solution::num_decodings(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let s = "226".to_string();
        let expected = 3;
        let output = Solution::num_decodings(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let s = "06".to_string();
        let expected = 0;
        let output = Solution::num_decodings(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case4() {
        let s = "111111111111111111111111111111111111111111111".to_string();
        let expected = 1836311903;
        let output = Solution::num_decodings(s);
        assert_eq!(expected, output);
    }
}
