extern crate serde_json;
pub use serde_json::json;

extern crate lazy_static;

#[doc(inline)]
pub use self::rule::Rule;
#[doc(inline)]
pub use self::error::{Error, Result};

pub mod rule;
pub mod op;
pub mod error;
