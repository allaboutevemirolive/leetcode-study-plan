// https://leetcode.com/problems/reverse-bits/solutions/1657141/reverse-bits-python-using-stack/
class Solution:
    def reverseBits(self, n: int) -> int:
        stack = []
        for i in range(32):
            stack.append(n%2)
            n=n//2
        
        res=0
        for i in range(32):
            res+= (int(math.pow(2,i))*stack.pop())          
        return res