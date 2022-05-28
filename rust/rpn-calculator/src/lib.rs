#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut operands = vec![];
    let mut result: Option<i32> = None;
    let mut results = vec![];

    for input in inputs.iter() {
        match input {
            CalculatorInput::Add => {
                if operands.len() >= 2 {
                    results.push(operands.iter().sum());
                    operands.clear();
                } else if results.len() >= 2 {
                    let r = results.iter().fold(0, |acc, op| acc + op);
                    result = Some(result.unwrap_or(0) + r);
                    results.clear();
                } else {
                    return None;
                }
            }

            CalculatorInput::Subtract => {
                if operands.len() >= 2 {
                    let r = operands.iter().copied().reduce(|acc, op| acc - op).unwrap();
                    results.push(r);
                    operands.clear();
                } else if results.len() >= 2 {
                    let r = results.iter().copied().reduce(|acc, op| acc - op).unwrap();
                    result = Some(result.unwrap_or(0) - r);
                    results.clear();
                } else {
                    return None;
                }
            }

            CalculatorInput::Multiply => {
                if operands.len() >= 2 {
                    let r = operands.iter().copied().reduce(|acc, op| acc * op).unwrap();
                    results.push(r);
                    operands.clear();
                } else if results.len() >= 2 {
                    let r = results.iter().copied().reduce(|acc, op| acc * op).unwrap();
                    result = Some(result.unwrap_or(1) * r);
                    results.clear();
                } else {
                    return None;
                }
            }

            CalculatorInput::Divide => {
                if operands.len() >= 2 {
                    let r = operands.iter().copied().reduce(|acc, op| acc / op).unwrap();
                    results.push(r);
                    operands.clear();
                } else if results.len() >= 2 {
                    let r = results.iter().copied().reduce(|acc, op| acc / op).unwrap();
                    result = Some(result.unwrap_or(1) * r);
                    results.clear();
                } else {
                    return None;
                }
            }

            CalculatorInput::Value(x) => {
                operands.push(*x);
            }
        }
    }

    if operands.len() == 1 {
        Some(operands[0])
    } else if results.is_empty() {
        result
    } else {
        Some(results[0])
    }
}
