// https://leetcode.com/problems/valid-anagram/solutions/3045297/top-12-by-speed-python/

class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t):
            return False
        
        hash_s = self.createHash(s)
        
        for char in t:
            if char not in hash_s:
                return False
            else:
                hash_s[char] -= 1
                if hash_s[char] == 0:
                    del hash_s[char]
        
        return True
        
    @staticmethod
    def createHash(string: str):
        hash_map = {}
        for char in string:
            if char in hash_map:
                hash_map[char] += 1
            else:
                hash_map[char] = 1
        
        return hash_map