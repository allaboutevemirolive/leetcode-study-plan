// https://leetcode.com/problems/sqrtx/solutions/2413753/actual-square-root-implementation-in-rust/

/*
Note: This is an algorithm for calculating square roots. The return type is modified to use an
i32 in order to truncate the value to meet the exercise requirement.
*/

pub fn my_sqrt(mut x: i32) -> i32 {
    let mut dec_places = 2; // use to increase precision, but beware of large numbers.
    let mut dec_point_reached = false;
    let mut dec_factor = 10.0;
    let mut dividend = 0_i64;
    let mut divisor = 0;
    let mut quotient = 0.0;
    let mut quotient_next_digit = 0;
    let mut reminder = 0;
    let mut rounding_factor = 1.0;
    let mut pairs = Vec::new();

    for _ in 0..(dec_places + 1) {
        rounding_factor *= 10.0;
    }

    while x > 99 {
        pairs.insert(0, x % 100);
        x /= 100;
    }
    pairs.insert(0, x);
    let mut pairs = pairs.iter();

    loop {
        match pairs.next() {
            Some(p) => {
                dividend = (reminder * 100 + p.clone()) as i64;
            }
            None => {
                dec_point_reached = true;
                dividend = (reminder * 100) as i64;
            }
        };

        quotient_next_digit = 0;
        divisor = divisor + (divisor % 10);
        while (divisor * 10 + quotient_next_digit) * quotient_next_digit <= dividend {
            quotient_next_digit += 1;
        }
        quotient_next_digit -= 1;
        divisor = divisor * 10 + quotient_next_digit;
        reminder = (dividend - (divisor as i64) * (quotient_next_digit as i64)) as i32;

        if !dec_point_reached {
            quotient = (quotient * 10.0) + (quotient_next_digit) as f64;
        } else {
            quotient += (quotient_next_digit) as f64 / (dec_factor);
            dec_factor *= 10.0;
            dec_places -= 1;
        }

        if dec_places < 1 {
            let res = ((quotient * rounding_factor) as i32) as f64 / rounding_factor;
            break res as i32;
        }
    }
}