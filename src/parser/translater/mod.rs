use super::ast::{
        Expression, Statement,
    };

#[derive(Debug, Clone)]
pub enum CElement {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Text(String),

    Include(String),
    Module(String, Box<Vec<CElement>>),
    Assignment(String, Box<CElement>),
} 

#[derive(Debug, Clone)]
pub struct Environment {
    title:   String,
    imports: Vec<String>,
    pub modules: Vec<CElement>,
}

impl<'a> Environment {
    pub fn new(title: String) -> Environment {
        Environment {
            title:   title,
            imports: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn translate_imports(&self) -> String {
        let mut imports = "".to_string();

        for import in self.imports.iter() {
            imports.push_str(
                    &format!("#include {}\n", import),
                )
        }

        imports
    }

    pub fn translate_modules(&mut self) -> String {
        let mut modules = "".to_string();

        for module in self.modules.iter() {
            modules.push_str(
                    &translate_element(module),
                )
        }

        modules
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
                    CElement::Module(_, _) => self.environment.modules.push(c),
                    _ => continue,
                }
            }
        }
    }

    pub fn translate(&mut self) -> String {
        let mut source = "".to_string();

        source.push_str(&self.environment.translate_imports());
        source.push_str(&self.environment.translate_modules());

        source
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }
}

pub fn translate_element(ce: &CElement) -> String {
    return match *ce {
        CElement::Integer(ref i) => i.to_string(),
        CElement::Float(ref i) => i.to_string(),

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
        Expression::Integer(ref i) => CElement::Integer(i.clone()),
        Expression::Float(ref f)   => CElement::Float(f.clone()),
        Expression::Text(ref f)    => CElement::Text(f.clone()),
        Expression::Boolean(ref f) => CElement::Boolean(f.clone()),
        _ => panic!("unknown expression: {:?}", ex),
    }
}

pub fn statement(st: &Statement) -> Option<CElement> {
    match *st {
        Statement::Import(ref p, ref l) => if *l {
                Some(
                        CElement::Include(
                            format!("<{}>", p)
                        )
                    )
            } else {
                Some(
                        CElement::Include(
                            format!("\"{}\"", p)
                        )
                    )
            },
        
        Statement::Module(ref n, ref c) => {
                let mut statement_stack: Vec<CElement> = Vec::new();

                for s in c.iter() {
                    if let Some(c) = statement(s) {
                        statement_stack.push(c)
                    }
                }

                Some(
                    CElement::Module(
                            n.clone(),
                            Box::new(statement_stack),
                        ),
                )
            },

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
        CElement::Float(_)   => "float",
        CElement::Integer(_) => "int",
        CElement::Boolean(_) => "bool",
        CElement::Text(_)    => "*char[]",
        _                    => panic!("can't infer type of element: {:?}", element),
    }
}