#[macro_use]
extern crate rule;

use serde::Serialize;

use rule::{Rule, Result};

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
fn rule_new_macro() {
    let _rule = rule!["=", "a", 1].unwrap();
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
    assert!(_rule.matches(&context).unwrap());
}

#[test]
fn rule_match_handle_error() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::new(json!(["=", "a", 1]))?.matches(&context)?);
    assert!(rule!["=", "a", 1]?.matches(&context)?);
    Ok(())
}

#[test]
fn rule_recur_match() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::new(json!(["=", ["var", "a"], 1]))?.matches(&context)?);
    Ok(())
}

#[test]
fn rule_match_from_str() -> Result<()>{
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::from_str(r#"["=", ["var", "a"], 1]"#)?.matches(&context)?);
    Ok(())
}

#[test]
fn rule_match_from_value() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::from_value(["=", "world", "hello"])?.matches(&context)?);
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
    assert!(Rule::new(json!(["=", "a", 1]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", "world", "hello"]))?.matches(&context)?);
    Ok(())
}

#[test]
fn rule_match_logic_op() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::new(json!([">", "world", "hell"]))?.matches(&context)?);
    assert!(Rule::new(json!([">", "a", 0]))?.matches(&context)?);
    assert!(Rule::new(json!(["!", ["<", 1000, 234]]))?.matches(&context)?);
    assert!(Rule::new(json!([">=", 1000, 234]))?.matches(&context)?);
    assert!(Rule::new(json!(["!=", 1000, 234]))?.matches(&context)?);
    assert!(Rule::new(json!(["all", 1000, 234, true]))?.matches(&context)?);
    assert!(Rule::new(json!(["any", 1000, 0]))?.matches(&context)?);
    assert_eq!(Rule::new(json!(["any", false, 0]))?.matches(&context)?, false);
    Ok(())
}

#[test]
fn rule_match_arithmetic_op() -> Result<()> {
    let context = json!({"a": 1, "world": "hello"});
    assert!(Rule::new(json!(["=", ["+", "world", " world"], "hello world"]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["+", "a", 100], 101]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["-", "a", 100], -99]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["-", "10", 1], 9]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["neg", "10"], -10]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["*", "a", 10, -2], -20]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["/", 100, ["var", "a"], 5], 20]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["%", 100, 3], 1]))?.matches(&context)?);
    assert!(Rule::new(json!(["=", ["abs", -123], 123]))?.matches(&json!({}))?);
    Ok(())
}

#[test]
fn rule_match_collection_op() -> Result<()> {
    assert!(rule!["in", 1, 1, 2, 3]?.matches(&json!({}))?);
    assert!(rule!["startswith", "hello", "he"]?.matches(&json!({}))?);
    assert!(rule!["startswith", "arr", "foo", "bar"]?.matches(&json!({"arr": ["foo", "bar", "baz"]}))?);
    assert!(rule!["endswith", "arr", "bar", "baz"]?.matches(&json!({"arr": ["foo", "bar", "baz"]}))?);
    assert!(rule!["startswith", ["split", "apple,pear", ","], "apple"]?.matches(&json!({}))?);
    assert!(rule!["=", ["join", " ", "hello", "world"], "hello world"]?.matches(&json!({}))?);
    Ok(())
}

#[test]
fn rule_match_string_op() -> Result<()> {
    assert!(rule!["=", ["lower", "Hi"], "hi"]?.matches(&json!({}))?);
    assert!(rule!["=", ["upper", "Hi"], "HI"]?.matches(&json!({}))?);
    assert!(rule!["match", "hello", "he*"]?.matches(&json!({}))?);
    assert!(rule!["regex", "hello", "^he[l-o]{3}$"]?.matches(&json!({}))?);
    Ok(())
}

#[test]
fn rule_match_casting_op() -> Result<()> {
    assert!(rule!["=", ["num", "100"], 100]?.matches(&json!({}))?);
    assert!(rule!["=", ["num", "1.23"], 1.23]?.matches(&json!({}))?);
    assert_eq!(rule!["=", "100", 100]?.matches(&json!({}))?, false);
    assert!(rule!["=", ["string", 100], "100"]?.matches(&json!({}))?);
    Ok(())
}

