use crossterm::{
    cursor,
    terminal, ExecutableCommand,
};

use std::io::{self, Write};
use crate::token::MyToken;

use once_cell::sync::Lazy; 
use std::sync::Mutex;

static struct_level: Lazy<Mutex<u16>> = Lazy::new(|| {
    Mutex::new(0)
});

static struct_begin: Lazy<Mutex<bool>> = Lazy::new(|| {
    Mutex::new(true)
});

static operation_level: Lazy<Mutex<u16>> = Lazy::new(|| {
    Mutex::new(0)
});

pub fn get_current_console_position() -> (u16, u16) {
    let mut cols: u16 = 0;
    let mut rows: u16 = 0;
    match terminal::size() {
        Ok(dimens) => {
            cols = dimens.0; rows = dimens.1;
        }
        Err(_) => {}
    }
    return (cols, rows);
}

pub fn print_declaration(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ═ DCL\n", b);
        },
        _ => {}
    }
}

pub fn print_declaration_with_init(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╗", b);
        },
        _ => {}
    }

    match &tokens[tokens.len() - 3] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{:12} ╩ {:12} ╗", b, "INIT");
            println!("{:27} ╚ DCL", " ");
        },
        _ => {}
    }
}

pub fn print_array_declaration(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    match &tokens[0] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╗", b);
        },
        _ => {}
    }

    match &tokens[2] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("[{:3}]        ╣", b);
        },
        _ => {}
    }

    match &tokens[6] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{:12} ╩ DCL\n", b);
        },
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╩ DCL\n", b);
        },
        _ => {}
    }
}

pub fn print_struct_part_declaration(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    let mut level: u16 = 0;
    let mut ident: String = String::new();

    match &tokens[0] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            match b.parse::<u16>() {
                Ok(number) => level = number,
                Err(_) => {},
            }
        },
        _ => {}
    }

    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            ident = b.clone();
        },
        _ => {}
    }

    let mut strct_begin = struct_begin.lock().unwrap();

    let mut strct_level = struct_level.lock().unwrap();
    if *strct_level < level {
       *strct_level = level;
    }

    if (*strct_level - level)*12 > 0 {
        let _ = stdout.execute(cursor::MoveRight((*strct_level - level)*12));
    }

    if level == 1 {
        *strct_begin = true;
         println!(" ╚ {:12} ═ DCL ", ident);
    } else  {
        if *strct_begin == true {
            println!("{:12} ╗", ident);
            *strct_begin = false;
        } else {
            println!("{:12} ╣", ident);
        }
    }
}

pub fn print_bin_operation(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    match &tokens[1] {
        MyToken::BinOperator{token: _a, lexeme: b1} => { 
            match &tokens[0] {
                MyToken::Literal{token: _a, lexeme: b} => { 
                    println!("{:12} ╗", b);
                },
                MyToken::Identification{token: _a, lexeme: b} => { 
                    println!("{:12} ╗", b);
                },
                _ => {}
            }

            match &tokens[2] {
                MyToken::Literal{token: _a, lexeme: b2} => { 
                    println!("{:12} ╩ {:12}", b2, b1);
                },
                MyToken::Identification{token: _a, lexeme: b2} => { 
                    println!("{:12} ╩ {:12}", b2, b1);
                },
                _ => {}
            }
        },

        MyToken::Delimiter{token: _a, lexeme: b1} => { 
            match &tokens[0] {
                MyToken::Literal{token: _a, lexeme: b2} => { 
                    let _ = stdout.execute(cursor::MoveRight(24));
                    println!("    {:12} ╩ {:12}", b2, b1);
                },
                MyToken::Identification{token: _a, lexeme: b2} => { 
                    let _ = stdout.execute(cursor::MoveRight(24));
                    print!("    {:12}", b2);
                    let _ = stdout.execute(cursor::MoveUp(1));
                    print!("╗");
                    let _ = stdout.execute(cursor::MoveDown(1));
                    let _ = stdout.execute(cursor::MoveLeft(1));
                    println!("╩ {}", b1);
                },
                _ => {}
            }
        },
        _ => {}
    }
}

pub fn print_loop(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    // retrieve condition
    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╗", b);
        },
        _ => {}
    }

    match &tokens[3] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{:12} ╩ CL_INIT{:6}╗", b, " ");
        },
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╩ CL_INIT{:6}╗", b, " ");
        },
        _ => {}
    }

    match &tokens[5] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{:12} ═ TO {:9} ╣", b, " ");
        },
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ═ TO {:9} ╣", b, " ");
        },
        _ => {}
    }

    match &tokens[7] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{:12} ═ BY {:9} ╩ CYCLE CONDITION\n", b, " ");
        },
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ═ BY {:9} ╩ CYCLE CONDITION\n", b, " ");
        },
        _ => {}
    }
}

pub fn print_loop2(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(20));
    println!(" {:6} ═ CYCLE CONDITION\n", " ");
    let _ = stdout.execute(cursor::MoveToColumn(0));
}

pub fn print_loop3(tokens: &Vec<MyToken>) {
    println!("FOREVER ═ CYCLE CONDITION\n");
}