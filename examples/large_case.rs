extern crate pretable;

use pretable::PreTable;

pub fn main() {
    let n = 1_000_000;

    let numbers = (0..n).map(|i| (i + 1).to_string()).collect::<Vec<_>>();

    let mut table = PreTable::new();
    table.set_header(vec!["i"]);
    for n in numbers.iter() {
        table.add_body(vec![n]);
    }

    let output = table.output();

    // 処理が削除されないように output を使うフリをする。
    let t = output.as_bytes();
    println!("{}", t[3] + t[t.len() - 3]);
}
