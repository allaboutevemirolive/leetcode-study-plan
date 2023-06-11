// https://leetcode.com/problems/happy-number/solutions/2652844/rust-four-solutions-iterator-strips-digits-with-and/
use std::collections::HashSet;

/// Detect if iterator loops using two pointers or HashSet
///
/// Returns false if n < 1
impl Solution {
    /// See if happy using Brent fast and jumping pointers. Fastest
    pub fn is_happy(n: i32) -> bool { // From RosettaCode Java impl
        if n < 1 {return false}
        let mut hare = n as u32;
        let mut power = 1;  // steps limit, a power of 2
        loop {
            let tort = hare;
            for _λ in 1..=power { // steps taken past tort, lambda distance
                hare = next(hare);
                if tort == hare || hare == 1 {return hare == 1}
            }
            power *= 2; // start a new power of two
        }
    }

    /// See if happy using Floyd fast and slow pointers.
    pub fn is_happy_floyd(n: i32) -> bool {
        if n < 1 {return false}
        let mut slow = next(n as u32);
        let mut fast = next(next(n as u32));
        while fast != 1 && fast != slow {
            slow = next(slow);
            fast = next(next(fast));
        }
        fast == 1
    }

    /// See if happy using Brent fast and jumping pointers.
    pub fn is_happy_brent_2(n: i32) -> bool { // Traditional implementation
        if n < 1 {return false}
        let mut power = 1; // steps limit, a power of 2
        let mut λ = 1; // steps taken past tort, lambda for distance

        let mut tort = n as u32;
        let mut hare = next(n as u32);

        while tort != hare && hare != 1 {
            if λ == power { // start new power of two
                tort = hare;
                power *= 2;
                λ = 0;
            }
            hare = next(hare);
            λ += 1;
        }
        hare == 1
    }

    /// See if happy by remembering in HashSet. High const factor to O(n)
    pub fn is_happy_hash(n: i32) -> bool {
        if n < 1 {return false}
        let mut n = n as u32;
        let mut seen = HashSet::new();
        while n != 1 {
            if !seen.insert(n) { return false }
            n = next(n);
        }
        true
    }

}

/// Compute next by summing squares of digits.
fn next(n: u32) -> u32 {
    Digits(n).fold(0, |acc, d| acc + d * d)
}

/// Iterate through digits from low to high.
struct Digits(u32);
impl Iterator for Digits {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            n => {
                self.0 /= 10;
                Some(n % 10)
            }
        }
    }
}


/// Test with a few cases
#[cfg(test)]
mod tests {
    use super::*;

    const CASES : [(i32, bool); 5] = [
    (-1, false), (0, false), // invalid input
    (1, true), // base case
    (2, false), (19, true)
    ];

    macro_rules! test {
        ( $( $fn_name:ident ),* ) => {
            $(
            #[test]
            fn $fn_name() {
                for (arg, res) in CASES {
                    assert_eq!(Solution::$fn_name(arg), res, 
                        "arg {} res {}", arg, res);
                }
            }
            )*
        };
    }
    test!(is_happy, is_happy_brent_2, is_happy_floyd, is_happy_hash);

}

