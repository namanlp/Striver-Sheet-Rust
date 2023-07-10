impl Solution {
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
