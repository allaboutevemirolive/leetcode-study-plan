// https://leetcode.com/problems/rotate-image/solutions/3344375/rust-simple-implementation/
impl Solution {
    fn rotate_frame(apex: (usize, usize), n: usize, frame: &mut Vec<Vec<i32>>) {
        if n <= 1 {
            return;
        }
        for i in 0..n-1 {
            // apexes values for n = 3
            // 0,0 > 0,3 > 3,3 > 3,0
            // 0,1 > 1,3 > 3,2 > 2,0
            // 0,2 > 2,3 > 3,1 > 1,0
            let apexes = vec![
                (apex.0, apex.1 + i),
                (apex.0 + i, apex.1 + n - 1),
                (apex.0 + n - 1, apex.1 + n - 1 - i),
                (apex.0 + n - 1 - i, apex.1),
            ];
            let mut values = Vec::with_capacity(4);
            for ap in apexes.iter() {
                values.push(frame[ap.0][ap.1]);
            }
            for ai in 0..4 {
                let ap = apexes[ai];
                let mut value_index = ai as i32 - 1;
                if value_index < 0 {
                    value_index = 3
                }
                let value = values[value_index as usize];
                frame[ap.0][ap.1] = value;
            }
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut n = matrix.len();
        let num_of_frames = (n as f64 / 2.0).floor() as i32;
        'rotation: for i in 0..num_of_frames {
            let frame_apex = (i as usize, i as usize);
            Self::rotate_frame(frame_apex, n, matrix);
            n -= 2;
            if n <= 1 {
                break 'rotation
            }
        }
    }
}