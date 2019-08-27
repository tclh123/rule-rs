use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::rule::Arg;

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

lazy_static! {
    static ref OPS: HashMap<&'static str, Op> = {
        let mut map = HashMap::new();
        map.insert("var", Op::new("var", var as Func));

        map.insert("=", Op::new("=", eq as Func));

        map.insert("<", Op::new("<", lt as Func));
        map.insert("lt", Op::new("lt", lt as Func));

        map.insert("<=", Op::new("<=", le as Func));
        map.insert("le", Op::new("le", le as Func));

        map.insert("!=", Op::new("!=", ne as Func));
        map.insert("ne", Op::new("ne", ne as Func));

        map.insert(">=", Op::new(">=", ge as Func));
        map.insert("ge", Op::new("ge", ge as Func));

        map.insert(">", Op::new(">", gt as Func));
        map.insert("gt", Op::new("gt", gt as Func));

        map.insert("and", Op::new("and", and as Func));
        map.insert("all", Op::new("all", and as Func));

        map.insert("or", Op::new("or", or as Func));
        map.insert("any", Op::new("any", or as Func));

        map.insert("not", Op::new("not", not as Func));
        map.insert("!", Op::new("!", not as Func));

        map
    };
}

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

// fn eq<T: PartialEq<U>, U>(a: T, b: U) -> bool {
//     a == b
// }
//
// fn ne<T: PartialEq<U>, U>(a: T, b: U) -> bool {
//     a != b
// }
// 
// use std::ops::Add;
// fn add<T: Add<U>, U>(a: T, b: U) -> T::Output {
//     a + b
// }

// just a placeholder, will not be called
fn var(args: Vec<Arg>) -> Arg {
    args[0].clone()
}

fn eq(args: Vec<Arg>) -> Arg {
    Arg::Bool(args.windows(2).all(|w| w[0] == w[1]))
    // Arg::Bool(true)
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
