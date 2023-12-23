struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                },
                ')' | ']' | '}' => {
                    match (c, stack.pop()) {
                        (')', Some('(')) => {}, 
                        (']', Some('[')) => {}, 
                        ('}', Some('{')) => {},
                        _ => { return false; },
                    }
                },
                _ => {},
            }
        }

        stack.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "()".to_string();
        let expected = true;
        let output = Solution::is_valid(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let s = "()[]{}".to_string();
        let expected = true;
        let output = Solution::is_valid(s);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let s = "(]".to_string();
        let expected = false;
        let output = Solution::is_valid(s);
        assert_eq!(expected, output);
    }
}
