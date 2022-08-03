#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(clippy::use_self, rust_2018_idioms)]
#![allow(
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::match_wildcard_for_single_variants,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cognitive_complexity,
    clippy::float_cmp,
    clippy::similar_names
)]
pub mod cli;
pub mod error;
pub mod eval;
pub mod keywords;
pub mod lexer;
pub mod parser;
pub mod token;

use lazy_static::lazy_static;

use crate::keywords::Keyword;
lazy_static! {
    pub static ref KEYWORDS: Keyword = Keyword::new();
}
