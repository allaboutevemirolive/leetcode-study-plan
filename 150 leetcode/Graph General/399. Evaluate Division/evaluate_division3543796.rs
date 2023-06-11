// https://leetcode.com/problems/evaluate-division/solutions/3543796/rust-dfs/
use std::collections::{HashSet, HashMap};
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: HashMap<String, Vec<(String, f64)>> = HashMap::new();
        for i in 0..equations.len() {
            graph
                .entry(equations[i][0].clone())
                .or_default()
                .push((equations[i][1].clone(), values[i]));
            graph
                .entry(equations[i][1].clone())
                .or_default()
                .push((equations[i][0].clone(), 1.0 / values[i]));
        }
        let mut res = vec![];
        let mut visited = HashSet::new();
        for query in queries {
            if !graph.contains_key(&query[0]) || !graph.contains_key(&query[1]) {
                res.push(-1.0);
                continue;
            }
            let r = Self::dfs(&graph, &mut visited, &query[0], &query[1]);
            visited.clear();
            res.push(if !r.is_nan() { r } else { -1.0 });
        }
        return res;
    }
    fn dfs(
        graph: &HashMap<String, Vec<(String, f64)>>,
        visited: &mut HashSet<String>,
        s: &str,
        e: &str,
    ) -> f64 {
        if s == e {
            return 1.0;
        }
        if visited.contains(s) {
            return f64::NAN;
        }
        visited.insert(s.to_string());
        for n in &graph[s] {
            let r = Self::dfs(graph, visited, &n.0, e);
            if !r.is_nan() {
                return n.1 * r;
            }
        }
        visited.remove(s);
        return f64::NAN;
    }
}