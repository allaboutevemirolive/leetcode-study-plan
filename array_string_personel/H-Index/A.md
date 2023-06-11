274. H-Index
Medium
200
55
Companies
Given an array of integers citations where citations[i] is the number of citations a researcher received for their ith paper, return the researcher's h-index.

According to the definition of h-index on Wikipedia: The h-index is defined as the maximum value of h such that the given researcher has published at least h papers that have each been cited at least h times.

 

Example 1:

Input: citations = [3,0,6,1,5]
Output: 3
Explanation: [3,0,6,1,5] means the researcher has 5 papers in total and each of them had received 3, 0, 6, 1, 5 citations respectively.
Since the researcher has 3 papers with at least 3 citations each and the remaining two with no more than 3 citations each, their h-index is 3.
Example 2:

Input: citations = [1,3,1]
Output: 1
 

Constraints:

n == citations.length
1 <= n <= 5000
0 <= citations[i] <= 1000

___



The given problem is about determining a researcher's h-index based on their citation records.

Here's a simplified explanation of the problem:

- You are given an array of integers called `citations`.
- Each element `citations[i]` represents the number of citations the researcher received for their ith paper.
- The goal is to find the researcher's h-index, which is a measure of their research impact.
- The h-index is defined as the maximum value of `h` such that the researcher has published at least `h` papers that have each been cited at least `h` times.

For example:
- If `citations = [3, 0, 6, 1, 5]`, it means the researcher has 5 papers in total with citation counts of 3, 0, 6, 1, and 5 respectively.
- Among these papers, the researcher has 3 papers with at least 3 citations each, and the remaining two papers have no more than 3 citations each.
- Therefore, the researcher's h-index is 3.

The task is to determine the researcher's h-index based on their citation records provided in the array `citations`.

Constraints:
- The length of the array `citations` is between 1 and 5000.
- The values in `citations` are non-negative integers between 0 and 1000.


___



Certainly! The h-index is a measure that quantifies the impact and productivity of a researcher's work. It is commonly used in academic fields to evaluate the significance of their publications.

Here's a simple explanation of the h-index:

- The h-index is based on the number of citations a researcher's papers receive.
- Imagine you have a list of a researcher's papers, ordered by the number of citations each paper has received, from highest to lowest.
- The h-index is the largest number `h` such that the researcher has at least `h` papers that have each been cited at least `h` times.
- In other words, if a researcher has an h-index of 5, it means they have published at least 5 papers that have each received at least 5 citations.

Here's an example to illustrate the concept:

Consider a researcher who has published 8 papers and their citations are as follows:
Paper 1: 10 citations
Paper 2: 8 citations
Paper 3: 5 citations
Paper 4: 4 citations
Paper 5: 3 citations
Paper 6: 3 citations
Paper 7: 2 citations
Paper 8: 1 citation

If we sort the papers by the number of citations in descending order, we have:
10, 8, 5, 4, 3, 3, 2, 1

To find the h-index:
- We start from the highest citation count, which is 10.
- We check how many papers have at least 10 citations. In this case, only 1 paper has 10 or more citations.
- We then move to the next highest citation count, which is 8.
- We check how many papers have at least 8 citations. In this case, 2 papers have 8 or more citations.
- We continue this process until we find the largest number `h` for which there are `h` papers with `h` or more citations.

In this example, the researcher's h-index is 3 because they have at least 3 papers that have each been cited at least 3 times.

The h-index provides a measure that balances the quantity and impact of a researcher's work. It indicates both the number of highly cited papers and the researcher's ability to maintain consistent impact across their publications.