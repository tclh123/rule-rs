# Rule

A rule engine written in rust.
There's also a [python fork](https://github.com/tclh123/rule).

The rule is a json string or rust object of a list expression.
The expression is like `[op, arg0, arg1, ..., argn]`, the `op` is the operator,
and `arg0..n` is the arguments for the operator. Any argument can be another expression.

For writing convenience, the first argument will be tried to resolve as the context parameter.
Or, you can just use the special `var` operator to indicate the context parameter.

## Usage

```rust
use rule::{Rule, Result, json};

fn main() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule(json!(["=", "a", 1])).match(context)?);
    assert!(Rule(json!(["=", ["var", "a"], 1])).match(context)?);
    assert!(Rule(r#"["=", ["var", "a"], 1]"#).match(context)?);

    Ok(())
}
```

## License

http://tclh123.mit-license.org/
