/// A `PreTable` struct used for table representation
#[derive(Debug)]
pub struct PreTable {
    items: Vec<Item>,
    body_length: usize,
    show_header: bool,
    is_body_split: bool,
    line_char: char,
    vertical_char: char,
    corner_char: char,
    default_alignment: Alignment,
}

impl PreTable {
    /// Creates a new instance of `PreTable`
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            body_length: 0,
            show_header: true,
            is_body_split: false,
            line_char: '-',
            vertical_char: '|',
            corner_char: '+',
            default_alignment: Alignment::Center,
        }
    }

    /// Adds a header to the table with default alignment
    pub fn add_header(&mut self, v: &str) {
        self.add_header_with_alignment(v, self.default_alignment);
    }

    /// Adds a header with a specified alignment
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretable::{Alignment, PreTable};
    /// let mut table = PreTable::new();
    /// table.add_header_with_alignment("KEY", Alignment::Left);
    /// ```
    pub fn add_header_with_alignment(&mut self, v: &str, alignment: Alignment) {
        self.items.push(Item::new(v, alignment));
    }

    /// Sets the headers of the table with default alignment
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretable::{Alignment, PreTable};
    /// let mut table = PreTable::new();
    /// table.set_header(vec!["KEY", "VALUE", "DESCRIPTION"]);
    /// ```
    pub fn set_header(&mut self, v: Vec<&str>) {
        self.items = Vec::new();
        for value in v {
            self.add_header_with_alignment(value, self.default_alignment);
        }
    }

    /// Sets the headers of the table with a specified alignment
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretable::{Alignment, PreTable};
    /// let mut table = PreTable::new();
    /// table.set_header_with_alignment(vec![
    ///     ("KEY", Alignment::Left),
    ///     ("VALUE", Alignment::Center),
    ///     ("DESCRIPTION", Alignment::Right),
    /// ]);
    /// ```
    pub fn set_header_with_alignment(&mut self, v: Vec<(&str, Alignment)>) {
        self.items = Vec::new();
        for value in v {
            self.add_header_with_alignment(value.0, value.1);
        }
    }

    /// Adds a row to the body of the table
    ///
    /// # Examples
    ///
    /// ```
    /// # use pretable::{Alignment, PreTable};
    /// let mut table = PreTable::new();
    /// table.set_header(vec!["KEY", "VALUE", "DESCRIPTION"]);
    /// table.add_body(vec!["key1", "value1", "description1"]);
    /// ```
    pub fn add_body(&mut self, v: Vec<&str>) {
        self.body_length += 1;
        for n in 0..self.items.len() {
            let value = if v.len() > n { v[n] } else { "" };
            self.items[n].value.push(value.to_string());
            if self.items[n].max_value_len < value.len() {
                self.items[n].max_value_len = value.len();
            }
        }
    }

    /// Generates a line string based on the current state of table
    pub fn line(&self) -> String {
        let s: Vec<String> = self
            .items
            .iter()
            .map(|item| {
                let mut s = String::with_capacity(1 + item.max_value_len + 2);
                s.push(self.corner_char);
                s.extend(std::iter::repeat(self.line_char).take(item.max_value_len + 2));
                s
            })
            .collect();

        format!("{}{}", s.concat(), self.corner_char)
    }

    /// Formats and returns the header as a string
    fn header(&self) -> String {
        let s: Vec<String> = self
            .items
            .iter()
            .map(|item| {
                let mut s = self.vertical_char.to_string();
                Self::format_align(&item.key, &item.max_value_len + 2, item.alignment, &mut s);
                s
            })
            .collect();

        format!("{}{}", s.concat(), self.vertical_char)
    }

    /// Formats and returns the body as a vector of strings
    fn body(&self) -> Vec<String> {
        let mut v: Vec<_> = self.items.iter().map(|item| item.value.iter()).collect();
        let value_len_vec: Vec<_> = self.items.iter().map(|item| item.max_value_len).collect();

        let mut vec = Vec::with_capacity(self.body_length);
        for _ in 0..self.body_length {
            let row = v.next();
            let mut n = 0;
            let mut inc = || {
                n += 1;
                n - 1
            };

            let mut result = String::new();
            for ref value in row {
                result.push(self.vertical_char);

                let item_index = inc();
                let value = value.map_or("", |s| s);

                Self::format_align(
                    value,
                    value_len_vec[item_index] + 2,
                    self.items[item_index].alignment,
                    &mut result,
                );
            }
            result.push(self.vertical_char);
            vec.push(result);
        }

        vec
    }

    /// Returns the complete table as a string
    pub fn output(&self) -> String {
        let mut buf = vec![];
        let l = self.line();

        buf.push(l.clone());
        if self.show_header && !self.items.is_empty() {
            buf.push(self.header());
            buf.push(l.clone());
        }
        let body = self.body();
        if self.is_body_split {
            for b in body {
                buf.push(b);
                buf.push(l.clone());
            }
        } else {
            buf.push(body.join("\n"));
            buf.push(l.clone());
        }

        buf.join("\n")
    }

    /// Decides whether to show the header in output
    pub fn set_show_header(&mut self, b: bool) {
        self.show_header = b;
    }

    /// Decides whether to split the body in output
    pub fn set_body_split(&mut self, b: bool) {
        self.is_body_split = b;
    }

    /// Sets the character used for line separation
    pub fn set_line_char(&mut self, c: char) {
        self.line_char = c;
    }

    /// Sets the character used for vertical separation
    pub fn set_vertical_char(&mut self, c: char) {
        self.vertical_char = c;
    }

    /// Sets the character used for corners
    pub fn set_corner_char(&mut self, c: char) {
        self.corner_char = c;
    }

    /// Sets the alignment used for format align
    pub fn set_default_alignment(&mut self, a: Alignment) {
        self.default_alignment = a;
    }

    /// Helper function to format string according to alignment
    fn format_align(v: &str, count: usize, alignment: Alignment, buf: &mut String) {
        let padding = count - v.len();
        let start = match alignment {
            Alignment::Left => 1,
            Alignment::Center => padding / 2,
            Alignment::Right => padding - 1,
        };
        let end = padding - start;

        buf.extend(std::iter::repeat(' ').take(start));
        *buf += v;
        buf.extend(std::iter::repeat(' ').take(end));
    }
}

/// An `Item` struct represents a column in the table
#[derive(Debug)]
pub struct Item {
    key: String,
    value: Vec<String>,
    max_value_len: usize,
    alignment: Alignment,
}

impl Item {
    fn new(key: &str, alignment: Alignment) -> Self {
        Self {
            key: key.to_string(),
            value: Vec::new(),
            max_value_len: key.len(),
            alignment: alignment,
        }
    }
}

/// An enum for alignment options
#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Center,
    Left,
    Right,
}

trait SliceItarator {
    fn next(&mut self) -> Vec<Option<&String>>;
}

impl<'a> SliceItarator for Vec<std::slice::Iter<'a, String>> {
    fn next(&mut self) -> Vec<Option<&String>> {
        let mut values = Vec::with_capacity(self.len());
        for v in self {
            values.push(v.next());
        }
        values
    }
}

#[cfg(test)]
mod tests {
    use super::{Alignment, PreTable};

    fn generate_test_table() -> PreTable {
        let mut table = PreTable::new();
        table.set_header(vec!["KEY", "VALUE", "DESCRIPTION"]);
        table.add_body(vec!["key1", "value1", "description1"]);
        table.add_body(vec!["key2", "long value 2", "description2"]);
        table.add_body(vec!["key3", "value3", "description3"]);

        table
    }

    #[test]
    fn test_output() {
        let table = generate_test_table();

        let actual = table.output();
        let expected = "+------+--------------+--------------+
| KEY  |    VALUE     | DESCRIPTION  |
+------+--------------+--------------+
| key1 |    value1    | description1 |
| key2 | long value 2 | description2 |
| key3 |    value3    | description3 |
+------+--------------+--------------+";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_skip_item() {
        let mut table = PreTable::new();
        table.set_header(vec!["KEY", "VALUE", "DESCRIPTION"]);
        table.add_body(vec!["key1", "value1"]);
        table.add_body(vec!["key2", "", "description2"]);

        let actual = table.output();
        let expected = "+------+--------+--------------+
| KEY  | VALUE  | DESCRIPTION  |
+------+--------+--------------+
| key1 | value1 |              |
| key2 |        | description2 |
+------+--------+--------------+";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_line_char() {
        let mut table = generate_test_table();
        table.set_line_char('x');

        let actual = table.output();
        let expected = "+xxxxxx+xxxxxxxxxxxxxx+xxxxxxxxxxxxxx+
| KEY  |    VALUE     | DESCRIPTION  |
+xxxxxx+xxxxxxxxxxxxxx+xxxxxxxxxxxxxx+
| key1 |    value1    | description1 |
| key2 | long value 2 | description2 |
| key3 |    value3    | description3 |
+xxxxxx+xxxxxxxxxxxxxx+xxxxxxxxxxxxxx+";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_vertical_char() {
        let mut table = generate_test_table();
        table.set_vertical_char('x');

        let actual = table.output();
        let expected = "+------+--------------+--------------+
x KEY  x    VALUE     x DESCRIPTION  x
+------+--------------+--------------+
x key1 x    value1    x description1 x
x key2 x long value 2 x description2 x
x key3 x    value3    x description3 x
+------+--------------+--------------+";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_corner_char() {
        let mut table = generate_test_table();
        table.set_corner_char('x');

        let actual = table.output();
        let expected = "x------x--------------x--------------x
| KEY  |    VALUE     | DESCRIPTION  |
x------x--------------x--------------x
| key1 |    value1    | description1 |
| key2 | long value 2 | description2 |
| key3 |    value3    | description3 |
x------x--------------x--------------x";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_show_header() {
        let mut table = generate_test_table();
        table.set_show_header(false);

        let actual = table.output();
        let expected = "+------+--------------+--------------+
| key1 |    value1    | description1 |
| key2 | long value 2 | description2 |
| key3 |    value3    | description3 |
+------+--------------+--------------+";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_set_body_split() {
        let mut table = generate_test_table();
        table.set_body_split(true);

        let actual = table.output();
        let expected = "+------+--------------+--------------+
| KEY  |    VALUE     | DESCRIPTION  |
+------+--------------+--------------+
| key1 |    value1    | description1 |
+------+--------------+--------------+
| key2 | long value 2 | description2 |
+------+--------------+--------------+
| key3 |    value3    | description3 |
+------+--------------+--------------+";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_format_align_center() {
        let mut buf = String::new();
        PreTable::format_align("abcde", 15, Alignment::Center, &mut buf);
        assert_eq!(buf, "     abcde     ");
    }

    #[test]
    fn test_format_align_left() {
        let mut buf = String::new();
        PreTable::format_align("abcde", 15, Alignment::Left, &mut buf);
        assert_eq!(buf, " abcde         ");
    }

    #[test]
    fn test_format_align_right() {
        let mut buf = String::new();
        PreTable::format_align("abcde", 15, Alignment::Right, &mut buf);
        assert_eq!(buf, "         abcde ");
    }
}
