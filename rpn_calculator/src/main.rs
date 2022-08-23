use std::collections::VecDeque;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut deq = VecDeque::new();
    for i in inputs {
        match i {
            CalculatorInput::Value(v) => deq.push_back(*v),
            _ => {
                if deq.len() < 2 {
                    return None;
                } else {
                    let v1 = deq.pop_back().unwrap();
                    let v2 = deq.pop_back().unwrap();
                    let result = match i {
                        CalculatorInput::Add => v1 + v2,
                        CalculatorInput::Subtract => v2 - v1,
                        CalculatorInput::Multiply => v1 * v2,
                        CalculatorInput::Divide => v2 / v1,
                        _ => return None,
                    };
                    deq.push_back(result);
                }
            }
        }
    }
    return deq.pop_back();
}

fn main() {
    let mut vec = Vec::new();
    vec.push(CalculatorInput::Value(2));
    vec.push(CalculatorInput::Value(4));
    vec.push(CalculatorInput::Add);
    assert_eq!(evaluate(&vec), Some(6));

    let mut vec = Vec::new();
    vec.push(CalculatorInput::Value(2));
    vec.push(CalculatorInput::Add);
    assert_eq!(evaluate(&vec), None);

    let mut vec = Vec::new();
    vec.push(CalculatorInput::Value(4));
    vec.push(CalculatorInput::Value(8));
    vec.push(CalculatorInput::Add);
    vec.push(CalculatorInput::Value(7));
    vec.push(CalculatorInput::Value(5));
    vec.push(CalculatorInput::Subtract);
    vec.push(CalculatorInput::Divide);
    assert_eq!(evaluate(&vec), Some(6));
}
