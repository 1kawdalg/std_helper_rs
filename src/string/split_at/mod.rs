mod str;
mod string;

use super::StrHelper;

impl StrHelper {
    /// split ```self''' at index and get ```(Option<StrHelper>, Option<StrHelper>)```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Crab is Rust!");
    ///
    /// let crab = helper.split_at(4).0.unwrap();
    /// assert_eq!(crab.as_str(), "Crab");
    /// ```
    pub fn split_at(&self, mid: usize) -> (Option<StrHelper>, Option<StrHelper>) {
        let values = self
            .string
            .split_at(mid);

        (Some(StrHelper::new(values.0)), Some(StrHelper::new(values.1)))
    }

    /// split ```self''' at indexes and get ```StrHelper```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("Crab is Rust!");
    ///
    /// let is = helper.split_on_the_sides_at((5, 7)).unwrap();
    /// assert_eq!(is.as_str(), "is");
    /// ```
    pub fn split_on_the_sides_at(&self, mid: (usize, usize)) -> Option<StrHelper> {
        let v = self
            .split_str_at(mid.0).1?
            .split_at(mid.1 - mid.0).0;

        Some(StrHelper::new(v))
    }
}