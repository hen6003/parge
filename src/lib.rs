//! Parge is a simple and easy to use argument parser
//!
//! # Features
//! * Long and short varients
//! * Switch options
//! * Value options
//!
//! ## Switch options
//! Options that are a simple on/off switch
//!
//! Styles supported:
//! * `example --option`
//! * `example -o`
//!
//! ## Value options
//! Options that contain some data
//!
//! Styles supported:
//! * `example --hello=world`
//! * `example --hello world`
//! * `example -hworld`
//! * `example -h world`
//!
//! # Usage
//!
//! ```rust
//! use parge::parser::Parser;
//!
//! fn main() {
//!     let mut switch = false;
//!
//!     let args = Parser::new()
//!         .add_switch_opt(
//!             Some("option"),
//!             Some('o'),
//!             &mut switch)
//!         .parse();
//!
//!     println!("{}", switch);
//! }
//! ```

pub mod errors;
pub mod parser;
mod tests;
