// https://leetcode.com/problems/letter-combinations-of-a-phone-number/solutions/2274208/rust-loop-solution-preallocating-result/
pub fn letter_combinations(digits: String) -> Vec<String> {
	let chars: Vec<Vec<u8>> = digits
		.into_bytes()
		.into_iter()
		.map(|d| {
			match d {
				b'2' => b"abc".to_vec(),
				b'3' => b"def".to_vec(),
				b'4' => b"ghi".to_vec(),
				b'5' => b"jkl".to_vec(),
				b'6' => b"mno".to_vec(),
				b'7' => b"pqrs".to_vec(),
				b'8' => b"tuv".to_vec(),
				b'9' => b"wxyz".to_vec(),
				_    => b"".to_vec()
			}
		})
		.collect();
	let out_len = if chars.len() > 0 {
		chars.iter().map(|cs| cs.len()).product()
	} else {
		0
	};
	let mut out: Vec<Vec<u8>> = vec![Vec::with_capacity(chars.len()); out_len];
	let mut rep_len = out.len();
	for cs in chars {
		let crnt_len = cs.len();
		rep_len /= crnt_len;
		for i in 0..out.len() {
			out[i].push(cs[(i/rep_len) % crnt_len])
		}
	}
	out.iter().map(|cs| String::from_utf8(cs.clone()).unwrap()).collect()
}