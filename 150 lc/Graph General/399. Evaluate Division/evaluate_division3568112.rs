// https://leetcode.com/problems/evaluate-division/solutions/3568112/rust-overengineered-unionfind/
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::iter::FromIterator;
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut uf = equations
        .iter()
        .zip(values.into_iter())
        .map(|(equation, ratio)| {
            Equation {
                num: &equation[0],
                denom: &equation[1],
                ratio,
            }
        })
        .collect::<UnionFind<'_>>();

        queries
        .iter()
        .map(|query| Query {
            num: &query[0],
            denom: &query[1]
        })
        .scan(uf, |uf, query| uf.query(query).or(Some(-1.0)))
        .collect()
    }
}

#[derive(Debug, Clone, Copy)]
struct Equation<'a> {
    num: &'a str,
    denom: &'a str,
    ratio: f64,
}

#[derive(Debug, Clone, Copy)]
struct Query<'a> {
    num: &'a str,
    denom: &'a str,
}

#[derive(Debug, Clone, Copy)]
struct UnionFindNode {
    root_index: usize,
    ratio: f64,
}

impl UnionFindNode {
    fn new(i: usize) -> Self {
        Self {
            root_index: i,
            ratio: 1.0,
        }
    }
}

#[derive(Debug)]
struct UnionFind<'a> {
    node_index: HashMap<&'a str, usize>,
    nodes: Vec<UnionFindNode>,
}

impl<'a> FromIterator<Equation<'a>> for UnionFind<'a> {
    fn from_iter<T>(iter: T) -> Self 
        where T: IntoIterator<Item = Equation<'a>> {
        iter
        .into_iter()
        .fold(UnionFind::new(), |mut uf, equation| {
            uf.union(equation);
            uf
        })
    }
}

impl<'a> UnionFind<'a> {
    pub fn new() -> Self {
        Self {
            node_index: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn get_or_insert_index(&mut self, node_name: &'a str) -> usize {
        match self.node_index.entry(node_name) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                let i = self.nodes.len();
                e.insert(i);
                self.nodes.push(UnionFindNode::new(i));
                i
            }
        }
    }

    fn get_index(&self, node_name: &str) -> Option<usize> {
        self.node_index.get(&node_name).copied()
    }

    fn find_root(&mut self, x: usize) -> UnionFindNode {
        if self.nodes[x].root_index != x {
            let x_parent = self.find_root(self.nodes[x].root_index);
            self.nodes[x].root_index = x_parent.root_index;
            self.nodes[x].ratio *= x_parent.ratio;
        }
        self.nodes[x]
    }

    fn union_index(&mut self, num: usize, denom: usize, ratio: f64) {
        let num_node = self.find_root(num);
        let denom_node = self.find_root(denom);
        if num_node.root_index == denom_node.root_index {
            return;
        } else {
            let new_root_index = denom_node.root_index;
            self.nodes[num_node.root_index] = UnionFindNode {
                root_index: denom_node.root_index,
                ratio: ratio * (denom_node.ratio / num_node.ratio),
            };
        }
    }

    pub fn union(&mut self, equation: Equation<'a>) {
        let num = self.get_or_insert_index(equation.num);
        let denom = self.get_or_insert_index(equation.denom);
        self.union_index(num, denom, equation.ratio);
    }

    fn query_index(&mut self, num: usize, denom: usize) -> Option<f64> {
        let num_node = self.find_root(num);
        let denom_node = self.find_root(denom);
        (num_node.root_index == denom_node.root_index)
        .then(|| num_node.ratio / denom_node.ratio)
    }

    pub fn query(&mut self, query: Query<'a>) -> Option<f64> {
        let num_index = self.get_index(query.num)?;
        let denom_index = self.get_index(query.denom)?;
        self.query_index(num_index, denom_index)
    }
}