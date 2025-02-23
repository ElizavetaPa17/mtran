mod token;
mod ast_utility;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parsers); 

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read};
use colored::Colorize;
use token::MyToken;

fn split_by_exprs(input: String) -> Vec<String> {
    input.split(';').map(|s| format!("{};", s)).collect()
}

fn handle_tokens(tokens: &Vec<MyToken>, full_text: String) { 
    let mut literal_table:  HashMap<(String, String), usize> = HashMap::new();
    let mut keyword_table:  HashMap<String, usize> = HashMap::new();
    let mut operator_table: HashMap<String, usize> = HashMap::new();
    let mut identif_table:  HashMap<String, usize> = HashMap::new(); 
    let mut delimit_table:  HashMap<String, usize> = HashMap::new();

    let mut literal_id:  usize = 0;
    let mut keyword_id:  usize = 0;
    let mut operator_id: usize = 0;
    let mut identif_id:  usize = 0;
    let mut delimit_id:  usize = 0;

    println!("{:-<43} {} {:-<43} ", "", "Token List".yellow().bold(), "");
    println!("{:^23}|{:^34}|{:^23}|{:^15}", "Token".blue(), "Lexeme".blue(), "Position".blue(), "Reference".blue());
    println!("{:-<98} ", "");

    let lines: Vec<&str> = full_text.lines().collect();
    let mut _token_index: usize = 0;
    
    let mut index: usize = 0;
    for (line_row, line) in lines.iter().enumerate() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if !words.is_empty() {
            let mut line_index: usize = 0;
            let mut token_value:  String;
            let mut lexeme_value: String;
            let mut reference:    String = String::new();

            loop { 
                match &tokens[index] {
                    MyToken::Type{token: a, lexeme: b} => { 
                        token_value = a.clone(); lexeme_value = b.clone();
                    },
                    MyToken::Delimiter{token: a, lexeme: b} => { 
                        token_value = a.clone(); lexeme_value = b.clone();
                        if !delimit_table.contains_key(&token_value) {
                            delimit_table.insert(token_value.clone(), delimit_id);
                            delimit_id += 1;
                        } 
                        reference = format!("DEL_T:{}", delimit_table[&token_value]);
                    },
                    MyToken::Literal{token: a, lexeme: b} => { 
                        token_value = a.clone(); lexeme_value = b.clone();
                        if !literal_table.contains_key(&(lexeme_value.clone(), token_value.clone())) {
                            literal_table.insert((lexeme_value.clone(), token_value.clone()), literal_id);
                            literal_id += 1;
                        } 
                        reference = format!("LIT_T:{}", literal_table[&(lexeme_value.clone(), token_value.clone())]);
                    },
                    MyToken::Identification{token: a, lexeme: b} => { 
                        token_value = a.clone(); lexeme_value = b.clone();
                        if !identif_table.contains_key(&lexeme_value.clone()) {
                            identif_table.insert(lexeme_value.clone(), identif_id);
                            identif_id += 1;
                        } 
                        reference = format!("ID_T:{}", identif_table[&lexeme_value]);
                    },
                    MyToken::BinOperator{token: a, lexeme: b} => {
                        token_value = a.clone(); lexeme_value = b.clone();
                        if !operator_table.contains_key(&token_value) {
                            operator_table.insert(token_value.clone(), operator_id);
                            operator_id += 1;
                        } 
                        reference = format!("OP_T:{}", operator_table[&token_value]);
                    },
                    MyToken::Keywoard{token: a, lexeme: b} => { 
                        token_value = a.clone(); lexeme_value = b.clone();
                        if !keyword_table.contains_key(&token_value) {
                            keyword_table.insert(token_value.clone(), keyword_id);
                            keyword_id += 1;
                        } 
                        reference = format!("KW_T:{}", keyword_table[&token_value]);
                    },
                };

                match line[line_index..].find(&lexeme_value) {
                    Some(found_index) => {
                        let position = format!("{}:{}", line_row+1, line_index +  found_index + 1);
                        println!("{:^23}|{:^34}|{:^23}|{:>10}", token_value.green(), lexeme_value.white(), position, reference);
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

    let mut vec1: Vec<(String, usize)>;
    let mut vec2: Vec<((String, String), usize)>;

    println!("\n{:-<22} {} {:-<22}", "", "Literal Table".yellow().bold(), "");
    println!("{:^8}|{:^20}|   {}\n{:-<59}", "ID", "Value", "Type", "");
    vec2 = literal_table.into_iter().collect();
    vec2.sort_by(|a, b| a.1.cmp(&b.1));
    for (key, value) in vec2 {
        println!(" {:<7}|{:^20}|   {}", value, key.0.green(), key.1);
    }

    println!("\n{:-<4} {} {:-<4} ", "", "Identificator Table".yellow().bold(), "");
    println!("{:^8}|   {}\n{:-<29}", "ID", "Identificator", "");
    vec1 = identif_table.into_iter().collect();
    vec1.sort_by(|a, b| a.1.cmp(&b.1));
    for (key, value) in vec1 {
        println!(" {:<7}|   {}", value, key);
    }

    println!("\n{:-<5} {} {:-<5} ", "", "Keyword Table".yellow().bold(), "");
    println!("{:^8}|{:^15}\n{:-<25}", "ID", "Keyword", "");
    vec1 = keyword_table.into_iter().collect();
    vec1.sort_by(|a, b| a.1.cmp(&b.1));
    for (key, value) in vec1 {
        println!(" {:<7}|{:^15}", value, key);
    }

    println!("\n{:-<5} {} {:-<5} ", "", "Operator Table".yellow().bold(), "");
    println!("{:^8}|{:^15}\n{:-<26}", "ID", "Operator", "");
    vec1 = operator_table.into_iter().collect();
    vec1.sort_by(|a, b| a.1.cmp(&b.1));
    for (key, value) in vec1 {
        println!(" {:<7}|{:^15}", value, key);
    }

    println!("\n{:-<5} {} {:-<5} ", "", "Delimiter Table".yellow().bold(), "");
    println!("{:^8}|{:^15}\n{:-<27}", "ID", "Delimiter", "");
    vec1 = delimit_table.into_iter().collect();
    vec1.sort_by(|a, b| a.1.cmp(&b.1));
    for (key, value) in vec1 {
        println!(" {:<7}|{:^15}", value, key);
    }
}

fn find_token_in_text(text: &str, word: &str) -> Option<(usize, usize)> {
    for (line_number, line) in text.lines().enumerate() {
        if let Some(column_number) = line.find(word) {
            return Some((line_number + 1, column_number + 1));
        }
    }
    None
}

fn handle_error(expr: &str, text: &str) {
    match find_token_in_text(text, expr) {
        Some((line, column)) => println!("{}: Unrecognized token {} at {}:{}", "ERROR: ".red(), expr, line, column),
        None => {}
    }
}

#[allow(unused_variables)]
fn lex_analyze(exprs: &Vec<String>, full_text: String) {
    #[allow(unused_variables)]
    // Init parsers
    let inst_p   = parsers::INSTRUCTIONParser::new();
    let mut all_tokens: Vec<MyToken> = vec![];
    let mut was_error: bool = false;

    for expr in exprs {
        match inst_p.parse(expr) {
            Ok(tokens) => { 
                all_tokens.extend(tokens);
                continue;
            },
            Err(error) => { handle_error(&(expr.trim()), &full_text); was_error = true; }
        };
    }

    if !was_error {
        //handle_tokens(&all_tokens, full_text.clone());
    }
}

fn main() {
    #[allow(unused_variables)]
    let file_path = "./resources/program_test.txt";
    let mut file = File::open(file_path).expect("Not such file");
    let mut contents = String::new();

    let _ = file.read_to_string(&mut contents);
    let exprs: Vec<String> = split_by_exprs(contents.clone());
    lex_analyze(&exprs, contents.clone());
}