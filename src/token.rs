#[derive(Debug)]
pub enum MyToken {
    Delimiter { val: String },
    Literal { t: String, val: String},
    Type { val: String },
    Identification { val: String},
    BinOperator { val: String },
    CmpOperator { val: String },
    Keywoard { val: String },
}