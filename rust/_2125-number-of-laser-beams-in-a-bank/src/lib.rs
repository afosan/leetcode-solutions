struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let v = bank.iter().map(|row| row.chars().filter(|c| *c == '1').count()).enumerate().filter(|(_, count)| *count > 0).collect::<Vec<_>>();
        v.windows(2).map(|pair| pair[0].1 * pair[1].1).sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let bank = vec!["011001", "000000", "010100", "001000"].iter().map(|v| v.to_string()).collect::<Vec<String>>();
        let expected = 8;
        let output = Solution::number_of_beams(bank);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let bank = vec!["000", "111", "000"].iter().map(|v| v.to_string()).collect::<Vec<String>>();
        let expected = 0;
        let output = Solution::number_of_beams(bank);
        assert_eq!(expected, output);
    }
}
