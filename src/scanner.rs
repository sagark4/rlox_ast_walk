use crate::token::Token;
use crate::token_type::TokenType;
use crate::token::Literal;
pub(crate) fn scan_tokens(source: &str, had_error: &mut bool) -> Vec<Token>{
    let mut ans = Vec::new();
    // ans.push(
    //     Token {
    //         token_type: TokenType::StringLiteral,
    //         lexeme: String::from("Sample token."),
    //         literal: Literal::StringLiteral(String::from("Sample token.")),
    //         line: 42
    //     }
    // );
    // println!("{}{}", source, had_error);
    // crate::error("line", "message", had_error);
    ans
}