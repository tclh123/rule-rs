use std::convert::Into;

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
        self.expr.matches(&context).map(|x| x.as_bool().unwrap())
        // Ok(true)
    }
}

#[derive(Clone, Debug)]
struct Expr {
    op: Op,
    args: Vec<Arg>,
}

#[derive(Clone, Debug)]
enum Arg {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Expr(Expr),
}

impl Into<Option<Expr>> for Arg {
    fn into(self) -> Option<Expr> {
        match self {
            Arg::Expr(e) => Some(e),
            _ => None,
        }
    }
}

impl Arg {
    fn as_bool(self) -> Option<bool> {
        match self {
            Arg::Bool(b) => Some(b),
            _ => None,
        }
    }
}

// TODO:
impl Expr {
    fn new(val: Json) -> Result<Expr> {
        // let args = match val {
        //     // TODO: Vec<Json> to Vec<Arg>
        //     Json::Array(args) => args,
        //     _ => None,
        // };
        // if val.as_array()
        //     ExprNotArrayError

        let op = Op {};
        let args = vec![];
        Ok(Expr { op: op, args: args })
    }

    pub fn matches<T: Serialize>(&self, context: &T) -> Result<Arg> {
        let args = self.args.iter().map(|arg|
            if let Arg::Expr(expr) = arg { expr.matches(context).unwrap() } else { arg.clone() }
            ).collect::<Vec<_>>();
        println!("DEBUG: args: {:?}", args);
        // for arg in self.args {
        //     if arg {
        //     }
        // }
        Ok(Arg::Bool(true))
    }
}
