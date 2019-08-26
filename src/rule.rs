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
pub struct Expr {
    op: Op,
    args: Vec<Arg>,
}

#[derive(Clone, Debug)]
pub enum Arg {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Expr(Expr),
}

impl Into<Option<Expr>> for Arg {
    fn into(self) -> Option<Expr> {
        match self {
            Arg::Expr(v) => Some(v),
            _ => None,
        }
    }
}

impl Into<Arg> for Json {
    fn into(self) -> Arg {
        match self {
            Json::Null => Arg::Null,
            Json::Bool(v) => Arg::Bool(v),
            Json::Number(v) => Arg::Number(v),
            Json::String(v) => Arg::String(v),
            Json::Array(v) => Arg::Expr(Expr::from_vec(v).unwrap()),
            Json::Object(v) => Arg::Expr(Expr::from_vec(v.values().cloned().collect()).unwrap()),
        }
    }
}

impl Arg {
    fn as_bool(self) -> Option<bool> {
        match self {
            Arg::Bool(v) => Some(v),
            _ => None,
        }
    }

    fn from_json(val: Json) -> Result<Arg> {
        match val {
            Json::Null => Ok(Arg::Null),
            Json::Bool(v) => Ok(Arg::Bool(v)),
            Json::Number(v) => Ok(Arg::Number(v)),
            Json::String(v) => Ok(Arg::String(v)),
            Json::Array(v) => Ok(Arg::Expr(Expr::from_vec(v)?)),
            Json::Object(v) => Ok(Arg::Expr(Expr::from_vec(v.values().cloned().collect())?)),
        }
    }
}

// TODO:
impl Expr {
    fn new(val: Json) -> Result<Expr> {
        match val {
            Json::Array(args) => {
                Expr::from_vec(args)
            },
            _ => Err(Error::ExprIsNotArrayError),
        }
    }

    fn from_vec(val: Vec<Json>) -> Result<Expr> {
        let mut args: Vec<Arg> = val.into_iter().map(|x| Arg::from_json(x)).collect::<Result<Vec<_>>>()?;
        let op_s = match args.remove(0) {
            Arg::String(s) => s,
            _ => return Err(Error::ExprOpIsNotStringError),
        };
        let op = match Op::get(&op_s) {
            Some(v) => v,
            None => return Err(Error::NoSuchOpError),
        };
        Ok(Expr { op: op.clone(), args: args })
    }

    pub fn matches<T: Serialize>(&self, context: &T) -> Result<Arg> {
        let args = self.args.iter().map(|arg|
            if let Arg::Expr(expr) = arg { expr.matches(context).unwrap() } else { arg.clone() }
            ).collect::<Vec<_>>();
        println!("DEBUG: args: {:?}", args);
        println!("DEBUG: op: {:?}", self.op);
        Ok((self.op.func)(args))
        // Ok(Arg::Bool(true))
    }
}
