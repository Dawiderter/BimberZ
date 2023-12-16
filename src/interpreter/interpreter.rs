use std::collections::HashMap;

use crate::parser::{parser::{Value, Statement, Expression}, error::Error, token::{Token, TokenType}};



#[derive(Debug)]
pub struct Environment {
    pub variables: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
struct Interpreter<'a> {
    environment: &'a mut Environment,
}


impl<'a> Interpreter<'a> {
    fn new(environment: &'a mut Environment) -> Self {
        Self { environment }
    }

    fn execute(&mut self, statement: Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression{expr} => {self.evaluate(*expr)?;},
            Statement::Print{expr} => {
                let value = self.evaluate(*expr)?;
                println!("{}", value);
            }
        };
        Ok(())
    } 

    fn evaluate(&mut self, expression: Expression) -> Result<Value, Error> {
        match expression {
            Expression::Value(value) => self.evaluate_value(value),
            Expression::Unary { operator, right } => self.evaluate_unary(operator, *right),
            Expression::BinaryExpr { operator, left, right } => self.evaluate_binary(operator, *left, *right),
            Expression::LogicalExpr { operator, left, right } => todo!(),
            Expression::Grouping { expr } => self.evaluate(*expr),
            Expression::Assign { assignee, value } => self.evaluate_assign(*assignee, *value),
            Expression::Variable { name, member } => self.evaluate_variable(name, member),
        }
    }

    fn evaluate_value(&mut self, value: Value) -> Result<Value, Error> {
        match value {
            Value::Integer(int) => Ok(Value::Integer(int)),
            Value::Boolean(bool) => Ok(Value::Boolean(bool)),
            Value::Real(real) => Ok(Value::Real(real)),
        }
    }

    fn evaluate_unary(&mut self, operator: Token, right: Expression) -> Result<Value, Error> {
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

    fn evaluate_binary(&mut self, operator: Token, left: Expression, right: Expression) -> Result<Value, Error>{
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
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Boolean((left as f64) < right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Boolean(left < right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::LessEquals => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left <= right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left <= right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Boolean(left as f64 <= right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Boolean(left <= right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::Greater => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left > right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left > right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Boolean(left as f64 > right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Boolean(left > right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            TokenType::GreaterEquals => match (left, right) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Boolean(left >= right)),
                (Value::Real(left), Value::Real(right)) => Ok(Value::Boolean(left >= right)),
                (Value::Integer(left), Value::Real(right)) => Ok(Value::Boolean(left as f64 >= right)),
                (Value::Real(left), Value::Integer(right)) => Ok(Value::Boolean(left >= right as f64)),
                _ => Err(Error::new("Expected number".to_string())),
            },
            _ => Err(Error::new("Expected binary operator".to_string())),
        }
    }

    fn evaluate_assign(&mut self, assignee: Expression, value: Expression) -> Result<Value, Error> {
        let value = self.evaluate(value)?;

        let Expression::Variable{name, member} = assignee else {
            return Err(Error::new("Expected variable".to_string()));
        };

        // TODO: Handle members

        self.environment.variables.insert(name.lexeme, value.clone());

        Ok(value)
    }

    fn evaluate_variable(&mut self, name: Token, member: Option<Box<Expression>>) -> Result<Value, Error> {
        let name = name.lexeme;
        let value = self.environment.variables.get(&name).ok_or_else(|| Error::new(format!("Variable {} not found", name)))?;

        Ok(value.clone())
    }
}

pub fn interpret(environment: &mut Environment, statements: Vec<Statement>) -> Result<(), Error> {
    let mut interpreter = Interpreter::new(environment);
    for statement in statements {
        interpreter.execute(statement)?;
    }
    Ok(())
}
