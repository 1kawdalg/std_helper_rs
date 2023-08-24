use super::StrHelper;

impl StrHelper {
    /// split ```self''' at index and get ```(Option<String>, Option<String>)```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Crab is Rust!");
    ///
    /// let crab = helper.split_string_at(4).0.unwrap();
    /// assert_eq!(crab, "Crab");
    /// ```
    pub fn split_string_at(&self, mid: usize) -> (Option<String>, Option<String>) {
        let values = self
            .string
            .split_at(mid);

        (Some(values.0.to_string()), Some(values.1.to_string()))
    }

    /// split ```self''' at indexes and get ```String```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Crab is Rust!");
    ///
    /// let is = helper.split_string_on_the_sides_at((5, 7)).unwrap();
    /// assert_eq!(is, "is");
    /// ```
    pub fn split_string_on_the_sides_at(&self, mid: (usize, usize)) -> Option<String> {
        let v = self
            .split_str_on_the_sides_at(mid)?.to_string();

        Some(v)
    }
}