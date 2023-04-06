use crafting_interpreters::{
    scanner::Scanner,
    token::{LiteralVal, TokenType},
    Lox,
};

#[test]
fn simple_test() {
    let source = r#"// this is a comment
( // grouping stuff
!!=* // operators
"hey"
1.25"#
        .to_string();
    let mut scanner = Scanner::new(source);
    let mut lox = Lox::default();
    let tokens = scanner.scan_tokens(&mut lox);

    assert_eq!(tokens[0].token_type, TokenType::LeftParen);
    assert_eq!(tokens[1].token_type, TokenType::Bang);
    assert_eq!(tokens[2].token_type, TokenType::BangEqual);
    assert_eq!(tokens[3].token_type, TokenType::Star);
    assert_eq!(
        tokens[4].literal,
        Some(LiteralVal::Str(String::from("hey")))
    );
    assert_eq!(tokens[5].literal, Some(LiteralVal::Number(1.25)));
}
