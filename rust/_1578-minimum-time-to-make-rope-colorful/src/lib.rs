struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut prev_c: Option<char> = None;
        let mut acc_sum: i32 = 0;
        let mut acc_max: i32 = 0;
        let mut i_time: i32;
        let mut total = 0i32;

        for (i, c) in colors.chars().enumerate() {
            i_time = needed_time[i];
            if let Some(pc) = prev_c {
                if pc == c {
                    acc_sum += i_time;
                    acc_max = acc_max.max(i_time);
                } else {
                    total += acc_sum - acc_max;
                    acc_sum = i_time;
                    acc_max = i_time;
                    prev_c = Some(c);
                }
            } else {
                prev_c = Some(c);
                acc_sum = i_time;
                acc_max = i_time;
            }
        }
        total += acc_sum - acc_max;

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let colors = "abaac".to_string();
        let needed_time = vec![1, 2, 3, 4, 5];
        let expected = 3;
        let output = Solution::min_cost(colors, needed_time);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let colors = "abc".to_string();
        let needed_time = vec![1, 2, 3];
        let expected = 0;
        let output = Solution::min_cost(colors, needed_time);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let colors = "aabaa".to_string();
        let needed_time = vec![1, 2, 3, 4, 1];
        let expected = 2;
        let output = Solution::min_cost(colors, needed_time);
        assert_eq!(expected, output);
    }
}
