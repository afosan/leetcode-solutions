use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut seen = HashSet::<u32>::new();
        let mut n = n as u32;
        
        loop {
            n = step(n);

            if seen.contains(&n) {
                return false;
            } else {
                seen.insert(n);
            }

            if n == 1 {
                return true;
            }
        }
    }
}

fn step(n: u32) -> u32 {
    let mut num = n;
    let mut res = 0;

    while num > 0 {
        let d = num % 10;
        num /= 10;
        res += d * d;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 19;
        let expected = true;
        let output = Solution::is_happy(n);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let n = 2;
        let expected = false;
        let output = Solution::is_happy(n);
        assert_eq!(expected, output);
    }
}
