extern crate rule;

use serde::Serialize;

use rule::{Rule, Result, json};

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }

#[test]
fn rule_new() {
    let _rule = Rule::new(json!(["=", "a", 1])).unwrap();
}

#[test]
fn rule_from_value() {
    let _rule = Rule::from_value(["=", "world", "hello"]).unwrap();
}

#[test]
fn rule_from_str() {
    let _rule = Rule::from_str(r#"["=", ["var", "a"], 1]"#).unwrap();
}

#[test]
fn rule_match() {
    let _rule = Rule::new(json!(["=", "a", 1])).unwrap();

    let context = json!({"a": 1, "world": "hello"});
    assert!(_rule.matches(context).unwrap());
}

#[test]
fn rule_match_handle_error() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::new(json!(["=", "a", 1]))?.matches(context)?);
    Ok(())
}

#[test]
fn rule_recur_match() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::new(json!(["=", ["var", "a"], 1]))?.matches(context)?);
    Ok(())
}

#[test]
fn rule_match_from_str() -> Result<()>{
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::from_str(r#"["=", ["var", "a"], 1]"#)?.matches(context)?);
    Ok(())
}

#[test]
fn rule_match_from_value() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::from_value(["=", "world", "hello"])?.matches(context)?);
    Ok(())
}

#[derive(Serialize, Clone, Debug)]
struct Context<'a> {
    a: i32,
    world: &'a str,
}

#[test]
fn rule_match_context_struct() -> Result<()>{
    let context = Context { a: 1, world: "hello" };
    assert!(Rule::new(json!(["=", "a", 1]))?.matches(context.clone())?);
    assert!(Rule::new(json!(["=", "world", "hello"]))?.matches(context.clone())?);
    Ok(())
}
