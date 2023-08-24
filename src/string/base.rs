use super::StrHelper;

impl StrHelper {
    /// get '''String''' version of helper.
    pub fn to_string(&self) -> String {
        (*self.string).to_string()
    }

    /// get '''&str''' version of helper.
    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }

    /// get '''Vec<char>''' version of helper.
    pub fn as_chars(&self) -> Vec<char> {
        self.string.chars().collect::<Vec<char>>()
    }

    /// get element location status in string.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Hi!");
    /// assert_eq!(helper.contains("Hi"), true);
    /// assert_eq!(helper.contains("crab"), false);
    /// ```
    pub fn contains(&self, pat: &str) -> bool {
        self.string.contains(pat)
    }

    /// get '''Vec<char>''' version of other '''&str'''.
    pub fn other_as_chars(value: &str) -> Vec<char> {
        value.to_string().chars().collect::<Vec<char>>()
    }
}