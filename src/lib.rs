#[derive(Debug)]
pub struct PreTable {
    items: Vec<Item>,
    header_len: usize,
    body_length: usize,
    show_header: bool,
    is_body_split: bool,
    line_char: char,
    vertical_char: char,
    corner_char: char,
}

impl PreTable {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            header_len: 0,
            body_length: 0,
            show_header: true,
            is_body_split: false,
            line_char: '-',
            vertical_char: '|',
            corner_char: '+',
        }
    }

    pub fn add_header(&mut self, v: &str) {
        self.items.push(Item::new(v));
        self.header_len = self.items.len();
    }

    pub fn set_header(&mut self, v: Vec<&str>) {
        self.items = Vec::new();
        for value in v {
            self.add_header(value);
        }
    }

    pub fn add_body(&mut self, v: Vec<&str>) {
        self.body_length += 1;
        for n in 0..self.header_len {
            let value = v.get(n).or_else(|| Some(&"")).unwrap();
            self.items[n].value.push(value.to_string());
            if self.items[n].max_value_len < value.len() {
                self.items[n].max_value_len = value.len();
            }
        }
    }

    pub fn line(&self) -> String {
        let s: Vec<String> = self.items.iter().map(|item| {
            format!("{}{}", self.corner_char, Self::repeat(&self.line_char.to_string(), item.max_value_len + 2))
        }).collect();

        format!("{}{}", s.join(""), self.corner_char)
    }

    fn header(&self) -> String {
        let s: Vec<String> = self.items.iter().map(|item| {
            format!("{}{}", self.vertical_char, Self::format_center(&item.key, &item.max_value_len + 2))
        }).collect();

        format!("{}{}", s.join(""), self.vertical_char)
    }

    fn body(&self) -> Vec<String> {
        let mut v: Vec<_> = self.items.iter().map(|item| {
            item.value.iter()
        }).collect();
        let value_len_vec: Vec<_> = self.items.iter().map(|item| item.max_value_len).collect();

        let mut vec = Vec::with_capacity(self.body_length);
        for _ in 0..self.body_length {
            let r = v.next();
            let mut l = value_len_vec.iter();

            let result: Vec<_> = r.iter().map(|value| {
                format!("{}{}", self.vertical_char, Self::format_center(match value {
                    &Some(vv) => vv,
                    &None => "",
                }, *l.next().unwrap() + 2))
            }).collect();
            vec.push(format!("{}{}", result.join(""), self.vertical_char));
        }

        vec
    }

    pub fn output(self) -> String {
        let mut s = format!("{}\n", self.line());
        if self.show_header && !self.items.is_empty() {
            s += &format!("{}\n", self.header());
            s += &format!("{}\n", self.line());
        }
        let body = self.body();
        if self.is_body_split {
            for b in body {
                s += &format!("{}\n", b);
                s += &format!("{}\n", self.line());
            }
        } else {
            s += &format!("{}\n", body.join("\n"));
            s += &format!("{}\n", self.line());
        }
        s
    }

    pub fn show_header(&mut self, b: bool) {
        self.show_header = b;
    }

    pub fn is_body_split(&mut self, b: bool) {
        self.is_body_split = b;
    }

    pub fn set_line_char(&mut self, c: char) {
        self.line_char = c;
    }

    pub fn set_vertical_char(&mut self, c: char) {
        self.vertical_char = c;
    }

    pub fn set_corner_char(&mut self, c: char) {
        self.corner_char = c;
    }

    fn format_center(v: &str, count: usize) -> String {
        let start = (count - v.len()) / 2;
        let end = count - v.len() - start;
        format!("{}{}{}", Self::repeat(" ", start), v, Self::repeat(" ", end))
    }

    fn repeat(s: &str, count: usize) -> String {
        let mut v = String::new();
        for _ in 0..count {
            v = format!("{}{}", v, s);
        }
        v
    }
}

#[derive(Debug)]
pub struct Item {
    key: String,
    value: Vec<String>,
    max_value_len: usize,
}

impl Item {
    fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            value: Vec::new(),
            max_value_len: key.len(),
        }
    }
}

trait SliceItarator {
    fn next(&mut self) -> Vec<Option<&String>>;
}

impl<'a> SliceItarator for Vec<std::slice::Iter<'a, String>> {
    fn next(&mut self) -> Vec<Option<&String>> {
        let mut values = Vec::new();
        for v in self {
            let mut v = v;
            values.push(v.next());
        }
        values
    }
}