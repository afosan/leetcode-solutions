struct Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut memo = matrix[0].to_vec();
        for row in &matrix[1..] {
            let mut temp = vec![0; n];
            for i in 0..n {
                let possible = if i == 0 {
                    &memo[..2]
                } else if i == n - 1 {
                    &memo[n - 2..]
                } else {
                    &memo[i - 1..=i + 1]
                };
                temp[i] = row[i] + possible.iter().min().unwrap();
            }
            memo = temp;
        }

        *memo.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let matrix = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
        let expected = 13;
        let output = Solution::min_falling_path_sum(matrix);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let matrix = vec![vec![-19, 57], vec![-40, -5]];
        let expected = -59;
        let output = Solution::min_falling_path_sum(matrix);
        assert_eq!(expected, output);
    }
}
