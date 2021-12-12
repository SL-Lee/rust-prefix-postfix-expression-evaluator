pub mod error;

use crate::error::{Error, ErrorKind};

pub fn evaluate(expr_type: ExpressionType, expr: &str) -> Result<f64, Error> {
    let mut tokens = expr
        .split_ascii_whitespace()
        .map(|slice| slice.to_string())
        .collect::<Vec<String>>();

    if let ExpressionType::Prefix = expr_type {
        tokens.reverse();
    }

    let mut stack = Vec::<f64>::new();

    while !tokens.is_empty() {
        let token = tokens.remove(0);

        if ["+", "-", "*", "/", "^"].contains(&token.as_str()) {
            let mut right = stack
                .pop()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInputExpression))?;
            let mut left = stack
                .pop()
                .ok_or_else(|| Error::new(ErrorKind::InvalidInputExpression))?;

            if let ExpressionType::Prefix = expr_type {
                std::mem::swap(&mut left, &mut right);
            }

            if token == "+" {
                stack.push(left + right);
            } else if token == "-" {
                stack.push(left - right);
            } else if token == "*" {
                stack.push(left * right);
            } else if token == "/" {
                stack.push(left / right);
            } else if token == "^" {
                stack.push(left.powf(right));
            }
        } else {
            stack.push(token.parse::<f64>()?);
        }
    }

    if stack.len() == 1 {
        stack
            .pop()
            .ok_or_else(|| Error::new(ErrorKind::InvalidInputExpression))
    } else {
        Err(Error::new(ErrorKind::InvalidInputExpression))
    }
}

pub enum ExpressionType {
    Prefix,
    Postfix,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_MARGIN: f64 = f64::EPSILON;

    #[test]
    fn prefix_evaluation_test() {
        assert!(
            12.0 - evaluate(ExpressionType::Prefix, "+ - 3 5 / * 7 6 3").unwrap() < ERROR_MARGIN
        );
    }

    #[test]
    fn postfix_evaluation_test() {
        assert!(
            12.0 - evaluate(ExpressionType::Postfix, "3 5 - 7 6 * 3 / +").unwrap() < ERROR_MARGIN
        );
    }
}
