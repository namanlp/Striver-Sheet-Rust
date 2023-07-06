use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {

        // We don't take max_sum as 0 due the negative array case.
        // For example, when array = [-10, -2, -3]
        let mut max_sum = nums[0];

        let mut current_sum = 0;

        for i in 0..nums.len() {
            current_sum += nums[i];
            max_sum = max(current_sum, max_sum);
            if current_sum < 0 {
                current_sum = 0;
            }
        }

        return max_sum;
    }
}