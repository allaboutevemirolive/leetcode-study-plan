// https://leetcode.com/problems/sqrtx/solutions/2514381/rust-amusingly-solution/
impl Solution
{
    pub fn my_sqrt(x: i32) -> i32
    {
        let mut irm_st = x as i64;
        let mut inc = 1_i64;

        loop
        {
            let f = x as i64 - irm_st * irm_st;
            if f < 0
                { irm_st >>= 1; }
            else if f > irm_st * 2
            {
                let mut _irm_st = irm_st;

                loop
                {
                    _irm_st = irm_st + irm_st / inc;
                    inc += 1;

                    if _irm_st * _irm_st <= x as i64
                    {
                        irm_st = _irm_st;
                        break;
                    }
                }
            }
            else
                { return irm_st as i32; }
        }
    }
}