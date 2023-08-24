use super::StrHelper;

impl StrHelper {
    /// insert element at string.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("Hello!");
    /// let result = helper.insert(", World", 5).unwrap();
    ///
    /// assert_eq!(result, "Hello, World!")
    /// ```
    pub fn insert(&mut self, new_part: &str, index: usize) -> Option<String> {
        let (part1, part2) = self
            .split_str_at(index);

        let mut new_string = part1?
            .to_string();

        for ch in StrHelper::other_as_chars(new_part) {
            new_string.push(ch);
        }

        for ch in StrHelper::other_as_chars(part2?) {
            new_string.push(ch);
        }

        self.string = new_string;
        Some(self.string.clone())
    }

    /// insert element at the beginning of string.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("World!");
    /// let result = helper.insert_at_the_beginning("Hello, ").unwrap();
    ///
    /// assert_eq!(result, "Hello, World!")
    /// ```
    pub fn insert_at_the_beginning(&mut self, new_part: &str) -> Option<String> {
        let old_string = self.string.clone();
        self.string = new_part.to_string();

        for ch in StrHelper::other_as_chars(old_string.as_str()) {
            self.string.push(ch);
        }

        Some(self.string.clone())
    }

    /// insert element at the end of string.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("Hello");
    /// let result = helper.push(", World!").unwrap();
    ///
    /// assert_eq!(result, "Hello, World!")
    /// ```
    pub fn push(&mut self, new_part: &str) -> Option<String> {
        for ch in StrHelper::other_as_chars(new_part) {
            self.string.push(ch);
        }

        Some(self.string.clone())
    }
}