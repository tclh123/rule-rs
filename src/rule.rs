use serde::Serialize;
use serde_json::value::{to_value, Value as Json, Number};

use crate::op::Op;
use crate::error::{Error, Result};

pub struct Rule {
    // rule: Json,
    expr: Expr,
}

impl Rule {
    pub fn new(val: Json) -> Result<Rule> {
        Ok(Rule {
            // rule: val,
            expr: Expr::new(val)?,
        })
    }

    pub fn from_value<T: Serialize>(val: T) -> Result<Rule> {
        Rule::new(to_value(val)?)
    }

    // TODO: from_str

    pub fn matches<T: Serialize>(&self, context: T) -> Result<bool> {
        Ok(true)
    }
}

struct Expr {
    op: Op,
    args: Vec<Arg>,
}

enum Arg {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Expr(Expr),
}

// TODO:
impl Expr {
    fn new(val: Json) -> Result<Expr> {
        let op = Op {};
        let args = vec![];
        Ok(Expr { op: op, args: args })
    }
}
