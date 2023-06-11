// https://leetcode.com/problems/minimum-genetic-mutation/solutions/3529043/rust-newtype-bitmask-bfs/
use std::collections::VecDeque;
impl Solution {
    pub fn min_mutation(start_gene: String, end_gene: String, bank: Vec<String>) -> i32 {
        let start_gene = Gene::new(start_gene.as_bytes());
        let end_gene = Gene::new(end_gene.as_bytes());
        let mut bank = bank.iter().map(|gene| Gene::new(gene.as_bytes())).collect::<Vec<Gene>>();

        let mut queue = VecDeque::from([(start_gene, 0)]);

        while let Some((gene, steps)) = queue.pop_front() {
            if gene == end_gene {
                return steps;
            }

            bank = bank
            .into_iter()
            .filter_map(|bank_gene| {
                if gene.distance(&bank_gene) == 1 {
                    queue.push_back((bank_gene, steps+1));
                    None
                } else {
                    Some(bank_gene)
                }
            }).collect();
        }
        -1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Gene(u16);

impl Gene {
    pub fn new(bytes: &[u8]) -> Self {
        let mut bitmask = 0;
        for &byte in bytes.iter() {
            bitmask <<= 2;
            
            match byte {
                b'A' => {bitmask += 0}
                b'C' => {bitmask += 1}
                b'G' => {bitmask += 2}
                b'T' => {bitmask += 3}
                _ => {}
            }
        }
        Self(bitmask)
    }

    pub fn distance(&self, other: &Self) -> u8 {
        (0..16)
        .step_by(2)
        .map(|offset| {
            u8::from(
                (self.0 >> offset) & 3 
                != (other.0 >> offset) & 3)
        })
        .sum()
    }
}