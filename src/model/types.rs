
use std::collections::HashMap;

#[derive(Debug)]
pub struct Environment {
    pub functions: Vec<Function>,
    pub imports: Vec<String>,   // placeholder
    pub env_vars: HashMap<String, String>
}

#[derive(Debug)]
pub struct Function {
    parameters: Vec<Variable>,
    statements: Vec<Statement>,
    returns: Variable
}

#[derive(Debug)]
pub struct Property {
    variable: Variable
}

#[derive(Debug)]
pub struct Variable {
    var_type: VarType
}

#[derive(Debug)]
pub struct Statement {
    // ?? I'm too dumb for this :)
}

#[derive(Debug)]
pub struct VarType {
    lexeme: String
    // differentiate void, primitive or other type (are there other types?)
}

impl Environment {
    pub fn new() -> Self {
        Self { functions: vec![], imports: vec![], env_vars: HashMap::new() }
    }
}