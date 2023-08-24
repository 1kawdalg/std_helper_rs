use super::StrHelper;

impl StrHelper {
    /// generate ```String``` from ```Vec<char>```
    pub fn generate_string_from_char_vec(mut vector: Vec<char>) -> String {
        let mut string = String::new();
        vector.reverse();

        for ch in vector {
            string.insert(0, ch);
        }

        string
    }

    /// generate ```String``` from ```&[char]```
    pub fn generate_string_from_chars(values: &[char]) -> String {
        let values: Vec<char> = values.iter().map(|x| *x).collect();
        StrHelper::generate_string_from_char_vec(values)
    }
}