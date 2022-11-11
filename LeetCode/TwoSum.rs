use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            if let Some(&i) = map.get(&(target - value)) {
                return vec![i as i32, index as i32];
            } else {
                map.insert(value, index as i32);
            }
        }

        vec![]

    }
}
