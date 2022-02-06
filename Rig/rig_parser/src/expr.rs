use crate::Parser;
use rig_ast::expr::Expr;
use rig_ast::op::{BinaryOperator, LogicalOperator, UnaryOperator};
use rig_ast::token::TokenType;
use rig_error::{ErrorType, RigError};
use rig_span::Span;
use std::str::FromStr;

pub fn expr(parser: &mut Parser) -> Result<Expr, RigError> {
    assignment(parser)
}

pub fn assignment(parser: &mut Parser) -> Result<Expr, RigError> {
    let expr = logical_or(parser)?;

    if let Ok(_) = parser.consume(TokenType::Equal, "", None) {
        let eq_span = parser.previous().span.clone();
        let rhs = crate::expr::expr(parser)?;

        return match expr {
            Expr::GetExpr { object, name, span } => Ok(Expr::SetExpr {
                object,
                name: name.clone(),
                value: Box::from(rhs),
                span: Span::merge(span, parser.previous().span.clone()),
            }),
            Expr::VariableExpr { name, span } => Ok(Expr::AssignmentExpr {
                name,
                value: Box::new(rhs),
                span: Span::merge(span, parser.previous().span.clone()),
            }),
            _ => Err(RigError {
                error_code: String::from("EOOO6"),
                message: String::from(
                    "Invalid assignment. Expected property or a variable as lvalue",
                ),
                hint: None,
                error_type: ErrorType::Hard,
                file_path: parser.source_path.to_string(),
                span: eq_span,
            }),
        };
    }

    Ok(expr)
}

pub fn logical_or(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = logical_and(parser)?;

    let op = LogicalOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == LogicalOperator::Or {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::LogicalExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn logical_and(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = equality(parser)?;

    let op = LogicalOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == LogicalOperator::And {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::LogicalExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn equality(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = comparison(parser)?;

    let op = LogicalOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == LogicalOperator::Equal || op == LogicalOperator::NotEqual {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::LogicalExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn comparison(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = bitwise_or(parser)?;

    let op = LogicalOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == LogicalOperator::Less
            || op == LogicalOperator::LessEq
            || op == LogicalOperator::Greater
            || op == LogicalOperator::GreaterEq
        {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::LogicalExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn bitwise_or(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = bitwise_xor(parser)?;

    let op = BinaryOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == BinaryOperator::Or {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::BinaryExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn bitwise_xor(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = bitwise_and(parser)?;

    let op = BinaryOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == BinaryOperator::Xor {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::BinaryExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn bitwise_and(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = bitwise_shift(parser)?;

    let op = BinaryOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == BinaryOperator::And {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::BinaryExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn bitwise_shift(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = term(parser)?;

    let op = BinaryOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == BinaryOperator::LeftShift || op == BinaryOperator::RightShift {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::BinaryExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn term(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = factor(parser)?;

    let op = BinaryOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == BinaryOperator::Plus || op == BinaryOperator::Minus {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::BinaryExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn factor(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let expr = unary(parser)?;

    let op = BinaryOperator::from_str(&parser.peek().lexeme);

    if let Ok(op) = op.clone() {
        if op == BinaryOperator::Multiply
            || op == BinaryOperator::Divide
            || op == BinaryOperator::Modulus
        {
            parser.advance();
            let rhs = crate::expr::expr(parser)?;

            return Ok(Expr::BinaryExpr {
                lhs: Box::new(expr),
                op: op.clone(),
                rhs: Box::new(rhs),
                span: Span::merge(sp_start, parser.previous().span.clone()),
            });
        }
    }

    Ok(expr)
}

pub fn unary(parser: &mut Parser) -> Result<Expr, RigError> {
    let op = match UnaryOperator::from_str(&parser.peek().lexeme) {
        Ok(o) => o,
        Err(_) => return call(parser),
    };
    let sp_start = parser.peek().span.clone();
    parser.advance();

    Ok(Expr::UnaryExpr {
        op,
        rhs: Box::new(crate::expr::expr(parser)?),
        span: Span::merge(sp_start, parser.previous().span.clone()),
    })
}

pub fn call(parser: &mut Parser) -> Result<Expr, RigError> {
    let sp_start = parser.peek().span.clone();
    let mut expr = primary(parser)?;

    loop {
        if parser.peek().token_type == TokenType::LeftParen {
            parser.advance();
            let args = arguments(parser)?;

            let _ = parser.consume(
                TokenType::RightParen,
                "Expected `)` after argument list",
                None,
            );
            expr = Expr::CallExpr {
                name: Box::new(expr),
                args,
                span: Span::merge(sp_start.clone(), parser.previous().span.clone()),
            }
        } else if parser.peek().token_type == TokenType::Dot {
            parser.advance();
            let name = parser.consume(TokenType::Identifier, "Expected identifier", None)?;
            expr = Expr::GetExpr {
                name: name.lexeme.clone(),
                object: Box::new(expr),
                span: Span::merge(sp_start.clone(), parser.previous().span.clone()),
            }
        } else {
            break;
        }
    }

    Ok(expr)
}

pub fn arguments(parser: &mut Parser) -> Result<Vec<Box<Expr>>, RigError> {
    let mut args = Vec::new();
    if parser.peek().token_type == TokenType::RightParen {
        return Ok(args);
    }
    args.push(Box::new(expr(parser)?));

    loop {
        if parser.peek().token_type != TokenType::Comma {
            break;
        }

        args.push(Box::new(expr(parser)?));
    }

    return Ok(args);
}

pub fn primary(parser: &mut Parser) -> Result<Expr, RigError> {
    match parser.peek().token_type {
        TokenType::Identifier => path(parser),
        TokenType::StringLiteral => {
            let ret = Ok(Expr::StringLiteralExpr {
                value: parser.peek().literal.clone(),
                span: parser.peek().span.clone(),
            });

            parser.advance();
            ret
        }
        TokenType::NumberLiteral => {
            if parser.peek().lexeme.contains('.') {
                let ret = Ok(Expr::FloatLiteralExpr {
                    value: parser.peek().literal.parse().expect(
                        "Lexer emitted invalid float literal or value is too big to store in f64",
                    ),
                    span: parser.peek().span.clone(),
                });

                parser.advance();
                ret
            } else {
                let ret = Ok(Expr::IntegerLiteralExpr {
                    value: parser.peek().literal.parse().expect(
                        "Lexer emitted invalid integer literal or value is too big to store in i64",
                    ),
                    span: parser.peek().span.clone(),
                });

                parser.advance();
                ret
            }
        }
        TokenType::Keyword => {
            let ret = match parser.peek().lexeme.as_str() {
                "true" => Ok(Expr::BooleanLiteralExpr {
                    value: true,
                    span: parser.peek().span.clone(),
                }),
                "false" => Ok(Expr::BooleanLiteralExpr {
                    value: false,
                    span: parser.peek().span.clone(),
                }),
                "null" => Ok(Expr::NullLiteralExpr {
                    span: parser.peek().span.clone(),
                }),
                "self" => Ok(Expr::SelfExpr {
                    span: parser.peek().span.clone(),
                }),
                _ => Err(RigError {
                    error_type: ErrorType::Hard,
                    error_code: String::from("E0005"),
                    message: format!(
                        "Expected `true`/`false`/`null`, found `{}`",
                        &parser.peek().lexeme
                    ),
                    hint: None,
                    span: parser.peek().span.clone(),
                    file_path: parser.source_path.to_string(),
                }),
            };

            if let Ok(_) = ret {
                parser.advance();
            }

            ret
        }
        TokenType::LeftParen => {
            let _sp_start = parser.peek().span.clone();
            parser.advance();
            let expr = Box::new(expr(parser)?);
            parser.consume(TokenType::RightParen, "Expected `)` after expression", None)?;

            Ok(Expr::GroupingExpr {
                expr,
                span: parser.previous().span.clone(),
            })
        }
        _ => Err(RigError {
            error_type: ErrorType::Hard,
            error_code: String::from("E0005"),
            message: format!(
                "Expected primary expression, found `{}`",
                &parser.peek().lexeme
            ),
            hint: None,
            span: parser.peek().span.clone(),
            file_path: parser.source_path.to_string(),
        }),
    }
}

pub fn path(parser: &mut Parser) -> Result<Expr, RigError> {
    let mut path = Vec::new();
    let start_span = parser.peek().span.clone();
    path.push(
        parser
            .consume(TokenType::Identifier, "Expected identifier", None)?
            .lexeme
            .clone(),
    );

    if parser.check(TokenType::Scope) {
        parser.advance();
        path.extend(parse_path(parser)?);
        let end_span = parser.peek().span.clone();

        Ok(Expr::PathExpr {
            path,
            span: Span::merge(start_span, end_span),
        })
    } else {
        Ok(Expr::VariableExpr {
            name: path[0].clone(),
            span: parser.peek().span.clone(),
        })
    }
}

fn parse_path(parser: &mut Parser) -> Result<Vec<String>, RigError> {
    let name = parser.consume(TokenType::Identifier, "Expected identifier", None)?;
    let mut path = Vec::new();
    path.push(name.lexeme.clone());

    if parser.check(TokenType::Scope) {
        parser.advance();
        path.extend(parse_path(parser)?);
    }

    Ok(path)
}
