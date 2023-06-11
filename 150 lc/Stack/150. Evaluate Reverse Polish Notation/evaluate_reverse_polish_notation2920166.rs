// https://leetcode.com/problems/evaluate-reverse-polish-notation/solutions/2920166/rust-100-fast/
fn is_operator(t: &str) -> bool {
    match t {
        "+" => true,
        "-" => true,
        "*" => true,
        "/" => true,
        _ => false,
    }
}

fn get_result(op1: &String, operator: &str, op2: &String) -> String {
    let op1 = op1.parse::<i32>().unwrap();
    let op2 = op2.parse::<i32>().unwrap();
    let result;
    match operator {
        "+" => result = op1 + op2,
        "-" => result = op1 - op2,
        "*" => result = op1 * op2,
        "/" => result = op1 / op2,
        _ => panic!("Invalid operation"),
    };
    format!("{}", result)
}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
      let mut eval_stack = vec![];
        for t in tokens {
            if is_operator(&t) {
                // eval
                //println!("{:?}", eval_stack);
                let op2;
                match eval_stack.pop() {
                    Some(val) => {
                        op2 = val;
                    },
                    None => {
                        panic!("Invalid operation sequence");
                    }
                }
                let op1;
                match eval_stack.pop() {
                    Some(val) => {
                        op1 = val;
                    }, None => {
                        panic!("Invalid operation sequence");
                    }
                }
                let result = get_result(&op1, &t, &op2);
                eval_stack.push(result);
            }else {
                eval_stack.push(t);
            }
        }
        eval_stack.last().unwrap().parse::<i32>().unwrap()  
    }
}