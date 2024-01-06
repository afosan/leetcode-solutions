struct Solution;

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs = start_time.iter().zip(end_time).zip(profit).map(|((s, e), p)| (*s, e, p)).collect::<Vec<_>>();
        jobs.sort_by_key(|j| j.1);

        let mut pareto: Vec<(i32, i32)> = Vec::with_capacity(jobs.len());
        pareto.push((0, 0));
        let mut profit_without_job;
        let mut profit_with_job;

        for (s, e, p) in jobs {
            profit_without_job = pareto.last().unwrap().1;
            let prev_nonconflicting_job_index = match pareto.binary_search_by_key(&s, |&front| front.0) {
                Ok(i) => i,
                Err(i) => i - 1,
            };
            profit_with_job = pareto[prev_nonconflicting_job_index].1 + p;

            if profit_with_job > profit_without_job {
                let l = pareto.len();
                if pareto[l - 1].0 == e {
                    pareto[l - 1] = (e, profit_with_job.max(pareto[l - 1].1));
                } else {
                    pareto.push((e, profit_with_job));
                }
            }
        }

        pareto.last().unwrap().1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let start_time = vec![1, 2, 3, 3];
        let end_time = vec![3, 4, 5, 6];
        let profit = vec![50, 10, 40, 70];
        let expected = 120;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }

    #[test]
    fn case2() {
        let start_time = vec![1, 2, 3, 4, 6];
        let end_time = vec![3, 5, 10, 6, 9];
        let profit = vec![20, 20, 100, 70, 60];
        let expected = 150;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }

    #[test]
    fn case3() {
        let start_time = vec![1, 1, 1];
        let end_time = vec![2, 3, 4];
        let profit = vec![5, 6, 4];
        let expected = 6;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }

    #[test]
    fn case4() {
        let start_time = vec![6, 15, 7, 11, 1, 3, 16, 2];
        let end_time = vec![19, 18, 19, 16, 10, 8, 19, 8];
        let profit = vec![2, 9, 1, 19, 5, 7, 3, 19];
        let expected = 41;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }

    #[test]
    fn case5() {
        let start_time = vec![11,10,14,24,5,9,3,17,27,20];
        let end_time = vec![20,23,22,29,9,13,9,23,28,30];
        let profit = vec![2,2,3,2,4,3,4,4,7,2];
        let expected = 18;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }

    #[test]
    fn case6() {
        let start_time = vec![4,3,1,2,4,8,3,3,3,9];
        let end_time = vec![5,6,3,5,9,9,8,5,7,10];
        let profit = vec![9,9,5,12,10,11,10,4,14,7];
        let expected = 37;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }

    #[test]
    fn case7() {
        let start_time = vec![11,13,2,40,26,6,25,21,62,45];
        let end_time = vec![91,76,45,56,27,99,26,26,93,52];
        let profit = vec![80,31,56,80,52,57,12,59,70,4];
        let expected = 261;
        let output = Solution::job_scheduling(start_time, end_time, profit);
        assert_eq!(expected, output);
    }
}
