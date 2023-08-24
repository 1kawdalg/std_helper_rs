mod str;
mod string;

use super::StrHelper;

impl StrHelper {
    /// split ```self''' at separators and get ```Vec<Option<StrHelper>>```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|g|cc");
    ///
    /// let g = helper.split("|").get(1).unwrap().clone();
    /// assert_eq!(g.unwrap().as_str(), "g");
    /// ```
    pub fn split(&self, separator: &str) -> Vec<Option<StrHelper>> {
        self.split_str(separator)
            .iter()
            .map(|v| Some(StrHelper::new((*v)?)))
            .collect::<Vec<Option<StrHelper>>>()
    }

    /// split ```self''' at separators and get ```StrHelper'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get("|", 1).unwrap();
    /// assert_eq!(part.as_str(), "world");
    /// ```
    pub fn split_and_get(&self, separator: &str, n: usize) -> Option<StrHelper> {
        let v = self
            .split_and_get_str(separator, n)?;

        Some(StrHelper::new(v))
    }

    /// split ```self''' at separators and get first ```StrHelper'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_first("|").unwrap();
    /// assert_eq!(part.as_str(), "hello");
    /// ```
    pub fn split_and_get_first(&self, separator: &str) -> Option<StrHelper> {
        self.split_and_get(separator, 0)
    }

    /// split ```self''' at separators and get last ```StrHelper'''.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello|world|rust");
    ///
    /// let part = helper.split_and_get_last("|").unwrap();
    /// assert_eq!(part.as_str(), "rust");
    /// ```
    pub fn split_and_get_last(&self, separator: &str) -> Option<StrHelper> {
        let v = self
            .split_and_get_last_str(separator)?;

        Some(StrHelper::new(v))
    }

    /// split ```self''' at separators on the sides and get ```StrHelper```.
    ///
    /// # Example:
    /// ```
    /// #[macro_use] extern crate std_helper;
    /// use std_helper::StrHelper;
    ///
    /// let helper = str!("hello");
    ///
    /// let l = helper.split_on_the_sides(("e", "lo")).unwrap();
    /// assert_eq!(l.as_str(), "l");
    /// ```
    pub fn split_on_the_sides(&self, separator: (&str, &str)) -> Option<StrHelper> {
        let v = self
            .split_str_on_the_sides(separator)?;

        Some(StrHelper::new(v))
    }
}