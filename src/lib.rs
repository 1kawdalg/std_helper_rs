//! Modification of standard Rust library.
//!
//! # Example:
//! ```
//! #[macro_use] extern crate std_helper;
//! use std_helper::StrHelper;
//!
//! fn str_helper_example() -> std::io::Result<()> {
//!     let mut helper = str!("Hello, Rust!");
//!
//!     let result = helper.remove("Rust").unwrap();
//!     assert_eq!(result, "Hello, !");
//!
//!     helper.push("World");
//!     assert_eq!(helper.as_str(), "Hello, !World");
//!
//!     helper.move_to_the_end((7, 8));
//!     assert_eq!(helper.as_str(), "Hello, World!");
//!
//!     Ok(())
//! }
//!
//! str_helper_example().unwrap();
//! ```
mod string;

pub use string::StrHelper;