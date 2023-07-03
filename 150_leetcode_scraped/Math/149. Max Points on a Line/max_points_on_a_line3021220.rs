// https://leetcode.com/problems/max-points-on-a-line/solutions/3021220/97-faster-python-solution/
class Solution:
    def maxPoints(self, points: List[List[int]]) -> int:
        length = len(points)
        if length < 2:
            return length

        max_points = 1

        for i in range(length - 1): 
            if max_points > length - i:
                break
                
            plane_points = {}

            for j in range(i + 1, length):
                x1, y1 = points[i]
                x2, y2 = points[j]

                if x1 == x2:
                    k = float('inf')
                else:
                    k = (y1 - y2) / (x1 - x2)

                plane_points[k] = plane_points[k] + 1 if k in plane_points else 2
                max_points = max(max_points, plane_points[k]) 

        return max_points
