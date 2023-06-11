// https://leetcode.com/problems/evaluate-division/solutions/3546156/rust-python-graph-search-detailed-explanation/
use std::collections::HashMap;
use std::collections::HashSet;

struct Edge {
  node: String,
  val: f64,
}

impl Solution {

  fn build_graph(data: Vec<Vec<String>>, vals: Vec<f64>) -> HashMap<String, Vec<Edge>> {
    let mut g = HashMap::new();
    for i in 0 .. data.len() {
      g.entry(data[i][0].clone()).or_insert(Vec::new()).push(Edge{
        node: data[i][1].clone(),
        val: vals[i],
      });

      g.entry(data[i][1].clone()).or_insert(Vec::new()).push(Edge{
        node: data[i][0].clone(),
        val: 1. / vals[i],
      });
    }

    return g;
  }

  fn search(v1: String, v2: String, g: &HashMap<String, Vec<Edge>>) -> f64 {
    let mut frontier = vec![Edge{node: v1.clone(), val: 1.}];
    let mut seen = HashSet::new();
    seen.insert(&v1);

    while !frontier.is_empty() {
      let mut new_frontier = Vec::new();
      for v in frontier {
        if !g.contains_key(&v.node) { continue; }

        for n in &g[&v.node] {
          if n.node == v2 {
            return v.val * n.val;
          }

          if !seen.contains(&n.node) {
            seen.insert(&n.node);
            new_frontier.push(Edge{
              node: n.node.clone(),
              val: v.val * n.val,
            });
          }
        }
      }

      frontier = new_frontier;
    }

    return -1.;
  }

  fn answer_query(q: Vec<String>, g: &HashMap<String, Vec<Edge>>) -> f64 {
    if q[1] == q[0] {
      if g.contains_key(&q[1]) {
        return 1.;
      }

      return -1.;
    }

    return Self::search(q[0].clone(), q[1].clone(), g);
  }

  pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
    let g = Self::build_graph(equations, values);
    let mut res = Vec::with_capacity(queries.len());
    for q in queries {
      res.push(Self::answer_query(q, &g));
    }

    return res;
  }
}