// https://leetcode.com/problems/container-with-most-water/solutions/3275425/rust-dnc-method/
    pub fn find_min(a:i32,b:i32)->i32{
        if (a>b){
            return b;
        }
        a
    }
    pub fn find_max(a:i32, b:i32)->i32{
        if (a>b){
            return a;
        }
        b
    }
impl Solution {

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area:i32 =0;
        let mut l :i32 = 0 ;
        let mut r: i32 = height.len() as i32;
        r=r-1;
        while l<=r{
            max_area = find_max(max_area, find_min(height[l as usize],height[r as usize])*(r-l));
            if height[l as usize]>height[r as usize]{
                r= r-1;
            }
            else{
                l= l+1;
            }
        }
        max_area
    }
}