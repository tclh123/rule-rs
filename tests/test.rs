extern crate rule;

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
    // let _rule = Rule::new(json!(["=", "a", 1]));
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
