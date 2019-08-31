
#[macro_export(local_inner_macros)]
macro_rules! rule {
    ( [$($e:tt)*] ) => {
        rule![$($e)*]
    };
    ( $($e:tt)* ) => {
        $crate::Rule::new(json!([$($e)*]))
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_macro() -> Result<()> {
        assert_eq!(rule!["=", "a", 1]?, Rule::new(json!(["=", "a", 1]))?);
        assert_eq!(rule!["=", ["var", "a"], 1]?, Rule::new(json!(["=", ["var", "a"], 1]))?);
        assert_eq!(rule!["=", ["<", "a", 2], true]?, Rule::new(json!(["=", ["<", "a", 2], true]))?);
        Ok(())
    }
}
