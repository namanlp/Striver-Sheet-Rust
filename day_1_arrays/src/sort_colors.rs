// Question Link : https://leetcode.com/problems/sort-colors/

// My solution time complexity : O(N)
// My solution space complexity : O(1)

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zero = 0;
        let mut one = 0;
        let mut two = 0;
        for i in 0..nums.len() {
            if nums[i]&1 == 1 {
                one+=1;
            }else if nums[i]&2 == 2 {
                two+=1;
            }else {
                zero+=1;
            }
        }
        let mut i = 0;
        while zero>0 {
            nums[i] = 0;
            i += 1;
            zero-=1;
        }
        while one>0 {
            nums[i] = 1;
            i += 1;
            one-=1;
        }
        while two>0 {
            nums[i] = 2;
            i += 1;
            two-=1;
        }
    }
}