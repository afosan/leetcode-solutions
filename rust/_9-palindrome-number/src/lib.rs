struct Solution;

impl Solution {
    // pub fn is_palindrome(x: i32) -> bool {
    //     let s = x.to_string();

    //     s == s.chars().rev().collect::<String>()
    // }

    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let ds = digits(x as u32);
        
        ds == ds.iter().rev().copied().collect::<Vec<_>>()
    }
}

fn digits(n: u32) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut ds = vec![];
    let mut n = n;

    while n > 0 {
        ds.push((n % 10) as u8);
        n /= 10;
    }

    ds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let x = 121;
        let expected = true;
        let output = Solution::is_palindrome(x);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let x = -121;
        let expected = false;
        let output = Solution::is_palindrome(x);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let x = 10;
        let expected = false;
        let output = Solution::is_palindrome(x);
        assert_eq!(expected, output);
    }
}
