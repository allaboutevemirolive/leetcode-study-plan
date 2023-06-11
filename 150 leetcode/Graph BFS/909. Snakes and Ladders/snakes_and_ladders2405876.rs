// https://leetcode.com/problems/snakes-and-ladders/solutions/2405876/rust-dijkstra/
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const NO_PATH_FOUND: i32 = -1;

pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    assert!(board.len() > 0);
    assert_eq!(board.len(), board[0].len());

    let max = board.len() * board.len();

    let mut pq = BinaryHeap::new();
    let mut visited = HashSet::new();

    // Push the distance and the starting label
    pq.push((Reverse(0), 1));

    while let Some((Reverse(distance), label)) = pq.pop() {
        if !visited.insert(label) {
            continue;
        }

        if label == max {
            return distance;
        }

        for dice_roll in label + 1..=max.min(label + 6) {
            if let Some((r, c)) = to_index(&board, dice_roll) {
                let next = if board[r][c] > 0 {
                    // if it's a snake or a ladder -> follow it
                    board[r][c] as usize
                } else {
                    dice_roll
                };

                if !visited.contains(&next) {
                    pq.push((Reverse(distance + 1), next));
                }
            }
        }
    }

    NO_PATH_FOUND
}

fn to_index(board: &[Vec<i32>], label: usize) -> Option<(usize, usize)> {
    assert!(label > 0);
    let n = board.len();
    if label > n * n {
        return None;
    }

    let x = n * n - label;
    let r = x / n;
    let c = if (n - 1 - r) % 2 == 0 {
        n - (x - r * n) - 1
    } else {
        x - r * n
    };

    Some((r, c))
}