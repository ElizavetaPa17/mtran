#[derive(Debug)]
pub enum MyToken {
    Delimiter      { token: String, lexeme: String },
    Literal        { token: String, lexeme: String },
    Type           { token: String, lexeme: String },
    Identification { token: String, lexeme: String },
    BinOperator    { token: String, lexeme: String },
    Keywoard       { token: String, lexeme: String }
}