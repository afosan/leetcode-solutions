use std::collections::HashMap;
use rand::Rng;

struct RandomizedSet {
    mp: HashMap<i32, usize>,
    vc: Vec<i32>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        RandomizedSet {
            mp: HashMap::new(),
            vc: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.mp.contains_key(&val) {
            false
        } else {
            self.vc.push(val);
            self.mp.insert(val, self.vc.len() - 1);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        // println!("{:?} {:?}", self.mp, self.vc);
        match self.mp.remove(&val) {
            Some(index) => {
                let last_index = self.vc.len() - 1;
                if last_index != index {
                    let last_value = self.vc[last_index];
                    self.vc.swap(index, last_index);
                    self.mp.insert(last_value, index);
                }
                self.vc.pop();
                // println!("{:?} {:?}", self.mp, self.vc);
                true
            },
            None => {
                false
            },
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..self.vc.len());
        self.vc[r]
    }
}
