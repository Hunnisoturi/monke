use crate::lexer::lexer::{Lexer, Token};
use anyhow::Result;
use std::io::stdin;

pub fn start_repl() -> Result<()> {
    stdin().lines().for_each(|line| {
        if let Ok(line) = line {
            let mut lexer = Lexer::new(line);

            while let Ok(token) = lexer.next_token() {
                println!("{}", token);

                if let Token::Eof = token {
                    break;
                }
            }
        }
    });

    return Ok(());
}
