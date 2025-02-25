use crossterm::{
    cursor,
    terminal, ExecutableCommand,
};

use crossterm::terminal::ClearType;

use std::io::{self};
use crate::token::MyToken;

use once_cell::sync::Lazy; 
use std::sync::Mutex;

static STRUCT_LEVEL: Lazy<Mutex<u16>> = Lazy::new(|| {
    Mutex::new(0)
});

static STRUCT_BEGIN: Lazy<Mutex<bool>> = Lazy::new(|| {
    Mutex::new(true)
});

static OPERATION_LEVEL: Lazy<Mutex<usize>> = Lazy::new(|| {
    Mutex::new(0)
});
static END_STR: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(Vec::new())
});

pub fn print_declaration(tokens: &Vec<MyToken>) {
    #[warn(unused_mut)]
    let opr_lvl = OPERATION_LEVEL.lock().unwrap();
    let spaces = " ".repeat(*opr_lvl);

    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:}{:12} ═ DCL\n", spaces, b);
        },
        _ => {}
    }
}

pub fn print_declaration_with_init(tokens: &Vec<MyToken>) {
    let opr_lvl = OPERATION_LEVEL.lock().unwrap();
    let spaces = " ".repeat(*opr_lvl);

    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:}{:12} ╗", spaces, b);
        },
        _ => {}
    }

    match &tokens[tokens.len() - 3] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{}{:12} ╩ {:12} ╗", spaces, b, "INIT");
            println!("{}{:27} ╚ DCL", spaces, " ");
        },
        _ => {}
    }
}

pub fn print_array_declaration(tokens: &Vec<MyToken>) {
    let opr_lvl = OPERATION_LEVEL.lock().unwrap();
    let spaces = " ".repeat(*opr_lvl);

    match &tokens[0] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{}{:12} ╗", spaces, b);
        },
        _ => {}
    }

    match &tokens[2] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{}[{:3}]        ╣", spaces, b);
        },
        _ => {}
    }

    match &tokens[6] {
        MyToken::Literal{token: _a, lexeme: b} => { 
            println!("{}{:12} ╩ DCL\n", spaces, b);
        },
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{}{:12} ╩ DCL\n", spaces, b);
        },
        _ => {}
    }
}

pub fn print_struct_part_declaration(tokens: &Vec<MyToken>) {
    let opr_lvl = OPERATION_LEVEL.lock().unwrap();
    let spaces = " ".repeat(*opr_lvl);
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

    let mut strct_begin = STRUCT_BEGIN.lock().unwrap();

    let mut strct_level = STRUCT_LEVEL.lock().unwrap();
    if *strct_level < level {
       *strct_level = level;
    }

    if (*strct_level - level)*12 > 0 {
        let _ = stdout.execute(cursor::MoveRight((*strct_level - level)*12));
    }

    if level == 1 {
        *strct_begin = true;
         println!("{} ╚ {:12} ═ DCL ", spaces, ident);
    } else  {
        if *strct_begin == true {
            println!("{}{:12} ╗", spaces, ident);
            *strct_begin = false;
        } else {
            println!("{}{:12} ╣", spaces, ident);
        }
    }
}

pub fn print_bin_operation(tokens: &Vec<MyToken>) {
    let opr_lvl = OPERATION_LEVEL.lock().unwrap();
    let spaces = " ".repeat(*opr_lvl);
    let mut stdout = io::stdout();

    match &tokens[1] {
        MyToken::BinOperator{token: _a, lexeme: b1} => { 
            match &tokens[0] {
                MyToken::Literal{token: _a, lexeme: b} => { 
                    println!("{}{:12} ╗", spaces, b);
                },
                MyToken::Identification{token: _a, lexeme: b} => { 
                    println!("{}{:12} ╗", spaces, b);
                },
                _ => {}
            }

            match &tokens[2] {
                MyToken::Literal{token: _a, lexeme: b2} => { 
                    println!("{}{:12} ╩ {:12}", spaces, b2, b1);
                },
                MyToken::Identification{token: _a, lexeme: b2} => { 
                    println!("{}{:12} ╩ {:12}", spaces, b2, b1);
                },
                _ => {}
            }
        },

        MyToken::Delimiter{token: _a, lexeme: b1} => { 
            if tokens.len() > 4 {
                match &tokens[0] {
                    MyToken::Literal{token: _a, lexeme: b2} => { 
                        let _ = stdout.execute(cursor::MoveRight(24));
                        print!("{}    {:12}", spaces, b2);
                        let _ = stdout.execute(cursor::MoveUp(1));
                        print!("{}╗", spaces);
                        let _ = stdout.execute(cursor::MoveDown(1));
                        let _ = stdout.execute(cursor::MoveLeft(1));
                        println!("╩ {}", b1);
                    },
                    MyToken::Identification{token: _a, lexeme: b2} => { 
                        let _ = stdout.execute(cursor::MoveRight(24));
                        print!("{}    {:12}", spaces, b2);
                        let _ = stdout.execute(cursor::MoveUp(1));
                        print!("{}╗", spaces);
                        let _ = stdout.execute(cursor::MoveDown(1));
                        let _ = stdout.execute(cursor::MoveLeft(1));
                        println!("╩ {}", b1);
                    },
                    _ => {}
                }
            } else {
                match &tokens[0] {
                    MyToken::Identification{token: _a, lexeme: b2} => { 
                        match &tokens[2] {
                            MyToken::Literal{token: _a, lexeme: b3} => { 
                                println!("{}{:12}╗", spaces, b3);
                                println!("{}{:12}╩ {}", spaces, b2, b1);
                            },
                            MyToken::Identification{token: _a, lexeme: b3} => { 
                                println!("{}{:12}╗", spaces, b3);
                                println!("{}{:12}╩ {}", spaces, b2, b1);
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        },
        _ => {}
    }
}

pub fn print_loop(tokens: &Vec<MyToken>) {
    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl += 12;

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

    let mut end = END_STR.lock().unwrap();
    end.push("CYCLE".to_string());
}

pub fn print_loop2(_tokens: &Vec<MyToken>) {
    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl += 12;

    let mut stdout = io::stdout();

    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(20));
    println!(" {:6} ═ CYCLE CONDITION\n", " ");
    let _ = stdout.execute(cursor::MoveToColumn(0));

    let mut end = END_STR.lock().unwrap();
    end.push("CYCLE".to_string());
}

pub fn print_loop3(_tokens: &Vec<MyToken>) {
    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl += 12;

    println!("FOREVER ═ CYCLE CONDITION\n");
    let mut end = END_STR.lock().unwrap();
    end.push("CYCLE".to_string());
}

pub fn print_condition(tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(20));
    stdout.execute(terminal::Clear(ClearType::CurrentLine)).unwrap();
    let _ = stdout.execute(cursor::MoveUp(1));
    stdout.execute(terminal::Clear(ClearType::CurrentLine)).unwrap();
    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(13));
    println!("{:6} ═ IF CONDITION\n", " ");

    {
        let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
        *opr_lvl += 12;
    }
    print_bin_operation(&tokens[5..9].to_vec());

    {
        let _ = stdout.execute(cursor::MoveUp(1));
        let _ = stdout.execute(cursor::MoveRight(32));
        println!("{:8}╗", " ");
        let _ = stdout.execute(cursor::MoveRight(12));
        println!("{:27} ╚ MAIN BRANCH\n", " ");
    }

    let mut end = END_STR.lock().unwrap();
    end.push("CONDITION".to_string());
}

pub fn print_else(_tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();
    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(32));
    println!("{:8}╗", " ");
    let _ = stdout.execute(cursor::MoveRight(12));
    println!("{:27} ╚ ELSE BRANCH", " ");
}

pub fn print_end() {
    let mut end = END_STR.lock().unwrap();
    println!("\nEND {:8} ═ {:} END\n", " ", end.last().unwrap());
    end.pop();

    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl -= 12;
}

pub fn print_proc_begin1(tokens: &Vec<MyToken>) {
    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl += 12;

    match &tokens[0] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╗", b);
            println!("{:12} ╚ {:12}", " ", "PROC BEGIN");
        },
        _ => {}
    }

    let mut end = END_STR.lock().unwrap();
    end.push("PROC".to_string());
}

pub fn print_proc_head(tokens: &Vec<MyToken>) {
    match &tokens[4] {
        MyToken::Identification{token: _a, lexeme: b1} => { 
            println!("{:12} ╗", b1);

            match &tokens[6] {
                MyToken::Identification{token: _a, lexeme: b2} => { 
                    println!("{:12} ╩ {:12}", b2, "PROC PARAMS\n");
                },
                _ => {}
            }
        },
        _ => {}
    }
}

pub fn print_proc_return(tokens: &Vec<MyToken>) {
    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╗", b);
            println!("{:12} ╚ {:12}", " ", "PROC RETURN");
        },
        _ => {}
    }
}

pub fn print_function_call(tokens: &Vec<MyToken>) {
    match &tokens[1] {
        MyToken::Identification{token: _a, lexeme: b} => { 
            println!("{:12} ╗", b);
            println!("{:12} ╚ {:12}", " ", "PROC CALL");
        },
        _ => {}
    }
}

pub fn print_select(_tokens: &Vec<MyToken>) {
    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl += 12;
    println!("SELECT {:20} ═ {:12}", " ", "CONDITION BEGIN");
}

pub fn print_when_condition(_tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(32));
    println!("{:8}╗", " ");
    let _ = stdout.execute(cursor::MoveRight(12));
    println!("{:28}╚ WHEN BRAIN\n", " ");
    let _ = stdout.execute(cursor::MoveToColumn(0));
}

pub fn print_otherwise_condition(_tokens: &Vec<MyToken>) {
    let mut stdout = io::stdout();

    let _ = stdout.execute(cursor::MoveUp(1));
    let _ = stdout.execute(cursor::MoveRight(32));
    println!("{:8}╗", " ");
    let _ = stdout.execute(cursor::MoveRight(12));
    println!("{:28}╚ OTHERWISE BRAIN\n", " ");
    let _ = stdout.execute(cursor::MoveToColumn(0));

    let mut opr_lvl = OPERATION_LEVEL.lock().unwrap();
    *opr_lvl -= 12;
}