use std::collections::BTreeMap;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut frequency : BTreeMap<i32, usize> = BTreeMap::new();
        let n = nums.len();

        for number in nums {
            let val = frequency.get(&number);
            if val.is_none() {
                frequency.insert(number, 1);
            }else {
                let val = *val.unwrap();
                frequency.insert(number, val+1);
            }
        }

        for (k, v) in frequency {
            if v>n/2 {
                return k;
            }
        }

        return -1;
    }
}