use std::ops::Add;

// TODO: var?
// how to get func from func name?
// 变长参数列表? Vec?
// Fn(Vec<Arg>) -> Arg
#[derive(Clone, Debug)]
pub struct Op {
}

fn eq<T: PartialEq<U>, U>(a: T, b: U) -> bool {
    a == b
}

fn ne<T: PartialEq<U>, U>(a: T, b: U) -> bool {
    a != b
}

fn add<T: Add<U>, U>(a: T, b: U) -> T::Output {
    a + b
}
