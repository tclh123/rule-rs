# Rule

[![circle-ci](https://img.shields.io/circleci/build/gh/tclh123/rule-rs?token=1a76063919507b8ff950b4e58ae3a12eee3a784a)](https://circleci.com/gh/tclh123/rule-rs)
[![crates.io](https://img.shields.io/crates/v/rule)](https://crates.io/crates/rule)
[![license](https://img.shields.io/crates/l/rule)](http://tclh123.mit-license.org/)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest-brightgreen)](https://docs.rs/rule)

A rule engine written in rust.
There's also a [python fork](https://github.com/tclh123/rule).

The rule is a json string or rust object of a list expression.
The expression is like `[op, arg0, arg1, ..., argn]`, the `op` is the operator,
and `arg0..n` is the arguments for the operator. Any argument can be another expression.

For writing convenience, the first argument will be tried to resolve as the context parameter.
Or, you can just use the special `var` operator to indicate the context parameter.

## Usage

```rust
#[macro_use]
extern crate rule;

use rule::{Rule, Result};

fn main() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});

    assert!(Rule::new(json!(["=", "a", 1]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["var", "a"], 1]))?.matches(&context)?);
    assert!(Rule::from_str(r#"["=", ["var", "a"], 1]"#)?.matches(&context)?);
    assert!(Rule::from_value(["=", "world", "hello"])?.matches(&context)?);

    assert!(rule!["=", "a", 1]?.matches(&context)?);

    Ok(())
}
```

## ToDos

- [ ] add more built-in `Op`s
- [ ] support register custom `Op`s
- [x] support `rule!` macro

## License

http://tclh123.mit-license.org/
