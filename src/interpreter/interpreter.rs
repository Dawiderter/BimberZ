use std::collections::HashMap;

use crate::parser::{
    error::Error,
    parser::{Expression, Statement, Value},
    token::{Token, TokenType},
};

#[derive(Debug)]
pub struct Environment<'a> {
    pub variables: HashMap<&'a str, Value>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl<'a> Default for Environment<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Interpreter<'a> {
    environment: &'a mut Environment<'a>,
}

impl<'a> Interpreter<'a> {
    fn new(environment: &'a mut Environment<'a>) -> Self {
        Self { environment }
    }

    fn execute(&mut self, statement: &'a Statement) -> Result<(), Error> {
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
                // TODO: Add nested scopes to environments
                for statement in statements {
                    self.execute(statement)?;
                }
            }
            Statement::For {
                variable,
                range,
                body,
            } => self.for_statement(variable, range, body)?,
        };
        Ok(())
    }

    fn for_statement(
        &mut self,
        name: &'a Token,
        range: &'a Expression,
        body: &'a Statement,
    ) -> Result<(), Error> {
        let range = self.evaluate(range)?;

        match range {
            Value::Range(a, b) => {
                let mut i = a;
                while i < b {
                    self.environment
                        .variables
                        .insert(&name.lexeme, Value::Integer(i));
                    self.execute(body)?;
                    i += 1;
                }
            }
            _ => return Err(Error::new("Expected range".to_string())),
        }

        Ok(())
    }

    fn evaluate(&mut self, expression: &'a Expression) -> Result<Value, Error> {
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

    fn evaluate_unary(&mut self, operator: &Token, right: &'a Expression) -> Result<Value, Error> {
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
        operator: &'a Token,
        left: &'a Expression,
        right: &'a Expression,
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
                (Value::Real(left), Value::Real(right)) => Ok(Value::Range(left as i64, right as i64)),
                (Value::Integer(left), Value::Real(right)) => {
                    Ok(Value::Range(left, right as i64))
                }
                (Value::Real(left), Value::Integer(right)) => {
                    Ok(Value::Range(left as i64, right))
                }
                _ => Err(Error::new("Expected number".to_string())),
            },
            _ => Err(Error::new("Expected binary operator".to_string())),
        }
    }

    fn evaluate_logical(
        &mut self,
        operator: &'a Token,
        left: &'a Expression,
        right: &'a Expression,
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
        assignee: &'a Expression,
        value: &'a Expression,
    ) -> Result<Value, Error> {
        let value = self.evaluate(value)?;

        let Expression::Variable { name, member } = assignee else {
            return Err(Error::new("Expected variable".to_string()));
        };

        // TODO: Handle members

        self.environment
            .variables
            .insert(&name.lexeme, value);

        Ok(value)
    }

    fn evaluate_variable(
        &mut self,
        name: &Token,
        member: &Option<Box<Expression>>,
    ) -> Result<Value, Error> {
        let name = name.lexeme.as_str();
        let value = self
            .environment
            .variables
            .get(name)
            .ok_or_else(|| Error::new(format!("Variable {} not found", name)))?;

        Ok(*value)
    }
}

pub fn interpret(statements: Vec<Statement>) -> Result<(), Error> {
    let mut environment = Environment::new();
    let mut interpreter = Interpreter::new(&mut environment);
    for statement in &statements {
        interpreter.execute(statement)?;
    }
    Ok(())
}
