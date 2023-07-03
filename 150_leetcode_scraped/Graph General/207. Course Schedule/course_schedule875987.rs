// https://leetcode.com/problems/course-schedule/solutions/875987/rust-topological-sort-time-o-e-v-space-o-e-v/

use std::collections::HashMap;

#[derive(Clone)]
enum State {
    Unknown,
    Visiting,
    Visited,
}


impl Solution {
      // Time O(E + V) Space O(E + V)
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // course -> list of next courses
        let mut course_map: HashMap<usize, Vec<usize>> = HashMap::new();

        // build the graph first
        for relation in prerequisites {
            // relation[0] depends on relation[1]
            let edges = course_map
                .entry(relation[1] as usize)
                .or_insert(vec![relation[0] as usize]);
            edges.push(relation[0] as usize);
        }

        let mut state = vec![State::Unknown; num_courses as usize];

        for curr_course in 0..num_courses as usize {
            if Self::is_cyclic(curr_course, &mut state, &course_map) {
                return false;
            }
        }

        true
    }

    fn is_cyclic(
        curr_course: usize,
        state: &mut Vec<State>,
        course_map: &HashMap<usize, Vec<usize>>,
    ) -> bool {
        match state[curr_course] {
            State::Visiting => true,
            State::Visited => false,
            State::Unknown => {
                state[curr_course] = State::Visiting;

                if let Some(courses) = course_map.get(&curr_course) {
                    // Topological Sort, to visit all its children first.
                    for next_course in courses.iter() {
                        if Self::is_cyclic(*next_course, state, course_map) {
                            return true;
                        }
                    }
                }

                state[curr_course] = State::Visited;

                false
            }
        }
    }

}
