use super::StrHelper;

impl StrHelper {
    /// move element to index, using part's indexes.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("He, lloWorld!");
    /// let result = helper.move_to((4, 7), 2).unwrap();
    ///
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn move_to(&mut self, p_indexes: (usize, usize), new_index: usize) -> Option<String> {
        let p_indexes = match p_indexes {
            (x, y) if y < x => (y, x),
            (x, y) => (x, y)
        };

        let len = p_indexes.1 - p_indexes.0;

        let part1 = self
            .split_at(p_indexes.0).1?;

        let part2 = part1
            .split_str_at(len)
            .0?;

        let (new_part1, new_part2) = self
            .split_string_at(new_index);

        let len_part1 = new_part1.clone()?.len();

        let new_p_indexes = (
            p_indexes.0 - len_part1,
            p_indexes.1 - len_part1
        );

        let new_part2 = StrHelper::new(new_part2?.as_str())
            .remove_at(new_p_indexes)?;

        self.string = new_part1? + part2;
        self.string += new_part2.as_str();

        Some(self.string.clone())
    }

    /// move element to the beginning, using part's indexes.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("World!Hello, ");
    /// let result = helper.move_to_the_beginning((6, 13)).unwrap();
    ///
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn move_to_the_beginning(&mut self, p_indexes: (usize, usize)) -> Option<String> {
        let p_indexes = match p_indexes {
            (x, y) if y < x => (y, x),
            (x, y) => (x, y)
        };

        let (part1, part2) = self
            .split_at(p_indexes.0);

        let (part2, part3) = part2?
            .split_string_at(p_indexes.1 - p_indexes.0);

        self.string = part2? + part1?.string.as_str();
        self.string += part3?.as_str();

        Some(self.string.clone())
    }

    /// move element to the end, using part's indexes.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("World!Hello, ");
    /// let result = helper.move_to_the_end((0, 6)).unwrap();
    ///
    /// assert_eq!(result, "Hello, World!");
    /// ```
    pub fn move_to_the_end(&mut self, p_indexes: (usize, usize)) -> Option<String> {
        let p_indexes = match p_indexes {
            (x, y) if y < x => (y, x),
            (x, y) => (x, y)
        };

        let (part1, part2) = self
            .split_at(p_indexes.0);

        let (part2, part3) = part2?
            .split_string_at(p_indexes.1 - p_indexes.0);

        self.string = part1?.string + part3?.as_str();
        self.string += part2?.as_str();

        Some(self.string.clone())
    }

    /// switch first and last chars of string.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let mut helper = str!("!ello, WorldH");
    /// let _ = helper.switch_beginning_and_end();
    ///
    /// assert_eq!(helper.as_str(), "Hello, World!")
    /// ```
    pub fn switch_beginning_and_end(&mut self) -> Option<String> {
        let mut old = self.clone();
        self.move_to_the_end((0, 1));

        old.reverse();
        let start_value = old.split_string_on_the_sides_at((0, 1))?;

        let len = old.to_string().len() - 1;
        self.remove_at((len - 1, len));

        self.update((start_value + self.as_str()).as_str());
        Some(self.to_string())
    }
}