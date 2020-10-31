use crate::lexer::Lexer;
use crate::token::Token;

struct Compiler {
    lexer: Lexer,
    current_token: Option(Token),
    next_token: Option(Token),
    out: Vec<u8>,
}

impl Compiler {
    pub fn new(lexer: Lexer) -> Compiler {
        Compiler {
            lexer,
            current_token: None,
            next_token: None,
            out:Vec::new(),
        }
    }
}