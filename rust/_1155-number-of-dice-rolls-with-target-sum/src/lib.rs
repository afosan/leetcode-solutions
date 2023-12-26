struct Solution;

impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut memo = vec![vec![None; target as usize + 1]; n as usize + 1];
        val(n, k, target, &mut memo) as i32
    }
}

fn val(n: i32, k: i32, target: i32, memo: &mut Vec<Vec<Option<u64>>>) -> u64 {
    const N: u64 = 10u64.pow(9u32) + 7u64;

    if n == 0 && target == 0 {
        return 1;
    }

    if (n == 0 && target > 0) || target < 0 {
        return 0;
    }

    if let Some(v) = memo[n as usize][target as usize] {
        return v;
    }

    let v = (1..=k).map(|i| val(n - 1, k, target - i, memo) % N).sum::<u64>() % N;
    memo[n as usize][target as usize] = Some(v);

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 1;
        let k = 6;
        let target = 3;
        let expected = 1;
        let output = Solution::num_rolls_to_target(n, k, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let n = 2;
        let k = 6;
        let target = 7;
        let expected = 6;
        let output = Solution::num_rolls_to_target(n, k, target);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let n = 30;
        let k = 30;
        let target = 500;
        let expected = 222616187;
        let output = Solution::num_rolls_to_target(n, k, target);
        assert_eq!(expected, output);
    }
}
