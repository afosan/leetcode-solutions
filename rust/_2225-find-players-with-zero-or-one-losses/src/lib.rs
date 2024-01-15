use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut loss_counts = HashMap::<i32, usize>::new();

        for m in matches {
            let w = m[0];
            let l = m[1];

            *loss_counts.entry(w).or_insert(0) += 0;
            *loss_counts.entry(l).or_insert(0) += 1;
        }

        let mut no_loss = vec![];
        let mut one_loss = vec![];

        loss_counts.iter().for_each(|(k, v)| {
            if *v == 0 {
                no_loss.push(*k);
            }
            if *v == 1 {
                one_loss.push(*k);
            }
        });

        no_loss.sort_unstable();
        one_loss.sort_unstable();

        vec![no_loss, one_loss]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
