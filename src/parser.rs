use crate::{lexer::Lexer, tokens::{Token, TokenType}, ast::{Program, Statement}};

pub struct Parser {
    token_stream: Vec<Token>,

    cur_token: Token,
    peek_token: Token,
    current_pos: usize,
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Self {
        let token_stream: Vec<Token> = lexer.lex();
        return Parser {
            cur_token: token_stream[0].clone(),
            peek_token: token_stream[0].clone(),
            current_pos: 0,
            token_stream: token_stream,
        };
    }

    pub fn next_token(&mut self) {
        self.current_pos += 1;
        self.cur_token = self.peek_token.clone();
        if self.current_pos + 1 < self.token_stream.len() {
            self.peek_token = self.token_stream[self.current_pos + 1].clone();
        }
    }

    pub fn parse_program(&self) -> Program {
        let mut program = Program::new(vec![]);

        while self.cur_token.0 != TokenType::EOF {
            let statement = self.parse_statement();
            if statement != Statement::EMPTY {
                program.statements.push(statement);
            }
        }
        return program;
    }

    pub fn parse_statement(&self) -> Statement {
        todo!()
    }
}
