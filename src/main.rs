mod token;

#[macro_use]
extern crate lalrpop_util;
use std::fs::File;
use std::io::{Read};
use colored::Colorize;
use token::MyToken;

fn split_by_exprs(input: String) -> Vec<String> {
    input.split(';').map(|s| format!("{};", s)).collect()
}

fn handle_tokens(tokens: &Vec<MyToken>, full_text: String) { 
    println!("{:-<35} {} {:-<35} ", "", "Token List".yellow().bold(), "");
    println!("{:^23}|{:^34}|{:^23}", "Token".blue(), "Lexeme".blue(), "Position".blue());
    println!("{:-<82} ", "");

    let lines: Vec<&str> = full_text.lines().collect();
    let mut _token_index: usize = 0;
    
    let mut index: usize = 0;
    for (line_row, line) in lines.iter().enumerate() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if !words.is_empty() {
            let mut line_index: usize = 0;
            let mut token_value:String;
            let mut lexeme_value:String;
            loop { 
                match &tokens[index] {
                    MyToken::Type{token: a, lexeme: b} |
                    MyToken::Delimiter{token: a, lexeme: b} | 
                    MyToken::Literal{token: a, lexeme: b} |
                    MyToken::Identification{token: a, lexeme: b} |
                    MyToken::BinOperator{token: a, lexeme: b} |
                    MyToken::Keywoard{token: a, lexeme: b} => { token_value = a.clone(); lexeme_value = b.clone() },
                };

                match line[line_index..].find(&lexeme_value) {
                    Some(found_index) => {
                        let position = format!("{}:{}", line_row+1, line_index +  found_index + 1);
                        println!("{:^23}|{:^34}|{:^23}", token_value.green(), lexeme_value.white(), position);
                        index += 1;
                        line_index += lexeme_value.len() + found_index;
                    }, 
                    None => {
                        break;
                    }
                }
            }
        }
    }
}

#[allow(unused_variables)]
fn lex_analyze(exprs: &Vec<String>, full_text: String) {
    #[allow(unused_variables)]
    // Init parsers
    let inst_p   = parsers::INSTRUCTIONParser::new();
    let row: i64 = 0;
    let col: i64 = 0;
    let mut all_tokens: Vec<MyToken> = vec![];

    for expr in exprs {
        match inst_p.parse(expr) {
            Ok(tokens) => { 
                all_tokens.extend(tokens);
                continue;
            },
            Err(error) => { println!("{}: {:?}", "ERROR: ".red(), error); return; }
        };
    }
    handle_tokens(&all_tokens, full_text.clone());
}

fn main() {
    #[allow(unused_variables)]
    let file_path = "./resources/program2.txt";
    let mut file = File::open(file_path).expect("Not such file");
    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents);
    let exprs: Vec<String> = split_by_exprs(contents.clone());
    lex_analyze(&exprs, contents.clone());
}

lalrpop_mod!(pub parsers); 