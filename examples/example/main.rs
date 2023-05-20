extern crate pretable;

use pretable::{Alignment, PreTable};

fn main() {
    let mut table = PreTable::new();

    table.set_header_with_alignment(vec![
        ("NAME", Alignment::Left),
        ("FORMAL", Alignment::Center),
        ("HEIGHT", Alignment::Right),
    ]);

    table.add_body(vec!["Everest", "Chomolungma", "8848m"]);
    table.add_body(vec!["K2", "Karakorum No.2", "8611m"]);
    table.add_body(vec!["Kanchenjunga", "", "8505m"]);

    println!("{}", table.output());
}
