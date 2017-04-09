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

    Include(String),
    Module(String, Box<Vec<CElement>>),
    Assignment(String, Box<CElement>),

    Operation(Box<CElement>, String, Box<CElement>),
    Function(String, Vec<String>, Box<Vec<CElement>>, Option<String>),

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
        CElement::Text(ref i)     => format!("\"{}\"", i.to_string()),

        CElement::Return(ref e) => format!("return {};\n", translate_element(&**e)),

        CElement::Function(ref n, ref a, ref c, ref t) => {
                let mut body = "".to_string();

                for e in c.iter() {
                    body.push_str(
                            &translate_element(&e)
                        )
                }

                let mut template = "template<".to_string();
                let mut args     = "".to_string();

                let mut accum: usize = 0;

                for n in a.iter() {
                    if accum > 0 {
                        template.push_str(",");
                        args.push_str(",")
                    }

                    let t = format!("TTT{}", accum);

                    template.push_str(&format!("class {}", &t));

                    args.push_str(&format!("{} {}", t, n));

                    accum += 1
                }

                template.push_str(">");

                let retty = match *t {
                        Some(ref rt) => if rt == "auto" {
                                            "double".to_string()
                                        } else {
                                            rt.to_string()
                                        },

                        None     => "void".to_string(),
                    };

                match n.as_str() {
                    "main" => format!(
                            "int {}({}) {{\n\t{}}}",
                            n, args, body,
                        ),

                    _ => format!(
                            "{}\nauto {}({}) -> {} {{\n\t{}}}",
                            template, n, args, retty, body,
                        )
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
        Expression::Return(ref e)                  => CElement::Return(Box::new(expression(&**e))),

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

                        let expr = match c.clone() {
                                CElement::Return(e) => Some(e),
                                _ => None,
                            };

                        if let Some(e) = expr {
                            retty = Some(type_of(&*e).to_string())
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

        _ => panic!("unknown expression: {:?}", ex),
    }
}

pub fn statement(st: &Statement) -> Option<CElement> {
    match *st {
        Statement::Expression(ref e) => Some(expression(&**e)),

        Statement::Assignment(ref n, ref r) => Some(
                CElement::Assignment(
                        n.clone(),
                        Box::new(expression(&**r)),
                    ),
            ),
        
        _ => None,
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