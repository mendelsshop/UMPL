#![deny(rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::must_use_candidate)]
#![deny(clippy::use_self)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::case_sensitive_file_extension_comparisons)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::float_cmp)]
#![allow(clippy::similar_names)]
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
