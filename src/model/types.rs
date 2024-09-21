
use std::iter::Map;

#[derive(Debug)]
pub struct Environment {
    classes: Vec<Class>,
    env_vars: Map<String, String>
}

#[derive(Debug)]
pub struct Class {
    properties: Vec<Property>, // ??
    functions: Vec<Function>
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

}

#[derive(Debug)]
pub struct Statement {
    // ?? I'm too dumb for this :)
}