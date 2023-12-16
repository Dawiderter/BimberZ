use std::fmt::Display;

use super::{
    error::Error,
    token::{Token, TokenType},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Range(f64, f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(int) => write!(f, "{}", int),
            Value::Real(real) => write!(f, "{}", real),
            Value::Boolean(bool) => write!(f, "{}", bool),
            Value::Range(start, end) => write!(f, "{}..{}", start, end),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    BinaryExpr {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    LogicalExpr {
        operator: Token,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Grouping {
        expr: Box<Expression>,
    },
    // Variable {
    //     name: Token,
    // },
    Assign {
        assignee: Box<Expression>,
        value: Box<Expression>,
    },
    // TODO: There might be a better way to do this
    Variable {
        name: Token,
        member: Option<Box<Expression>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression {
        expr: Box<Expression>,
    },
    Print {
        expr: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Block {
        statements: Vec<Statement>,
    },
}

#[derive(Debug, PartialEq)]
struct Parser<'a> {
    tokens: &'a [Token],
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens }
    }

    fn match_next(&self, token_types: &[TokenType]) -> bool {
        if self.tokens.is_empty() {
            return false;
        }

        for token_type in token_types {
            if self.tokens[0].token_type == *token_type {
                return true;
            }
        }

        false
    }

    fn chop(&mut self) -> Option<Token> {
        if self.tokens.is_empty() {
            return None;
        }
        let token = &self.tokens[0];
        self.tokens = &self.tokens[1..];
        Some(token.clone())
    }

    fn expect(&mut self, expected_type: TokenType, error_message: String) -> Result<Token, Error> {
        let next = self.chop().ok_or(Error::new(error_message.clone()))?;
        if next.token_type != expected_type {
            return Err(Error::new(error_message));
        }
        Ok(next)
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        if self.tokens.len() <= offset {
            return None;
        }
        Some(&self.tokens[offset])
    }

    fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while !self.tokens.is_empty() {
            statements.push(self.declaration()?);
            self.consume_whitespace();
        }

        Ok(statements)
    }

    fn consume_whitespace(&mut self) {
        while self.match_next(&[TokenType::Newline]) {
            self.chop();
        }
    }

    fn declaration(&mut self) -> Result<Statement, Error> {
        self.statement()
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        let next_type = &self
            .peek(0)
            .ok_or(Error::new("Expected a statement".to_string()))?
            .token_type;

        match next_type {
            TokenType::Print => self.print_statement(),
            TokenType::If => self.if_statement(),
            TokenType::LeftCurlyBracket => self.block_statement(),
            _ => self.expression_statement(),
        }
    }

    fn print_statement(&mut self) -> Result<Statement, Error> {
        let _print = self.chop().unwrap();
        let expr = self.expression()?;
        self.expect(
            TokenType::Newline,
            "Expected a newline after print statement".to_string(),
        )?;
        Ok(Statement::Print {
            expr: Box::new(expr),
        })
    }

    fn if_statement(&mut self) -> Result<Statement, Error> {
        let _if = self.chop().unwrap();
        let expr = self.expression()?;
        let block = self.block_statement()?;

        let next = self.peek(0);

        let mut else_branch = None;
        if next.is_some() && next.unwrap().token_type == TokenType::Else {
            let _else = self.chop().unwrap();
            let else_block = self.block_statement()?;
            else_branch = Some(Box::new(else_block));
        }

        Ok(Statement::If {
            condition: Box::new(expr),
            then_branch: Box::new(block),
            else_branch,
        })
    }

    fn block_statement(&mut self) -> Result<Statement, Error> {
        let _left_curly_bracket = self.expect(
            TokenType::LeftCurlyBracket,
            "Expected a curly bracket.".to_string(),
        );

        self.consume_whitespace();

        let mut statements = Vec::new();

        while !self.match_next(&[TokenType::RightCurlyBracket]) {
            statements.push(self.declaration()?);
        }

        let _right_curly_bracket = self.expect(
            TokenType::RightCurlyBracket,
            "Expected a closing curly bracket.".to_string(),
        );

        Ok(Statement::Block { statements })
    }

    fn expression_statement(&mut self) -> Result<Statement, Error> {
        let expr = self.expression()?;
        self.expect(
            TokenType::Newline,
            "Expected a newline after expression statement".to_string(),
        )?;
        Ok(Statement::Expression {
            expr: Box::new(expr),
        })
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.assignment_expression()
    }

    fn assignment_expression(&mut self) -> Result<Expression, Error> {
        let expr = self.logic_or_expression()?;

        if self.match_next(&[TokenType::Equals]) {
            let _equals = self.chop().unwrap();
            let value = self.assignment_expression()?;

            if let Expression::Variable { name, member } = expr {
                return Ok(Expression::Assign {
                    assignee: Box::new(Expression::Variable { name, member }),
                    value: Box::new(value),
                });
            }

            return Err(Error::new(
                "Can't assign that expression to a variable".to_string(),
            ));
        }

        Ok(expr)
    }

    fn logic_or_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.logic_and_expression()?;

        while self.match_next(&[TokenType::Or]) {
            let operator = self.chop().unwrap();
            let right = self.logic_and_expression()?;

            expr = Expression::BinaryExpr {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn logic_and_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.equality_expression()?;

        while self.match_next(&[TokenType::And]) {
            let operator = self.chop().unwrap();
            let right = self.equality_expression()?;

            expr = Expression::BinaryExpr {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.comparison_expression()?;

        while self.match_next(&[TokenType::BangEquals, TokenType::EqualsEquals]) {
            let operator = self.chop().unwrap();
            let right = self.comparison_expression()?;

            expr = Expression::BinaryExpr {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.term_expression()?;

        while self.match_next(&[
            TokenType::Greater,
            TokenType::GreaterEquals,
            TokenType::Less,
            TokenType::LessEquals,
        ]) {
            let operator = self.chop().unwrap();
            let right = self.term_expression()?;

            expr = Expression::BinaryExpr {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.factor_expression()?;

        while self.match_next(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.chop().unwrap();
            let right = self.factor_expression()?;

            expr = Expression::BinaryExpr {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.range_expression()?;

        while self.match_next(&[TokenType::Star, TokenType::Slash]) {
            let operator = self.chop().unwrap();
            let right = self.unary_expression()?;

            expr = Expression::BinaryExpr {
                operator,
                left: Box::new(expr),
                right: Box::new(right),
            };
        }


        Ok(expr)
    }

    fn range_expression(&mut self) -> Result<Expression, Error> {
        let mut expr = self.unary_expression()?;

        if self.match_next(&[TokenType::DotDot]) {
            let _dot_dot = self.chop().unwrap();
            let right = self.unary_expression()?;

            expr = Expression::BinaryExpr {
                operator: Token::new(TokenType::DotDot, "..".to_string()),
                left: Box::new(expr),
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary_expression(&mut self) -> Result<Expression, Error> {
        if self.match_next(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.chop().unwrap();
            let right = self.unary_expression()?;

            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary_expression()
    }

    fn primary_expression(&mut self) -> Result<Expression, Error> {
        let next = self
            .chop()
            .ok_or(Error::new("Expected an expression".to_string()))?;

        match next.token_type {
            TokenType::Integer => Ok(Expression::Value(Value::Integer(
                next.lexeme.parse::<i64>().unwrap(),
            ))),
            TokenType::Real => Ok(Expression::Value(Value::Real(
                next.lexeme.parse::<f64>().unwrap(),
            ))),
            TokenType::True => Ok(Expression::Value(Value::Boolean(true))),
            TokenType::False => Ok(Expression::Value(Value::Boolean(false))),
            TokenType::LeftParen => {
                let expr = self.expression()?;

                self.expect(
                    TokenType::RightParen,
                    "Expected a closing parenthesis".to_string(),
                )?;
                Ok(Expression::Grouping {
                    expr: Box::new(expr),
                })
            }
            TokenType::Identifier => self.variable(next),
            _ => Err(Error::new("Expected an expression".to_string())),
        }
    }

    fn variable(&mut self, name: Token) -> Result<Expression, Error> {
        if self.match_next(&[TokenType::Dot]) {
            let _dot = self.chop().unwrap();
            let next_name = self
                .chop()
                .ok_or(Error::new("Expected a member name".to_string()))?;
            let member = self.variable(next_name)?;

            return Ok(Expression::Variable {
                name,
                member: Some(Box::new(member)),
            });
        }

        Ok(Expression::Variable { name, member: None })
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Statement>, Error> {
    let mut parser = Parser::new(tokens);

    parser.parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assign_var() {
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Equals, "=".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::Assign {
                    assignee: Box::new(Expression::Variable {
                        name: Token::new(TokenType::Identifier, "x".to_string()),
                        member: None
                    }),
                    value: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_assign_var_print() {
        let tokens = vec![
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Equals, "=".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
            Token::new(TokenType::Print, "print".to_string()),
            Token::new(TokenType::Identifier, "x".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![
                Statement::Expression {
                    expr: Box::new(Expression::Assign {
                        assignee: Box::new(Expression::Variable {
                            name: Token::new(TokenType::Identifier, "x".to_string()),
                            member: None
                        }),
                        value: Box::new(Expression::Value(Value::Integer(5))),
                    })
                },
                Statement::Print {
                    expr: Box::new(Expression::Variable {
                        name: Token::new(TokenType::Identifier, "x".to_string()),
                        member: None
                    })
                }
            ]
        );
    }

    #[test]
    fn test_grouping() {
        let tokens = vec![
            Token::new(TokenType::LeftParen, "(".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::RightParen, ")".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::Grouping {
                    expr: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_logic_or() {
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::Or, "or".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::Or, "or".to_string()),
                    left: Box::new(Expression::Value(Value::Boolean(true))),
                    right: Box::new(Expression::Value(Value::Boolean(false))),
                })
            }]
        );
    }

    #[test]
    fn test_logic_and() {
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::And, "and".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::And, "and".to_string()),
                    left: Box::new(Expression::Value(Value::Boolean(true))),
                    right: Box::new(Expression::Value(Value::Boolean(false))),
                })
            }]
        );
    }

    #[test]
    fn test_logic_and_or() {
        let tokens = vec![
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::And, "and".to_string()),
            Token::new(TokenType::False, "false".to_string()),
            Token::new(TokenType::Or, "or".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::Or, "or".to_string()),
                    left: Box::new(Expression::BinaryExpr {
                        operator: Token::new(TokenType::And, "and".to_string()),
                        left: Box::new(Expression::Value(Value::Boolean(true))),
                        right: Box::new(Expression::Value(Value::Boolean(false))),
                    }),
                    right: Box::new(Expression::Value(Value::Boolean(true))),
                })
            }]
        );
    }

    #[test]
    fn test_equality() {
        let tokens = vec![
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::EqualsEquals, "==".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::EqualsEquals, "==".to_string()),
                    left: Box::new(Expression::Value(Value::Integer(5))),
                    right: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_comparison() {
        let tokens = vec![
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Less, "<".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::Less, "<".to_string()),
                    left: Box::new(Expression::Value(Value::Integer(5))),
                    right: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_term() {
        let tokens = vec![
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::Star, "*".to_string()),
                    left: Box::new(Expression::Value(Value::Integer(5))),
                    right: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_factor() {
        let tokens = vec![
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        println!("{:?}", statements);

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::Slash, "/".to_string()),
                    left: Box::new(Expression::BinaryExpr {
                        operator: Token::new(TokenType::Star, "*".to_string()),
                        left: Box::new(Expression::Value(Value::Integer(5))),
                        right: Box::new(Expression::Value(Value::Integer(5))),
                    }),
                    right: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_unary() {
        let tokens = vec![
            Token::new(TokenType::Minus, "-".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Star, "*".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Slash, "/".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        println!("{:?}", statements);

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::Slash, "/".to_string()),
                    left: Box::new(Expression::BinaryExpr {
                        operator: Token::new(TokenType::Star, "*".to_string()),
                        left: Box::new(Expression::Unary {
                            operator: Token::new(TokenType::Minus, "-".to_string()),
                            right: Box::new(Expression::Value(Value::Integer(5))),
                        }),
                        right: Box::new(Expression::Value(Value::Integer(5))),
                    }),
                    right: Box::new(Expression::Value(Value::Integer(5))),
                })
            }]
        );
    }

    #[test]
    fn test_block() {
        let tokens = vec![
            Token::new(TokenType::LeftCurlyBracket, "{".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
            Token::new(TokenType::RightCurlyBracket, "}".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        println!("{:?}", statements);

        assert_eq!(
            statements,
            vec![Statement::Block {
                statements: vec![Statement::Expression {
                    expr: Box::new(Expression::Value(Value::Integer(5))),
                }]
            }]
        );
    }

    #[test]
    fn test_if() {
        let tokens = vec![
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::LeftCurlyBracket, "{".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
            Token::new(TokenType::RightCurlyBracket, "}".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        println!("{:?}", statements);

        assert_eq!(
            statements,
            vec![Statement::If {
                condition: Box::new(Expression::Value(Value::Boolean(true))),
                then_branch: Box::new(Statement::Block {
                    statements: vec![Statement::Expression {
                        expr: Box::new(Expression::Value(Value::Integer(5))),
                    }]
                }),
                else_branch: None,
            }]
        );
    }

    #[test]
    fn test_if_else() {
        let tokens = vec![
            Token::new(TokenType::If, "if".to_string()),
            Token::new(TokenType::True, "true".to_string()),
            Token::new(TokenType::LeftCurlyBracket, "{".to_string()),
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
            Token::new(TokenType::RightCurlyBracket, "}".to_string()),
            Token::new(TokenType::Else, "else".to_string()),
            Token::new(TokenType::LeftCurlyBracket, "{".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
            Token::new(TokenType::RightCurlyBracket, "}".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        println!("{:?}", statements);

        assert_eq!(
            statements,
            vec![Statement::If {
                condition: Box::new(Expression::Value(Value::Boolean(true))),
                then_branch: Box::new(Statement::Block {
                    statements: vec![Statement::Expression {
                        expr: Box::new(Expression::Value(Value::Integer(5))),
                    }]
                }),
                else_branch: Some(Box::new(Statement::Block {
                    statements: vec![Statement::Expression {
                        expr: Box::new(Expression::Value(Value::Integer(10))),
                    }]
                })),
            }]
        );
    }

    #[test]
    fn test_range() {
        let tokens = vec![
            Token::new(TokenType::Integer, "5".to_string()),
            Token::new(TokenType::DotDot, "..".to_string()),
            Token::new(TokenType::Integer, "10".to_string()),
            Token::new(TokenType::Newline, "\n".to_string()),
        ];

        let statements = parse(&tokens).unwrap();

        println!("{:?}", statements);

        assert_eq!(
            statements,
            vec![Statement::Expression {
                expr: Box::new(Expression::BinaryExpr {
                    operator: Token::new(TokenType::DotDot, "..".to_string()),
                    left: Box::new(Expression::Value(Value::Integer(5))),
                    right: Box::new(Expression::Value(Value::Integer(10))),
                })
            }]
        );
    }

    // FIXME: That test should be passing but it does not
    // #[test]
    // fn test_assign_grouping() {
    //     let tokens = vec![
    //         Token::new(TokenType::LeftParen, "(".to_string()),
    //         Token::new(TokenType::Identifier, "x".to_string()),
    //         Token::new(TokenType::RightParen, ")".to_string()),
    //         Token::new(TokenType::Equals, "=".to_string()),
    //         Token::new(TokenType::LeftParen, "(".to_string()),
    //         Token::new(TokenType::Integer, "5".to_string()),
    //         Token::new(TokenType::RightParen, ")".to_string()),
    //         Token::new(TokenType::Newline, "\n".to_string()),
    //     ];
    //
    //     let statements = parse(&tokens).unwrap();
    //
    //     println!("{:?}", statements);
    //
    //     assert_eq!(
    //         statements,
    //         vec![Statement::Expression {
    //             expr: Box::new(Expression::Assign {
    //                 name: Token::new(TokenType::Identifier, "x".to_string()),
    //                 value: Box::new(Expression::Grouping {
    //                     expr: Box::new(Expression::Value(Value::Integer(5))),
    //                 }),
    //             })
    //         }]
    //     );
    // }
}
