//! These modules represent deps you might have in your project.
//!
//! Often libs will provide their own Error types.
//! These stand-ins will do the same so as to serve as an illustration of how to
//! juggle varied Error types within a single function body by *funneling them
//! into a single type* via the [`?`] operator.
//!
//! [`?`]: https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html

pub mod database;
pub mod rest_api;
