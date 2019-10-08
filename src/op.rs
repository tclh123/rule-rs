use std::ops::{Add, Sub, Mul, Div, Rem};
use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::arg::Arg;

pub type Func = fn(Vec<Arg>) -> Arg;

/// The Operator type, mainly contains a function pointer.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Op {
    pub name: String,
    pub func: Func,
}

// TODO:
// 1. static hashmap -> built-in ops [done]
// 2. custom ops register to whom? maybe just use a global static mut hashmap
// 3. add more ops [ing]
// 4. use macro to init register ops [done]
// 5. func return Result, error handling? or not.

impl Op {
    /// Constructs a new Operator.
    pub fn new(name: &str, func: Func) -> Op {
        Op {
            name: name.to_owned(),
            func: func,
        }
    }

    /// Get an Operator by name, returns an Option, `None` if not exists.
    pub fn get(name: &str) -> Option<&Op> {
        OPS.get(name)
    }
}

/// Register builtin OPs.
///
/// # Examples
///
/// ```
/// register_builtin!(
///     "var" => var,
///     "=" => eq,
///     "<" => lt,
/// )
/// ```
macro_rules! register_builtin {
    ( $($alias:tt => $func:tt),* $(,)? ) => {
        lazy_static! {
            /// All built-in OPs registered to OPS HashMap.
            static ref OPS: HashMap<&'static str, Op> = {
                let mut map = HashMap::new();
                $(
                map.insert($alias, Op::new($alias, $func as Func));
                map.insert(stringify!($func), Op::new(stringify!($func), $func as Func));
                )*
                map
            };
        }
    };
}

register_builtin!(
    "var" => var,

    // logic operator
    "=" => eq,
    "<" => lt,
    "<=" => le,
    "!=" => ne,
    ">=" => ge,
    ">=" => ge,
    ">" => gt,
    "&" => and,
    "&&" => and,
    "all" => and,
    "|" => or,
    "||" => or,
    "any" => or,
    "!" => not,

    // arithmetic operator
    "+" => add,
    "sum" => add,
    "-" => sub,
    "minus" => sub,
    "neg" => neg,
    "*" => mul,
    "/" => div,
    "%" => rem,
    "mod" => rem,
    "abs" => abs,

    // collection operator
    "in" => r#in,
    "startswith" => startswith,
    "endswith" => endswith,
    "split" => split,
    "join" => join,

    // string operator
    "lower" => lower,
    "upper" => upper,
    "match" => r#match,
    "regex" => regex,

    // casting operator
    "num" => num,
    "string" => string,
);

/// just a placeholder, will not be called
pub fn var(args: Vec<Arg>) -> Arg {
    args[0].clone()
}

pub fn eq(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] == w[1]))
}

/// `lt` is equivalent to the `<` sign, args[0] and args[1] must be the same type.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["lt", 1, 2].unwrap().matches(&json!({})).unwrap());
/// assert!(rule!["lt", "10", "2"].unwrap().matches(&json!({})).unwrap());
/// assert!(rule!["lt", 1.1, 1.23].unwrap().matches(&json!({})).unwrap());
/// assert_eq!(rule!["lt", 1.23, 1.1].unwrap().matches(&json!({})).unwrap(), false);
/// assert_eq!(rule!["lt", 2, 1].unwrap().matches(&json!({})).unwrap(), false);
/// ```
pub fn lt(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] < w[1]))
}

pub fn le(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] <= w[1]))
}

pub fn ne(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] != w[1]))
}

pub fn ge(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] >= w[1]))
}

pub fn gt(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] > w[1]))
}

pub fn and(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.iter().all(|v| v.into()))
}

pub fn or(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.iter().any(|v| v.into()))
}

pub fn not(args: Vec<Arg>) -> Arg {
    let b: bool = args.get(0).unwrap_or(&Arg::Null).into();
    Arg::Bool(!b)
}

pub fn add(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    it.next().map(|first| it.fold(first, Add::add)).unwrap_or(Arg::Null)
}

pub fn sub(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    it.next().map(|first| it.fold(first, Sub::sub)).unwrap_or(Arg::Null)
}

pub fn neg(args: Vec<Arg>) -> Arg {
    -args.get(0).unwrap_or(&Arg::Null)
}

pub fn mul(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    it.next().map(|first| it.fold(first, Mul::mul)).unwrap_or(Arg::Null)
}

pub fn div(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    it.next().map(|first| it.fold(first, Div::div)).unwrap_or(Arg::Null)
}

/// The remainder operator %.
/// Aliases: %, rem, mod
pub fn rem(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    it.next().map(|first| it.fold(first, Rem::rem)).unwrap_or(Arg::Null)
}

/// Computes the absolute value of arg[0].
pub fn abs(args: Vec<Arg>) -> Arg {
    let int: i64 = args.get(0).unwrap_or(&Arg::Null).into();
    Arg::Int(int.abs())
}

/// Return true if args[0] in args[1..].
/// e.g. rule json string: ["in", 1, 1, 2, 3]
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["in", 1, 1, 2, 3].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn r#in(args: Vec<Arg>) -> Arg {
    Arg::Bool(args[1..].contains(&args[0]))
}

/// Return true if args[0] starts with args[1]
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["startswith", "hello", "he"].unwrap().matches(&json!({})).unwrap());
/// assert!(rule!["startswith", "arr", "foo", "bar"].unwrap().matches(&json!({"arr": ["foo", "bar", "baz"]})).unwrap());
/// ```
pub fn startswith(args: Vec<Arg>) -> Arg {
    let ret = match &args[0] {
        Arg::String(s) => Arg::Bool((&s).starts_with(&args[1].to_string())),
        Arg::Array(a) => Arg::Bool(a.starts_with(&args[1..])),
        _ => Arg::Bool(false),
    };
    ret
}

/// Return true if args[0] ends with args[1]
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["endswith", "hello", "lo"].unwrap().matches(&json!({})).unwrap());
/// assert!(rule!["endswith", "arr", "bar", "baz"].unwrap().matches(&json!({"arr": ["foo", "bar", "baz"]})).unwrap());
/// ```
pub fn endswith(args: Vec<Arg>) -> Arg {
    let ret = match &args[0] {
        Arg::String(s) => Arg::Bool((&s).ends_with(&args[1].to_string())),
        Arg::Array(a) => Arg::Bool(a.ends_with(&args[1..])),
        _ => Arg::Bool(false),
    };
    ret
}

/// Convert upper case letters to lower case.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["=", ["lower", "Hi"], "hi"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn lower(args: Vec<Arg>) -> Arg {
    Arg::String(String::from(&args[0]).to_lowercase())
}

/// Convert lower case letters to upper case.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["=", ["upper", "Hi"], "HI"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn upper(args: Vec<Arg>) -> Arg {
    Arg::String(String::from(&args[0]).to_uppercase())
}

/// Split strings using a seperator.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["startswith", ["split", "apple,pear", ","], "apple"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn split(args: Vec<Arg>) -> Arg {
    let s = &String::from(&args[0]);
    let sep = &String::from(&args[1]);
    Arg::Array(s.split(sep).map(|x| Arg::String(x.to_owned())).collect())
}

/// Concatenate strings with a seperator.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["=", ["join", " ", "hello", "world"], "hello world"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn join(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    let sep = &String::from(&it.nth(0).unwrap_or(Arg::String("".to_owned())));
    Arg::String(it.map(|x| String::from(&x)).collect::<Vec<String>>().join(sep))
}

/// Match string using an Unix shell style pattern.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["match", "hello", "he*"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn r#match(args: Vec<Arg>) -> Arg {
    match glob::Pattern::new(&String::from(&args[1])) {
        Ok(patt) => {
            Arg::Bool(patt.matches(&String::from(&args[0])))
        },
        Err(_) => Arg::Bool(false),
    }
}

/// Match strings using regular expressions.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["regex", "hello", "^he[l-o]*$"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn regex(args: Vec<Arg>) -> Arg {
    match regex::Regex::new(&String::from(&args[1])) {
        Ok(re) => {
            Arg::Bool(re.is_match(&String::from(&args[0])))
        },
        Err(_) => Arg::Bool(false),
    }
}

/// Convert a string into a number.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["=", ["num", "100"], 100].unwrap().matches(&json!({})).unwrap());
/// assert!(rule!["=", ["num", "1.23"], 1.23].unwrap().matches(&json!({})).unwrap());
/// assert_eq!(rule!["=", "100", 100].unwrap().matches(&json!({})).unwrap(), false);
/// ```
pub fn num(args: Vec<Arg>) -> Arg {
    let a = args[0].clone();
    match a {
        Arg::Int(_) => a,
        Arg::Float(_) => a,
        Arg::String(v) => {
            match v.parse::<i64>() {
                Ok(i) => Arg::Int(i),
                Err(_) => match v.parse::<f64>() {
                    Ok(f) => Arg::Float(f),
                    Err(_) => Arg::Int(0i64),
                }
            }
        },
        _ => Arg::Int(Into::<i64>::into(a)),
    }
}

/// Convert a number into a string.
///
/// ```
/// use ::rule::{rule, json};
/// assert!(rule!["=", ["string", 100], "100"].unwrap().matches(&json!({})).unwrap());
/// ```
pub fn string(args: Vec<Arg>) -> Arg {
    Arg::String(String::from(&args[0]))
}

// TODO: add more OPs
//
//    ('contains', None),
//    ('onlycontains/allin', None),
//    uniq
//    bool/notempty
//    empty
