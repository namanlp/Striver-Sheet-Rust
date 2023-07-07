// Question Link : https://leetcode.com/problems/merge-intervals/description/

// My solution Time complexity : O(NlogN)
// My solution Space complexity : O(N)

use std::cmp::max;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();

        let mut ans = Vec::with_capacity(intervals.len());

        let mut current =  intervals[0].clone();

        for i in 1..intervals.len() {
            if intervals[i][0] <= current[1] {
                current[1] = max(intervals[i][1], current[1]);
                continue;
            }
            ans.push(current.clone());
            current = intervals[i].clone();
        }
        ans.push(current.clone());

        return ans;
    }
}