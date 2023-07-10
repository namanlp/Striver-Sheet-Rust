// Question Link : https://leetcode.com/problems/find-the-duplicate-number/
// My solution Time complexity : O(n)
// My solution Space complexity : O(1)


impl Solution {
    // My solution uses hash table for the solution.
    // But instead of using new vector, we store the numbers at index in original array itself.

    // We know that all the elements are from 1 to n
    // So, we can use n+1 as counter to store the frequency of the elements in the table itself.
    pub fn find_duplicate(mut nums: Vec<i32>) -> i32 {
        let num = nums.len()+3;

        for i in 0..nums.len() {
            let elem = nums[i] as usize%num -1;
            if nums[elem] as usize > num  {
                return elem as i32+1;
            }
            nums[elem] += num as i32;
        }
        return 0;
    }
}
