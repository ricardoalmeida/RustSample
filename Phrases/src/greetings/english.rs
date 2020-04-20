//! This module contains English phrases
//! 
//! # Examples
//! ```
//! let username = "John";
//! println!("{}, {}!",
//!     phrases::greetings::english::hello(),
//!     username);
//! ```

/// Applies comment to this function
pub fn hello() -> String { "hello".to_string() }
pub fn goodbye() -> String { "goodbye".to_string() }