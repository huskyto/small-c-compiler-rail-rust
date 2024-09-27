

    // Missing keywords
// &["auto", "break", "case", "default", "do", "enum",
// "extern", "register", "signed",
// "sizeof", "static", "struct", "switch", "typedef", "union", "unsigned", "volatile", "while"];

pub struct Keywords { }

const VarTypes: [&str; 6] = [ Keywords::CHAR, Keywords::SHORT, Keywords::INT,
                              Keywords::LONG, Keywords::FLOAT, Keywords::DOUBLE ];

impl Keywords {
    pub const INCLUDE: &'static str = "include";
    pub const CHAR: &'static str = "char";
    pub const SHORT: &'static str = "short";
    pub const INT: &'static str = "int";
    pub const LONG: &'static str = "long";
    pub const FLOAT: &'static str = "float";
    pub const DOUBLE: &'static str = "double";
    pub const CONST: &'static str = "const";
    pub const CONTINUE: &'static str = "continue";
    pub const IF: &'static str = "if";
    pub const ELSE: &'static str = "else";
    pub const FOR: &'static str = "for";
    pub const GOTO: &'static str = "goto";
    pub const RETURN: &'static str = "return";
    pub const VOID: &'static str = "void";
}