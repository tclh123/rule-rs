use std::convert::Into;

use serde_json::value::{Value as Json};
use serde_json::Map;

use crate::rule::{Expr};
use crate::error::{Error, Result};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Arg {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
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
            Json::Number(v) => { 
                v.as_i64().map_or_else(|| Arg::Float(v.as_f64().unwrap()), |i| Arg::Int(i))
            },
            Json::String(v) => Arg::String(v),
            Json::Array(v) => Arg::Expr(Expr::from_vec(v).unwrap()),
            Json::Object(v) => Arg::Expr(Expr::from_vec(v.values().cloned().collect()).unwrap()),
        }
    }
}

impl Into<bool> for Arg {
    fn into(self) -> bool {
        match self {
            Arg::Null => false,
            Arg::Bool(v) => v,
            Arg::Int(v) => v != 0,
            Arg::Float(v) => v != 0.0,
            Arg::String(v) => v.is_empty(),
            _ => false,
        }
    }
}

impl Into<bool> for &Arg {
    fn into(self) -> bool {
        match self {
            Arg::Null => false,
            Arg::Bool(v) => *v,
            Arg::Int(v) => *v != 0,
            Arg::Float(v) => *v != 0.0,
            Arg::String(v) => v.is_empty(),
            _ => false,
        }
    }
}

impl Arg {
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Arg::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Arg::String(ref v) => Some(v),
            _ => None,
        }
    }

    // pub fn as_string(&self) -> Option<String> {
    //     match *self {
    //         Arg::String(ref v) => Some(v.to_owned()),
    //         _ => None,
    //     }
    // }

    pub fn from_json(val: Json) -> Result<Arg> {
        match val {
            Json::Null => Ok(Arg::Null),
            Json::Bool(v) => Ok(Arg::Bool(v)),
            Json::Number(v) => {
                Ok(v.as_i64().map_or_else(|| Arg::Float(v.as_f64().unwrap()), |i| Arg::Int(i)))
            },
            Json::String(v) => Ok(Arg::String(v)),
            Json::Array(v) => Ok(Arg::Expr(Expr::from_vec(v)?)),
            Json::Object(v) => Ok(Arg::Expr(Expr::from_vec(v.values().cloned().collect())?)),
        }
    }

    pub fn from_context_var(args: &Vec<Arg>, context: &Map<String, Json>) -> Result<Arg> {
        Arg::from_json(context.get(args[0].as_str().ok_or(Error::ExprVarArgNotStringError)?)
            .ok_or(Error::ContextNoSuchVarError)?.clone())
    }
}
