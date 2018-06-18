pub struct PreTable {
    header: Vec<String>,
    body: Vec<Vec<String>>,
    max: Vec<usize>,
    show_header: bool,
    is_body_split: bool,
    line_char: char,
    vertical_char: char,
    corner_char: char,
}

impl PreTable {
    pub fn new() -> Self {
        Self {
            header: Vec::new(),
            body: Vec::new(),
            max: Vec::new(),
            show_header: true,
            is_body_split: false,
            line_char: '-',
            vertical_char: '|',
            corner_char: '+',
        }
    }

    pub fn add_header(&mut self, v: &str) {
        self.header.push(v.to_string());

        let n = v.len();
        self.max.push(n);
    }

    pub fn set_header(&mut self, v: Vec<&str>) {
        self.header = Vec::new();
        self.max = Vec::new();

        for value in v {
            self.add_header(value);
        }
    }

    pub fn add_body(&mut self, v: Vec<&str>) {
        self.body.push(v.iter().map(|s| s.to_string()).collect());

        while self.max.len() < v.len() {
            self.max.push(0);
        }

        for n in 0..v.len() {
            let l = v[n].len();
            let m = self.max[n];
            if m < l {
                self.max[n] = l;
            }
        }
    }

    fn line(&self) -> String {
        let mut line = self.corner_char.to_string();
        let lc = &self.line_char.to_string();

        for n in 0..self.header.len() {
            let c = self.max[n];
            line += &format!("{}{}", Self::repeat(lc, c + 2), self.corner_char);
        }

        line
    }

    fn header(&self) -> String {
        let mut s = self.vertical_char.to_string();
        for n in 0..self.header.len() {
            let ref h = self.header[n];
            let ref m = self.max[n];
            s += &format!("{}{}", Self::format_center(h, m + 2), self.vertical_char);
        }
        s
    }

    fn body(&self) -> Vec<String> {
        self.body
            .iter()
            .map(|v| {
                let mut s = self.vertical_char.to_string();
                let mut max_iter = self.max.iter();
                for n in 0..self.header.len() {
                    let m = max_iter.next().unwrap();
                    let value = v.get(n);
                    s += &format!(
                        "{}{}",
                        Self::format_center(
                            match value {
                                Some(v) => v,
                                None => "",
                            },
                            m + 2
                        ),
                        self.vertical_char
                    );
                }
                s
            })
            .collect()
    }

    pub fn output(self) -> String {
        let mut s = format!("{}\n", self.line());
        if self.show_header && !self.header.is_empty() {
            s += &format!("{}\n", self.header());
            s += &format!("{}\n", self.line());
        }
        if !self.body.is_empty() {
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
        Self::repeat(" ", start) + v + &Self::repeat(" ", end)
    }

    fn repeat(s: &str, count: usize) -> String {
        let mut v = String::new();
        for _ in 0..count {
            v += s;
        }
        v
    }
}
