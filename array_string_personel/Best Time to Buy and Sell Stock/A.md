121. Best Time to Buy and Sell Stock
Easy
25.8K
811
Companies

You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

 

Example 1:

Input: prices = [7,1,5,3,6,4]
Output: 5
Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.


Example 2:

Input: prices = [7,6,4,3,1]
Output: 0
Explanation: In this case, no transactions are done and the max profit = 0.
 

Constraints:

1 <= prices.length <= 105
0 <= prices[i] <= 104


___



The task is to find the maximum profit that can be achieved by buying and selling a stock represented by an array of prices.

Here's a simpler explanation:

You are given an array called `prices` where each element represents the price of a stock on a particular day. Your goal is to find the maximum profit you can achieve by buying one stock and selling it on a different day in the future.

To maximize the profit, you need to find the lowest price to buy the stock and the highest price to sell it. However, you cannot sell the stock before buying it.

To solve this, you can perform the following steps:

1. Initialize two variables, `minPrice` and `maxProfit`, to track the lowest price and maximum profit seen so far. Set `minPrice` to the maximum possible value and `maxProfit` to 0.

2. Iterate over the `prices` array. For each price, compare it with `minPrice`. If the current price is lower than `minPrice`, update `minPrice` to the current price. This ensures that `minPrice` always represents the lowest price seen so far.

3. Calculate the potential profit by subtracting `minPrice` from the current price. If the calculated profit is greater than `maxProfit`, update `maxProfit` to the new value. This ensures that `maxProfit` always represents the maximum profit achievable.

4. After iterating through all the prices, `maxProfit` will hold the maximum profit that can be achieved.

Return `maxProfit` as the output of the function.

If it is not possible to achieve any profit (i.e., the prices are continuously decreasing), return 0.

Consider the given constraints while solving the problem. The length of the `prices` array can be up to 10^5, and the values in the array are between 0 and 10^4.