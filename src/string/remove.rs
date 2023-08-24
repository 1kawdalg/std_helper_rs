use super::StrHelper;

impl StrHelper {
    /// remove part if it contains.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("Hello, F****** ***T *I***, World!");
    /// let result = helper.remove("F****** ***T *I***, ").unwrap();
    ///
    /// assert_eq!(result, helper.to_string(), "Hello, World!");
    /// ```
    pub fn remove(&mut self, p: &str) -> Option<String> {
        let len = p.len();

        for i in 0..self.string.len() {
            let remover_p = self.string
                .get(i..i + len)?;

            if remover_p == p {
                let (part1, part2) = self
                    .split_at(i);

                self.string = part1?.string + part2?
                    .split_str_at(len).1?;

                break
            }
        }

        Some(self.string.clone())
    }

    /// remove part at indexes.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("Hello, @@@World!");
    /// let result = helper.remove_at((7, 10)).unwrap();
    ///
    /// assert_eq!(result, helper.to_string(), "Hello, World!");
    /// ```
    pub fn remove_at(&mut self, p_indexes: (usize, usize)) -> Option<String> {
        let p_indexes = match p_indexes {
            (x, y) if y < x => (y, x),
            (x, y) => (x, y)
        };

        let first_part = self
            .split_string_at(p_indexes.0)
            .0?;

        let second_part = self
            .split_str_at(p_indexes.1)
            .1?;

        self.string = first_part + second_part;

        Some(self.string.clone())
    }
}