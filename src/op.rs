use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::arg::Arg;

type Func = fn(Vec<Arg>) -> Arg;

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
// 5. func return Result, error handling

impl Op {
    pub fn new(name: &str, func: Func) -> Op {
        Op {
            name: name.to_owned(),
            func: func,
        }
    }

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
    }
}

register_builtin!(
    "var" => var,
    "=" => eq,
    "<" => lt,
    "<=" => le,
    "!=" => ne,
    ">=" => ge,
    ">=" => ge,
    ">" => gt,
    "all" => and,
    "any" => or,
    "!" => not,
);

/// just a placeholder, will not be called
fn var(args: Vec<Arg>) -> Arg {
    args[0].clone()
}

fn eq(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] == w[1]))
}

fn lt(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] < w[1]))
}

fn le(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] <= w[1]))
}

fn ne(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] != w[1]))
}

fn ge(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] >= w[1]))
}

fn gt(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] > w[1]))
}

fn and(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.iter().all(|v| v.into()))
}

fn or(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.iter().any(|v| v.into()))
}

fn not(args: Vec<Arg>) -> Arg {
    let b: bool = args.get(0).unwrap().into();
    Arg::Bool(!b)
}

//    ('add', '+'),
//    ('sub', '-'),
//    ('neg', None),
//    ('mul', '*'),
//    ('pow', '**'),
//    ('div', '/'),
//    ('floordiv', '//'),
//    ('truediv', None),
//    ('mod', '%'),

// fn add(args: Vec<Arg>) -> Arg {
//     let b: bool = args.get(0).unwrap().into();
//     Arg::Bool(!b)
// }
