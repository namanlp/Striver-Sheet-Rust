// Question Link : https://leetcode.com/problems/missing-number/
// My solution Time complexity : O(n)
// My solution Space complexity : O(1)

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len()+1;

        // Take xor of all the numbers in range 0 to N
        let mut xor = 1;
        for i in 2..n{
            xor ^=i as i32;
        }

        // Take xor of all the numbers of the array
        // Then xor with numbers from 0 to N to get missing number
        for i in 0..n-1 {
            xor^=nums[i];
        }
        return xor;
    }
}