use std::mem::swap;

use crate::parser::{
    error::Error,
    parser::{Expression, Statement, Value},
    token::{Token, TokenType},
};

use super::environment::Environment;

#[derive(Debug)]
struct Interpreter<'s> {
    environment: Environment<'s>,
}

impl<'s> Interpreter<'s> {
    fn new(environment: Environment<'s>) -> Self {
        Self { environment }
    }

    fn execute(&mut self, statement: &'s Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression { expr } => {
                self.evaluate(expr)?;
            }
            Statement::Print { expr } => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.evaluate(condition)?;

                let truthiness = match condition {
                    Value::Boolean(bool) => bool,
                    _ => {
                        return Err(Error::new(
                            "Expected boolean in an if condition".to_string(),
                        ))
                    }
                };

                if truthiness {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
            }
            Statement::Block { statements } => {
                self.execute_block(statements)?
            }
            Statement::For {
                variable,
                range,
                body,
            } => self.for_statement(variable, range, body)?,
            Statement::While { condition, body } => self.while_statement(condition, body)?,
        };
        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &'s Vec<Statement>,
    ) -> Result<(), Error> {
        let mut env = Environment::new(None);
        swap(&mut env, &mut self.environment);
        self.environment.enclosing = Some(Box::new(env));

        for statement in statements {
            self.execute(statement)?;
        }

        self.environment = *self.environment.enclosing.take().unwrap();

        Ok(())
    }

    fn for_statement(
        &mut self,
        name: &'s Token,
        range: &'s Expression,
        body: &'s Statement,
    ) -> Result<(), Error> {
        let range = self.evaluate(range)?;

        match range {
            Value::Range(a, b) => {
                let mut i = a;
                while i < b {
                    self.environment.assign(&name.lexeme, Value::Integer(i));
                    self.execute(body)?;
                    i += 1;
                }
            }
            _ => return Err(Error::new("Expected range".to_string())),
        }

        Ok(())
    }

    fn while_statement(
        &mut self,
        condition: &'s Expression,
        body: &'s Statement,
    ) -> Result<(), Error> {
        while self.evaluate(condition)? == Value::Boolean(true) {
            self.execute(body)?;
        }

        Ok(())
    }

    fn evaluate(&mut self, expression: &'s Expression) -> Result<Value, Error> {
        match expression {
            Expression::Value(value) => self.evaluate_value(*value),
            Expression::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expression::BinaryExpr {
                operator,
                left,
                right,
            } => self.evaluate_binary(operator, left, right),
            Expression::LogicalExpr {
                operator,
                left,
                right,
            } => self.evaluate_logical(operator, left, right),
            Expression::Grouping { expr } => self.evaluate(expr),
            Expression::Assign { assignee, value } => self.evaluate_assign(assignee, value),
            Expression::Variable { name, member } => self.evaluate_variable(name, member),
            Expression::Ternary { condition, then_branch, else_branch } => self.evaluate_ternary(condition, then_branch, else_branch),
        }
    }

    fn evaluate_value(&mut self, value: Value) -> Result<Value, Error> {
        match value {
            Value::Integer(int) => Ok(Value::Integer(int)),
            Value::Boolean(bool) => Ok(Value::Boolean(bool)),
            Value::Real(real) => Ok(Value::Real(real)),
            Value::Range(a, b) => Ok(Value::Range(a, b)),
        }
    }

    fn evaluate_unary(&mut self, operator: &'s Token, right: &'s Expression) -> Result<Value, Error> {
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Minus => match right {
                Value::Integer(int) => Ok(Value::Integer(-int)),
                Value::Real(real) => Ok(Value::Real(-real)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::Bang => match right {
                Value::Boolean(bool) => Ok(Value::Boolean(!bool)),
                _ => Err(Error::new("Expected boolean".to_string())),
            },
            _ => Err(Error::new("Expected unary operator".to_string())),
        }
    }

    fn evaluate_binary(
        &mut self,
        operator: &'s Token,
        left: &'s Expression,
        right: &'s Expression,
    ) -> Result<Value, Error> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::Star => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left * right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left * right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Real(left as f64 * right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Real(left * right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::Slash => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left / right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left / right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Real(left as f64 / right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Real(left / right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::Plus => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left + right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left + right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Real(left as f64 + right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Real(left + right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::Minus => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left - right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Real(left - right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Real(left as f64 - right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Real(left - right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::EqualsEquals => Ok(Value::Boolean(left == right)),
            TokenType::BangEquals => Ok(Value::Boolean(left != right)),
            TokenType::Less => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left < right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left < right)),
                (Value::Integer(left), Value::Real(right)) => {
                    Ok(Value::Boolean((left as f64) < right))
                }
                (Value::Real(left), Value::Integer(right)) => {
                    Ok(Value::Boolean(left < right as f64))
                }
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::LessEquals => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left <= right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left <= right)),
                (Value::Integer(left), Value::Real(right)) => {
                    Ok(Value::Boolean(left as f64 <= right))
                }
                (Value::Real(left), Value::Integer(right)) => {
                    Ok(Value::Boolean(left <= right as f64))
                }
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::Greater => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left > right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left > right)),
                (Value::Integer(left), Value::Real(right)) => {
                    Ok(Value::Boolean(left as f64 > right))
                }
                (Value::Real(left), Value::Integer(right)) => {
                    Ok(Value::Boolean(left > right as f64))
                }
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::GreaterEquals => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left >= right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left >= right)),
                (Value::Integer(left), Value::Real(right)) => {
                    Ok(Value::Boolean(left as f64 >= right))
                }
                (Value::Real(left), Value::Integer(right)) => {
                    Ok(Value::Boolean(left >= right as f64))
                }
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::DotDot => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Range(left, right)),
                (Value::Real(left), Value::Real(right)) => {
                    Ok(Value::Range(left as i64, right as i64))
                }
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Range(left, right as i64)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Range(left as i64, right)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            _ => Err(Error::new("Expected binary operator".to_string())),
        }
    }

    fn evaluate_logical(
        &mut self,
        operator: &'s Token,
        left: &'s Expression,
        right: &'s Expression,
    ) -> Result<Value, Error> {
        let left = self.evaluate(left)?;

        match operator.token_type {
            TokenType::And => {
                if let Value::Boolean(bool) = left {
                    if bool {
                        let right = self.evaluate(right)?;
                        if let Value::Boolean(bool) = right {
                            return Ok(Value::Boolean(bool));
                        }
                    }
                }
                Ok(Value::Boolean(false))
            }
            TokenType::Or => {
                if let Value::Boolean(bool) = left {
                    if bool {
                        return Ok(Value::Boolean(true));
                    }
                }
                let right = self.evaluate(right)?;
                if let Value::Boolean(bool) = right {
                    return Ok(Value::Boolean(bool));
                }
                Ok(Value::Boolean(false))
            }
            _ => Err(Error::new("Expected logical operator".to_string())),
        }
    }

    fn evaluate_assign(
        &mut self,
        assignee: &'s Expression,
        value: &'s Expression,
    ) -> Result<Value, Error> {
        let value = self.evaluate(value)?;

        let Expression::Variable { name, member } = assignee else {
            return Err(Error::new("Expected variable".to_string()));
        };

        // TODO: Handle members

        self.environment.assign(&name.lexeme, value);

        Ok(value)
    }

    fn evaluate_variable(
        &mut self,
        name: &'s Token,
        member: &'s Option<Box<Expression>>,
    ) -> Result<Value, Error> {
        let name = name.lexeme.as_str();
        let value = self.environment.get(name)?;

        Ok(*value)
    }

    fn evaluate_ternary(
        &mut self,
        condition: &'s Expression,
        then_branch: &'s Expression,
        else_branch: &'s Expression,
    ) -> Result<Value, Error> {
        let condition = self.evaluate(condition)?;

        match condition {
            Value::Boolean(bool) => {
                if bool {
                    self.evaluate(then_branch)
                } else {
                    self.evaluate(else_branch)
                }
            }
            _ => Err(Error::new("Expected boolean".to_string())),
        }
    }
}

pub fn interpret(statements: Vec<Statement>) -> Result<(), Error> {
    let environment = Environment::default();
    let mut interpreter = Interpreter::new(environment);

    for statement in &statements {
        interpreter.execute(statement)?;
    }

    Ok(())
}
