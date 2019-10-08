use std::fmt;
use std::convert::Into;
use std::ops::{Add, Sub, Neg, Mul, Div, Rem};

use serde_json::value::{Value as Json};
use serde_json::Map;

use crate::rule::{Expr};
use crate::error::{Error, Result};

/// The argument type. Each argument can be a json primitive type or a `Expr`.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Arg {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Arg>),
    Expr(Expr),
}

impl Add for Arg {
    type Output = Arg;

    fn add(self, other: Arg) -> Arg {
        match self {
            Arg::Null => Arg::Int(other.into()),
            Arg::Bool(v) => Arg::Int(v as i64 + Into::<i64>::into(other)),
            Arg::Int(v) => Arg::Int(v + Into::<i64>::into(other)),
            Arg::Float(v) => Arg::Float(v + Into::<f64>::into(other)),
            Arg::String(v) => Arg::String(v + &Into::<String>::into(other)),
            _ => Arg::Null,
        }
    }
}

impl<'a> Add<&'a Arg> for Arg {
    type Output = Arg;

    fn add(self, other: &'a Arg) -> Arg {
        match self {
            Arg::Null => Arg::Int(other.clone().into()),
            Arg::Bool(v) => Arg::Int(v as i64 + Into::<i64>::into(other.clone())),
            Arg::Int(v) => Arg::Int(v + Into::<i64>::into(other.clone())),
            Arg::Float(v) => Arg::Float(v + Into::<f64>::into(other.clone())),
            Arg::String(v) => Arg::String(v + &Into::<String>::into(other.clone())),
            _ => Arg::Null,
        }
    }
}

impl Sub for Arg {
    type Output = Arg;

    fn sub(self, other: Arg) -> Arg {
        match self {
            Arg::Null => Arg::Int(0i64 - Into::<i64>::into(other)),
            Arg::Bool(v) => Arg::Int(v as i64 - Into::<i64>::into(other)),
            Arg::Int(v) => Arg::Int(v - Into::<i64>::into(other)),
            Arg::Float(v) => Arg::Float(v - Into::<f64>::into(other)),
            Arg::String(ref _v) => Arg::Int(Into::<i64>::into(self) - Into::<i64>::into(other)),
            _ => Arg::Null,
        }
    }
}

impl Neg for Arg {
    type Output = Arg;

    fn neg(self) -> Arg {
        match self {
            Arg::Null => Arg::Int(0),
            Arg::Bool(v) => Arg::Int(-(v as i64)),
            Arg::Int(v) => Arg::Int(-v),
            Arg::Float(v) => Arg::Float(-v),
            Arg::String(ref _v) => Arg::Int(-Into::<i64>::into(self)),
            _ => Arg::Null,
        }
    }
}

impl<'a> Neg for &'a Arg {
    type Output = Arg;

    fn neg(self) -> Arg {
        match self {
            Arg::Null => Arg::Int(0),
            Arg::Bool(v) => Arg::Int(-(*v as i64)),
            Arg::Int(v) => Arg::Int(-v),
            Arg::Float(v) => Arg::Float(-v),
            Arg::String(ref _v) => Arg::Int(-Into::<i64>::into(self.clone())),
            _ => Arg::Null,
        }
    }
}

impl Mul for Arg {
    type Output = Arg;

    fn mul(self, rhs: Arg) -> Arg {
        match self {
            Arg::Null => Arg::Int(0),
            Arg::Bool(v) => Arg::Int((v as i64) * Into::<i64>::into(rhs)),
            Arg::Int(v) => Arg::Int(v * Into::<i64>::into(rhs)),
            Arg::Float(v) => Arg::Float(v * Into::<f64>::into(rhs)),
            Arg::String(ref _v) => Arg::Int(Into::<i64>::into(self) * Into::<i64>::into(rhs)),
            _ => Arg::Null,
        }
    }
}

impl Div for Arg {
    type Output = Arg;

    fn div(self, rhs: Arg) -> Arg {
        match self {
            Arg::Null => Arg::Int(0),
            Arg::Bool(v) => Arg::Int((v as i64) / Into::<i64>::into(rhs)),
            Arg::Int(v) => Arg::Int(v / Into::<i64>::into(rhs)),
            Arg::Float(v) => Arg::Float(v / Into::<f64>::into(rhs)),
            Arg::String(ref _v) => Arg::Int(Into::<i64>::into(self) / Into::<i64>::into(rhs)),
            _ => Arg::Null,
        }
    }
}

impl Rem for Arg {
    type Output = Arg;

    fn rem(self, rhs: Arg) -> Arg {
        match self {
            Arg::Null => Arg::Int(0),
            Arg::Bool(v) => Arg::Int((v as i64) % Into::<i64>::into(rhs)),
            Arg::Int(v) => Arg::Int(v % Into::<i64>::into(rhs)),
            Arg::Float(v) => Arg::Float(v % Into::<f64>::into(rhs)),
            Arg::String(ref _v) => Arg::Int(Into::<i64>::into(self) % Into::<i64>::into(rhs)),
            _ => Arg::Null,
        }
    }
}

impl Into<String> for Arg {
    fn into(self) -> String {
        match self {
            Arg::Null => "".to_owned(),
            Arg::Bool(v) => v.to_string(),
            Arg::Int(v) => v.to_string(),
            Arg::Float(v) => v.to_string(),
            Arg::String(v) => v,
            _ => "".to_owned(),
        }
    }
}

// should always prefer implementing From over Into
impl<'a> From<&'a Arg> for String {
    fn from(arg: &'a Arg) -> Self {
        match arg {
            Arg::Null => "".to_owned(),
            Arg::Bool(v) => v.to_string(),
            Arg::Int(v) => v.to_string(),
            Arg::Float(v) => v.to_string(),
            Arg::String(v) => v.to_string(),
            _ => "".to_owned(),
        }
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Into<i64> for Arg {
    fn into(self) -> i64 {
        match self {
            Arg::Null => 0,
            Arg::Bool(v) => v as i64,
            Arg::Int(v) => v,
            Arg::Float(v) => v as i64,
            Arg::String(v) => v.parse().unwrap_or(0i64),
            _ => 0,
        }
    }
}

impl<'a> Into<i64> for &'a Arg {
    fn into(self) -> i64 {
        match self {
            Arg::Null => 0,
            Arg::Bool(v) => *v as i64,
            Arg::Int(v) => *v,
            Arg::Float(v) => *v as i64,
            Arg::String(v) => v.parse().unwrap_or(0i64),
            _ => 0,
        }
    }
}

impl Into<f64> for Arg {
    fn into(self) -> f64 {
        match self {
            Arg::Null => 0.0,
            Arg::Bool(v) => (v as i64) as f64,
            Arg::Int(v) => v as f64,
            Arg::Float(v) => v,
            Arg::String(v) => v.parse().unwrap_or(0.0),
            _ => 0.0,
        }
    }
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
    /// If the `Arg` is a bool, returns the associated bool. Returns None otherwise.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Arg::Bool(v) => Some(v),
            _ => None,
        }
    }

    /// If the `Arg` is a String, returns the associated String. Returns None otherwise.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Arg::String(ref v) => Some(v),
            _ => None,
        }
    }

    // Json::Array => Arg::Expr
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

    // Json::Array => Arg::Array
    pub fn from_json_context_var(val: Json) -> Result<Arg> {
        // how to reuse partial match content with from_json?
        match val {
            Json::Null => Ok(Arg::Null),
            Json::Bool(v) => Ok(Arg::Bool(v)),
            Json::Number(v) => {
                Ok(v.as_i64().map_or_else(|| Arg::Float(v.as_f64().unwrap()), |i| Arg::Int(i)))
            },
            Json::String(v) => Ok(Arg::String(v)),
            Json::Array(v) => Ok(Arg::Array(v.into_iter().map(|v| Self::from_json_context_var(v)).collect::<Result<Vec<_>>>()?)),
            Json::Object(v) => Self::from_json_context_var(v.values().cloned().collect()),
        }
    }

    pub fn from_context_var(args: &Vec<Arg>, context: &Map<String, Json>) -> Result<Arg> {
        Arg::from_json_context_var(context.get(args[0].as_str().ok_or(Error::ExprVarArgNotStringError)?)
            .ok_or(Error::ContextNoSuchVarError)?.clone())
    }
}
