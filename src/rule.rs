use std::convert::Into;

use serde::Serialize;
use serde_json::value::{to_value, Value as Json, Number};
use serde_json::Map;

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

    pub fn from_str(s: &str) -> Result<Rule> {
        Rule::new(serde_json::from_str(s)?)
    }

    pub fn matches<T: Serialize>(&self, context: &T) -> Result<bool> {
        // self.expr.matches(&context).map(|x| x.as_bool().unwrap())
        self.expr.matches(context)?.as_bool().ok_or(Error::FinalResultNotBoolError)
        // Ok(true)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr {
    op: Op,
    args: Vec<Arg>,
}

#[derive(Clone, Debug, PartialEq)]
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
    fn as_bool(&self) -> Option<bool> {
        match *self {
            Arg::Bool(v) => Some(v),
            _ => None,
        }
    }

    fn as_str(&self) -> Option<&str> {
        match *self {
            Arg::String(ref v) => Some(v),
            _ => None,
        }
    }

    // fn as_string(&self) -> Option<String> {
    //     match *self {
    //         Arg::String(ref v) => Some(v.to_owned()),
    //         _ => None,
    //     }
    // }

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

    fn from_context_var(args: &Vec<Arg>, context: &Map<String, Json>) -> Result<Arg> {
        Arg::from_json(context.get(args[0].as_str().ok_or(Error::ExprVarArgNotStringError)?)
            .ok_or(Error::ContextNoSuchVarError)?.clone())
    }
}

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
        self.matches_json(&to_value(context)?)
    }

    pub fn matches_json(&self, context: &Json) -> Result<Arg> {
        self.matches_json_dict(context.as_object().ok_or(Error::ContextNotDictError)?)
    }

    pub fn matches_json_dict(&self, context: &Map<String, Json>) -> Result<Arg> {
        let mut args = self.args.iter().map(|arg|
            if let Arg::Expr(expr) = arg { expr.matches_json_dict(context) } else { Ok(arg.clone()) }
            ).collect::<Result<Vec<_>>>()?;
        // println!("DEBUG: args: {:?}", args);
        // println!("DEBUG: op: {:?}", self.op);

        if &self.op.name == "var" {
            // special op var
            Arg::from_context_var(&args, context)
        } else {
            // always try first arg with context var
            let var = Arg::from_context_var(&args, context);
            if var.is_ok() {
                args[0] = var?;
            }
            Ok((self.op.func)(args))
        }
        // Ok(Arg::Bool(true))
    }
}
