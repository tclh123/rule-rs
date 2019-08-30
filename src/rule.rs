use serde::Serialize;
use serde_json::value::{to_value, Value as Json};
use serde_json::Map;

use crate::op::Op;
use crate::error::{Error, Result};
use crate::arg::Arg;

pub struct Rule {
    expr: Expr,
}

impl Rule {
    pub fn new(val: Json) -> Result<Rule> {
        Ok(Rule {
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
        self.expr.matches(context)?.as_bool().ok_or(Error::FinalResultNotBoolError)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Expr {
    op: Op,
    args: Vec<Arg>,
}


impl Expr {
    pub fn new(val: Json) -> Result<Expr> {
        match val {
            Json::Array(args) => {
                Expr::from_vec(args)
            },
            _ => Err(Error::ExprIsNotArrayError),
        }
    }

    pub fn from_vec(val: Vec<Json>) -> Result<Expr> {
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
    }
}
