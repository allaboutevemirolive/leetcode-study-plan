122. Best Time to Buy and Sell Stock II
Medium
11.2K
2.5K
Companies
You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

Find and return the maximum profit you can achieve.

 

Example 1:

Input: prices = [7,1,5,3,6,4]
Output: 7
Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
Total profit is 4 + 3 = 7.
Example 2:

Input: prices = [1,2,3,4,5]
Output: 4
Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
Total profit is 4.
Example 3:

Input: prices = [7,6,4,3,1]
Output: 0
Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.
 

Constraints:

1 <= prices.length <= 3 * 104
0 <= prices[i] <= 104


___


The task is to find the maximum profit that can be achieved by buying and selling stocks on multiple days.

Here's a simpler explanation:

You are given an array called `prices` where each element represents the price of a stock on a particular day. Your goal is to find the maximum profit you can achieve by buying and selling stocks on different days.

Unlike the previous question, you can perform multiple transactions on the same day. This means you can buy and sell the stock multiple times.

To solve this, you can perform the following steps:

1. Initialize a variable called `maxProfit` to 0. This variable will keep track of the maximum profit achieved.

2. Iterate over the `prices` array starting from the second day. For each day, check if the current price is higher than the previous day's price. If it is, add the difference between the current price and the previous price to `maxProfit`. This represents the profit you can achieve by buying the stock on the previous day and selling it on the current day.

3. After iterating through all the prices, `maxProfit` will hold the maximum profit that can be achieved.

Return `maxProfit` as the output of the function.

Note that you can perform multiple transactions on the same day if it is profitable. For example, if the prices are increasing continuously, you can buy the stock on the first day and sell it on the last day to maximize the profit.

If it is not possible to achieve any profit (i.e., the prices are continuously decreasing), return 0.

Consider the given constraints while solving the problem. The length of the `prices` array can be up to 3 * 10^4, and the values in the array are between 0 and 10^4.