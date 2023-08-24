use super::StrHelper;

impl StrHelper {
    /// split ```self''' at separators and get ```Vec<Option<&str>>```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|g|cc");
    ///
    /// let g = helper.split_str("|").get(1).unwrap().clone();
    /// assert_eq!(g.unwrap(), "g");
    /// ```
    pub fn split_str(&self, separator: &str) -> Vec<Option<&str>> {
        self.string
            .split(separator)
            .collect::<Vec<&str>>()
            .iter()
            .map(|v| Some(*v))
            .collect::<Vec<Option<&str>>>()
    }

    /// split ```self''' at separators and get ```&str'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_str("|", 1).unwrap();
    /// assert_eq!(part, "world");
    /// ```
    pub fn split_and_get_str(&self, separator: &str, n: usize) -> Option<&str> {
        self.split_str(separator)
            .iter()
            .map(|item| *item)
            .nth(n)?
    }

    /// split ```self''' at separators and get first ```&str'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_first_str("|").unwrap();
    /// assert_eq!(part, "hello");
    /// ```
    pub fn split_and_get_first_str(&self, separator: &str) -> Option<&str> {
        self.split_and_get_str(separator, 0)
    }

    /// split ```self''' at separators and get last ```&str'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_last_str("|").unwrap();
    /// assert_eq!(part, "rust");
    /// ```
    pub fn split_and_get_last_str(&self, separator: &str) -> Option<&str> {
        self.split_str(separator)
            .iter()
            .map(|item| *item)
            .last()?
    }

    /// split ```self''' at separators on the sides and get ```&str```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello");
    ///
    /// let l = helper.split_str_on_the_sides(("e", "lo")).unwrap();
    /// assert_eq!(l, "l");
    /// ```
    pub fn split_str_on_the_sides(&self, separator: (&str, &str)) -> Option<&str> {
        self.split_and_get_last_str(separator.0)?
            .split(separator.1)
            .nth(0)
    }
}