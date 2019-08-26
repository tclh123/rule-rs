use std::collections::HashMap;

use lazy_static::lazy_static;

use crate::rule::Arg;

type Func = fn(Vec<Arg>) -> Arg;

// TODO: var?
// how to get func from func name?
// 变长参数列表? Vec?
// Fn(Vec<Arg>) -> Arg
#[derive(Clone, Debug)]
pub struct Op {
    pub name: String,
    pub aliases: Option<Vec<String>>,
    pub func: Func,
}

// TODO:
// 1. static hashmap -> built-in ops
// 2. custom ops register to whom? maybe just use a global static mut hashmap

lazy_static! {
    static ref OPS: HashMap<&'static str, Op> = {
        let mut map = HashMap::new();
        map.insert("var", Op::new("var", var as Func));
        map.insert("=", Op::new("=", eq as Func));
        map
    };
}

impl Op {
    pub fn new(name: &str, func: Func) -> Op {
        Op {
            name: name.to_owned(),
            func: func,
            aliases: None,
        }
    }

    pub fn alias(self, aliases: Vec<String>) -> Op {
        Op {
            aliases: Some(aliases),
            ..self
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

// TODO:
fn eq(args: Vec<Arg>) -> Arg {
    Arg::Bool(true)
}

fn var(args: Vec<Arg>) -> Arg {
    Arg::Bool(true)
}
