fn main() {
    let mut table = PreTable::new();
    table.add_header("name");
    table.add_header("value");
    table.add_body(vec!["name1", "value1"]);
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

    fn add_body(&mut self, v: Vec<&str>) {
        self.body.push(v.iter().map(|s| {
            s.to_string()
        }).collect());
        
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
        let mut max_iter = self.max.iter();
        let v: Vec<String> = self.body.iter().map(|v| {
            let mut s = "|".to_string();
            let m = max_iter.next().unwrap();
            for n in 0..self.header.len() {
                let ref value = v[n];
                s += &format!("{}|", Self::format_center(value, m + 2));                
            }
            s
        }).collect();
        v.join("\n")
    }

    fn output(self) -> String {
        let s = format!("{}\n{}\n{}\n{}\n{}\n",
            &self.line(),
            &self.header(),
            &self.line(),
            &self.body(),
            &self.line(),
        );
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