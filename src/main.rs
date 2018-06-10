fn main() {
    let mut table = PreTable::new();
    table.set_header(vec!["name", "value"]);
    table.add_body(vec!["name1", "value1", "sddsf"]);
    table.add_body(vec!["name1dsfsdf", "vdsfdsfalue"]);
    println!("{}", table.output());
}

struct PreTable {
    header: Vec<String>,
    body: Vec<Vec<String>>,
    max: Vec<usize>,
}

impl PreTable {
    pub fn new() -> Self {
        Self {
            header: Vec::new(),
            body: Vec::new(),
            max: Vec::new(),
        }
    }

    fn add_header(&mut self, v: &str) {
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

    fn add_body(&mut self, v: Vec<&str>) {
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
        let mut line = "+".to_string();

        for n in 0..self.header.len() {
            let c = self.max[n];
            line += &format!("{}+", Self::dush(c + 2));
        }

        line
    }

    fn header(&self) -> String {
        let mut s = "|".to_string();
        for n in 0..self.header.len() {
            let ref h = self.header[n];
            let ref m = self.max[n];
            s += &format!("{}|", Self::format_center(h, m + 2));
        }
        s
    }

    fn body(&self) -> String {
        let v: Vec<String> = self.body
            .iter()
            .map(|v| {
                let mut s = "|".to_string();
                let mut max_iter = self.max.iter();
                for n in 0..self.header.len() {
                    let m = max_iter.next().unwrap();
                    let value = v.get(n);
                    s += &format!(
                        "{}|",
                        Self::format_center(
                            match value {
                                Some(v) => v,
                                None => "",
                            },
                            m + 2
                        )
                    );
                }
                s
            })
            .collect();
        v.join("\n")
    }

    fn output(self) -> String {
        let mut s = format!("{}\n", self.line());
        if !self.header.is_empty() {
            s += &format!("{}\n", self.header());
            s += &format!("{}\n", self.line());
        }
        if !self.body.is_empty() {
            s += &format!("{}\n", self.body());
            s += &format!("{}\n", self.line());
        }
        s
    }

    fn dush(count: usize) -> String {
        Self::repeat("-", count)
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
