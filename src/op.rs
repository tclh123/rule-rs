use std::ops::Add;
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
// 1. static hashmap -> built-in ops
// 2. custom ops register to whom? maybe just use a global static mut hashmap
// 3. add more ops
// 4. use macro to init register ops
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
);

/// just a placeholder, will not be called
pub fn var(args: Vec<Arg>) -> Arg {
    args[0].clone()
}

pub fn eq(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] == w[1]))
}

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
    let b: bool = args.get(0).unwrap().into();
    Arg::Bool(!b)
}

// TODO: add more OPs
//    ('add', '+'),
//    ('sub', '-'),
//    ('neg', None),
//    ('mul', '*'),
//    ('pow', '**'),
//    ('div', '/'),
//    ('floordiv', '//'),
//    ('truediv', None),
//    ('mod', '%'),
//    ('abs', None),
//
//    in
//
//    startswith
//    endswith
//    lower
//    upper
//    split
//    match
//    regex
//
//    num
//    string
//
//    ('contains', None),
//    ('onlycontains/allin', None),
//    uniq
//    bool/notempty
//    empty

pub fn add(args: Vec<Arg>) -> Arg {
    let mut it = args.into_iter();
    it.next().map(|first| it.fold(first, Add::add)).unwrap_or(Arg::Null)
}
