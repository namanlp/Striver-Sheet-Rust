impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = n as usize;

        while i+j-1 >=0 {
            if j==0 {
                return;
            }
            if i==0 {
                nums1[j-1] = nums2[j-1];
                j-=1;
                continue;
            }
            if nums1[i-1] <= nums2[j-1] {
                nums1[i+j-1] = nums2[j-1];
                j-=1;
                continue;
            }
            nums1[i+j-1] = nums1[i-1];
            nums1[i-1] = 0;
            i-=1;
        }
    }
}
