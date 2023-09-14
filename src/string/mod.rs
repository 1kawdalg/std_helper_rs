mod split;
mod split_at;

mod generate;
mod base;

mod remove;
mod insert;
mod move_to;

/// Modification of ```&str``` / ```String```.
///
/// # Example:
/// ```
/// #[macro_use] extern crate std_helper;
/// use std_helper::StrHelper;
///
/// let mut helper = StrHelper::new("Hello, Rust!");
///
/// let result = helper.remove("Rust").unwrap();
/// assert_eq!(result, "Hello, !");
///
/// helper.push("World");
/// assert_eq!(helper.as_str(), "Hello, !World");
///
/// helper.move_to_the_end((7, 8));
/// assert_eq!(helper.as_str(), "Hello, World!");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct StrHelper {
    string: String
}

impl StrHelper {
    /// init struct from ```&str```.
    ///
    /// # Example:
    /// ```
    /// use std_helper::StrHelper;
    ///
    /// let helper = StrHelper::new("Language Crab is Rust...");
    /// assert_eq!(helper.as_str(), "Language Crab is Rust...");
    /// ```
    pub fn new(value: &str) -> StrHelper {
        StrHelper::from_string(value.to_string())
    }

    /// init empty struct.
    ///
    /// # Example:
    /// ```
    /// use std_helper::*;
    ///
    /// let empty_helper = StrHelper::empty();
    /// assert_eq!(empty_helper.as_str(), "");
    /// assert_eq!(empty_helper, str!());
    /// ```
    pub fn empty() -> StrHelper {
        StrHelper::new("")
    }

    /// init struct from ```String```.
    ///
    /// # Example:
    /// ```
    /// use std_helper::StrHelper;
    ///
    /// let helper = StrHelper::from_string(String::new());
    /// assert_eq!(helper.as_str(), "");
    /// ```
    pub fn from_string(value: String) -> StrHelper {
        StrHelper {string: value}
    }

    /// init struct from ```Vec<char>```.
    ///
    /// # Example:
    /// ```
    /// use std_helper::StrHelper;
    ///
    /// let chars = vec!['h', 'p'];
    /// let helper = StrHelper::from_chars_vector(chars);
    ///
    /// assert_eq!(helper.as_str(), "hp");
    /// ```
    pub fn from_chars_vector(vector: Vec<char>) -> StrHelper {
        StrHelper::from_string(
            StrHelper::generate_string_from_char_vec(vector)
        )
    }

    /// init struct from ```&[char]```.
    ///
    /// # Example:
    /// ```
    /// use std_helper::StrHelper;
    ///
    /// let chars = ['h', 'p'].as_ref();
    /// let helper = StrHelper::from_chars(chars);
    ///
    /// assert_eq!(helper.as_str(), "hp");
    /// ```
    pub fn from_chars(values: &[char]) -> StrHelper {
        StrHelper::from_string(
            StrHelper::generate_string_from_chars(values)
        )
    }

    /// change string.
    ///
    /// # Example:
    /// ```
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = StrHelper::new("Rust");
    /// helper.update("Crab");
    ///
    /// assert_eq!(helper.as_str(), "Crab");
    /// ```
    pub fn update(&mut self, new_value: &str) {
        self.string = new_value.to_string();
    }

    /// reverse string.
    ///
    /// # Example:
    /// ```
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = StrHelper::new("1234567890");
    /// helper.reverse();
    ///
    /// assert_eq!(helper.as_str(), "0987654321");
    /// ```
    pub fn reverse(&mut self) {
        let mut chars = self.as_chars();
        chars.reverse();

        self.update(StrHelper::generate_string_from_char_vec(chars).as_str());
    }
}

/// ```
/// #[macro_use] extern crate std_helper;
/// use std_helper::StrHelper;
///
/// let helper = str!("Hi!", ' ', "How are y".to_string(), "ou", "?");
/// assert_eq!(helper.as_str(), "Hi! How are you?");
/// assert_eq!(helper, str!("Hi! How are you?"));
/// ```
#[macro_export]
macro_rules! str {
    () => {StrHelper::empty()};
    ($elem:expr) => {StrHelper::from_string($elem.to_string())};

    ($($elem:expr),*) => {
        {
            let mut helper = StrHelper::empty();
            $(helper.push($elem.to_string().as_str());)*

            helper
        }
    };
}