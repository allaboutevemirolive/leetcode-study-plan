// https://leetcode.com/problems/climbing-stairs/solutions/2907262/rust-recurrent-binomial-coefficient-formula/
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        //   (n)          n!
        // C (-) = --------------
        //   (k)    k! * (n - k)!
        //
        //   (n - 1)            (n - 1)!
        // C (-----) = ---------------------------- =
        //   (k + 1)    (k + 1)! * (n - 1 - k - 1)!
        //
        //                                 n! / n
        //           = --------------------------------------------------- =
        //              k! * (k + 1) * (n - k)! / ((n - k) * (n - k - 1))
        //
        //              (n - k) * (n - k - 1)           n!
        //           = ----------------------- * --------------- =
        //                    n * (k + 1)         k! * (n - k)!
        //
        //              (n - k) * (n - k - 1)      (n)
        //           = ----------------------- * C (-)
        //                    n * (k + 1)          (k)
        let mut prev_nck: i128 = 1;
        let mut result = 1;
        for k in 1..n {
            let prev_n = (n - k + 1) as i128;
            let prev_k = (k - 1) as i128;
            let nck = (prev_nck * (prev_n - prev_k) * (prev_n - prev_k - 1)) / (prev_n * (prev_k + 1));
            result += nck as i32;
            prev_nck = nck;
            if prev_k > prev_n {
                break;
            }
        }

        result
    }
}