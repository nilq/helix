use super::tokenizer::Tokenizer;
use super::token::{TokenType, Operator};

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),

    Ident(String),

    Operation(Box<Expression>, Operator, Box<Expression>),

    Call(Box<Expression>, Box<Vec<Expression>>),

    Use(Box<Expression>),

    Module(String, Box<Vec<Statement>>),

    Implement(String, Box<Vec<Statement>>),

    Class(String, Box<Vec<Statement>>),

    Struct(String, Box<Vec<Statement>>),

    Typed(Box<Expression>, Box<Expression>),

    Import(String, bool),

    Function(String, Vec<(String, String)>, Box<Vec<Statement>>),

    IndexDot(Box<Expression>, Box<Expression>),

    IndexColon(Box<Expression>, Box<Expression>),

    IndexArray(Box<Expression>, Box<Expression>),

    Return(Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(String, Box<Expression>),

    Declaration(String, Box<Expression>),

    Block(Box<Vec<Statement>>),

    If(Box<Expression>, Box<Statement>),

    IfElse(Box<Expression>, Box<Statement>, Box<Statement>),

    Expression(Box<Expression>),
}

#[derive(Debug, Clone)]
pub struct Parser {
    tokenizer: Tokenizer,
}

impl Parser {
    pub fn new() -> Parser {
        Parser { tokenizer: Tokenizer::new() }
    }

    pub fn from(tokenizer: Tokenizer) -> Parser {
        Parser { tokenizer: tokenizer }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut stack = Vec::new();

        loop {
            if self.tokenizer.remaining() < 1 {
                break;
            }

            stack.push(try!(self.statement()));

            self.tokenizer.next_token();
        }

        Ok(stack)
    }

    fn operation(&mut self, expression: Expression) -> Result<Expression, String> {

        let mut ex_stack = vec![expression];
        let mut op_stack: Vec<(Operator, u8)> = Vec::new();

        op_stack.push(super::tokenizer::operator(&self.tokenizer.current_content()).unwrap());

        self.tokenizer.next_token();

        ex_stack.push(try!(self.term()));

        let mut done = false;
        while ex_stack.len() > 1 {

            if !done && self.tokenizer.next_token() {
                if self.tokenizer.current().get_type() != TokenType::Operator {
                    self.tokenizer.prev_token();

                    done = true;

                    continue;
                }

                let (op, prec) = super::tokenizer::operator(&self.tokenizer.current_content())
                    .unwrap();

                if prec > op_stack.last().unwrap().1 {
                    let left = ex_stack.pop().unwrap();
                    let right = ex_stack.pop().unwrap();

                    ex_stack.push(Expression::Operation(Box::new(left),
                                                        op_stack.pop().unwrap().0,
                                                        Box::new(right)));

                    self.tokenizer.next_token();

                    ex_stack.push(try!(self.term()));
                    op_stack.push((op, prec));

                    continue;
                }

                self.tokenizer.next_token();

                ex_stack.push(try!(self.term()));
                op_stack.push((op, prec));
            }

            let left = ex_stack.pop().unwrap();
            let right = ex_stack.pop().unwrap();

            ex_stack.push(Expression::Operation(Box::new(left),
                                                op_stack.pop().unwrap().0,
                                                Box::new(right)));
        }

        Ok(ex_stack.pop().unwrap())
    }

    fn call(&mut self, caller: Expression) -> Result<Expression, String> {
        let mut stack = Vec::new();

        self.tokenizer.next_token();

        while self.tokenizer.current().get_type() != TokenType::RParen {
            stack.push(try!(self.expression()));

            self.tokenizer.next_token();

            if self.tokenizer.current().get_type() == TokenType::Comma {
                self.tokenizer.next_token();
            }
        }

        Ok(Expression::Call(Box::new(caller), Box::new(stack)))
    }

    fn dot(&mut self, id: Expression) -> Result<Expression, String> {
        self.tokenizer.next_token();

        try!(self.tokenizer.match_current(TokenType::Ident));

        let index = try!(self.expression());

        Ok(Expression::IndexDot(Box::new(id), Box::new(index)))
    }

    fn colon(&mut self, id: Expression) -> Result<Expression, String> {
        self.tokenizer.next_token();

        try!(self.tokenizer.match_current(TokenType::Ident));

        let index = try!(self.expression());

        Ok(Expression::IndexColon(Box::new(id), Box::new(index)))
    }

    fn index(&mut self, id: Expression) -> Result<Expression, String> {
        self.tokenizer.next_token();

        let index = try!(self.expression());

        self.tokenizer.next_token();

        try!(self.tokenizer.match_current(TokenType::RBracket));

        self.tokenizer.next_token();

        let expression = Expression::IndexArray(Box::new(id), Box::new(index));

        match self.tokenizer.current().get_type() {
            TokenType::Period => Ok(try!(self.dot(expression))),
            TokenType::Colon => {
                self.tokenizer.next_token();

                if self.tokenizer.current().get_type() == TokenType::Colon {
                    self.colon(expression)
                } else {
                    self.typed(expression)
                }
            }
            TokenType::LParen => Ok(try!(self.call(expression))),            
            _ => Ok(expression),
        }
    }

    fn typed(&mut self, id: Expression) -> Result<Expression, String> {
        try!(self.tokenizer.match_current(TokenType::Ident));

        let ident = try!(self.term());

        Ok(Expression::Typed(Box::new(id), Box::new(ident)))
    }

    fn term(&mut self) -> Result<Expression, String> {
        let token_type = self.tokenizer.current().get_type();

        match token_type {
            TokenType::Integer => {
                Ok(Expression::Integer(self.tokenizer
                                           .current_content()
                                           .parse::<i64>()
                                           .unwrap()))
            }

            TokenType::Float => {
                Ok(Expression::Float(self.tokenizer
                                         .current_content()
                                         .parse::<f64>()
                                         .unwrap()))
            }

            TokenType::Boolean => {
                Ok(Expression::Boolean(self.tokenizer.current_content() == "true"))
            }

            TokenType::Text => Ok(Expression::Text(self.tokenizer.current_content())),

            TokenType::Ident => {
                let ident = Expression::Ident(self.tokenizer.current_content());

                if self.tokenizer.next_token() {
                    return match self.tokenizer.current().get_type() {
                               TokenType::Operator => self.operation(ident),
                               TokenType::Period => self.dot(ident),
                               TokenType::LBracket => self.index(ident),
                               TokenType::Colon => {
                        self.tokenizer.next_token();
                        if self.tokenizer.current().get_type() == TokenType::Colon {
                            self.colon(ident)
                        } else {
                            self.typed(ident)
                        }
                    }
                               TokenType::LParen => self.call(ident),
                               _ => {
                        self.tokenizer.prev_token();
                        Ok(ident)
                    }
                           };
                }

                Ok(ident)
            }

            TokenType::LParen => {

                self.tokenizer.next_token();

                let expression = try!(self.expression());

                self.tokenizer.next_token();

                try!(self.tokenizer.match_current(TokenType::RParen));

                self.tokenizer.next_token();

                if self.tokenizer.current().get_type() == TokenType::LParen {
                    return self.call(expression);
                }

                self.tokenizer.prev_token();

                Ok(expression)
            }

            TokenType::Module => {
                self.tokenizer.next_token();

                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                let body = try!(self.block());

                Ok(Expression::Module(ident, Box::new(body)))
            }

            TokenType::Class => {
                self.tokenizer.next_token();

                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                let body = try!(self.block());

                Ok(Expression::Class(ident, Box::new(body)))
            }

            TokenType::Implement => {
                self.tokenizer.next_token();

                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                let body = try!(self.block());

                Ok(Expression::Implement(ident, Box::new(body)))
            }

            TokenType::Struct => {
                self.tokenizer.next_token();

                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                let body = try!(self.block());

                Ok(Expression::Struct(ident, Box::new(body)))
            }

            TokenType::Import => {
                self.tokenizer.next_token();

                try!(self.tokenizer.match_current(TokenType::Text));

                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                if self.tokenizer.current().get_type() == TokenType::Library {
                    return Ok(Expression::Import(ident, true));
                }

                self.tokenizer.prev_token();

                Ok(Expression::Import(ident, false))
            }

            TokenType::Use => {
                self.tokenizer.next_token();

                Ok(Expression::Use(Box::new(try!(self.term()))))
            }

            TokenType::Def => {
                self.tokenizer.next_token();

                let name = self.tokenizer.current_content();

                self.tokenizer.next_token();

                let mut args = Vec::new();

                if self.tokenizer.current().get_type() == TokenType::Ident {
                    while self.tokenizer.current().get_type() == TokenType::Ident {
                        let n = self.tokenizer.current_content();

                        self.tokenizer.next_token();

                        try!(self.tokenizer.match_current(TokenType::Colon));

                        self.tokenizer.next_token();

                        let t = self.tokenizer.current_content();

                        self.tokenizer.next_token();

                        args.push((t, n));
                    }

                    self.tokenizer.next_token();
                }

                let body = try!(self.block());

                Ok(Expression::Function(name, args, Box::new(body)))
            }

            TokenType::Return => {
                self.tokenizer.next_token();

                Ok(Expression::Return(Box::new(try!(self.expression()))))
            }

            _ => Err(format!("unexpected term: {:#?}", token_type)),
        }
    }

    fn block(&mut self) -> Result<Vec<Statement>, String> {
        match self.tokenizer.current().get_type() {
            TokenType::Block(v) => {
                let mut p = Parser::from(Tokenizer::from(v));

                p.parse()
            }
            _ => {
                Err(format!(
                            "expected block, found: {:#?}",
                            self.tokenizer.current(),
                        ))
            }
        }
    }

    fn statement(&mut self) -> Result<Statement, String> {
        match self.tokenizer.current().get_type() {
            TokenType::Ident => {
                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                if self.tokenizer.current().get_type() != TokenType::Assign {
                    self.tokenizer.prev_token();

                    let expression = try!(self.expression());

                    return Ok(Statement::Expression(Box::new(expression)));
                }

                self.tokenizer.next_token();

                let expression = try!(self.expression());

                Ok(Statement::Assignment(ident, Box::new(expression)))
            }

            TokenType::Let => {
                self.tokenizer.next_token();

                let ident = self.tokenizer.current_content();

                self.tokenizer.next_token();

                try!(self.tokenizer.match_current(TokenType::Assign));

                self.tokenizer.next_token();

                let expression = try!(self.expression());

                Ok(Statement::Declaration(ident, Box::new(expression)))
            }

            TokenType::If => {
                self.tokenizer.next_token();

                let condition = try!(self.expression());

                self.tokenizer.next_token();

                let body = try!(self.block());

                self.tokenizer.next_token();

                if self.tokenizer.current().get_type() == TokenType::Else {
                    self.tokenizer.next_token();

                    let else_body = try!(self.block());

                    return Ok(Statement::IfElse(Box::new(condition),
                                                Box::new(Statement::Block(Box::new(body))),

                                                Box::new(Statement::Block(Box::new(else_body)))));
                }

                self.tokenizer.prev_token();

                Ok(Statement::If(Box::new(condition),
                                 Box::new(Statement::Block(Box::new(body)))))
            }

            _ => {
                let expression = try!(self.expression());

                Ok(Statement::Expression(Box::new(expression)))
            }
        }
    }

    fn expression(&mut self) -> Result<Expression, String> {
        let expr = try!(self.term());

        self.tokenizer.next_token();

        if self.tokenizer.remaining() > 0 {
            if self.tokenizer.current().get_type() == TokenType::Operator {
                return self.operation(expr);
            }

            self.tokenizer.prev_token();
        }

        Ok(expr)
    }
}
