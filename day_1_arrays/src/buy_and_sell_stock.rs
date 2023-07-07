// Question Link : https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use std::cmp::max;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut ans = 0;

        // Calculate the postfix maximum price array
        let mut max_price = vec![0; n];
        max_price[n-1] = prices[n-1];

        for i in (0..n-1).rev() {
            max_price[i] = max(max_price[i+1], prices[i]);
        }

        // for each stock, we compute max profit
        // By subtracting the value of stock on that day from maximum value of stock
        // after that day.

        for i in 0..n {
            ans = max(ans, max_price[i] - prices[i]);
        }

        return ans;
    }
}