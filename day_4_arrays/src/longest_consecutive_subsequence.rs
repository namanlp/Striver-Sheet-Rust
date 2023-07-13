// Question Link : https://leetcode.com/problems/longest-consecutive-sequence/

use std::cmp::max;
use std::collections:: HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut st = HashSet::new();

        for i in 0..nums.len() {
            st.insert(nums[i]);
        }

        if nums.len() <= 1 {
            return nums.len() as i32;
        }

        let mut ans = 1;

        for i in 0..nums.len() {

            // If set already contains previous, simply continue.
            let previous = nums[i] - 1;
            if st.contains(&previous ) {
                continue;
            }

            // Else, start sequence
            let mut current_length = 1;
            let mut current_element = nums[i];
            while st.contains(&current_element) {
                ans=max(current_length, ans);
                current_element+=1;
                current_length+=1;
            }
        }

        ans
    }
}