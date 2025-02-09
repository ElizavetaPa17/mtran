mod token;

#[macro_use]
extern crate lalrpop_util;
use std::fs::File;
use std::io::{Read};

fn split_by_exprs(input: String) -> Vec<String> {
    input.split(';').map(|s| format!("{};", s)).collect()
}

#[allow(unused_variables)]
fn lex_analyze(exprs: &Vec<String>) {
    #[allow(unused_variables)]
    // Init parsers
    let declar_p   = parsers::DECLARATIONParser::new();
    let struct_p   = parsers::STRUCT_TParser::new();

    for expr in exprs {
        println!("{}", expr);

        match declar_p.parse(expr) {
            Ok(some) => { 
                println!("decl: {:?}", some);
                continue;
            },
            _ => (),
        };

        match struct_p.parse(expr) {
            Ok(some) => { 
                println!("decl: {:?}", some);
                continue;
            },
            Err(some) => println!("err: {:?}", some),
        };
    }
}

fn main() {
    #[allow(unused_variables)]
    let file_path = "./resources/program1.txt";
    let mut file = File::open(file_path).expect("Not such file");
    let mut contents = String::new();

    file.read_to_string(&mut contents);
    let exprs: Vec<String> = split_by_exprs(contents);
    lex_analyze(&exprs);
}

lalrpop_mod!(pub parsers); 

#[test]
fn parsers1() {
    //assert!(parsers::DECLARATIONParser::new().parse("DCL a CHAR(30);").is_ok());
    //assert!(parsers::DECLARATIONParser::new().parse("DCL i FIXED BINARY(30);").is_ok());

    /*assert!(parsers::IDENTParser::new().parse("DCL ajknwd2o1p2ke").is_ok());
    assert!(parsers::IDENTParser::new().parse("DCL awJNAkwd_Wijdiq132_").is_ok());
    assert!(parsers::IDENTParser::new().parse("DCL 12awJNAkwd_Wijdiq132_").is_err());
    assert!(parsers::IDENTParser::new().parse("DCL _").is_err());

    let result = match parsers::CHAR_TParser::new().parse("CHAR(30)") {
        Ok(some) => println!("ok: {}", some),
        Err(some) => println!("err: {}", some)
    };

    assert!(parsers::CHAR_TParser::new().parse("CHAR(30)").is_ok());
    assert!(parsers::CHAR_TParser::new().parse("CHAR(-30)").is_err());
    assert!(parsers::CHAR_TParser::new().parse("CHAR(fqkmw)").is_err());

    assert!(parsers::CHAR_VAR_TParser::new().parse("VARYING CHARACTER(30)").is_ok());
    assert!(parsers::CHAR_VAR_TParser::new().parse("VARYING CHARACTER(-30)").is_err());
    assert!(parsers::CHAR_VAR_TParser::new().parse("VARYING CHARACTER(fqkmw)").is_err());

    assert!(parsers::LOG_TParser::new().parse("LOGICAL").is_ok());
    assert!(parsers::LOG_TParser::new().parse("LOGICALLO").is_err());

    assert!(parsers::PTR_TParser::new().parse("POINTER").is_ok());

    assert!(parsers::BIT_TParser::new().parse("BIT(30)").is_ok());
    assert!(parsers::BIT_TParser::new().parse("BIT(-30)").is_err());
    assert!(parsers::BIT_TParser::new().parse("BIT(fwkj)").is_err());

    assert!(parsers::FIX_BIN_TParser::new().parse("FIXED BINARY(30)").is_ok());
    assert!(parsers::FIX_BIN_TParser::new().parse("FIXED BIN(30)").is_ok());
    assert!(parsers::FIX_BIN_TParser::new().parse("FIXED BINARY(-30)").is_err());
    assert!(parsers::FIX_BIN_TParser::new().parse("FIXED BIN(-30)").is_err());
    assert!(parsers::FIX_BIN_TParser::new().parse("FIXED BINARY(3wd)").is_err());
    assert!(parsers::FIX_BIN_TParser::new().parse("FIXED BIN(awwad)").is_err());
    
    assert!(parsers::FLT_DEC_TParser::new().parse("FLOAT DECIMAL(10)").is_ok());
    assert!(parsers::FLT_DEC_TParser::new().parse("FLOAT DECIMAL(10, 10)").is_ok());
    assert!(parsers::FLT_DEC_TParser::new().parse("FLOAT DECIMAL(10,)").is_err());
    assert!(parsers::FLT_DEC_TParser::new().parse("FLOAT DECIMAL(10, awd)").is_err());
    assert!(parsers::FLT_DEC_TParser::new().parse("FLOAT DECIMAL(awd, 10)").is_err());
    assert!(parsers::FLT_DEC_TParser::new().parse("FLOAT DECIMAL").is_err());

    assert!(parsers::INT_TParser::new().parse("INTEGER").is_ok());
    assert!(parsers::INT_TParser::new().parse("INTEGERER").is_err());

    assert!(parsers::ARRAY_TParser::new().parse("DCL akwd(10) LIKE(Person)").is_ok());
    assert!(parsers::ARRAY_TParser::new().parse("DCL akwd(10, 10) LIKE(Person)").is_ok());
    assert!(parsers::ARRAY_TParser::new().parse("DCL A(10, 10) FIXED BINARY(10)").is_ok());
    assert!(parsers::ARRAY_TParser::new().parse("DCL akwd(10) LIKES(Person)").is_err());
    assert!(parsers::ARRAY_TParser::new().parse("DCL A(10, 10)").is_err());

    assert!(parsers::STRUCT_TParser::new().parse("DCL 1 Employee,
   	    2 ID FIXED BIN(31);").is_ok());
    assert!(parsers::STRUCT_TParser::new().parse("DCL 1 Person,
     	2 Name CHAR(50),
     	2 Age FIXED DECIMAL(3, 0);").is_ok());

    assert!(parsers::STRUCT_TParser::new().parse("DCL 1 Employee,
   	2 ID FIXED BIN(31),
   	2 Salary FIXED DECIMAL(10, 2),
   	2 Name CHAR(20);").is_ok());


    assert!(parsers::FLT_LITParser::new().parse("-123123.1212").is_ok());
    assert!(parsers::FLT_LITParser::new().parse("1.1212").is_ok());
    assert!(parsers::FLT_LITParser::new().parse(".1212").is_err());
    assert!(parsers::FLT_LITParser::new().parse("1.").is_err());
    assert!(parsers::FLT_LITParser::new().parse("-1.12dm").is_err());

    assert!(parsers::FLT_EXP_LITParser::new().parse("-123123.1212E-2").is_ok());
    assert!(parsers::FLT_EXP_LITParser::new().parse("1.1212E-2").is_ok());
    assert!(parsers::FLT_EXP_LITParser::new().parse("1.1212E-qkkqmwd").is_err());
    assert!(parsers::FLT_EXP_LITParser::new().parse("1.1E").is_err());
    assert!(parsers::FLT_EXP_LITParser::new().parse("-1s.12E1").is_err());

    assert!(parsers::DEC_INT_LITParser::new().parse("-3819").is_ok());
    assert!(parsers::UDEC_INT_LITParser::new().parse("9").is_ok());
    assert!(parsers::DEC_INT_LITParser::new().parse("9a").is_err());

    assert!(parsers::BIN_INT_LITParser::new().parse("010010010101B").is_ok());
    assert!(parsers::BIN_INT_LITParser::new().parse("010010010101").is_err());
    assert!(parsers::BIN_INT_LITParser::new().parse("01021B").is_err());

    assert!(parsers::CHR_LITParser::new().parse("'awkdmAKkwdmkawd2d21 1 2d   awkdm awd awkda  _dawd'").is_ok());
    assert!(parsers::CHR_LITParser::new().parse("awkdmAKkwdmkawd2d21 1 2d   awkdm awd awkda  _dawd'").is_err());

    assert!(parsers::PLUS_OPParser::new().parse("+").is_ok());
    assert!(parsers::MIN_OPParser::new().parse("-").is_ok());
    assert!(parsers::MUL_OPParser::new().parse("*").is_ok());
    assert!(parsers::DIV_OPParser::new().parse("/").is_ok());
    assert!(parsers::AND_BIT_OPParser::new().parse("AND").is_ok());
    assert!(parsers::AND_BIT_OPParser::new().parse("&").is_ok());
    assert!(parsers::OR_BIT_OPParser::new().parse("OR").is_ok());
    assert!(parsers::OR_BIT_OPParser::new().parse("|").is_ok());
    assert!(parsers::XOR_BIT_OPParser::new().parse("XOR").is_ok());
    assert!(parsers::XOR_BIT_OPParser::new().parse("^").is_ok());
    assert!(parsers::AND_CND_OPParser::new().parse("&&").is_ok());
    assert!(parsers::OR_CND_OPParser::new().parse("||").is_ok());*/
}