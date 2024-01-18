struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // fibonacci
        let (mut a0, mut a1) = (1, 1);
        let mut i = 0;

        while i < n {
            (a0, a1) = (a1, a0 + a1);
            i += 1;
        }

        a0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 2;
        let expected = 2;
        let output = Solution::climb_stairs(n);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let n = 3;
        let expected = 3;
        let output = Solution::climb_stairs(n);
        assert_eq!(expected, output);
    }
}
