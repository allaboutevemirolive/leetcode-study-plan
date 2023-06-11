// https://leetcode.com/problems/evaluate-division/solutions/3543346/hint-floating-number-comparison-may-occur-unexpectedly-problem-in-rust/
use std::collections::{ HashMap, HashSet };
impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();
        for (index, equation) in equations.iter().enumerate() {
            graph.entry(equation[0].clone()).or_default().insert(equation[1].clone(), values[index]);
            graph.entry(equation[1].clone()).or_default().insert(equation[0].clone(), 1.0 / values[index]);
        }
        
        let mut results = vec![];
        let mut visited: HashSet<String> = HashSet::new();
        for query in queries.iter() {
            let result = Self::dfs(query[0].clone(), &query[1], 1.0, &graph, &mut visited);
            match result {
                Some(result) => results.push(result),
                None => results.push(-1.0),
            };
            
            visited.clear();
        }

        return results
    }

    fn dfs(start: String, goal: &String, value: f64, graph: &HashMap<String, HashMap<String, f64>>, visited: &mut HashSet<String>) -> Option<f64> {
        if !graph.contains_key(&start) {
            return None
        }
        if start.eq(goal) {
            return Some(value)
        }
        visited.insert(start.clone());

        for (neighbor, neighbor_value) in graph[&start].iter() {
            if visited.contains(neighbor) {
                continue
            }

            let result = Self::dfs(neighbor.clone(), goal, value * neighbor_value, graph, visited);
            match result {
                Some(result) => {
                    return Some(result)
                },
                None => {
                    visited.remove(neighbor);
                },
            }
        }

        return None
    }
}