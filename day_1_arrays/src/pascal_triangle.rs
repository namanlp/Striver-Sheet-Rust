// Question Link : https://takeuforward.org/data-structure/program-to-generate-pascals-triangle/

// My solution Time Complexity : O( N^2 )
// My solution Space Complexity : O( N^2 )

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let n = num_rows as usize;
        let mut ans = Vec::with_capacity(n+3);
        ans.push(vec![1]);
        for i in 1..n {
            let mut v = Vec::with_capacity(i + 3);
            v.push(1);

            if i >= 2 {
                for j in 0..i-1 {
                    v.push(ans[i-1][j] + ans[i-1][j+1]);
                }
            }
            v.push(1);
            ans.push(v);
        }
        return ans;
    }
}