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

    Class(String, Box<Vec<CElement>>, Option<Box<CElement>>),
    Implement(String, Box<Vec<CElement>>),
    FunctionDef(String, Vec<(String, String)>, Box<CElement>),

    IndexDot(Box<CElement>, Box<CElement>),
    IndexColon(Box<CElement>, Box<CElement>),
    IndexArray(Box<CElement>, Box<CElement>),

    Include(String),
    Module(String, Box<Vec<CElement>>),
    Struct(String, Box<Vec<CElement>>),
    Typed(Box<CElement>, Box<CElement>),
    Declaration(String, Box<CElement>),
    Assignment(String, Box<CElement>),

    Use(Box<CElement>),

    Operation(Box<CElement>, String, Box<CElement>),
    Function(String, Vec<(String, String)>, Box<Vec<CElement>>, Option<String>),

    Return(Box<CElement>),
}

#[derive(Debug, Clone)]
pub struct Environment {
    title:   String,
    imports: Vec<String>,
    header:  Vec<String>,
    pub global: Vec<CElement>,
}

impl<'a> Environment {
    pub fn new(title: String) -> Environment {
        Environment {
            title:   title.clone(),
            header:  vec!(
                format!("#ifndef {0}\n#define {0}", title),
            ),
            imports: Vec::new(),
            global:  Vec::new(),
        }
    }

    pub fn translate_imports(&mut self) -> String {
        let mut imports = "".to_string();

        for import in self.imports.iter() {
            imports.push_str(
                    &format!("#include {}\n", import),
                )
        }

        self.header.insert(1, imports);

        format!("#include \"{}.hpp\"\n", self.title)
    }

    pub fn class(&mut self, c: CElement) {
        self.header.push(translate_element(&c))
    }

    pub fn header(&self) -> String {
        let mut header = "".to_string();

        for s in &self.header {
            header.push_str(&s);
            header.push('\n')
        }

        header.push_str("#endif");
        header
    }

    pub fn translate_global(&self) -> String {
        let mut global = "".to_string();

        for module in &self.global {
            global.push_str(
                    &translate_element(&module),
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
                    CElement::Function(_, _, _, _)
                    | CElement::Implement(_, _)
                    | CElement::Module(_, _) => self.environment.global.push(c),
                    CElement::Include(i)     => self.environment.import(i),
                    CElement::Class(_, _, _)    => self.environment.class(c),
                    _ => continue,
                }
            }
        }
    }

    pub fn translate(&mut self) -> (String, String) {
        let mut source = "".to_string();

        source.push_str(&self.environment.translate_imports());
        source.push_str(&self.environment.translate_global());

        (source, self.environment.header())
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
        CElement::Use(ref e)    => format!("using namespace {};\n", translate_element(&**e)),

        CElement::FunctionDef(ref n, ref a, ref t) => {
            let mut args = "".to_string();
            let mut accum: usize = 0;

            for &(ref t, ref n) in a.iter() {
                if accum > 0 {
                    args.push_str(",")
                }

                args.push_str(&format!("{} {}", t, n));

                accum += 1
            }

            format!("{} {} ({});", translate_element(t), n, args)
        },

        CElement::Class(ref n, ref c, ref p) => {

                let mut class = "".to_string();

                for e in c.iter() {
                    class.push_str(
                            &translate_element(&e)
                        )
                }

                if let Some(ref d) = *p {
                    format!(
                        "class {} : {} {{\npublic:\n\t{}\n}};",
                        n, translate_element(&d), class,
                    )
                } else {
                    format!(
                        "class {} {{\npublic:\n\t{}\n}};",
                        n, class,
                    )
                }
            },

        CElement::Implement(ref n, ref c) => {

                let mut implementation = "".to_string();

                for e in c.iter() {
                    match e {
                        &CElement::Function(ref n1, ref a, ref c, ref t) => implementation.push_str(
                            &translate_element(
                                &CElement::Function(format!("{}::{}", n, n1), a.clone(), c.clone(), t.clone()),
                            )
                        ),
                        _ => panic!("Unexpected impementation of: {:?}", e),
                    }
                }

                format!("\n{}\n", implementation)
            },

        CElement::IndexDot(ref a, ref b)   => format!("{}.{}", translate_element(&**a), translate_element(&**b)),
        CElement::IndexColon(ref a, ref b) => format!("{}::{}", translate_element(&**a), translate_element(&**b)),
        CElement::IndexArray(ref a, ref b) => format!("{}[{}]", translate_element(&**a), translate_element(&**b)),

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
                translate_element(r),
                o.clone(),
                translate_element(l),
            ),

        CElement::Declaration(ref i, ref r) => {
                let mut line = "".to_string();

                line.push_str(
                        &format!("{} {} = {};", type_of(&**r), i, translate_element(r))
                    );

                line
            },

        CElement::Assignment(ref i, ref r) => {
                let mut line = "".to_string();

                line.push_str(
                        &format!("{} = {};", i, translate_element(r))
                    );

                line
            },

        CElement::Typed(ref i, ref r) => {
                let mut line = "".to_string();

                line.push_str(
                        &format!("{} {};", translate_element(&**r), translate_element(&**i))
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

        CElement::Struct(ref n, ref c) => {

                let mut structure = "".to_string();

                for e in c.iter() {
                    structure.push_str(
                            &translate_element(&e)
                        )
                }

                format!(
                        "struct {} {{\n\t{}\n}};",
                        n, structure,
                    )
            },

        CElement::Include(ref n) => format!("#include {}\n", n),
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
        Expression::Use(ref e)                     => CElement::Use(Box::new(expression(&**e))),

        Expression::FunctionDef(ref n, ref a, ref t) => {
            let mut retty = expression(&**t);

            retty = match &retty {
                &CElement::Ident(ref t) => if &t.clone() == n {
                    CElement::Ident("".to_owned())
                } else {
                    CElement::Ident(t.clone())
                },
                c => c.clone(),
            };

            CElement::FunctionDef(n.clone(), a.clone(), Box::new(retty))
        },

        Expression::Class(ref n, ref c, ref p)       => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                for s in c.iter() {
                    if let Some(c) = statement(s) {
                        statement_stack.push(c)
                    }
                }

                let parent = match *p {
                    Some(ref p) => Some(Box::new(expression(&*p))),
                    None        => None,
                };

                CElement::Class(
                    n.clone(),
                    Box::new(statement_stack),
                    parent,
                )
            },

        Expression::Implement(ref n, ref c)            => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                for s in c.iter() {
                    if let Some(c) = statement(s) {
                        statement_stack.push(c)
                    }
                }

                CElement::Implement(
                    n.clone(),
                    Box::new(statement_stack),
                )
            },

        Expression::Typed(ref r, ref f)            => CElement::Typed(
                Box::new(expression(&**r)), Box::new(expression(&**f)),
            ),

        Expression::IndexDot(ref a, ref b)         => CElement::IndexDot(
                Box::new(expression(&**a)),
                Box::new(expression(&**b)),
            ),

        Expression::IndexArray(ref a, ref b)       => CElement::IndexArray(
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

        Expression::Struct(ref n, ref c) => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                for s in c.iter() {
                    if let Some(c) = statement(s) {
                        statement_stack.push(c)
                    }
                }

                CElement::Struct(
                    n.clone(),
                    Box::new(statement_stack),
                )
            },

        Expression::Function(ref n, ref a, ref c, ref t)  => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                let mut retty = match *t {
                    Some(ref t) => Some(translate_element(&expression(&*t))),
                    None    => None,
                };

                for s in c.iter() {
                    if let Some(c) = statement(s) {

                        if let None = retty {
                            let expr = get_return(&c);
                            if let Some(e) = expr {
                                retty = Some(type_of(&e).to_string())
                            }
                        }

                        statement_stack.push(c)
                    }
                }

                retty = match retty {
                    Some(ref t) => {
                        if &&t == &&n {
                            Some("".to_owned())
                        } else {
                            Some(t.clone())
                        }
                    },

                    None => None,
                };

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

        Statement::Declaration(ref n, ref r) => Some(
                CElement::Declaration(
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