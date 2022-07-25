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
