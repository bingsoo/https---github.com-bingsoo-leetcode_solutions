// O(N) solution using HashMap
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            match m.get(&(target - num)) {
                Some(&j) => return vec![i as i32, j as i32],
                None => {
                    m.insert(num, i as i32);
                }
            }
        }
        unreachable!()
    }
}
