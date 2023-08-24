use super::StrHelper;

impl StrHelper {
    /// split ```self''' at index and get ```(Option<&str>, Option<&str>)```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Crab is Rust!");
    ///
    /// let crab = helper.split_str_at(4).0.unwrap();
    /// assert_eq!(crab, "Crab");
    /// ```
    pub fn split_str_at(&self, mid: usize) -> (Option<&str>, Option<&str>) {
        let values = self
            .string
            .split_at(mid);

        (Some(values.0), Some(values.1))
    }

    /// split ```self''' at indexes and get ```&str```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Crab is Rust!");
    ///
    /// let is = helper.split_str_on_the_sides_at((5, 7)).unwrap();
    /// assert_eq!(is, "is");
    /// ```
    pub fn split_str_on_the_sides_at(&self, mid: (usize, usize)) -> Option<&str> {
        let v = self
            .split_str_at(mid.0).1?
            .split_at(mid.1 - mid.0).0;

        Some(v)
    }
}