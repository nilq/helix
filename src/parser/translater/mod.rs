use super::ast::{
        Expression, Statement,
    };

#[derive(Debug, Clone)]
pub enum CElement {
    Include(String),
    Module(Box<Vec<CElement>>),
} 

#[derive(Debug, Clone)]
pub struct Environment {
    title:   String,
    imports: Vec<CElement>,
    modules: Vec<CElement>,
}

impl<'a> Environment {
    pub fn new(title: String) -> Environment {
        Environment {
            title:   title,
            imports: Vec::new(),
            modules: Vec::new(),
        }
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

    pub fn statement(&self, statement: Statement) -> Option<CElement> {
        match statement {
            Statement::Import(p, l) => if l {
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
            
            _ => None,
        }
    }
}