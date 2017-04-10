use super::ast::{
        Expression, Statement
    };

use super::token::Operator;

#[derive(Debug, Clone)]
pub enum CElement {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),

    Ident(String),

    If(Box<CElement>, Box<CElement>),
    IfElse(Box<CElement>, Box<CElement>, Box<CElement>),

    Block(Box<Vec<CElement>>),

    Call(Box<CElement>, Box<Vec<CElement>>),

    IndexDot(Box<CElement>, Box<CElement>),
    IndexColon(Box<CElement>, Box<CElement>),

    Include(String),
    Module(String, Box<Vec<CElement>>),
    Assignment(String, Box<CElement>),

    Operation(Box<CElement>, String, Box<CElement>),
    Function(String, Vec<(String, String)>, Box<Vec<CElement>>, Option<String>),

    Return(Box<CElement>),
}

#[derive(Debug, Clone)]
pub struct Environment {
    title:   String,
    imports: Vec<String>,
    pub global: Vec<CElement>,
}

impl<'a> Environment {
    pub fn new(title: String) -> Environment {
        Environment {
            title:   title,
            imports: Vec::new(),
            global: Vec::new(),
        }
    }

    pub fn translate_imports(&self) -> String {
        let mut imports = "".to_string();

        for import in self.imports.iter() {
            imports.push_str(
                    &format!("#include {}\n", import),
                )
        }

        imports.push_str("using namespace std;\n");

        imports
    }

    pub fn translate_global(&mut self) -> String {
        let mut global = "".to_string();

        for module in self.global.iter() {
            global.push_str(
                    &translate_element(module),
                )
        }

        global
    }

    pub fn import(&mut self, element: String) {
        self.imports.push(element)
    }
}

#[derive(Debug, Clone)]
pub struct Translater {
    environment: Environment,
}

impl Translater {
    pub fn new(title: String) -> Translater {
        Translater {
            environment: Environment::new(title),
        }
    }

    pub fn make_environment(&mut self, ast: Vec<Statement>) {
        for s in ast.iter() {
            if let Some(c) = statement(s) {
                match c {
                    CElement::Include(i)   => self.environment.import(i),
                    CElement::Function(_, _, _, _)
                    | CElement::Module(_, _) => self.environment.global.push(c),
                    _ => continue,
                }
            }
        }
    }

    pub fn translate(&mut self) -> String {
        let mut source = "".to_string();

        source.push_str(&self.environment.translate_imports());
        source.push_str(&self.environment.translate_global());

        source
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }
}

pub fn translate_element(ce: &CElement) -> String {
    return match *ce {
        CElement::Integer(ref i)  => i.to_string(),
        CElement::Float(ref i)    => i.to_string(),
        CElement::Boolean(ref i)  => i.to_string(),
        CElement::Ident(ref i)    => i.to_string(),
        CElement::Text(ref i)     => format!("\"{}\"", i.to_string()),

        CElement::Return(ref e) => format!("return {};\n", translate_element(&**e)),

        CElement::IndexDot(ref a, ref b) => format!("{}.{}", translate_element(&**a), translate_element(&**b)),
        CElement::IndexColon(ref a, ref b) => format!("{}::{}", translate_element(&**a), translate_element(&**b)),

        CElement::Block(ref c) => {
                let mut block = "".to_string();

                for e in c.iter() {
                    block.push_str(
                            &translate_element(&e)
                        )
                }

                format!(
                        "{}",
                        block,
                    )
            },

        CElement::If(ref e, ref c) => format!(
            "if({}) {{{}}}", translate_element(&e), translate_element(&c),
        ),

        CElement::IfElse(ref e, ref c, ref c1) => format!(
            "if({}) {{{}}} else {{{}}}", 
            translate_element(&e),
            translate_element(&c),
            translate_element(&c1),
        ),

        CElement::Call(ref c, ref e) => {
                let mut args = "".to_string();

                let mut first = true;

                for a in e.iter() {
                    
                    if first {
                        first = false
                    } else {
                        args.push(',')
                    }

                    args.push_str(
                            &translate_element(&a)
                        )
                }

                format!(
                    "{}({})", translate_element(&c), args,
                )
            },

        CElement::Function(ref n, ref a, ref c, ref t) => {
                let mut body = "".to_string();

                for e in c.iter() {
                    body.push_str(
                            &format!("{};\n", translate_element(&e))
                        );
                }

                let mut args     = "".to_string();
                let mut accum: usize = 0;

                for &(ref t, ref n) in a.iter() {
                    if accum > 0 {
                        args.push_str(",")
                    }

                    args.push_str(&format!("{} {}", t, n));

                    accum += 1
                }

                let retty = match *t {
                    Some(ref rt) => rt.to_string(),
                    None         => "void".to_string(),
                };

                match n.as_str() {
                    "main" => format!(
                            "int main({}) {{\n\t{}}}",
                            args, body,
                        ),
                    
                    _ => format!(
                            "{} {}({}) {{\n\t{}}}",
                            retty, n, args, body,
                        ),
                }
            },

        CElement::Operation(ref l, ref o, ref r) => format!(
                "({} {} {})",
                translate_element(l),
                o.clone(),
                translate_element(r),
            ),

        CElement::Assignment(ref i, ref r) => {
                let mut line = "".to_string();

                line.push_str(
                        &format!("{} {} = {};", type_of(&**r), i, translate_element(r))
                    );

                line
            },

        CElement::Module(ref n, ref c) => {

                let mut module = "".to_string();

                for e in c.iter() {
                    module.push_str(
                            &translate_element(&e)
                        )
                }

                format!(
                        "namespace {} {{\n\t{}\n}}",
                        n, module,
                    )
            },

        _ => panic!("unknown element: {:?}", ce),
    }
}

pub fn expression(ex: &Expression) -> CElement {
    match *ex {
        Expression::Integer(ref i)                 => CElement::Integer(i.clone()),
        Expression::Float(ref f)                   => CElement::Float(f.clone()),
        Expression::Text(ref f)                    => CElement::Text(f.clone()),
        Expression::Boolean(ref f)                 => CElement::Boolean(f.clone()),
        Expression::Ident(ref f)                   => CElement::Ident(f.clone()),
        Expression::Return(ref e)                  => CElement::Return(Box::new(expression(&**e))),
        
        Expression::IndexDot(ref a, ref b)         => CElement::IndexDot(
                Box::new(expression(&**a)),
                Box::new(expression(&**b)),
            ),

        Expression::IndexColon(ref a, ref b)       => CElement::IndexColon(
                Box::new(expression(&**a)),
                Box::new(expression(&**b)),
            ),

        Expression::Call(ref e, ref c)             => {
                let mut expression_stack: Vec<CElement> = Vec::new();

                for s in c.iter() {
                    expression_stack.push(expression(s))
                }

                CElement::Call(
                    Box::new(expression(&**e)),
                    Box::new(expression_stack),
                )
            },

        Expression::Import(ref p, ref l) => if *l {
                CElement::Include(
                    format!("<{}>", p)
                )
            } else {
                CElement::Include(
                    format!("\"{}\"", p)
                )
            },

        Expression::Module(ref n, ref c) => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                for s in c.iter() {
                    if let Some(c) = statement(s) {
                        statement_stack.push(c)
                    }
                }

                CElement::Module(
                    n.clone(),
                    Box::new(statement_stack),
                )
            },

        Expression::Function(ref n, ref a, ref c)  => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                let mut retty = None;

                for s in c.iter() {
                    if let Some(c) = statement(s) {

                        let expr = get_return(&c);

                        if let Some(e)  = expr {
                            retty = Some(type_of(&e).to_string())
                        }

                        statement_stack.push(c)
                    }
                }

                CElement::Function(
                    n.clone(),
                    a.clone(),
                    Box::new(statement_stack),
                    retty,
                )
            },

        Expression::Operation(ref l, ref o, ref r) => CElement::Operation(
                Box::new(expression(l)),
                operator(o).to_string(),
                Box::new(expression(r)),
            ),
    }
}

pub fn statement(st: &Statement) -> Option<CElement> {
    match *st {
        Statement::Expression(ref e) => Some(expression(&**e)),

        Statement::If(ref e, ref c)  => Some(
                CElement::If(
                        Box::new(expression(&**e)),
                        Box::new(statement(&**c).unwrap()),
                    )
            ),

        Statement::IfElse(ref e, ref c, ref c1)  => Some(
                CElement::IfElse(
                        Box::new(expression(&**e)),
                        Box::new(statement(&**c).unwrap()),
                        Box::new(statement(&**c1).unwrap()),
                    )
            ),

        Statement::Block(ref c) => {
            let mut statement_stack: Vec<CElement> = Vec::new();

            for s in c.iter() {
                if let Some(c) = statement(s) {
                    statement_stack.push(c)
                }
            }

            Some(
                CElement::Block(
                        Box::new(statement_stack)
                    )
            )
        }

        Statement::Assignment(ref n, ref r) => Some(
                CElement::Assignment(
                        n.clone(),
                        Box::new(expression(&**r)),
                    ),
            ),
    }
}

fn type_of(element: &CElement) -> &str {
    match *element {
        CElement::Text(_)    => "string",
        CElement::Boolean(_) => "bool",
        CElement::Integer(_) => "int",
        CElement::Float(_)   => "float",
        _                    => "auto",
    }
}

fn get_return(st: &CElement) -> Option<CElement> {
    match *st {
        CElement::Return(ref e) => Some(*e.clone()),
        CElement::If(_, ref c)  => get_return(c),
        CElement::IfElse(_, ref c, ref c1)  => {
                match get_return(c) {
                    Some(e) => Some(e),
                    None    => get_return(c1),
                }
            },
        CElement::Block(ref c) => {
                for e in c.iter() {
                    return match get_return(&e) {
                        Some(e) => Some(e),
                        None    => continue
                    }
                }

                None
            },

        _ => None,
    }
}

fn operator<'a>(v: &Operator) -> &'a str {
    match *v {
        Operator::Mul     => "*",
        Operator::Mod     => "%",
        Operator::Div     => "/",
        Operator::Plus    => "+",
        Operator::Minus   => "-",
        Operator::Equal   => "==",
        Operator::NEqual  => "!=",
        Operator::Lt      => "<",
        Operator::Gt      => ">",
        Operator::LtEqual => "<=",
        Operator::GtEqual => ">=",
    }
}