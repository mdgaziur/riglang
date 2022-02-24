use crate::OutputType;
use colored::Colorize;
use rig_analyzer::code_analyzer::analyze_code;
use rig_lexer::Lexer;
use rig_parser::{parse, Parser};

pub fn run(file_name: String, unpretty: Option<OutputType>, reconstruct_from_ast: bool) {
    let file = match std::fs::read_to_string(&file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "{}",
                format!("Failed to open file \"{}\": {}", file_name, e).red()
            );
            return;
        }
    };

    let mut lexer = Lexer::new(&file, &file_name);
    let tokens = lexer.lex();

    if unpretty == Some(OutputType::LexicalTokens) {
        println!("{:#?}", tokens.0);
    }

    if !tokens.1.is_empty() {
        for err in tokens.1 {
            err.print(&file);
        }
        return;
    }

    let mut parser = Parser::new(&tokens.0);
    let ast = parse(&mut parser);

    if unpretty == Some(OutputType::Ast) {
        println!("{:#?}", ast.0);
        return;
    }

    if !ast.1.is_empty() {
        for err in &ast.1 {
            err.print(&file);
        }
    }

    if reconstruct_from_ast {
        for node in ast.0 {
            println!("{}", node.to_string(0));
        }
        return;
    }

    if !ast.1.is_empty() {
        return;
    }

    let res = analyze_code(&ast.0);

    // Show warnings first
    for err in res {
        err.print(&file);
    }
}
