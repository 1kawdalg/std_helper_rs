use super::StrHelper;

impl StrHelper {
    /// split ```self''' at separators and get ```Vec<Option<String>>```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|g|cc");
    ///
    /// let g = helper.split_string("|").get(1).unwrap().clone();
    /// assert_eq!(g.unwrap(), "g");
    /// ```
    pub fn split_string(&self, separator: &str) -> Vec<Option<String>> {
        self.split_str(separator)
            .iter()
            .map(|v|Some((*v)?.to_string()))
            .collect::<Vec<Option<String>>>()
    }

    /// split ```self''' at separators and get ```String'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_string("|", 1).unwrap();
    /// assert_eq!(part, "world");
    /// ```
    pub fn split_and_get_string(&self, separator: &str, n: usize) -> Option<String> {
        let v = self
            .split_str(separator)
            .iter()
            .map(|item| *item)
            .nth(n)??
            .to_string();

        Some(v)
    }

    /// split ```self''' at separators and get first ```String'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_first_string("|").unwrap();
    /// assert_eq!(part, "hello");
    /// ```
    pub fn split_and_get_first_string(&self, separator: &str) -> Option<String> {
        self.split_and_get_string(separator, 0)
    }

    /// split ```self''' at separators and get last ```String'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_last_string("|").unwrap();
    /// assert_eq!(part, "rust");
    /// ```
    pub fn split_and_get_last_string(&self, separator: &str) -> Option<String> {
        let v = self
            .split_str(separator)
            .iter()
            .map(|item| *item)
            .last()??.to_string();

        Some(v)
    }

    /// split ```self''' at separators on the sides and get ```String```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello");
    ///
    /// let l = helper.split_string_on_the_sides(("e", "lo")).unwrap();
    /// assert_eq!(l, "l");
    /// ```
    pub fn split_string_on_the_sides(&self, separator: (&str, &str)) -> Option<String> {
        let v = self
            .split_str_on_the_sides(separator)?
            .to_string();

        Some(v)
    }
}