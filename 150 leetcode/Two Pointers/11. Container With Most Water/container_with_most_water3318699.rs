// https://leetcode.com/problems/container-with-most-water/solutions/3318699/python3-golang-rust-two-pointers/
class Solution:
    def maxArea(self, height: List[int]) -> int:
        i, j = 0, len(height) - 1
        left_max, right_max = height[i], height[j]    
        water_counter_max, water_counter_cur = 0, 0
        
        while i <= j:
            left_max = max(left_max, height[i])
            right_max = max(right_max, height[j])
            
            if left_max <= right_max:
                water_counter_cur = height[i] * (abs(i - j))
                water_counter_max = max(water_counter_max, water_counter_cur)  
                i += 1
            else:
                water_counter_cur = height[j] * (abs(i - j))
                water_counter_max = max(water_counter_max, water_counter_cur)  
                j -= 1
                
            water_counter_max = max(water_counter_max, water_counter_cur)    
                
        return water_counter_max