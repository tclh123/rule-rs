#[macro_use]
extern crate serde_json;

#[doc(inline)]
pub use self::rule::Rule;
#[doc(inline)]
pub use self::error::{Error, Result};

pub mod rule;
pub mod op;
pub mod error;
