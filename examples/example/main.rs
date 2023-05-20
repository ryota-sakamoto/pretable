extern crate pretable;

use pretable::{Alignment, PreTable};

fn main() {
    let mut table = PreTable::new();

    table.add_header_with_alignment("NAME", Alignment::Left);
    table.add_header_with_alignment("FORMAL", Alignment::Center);
    table.add_header_with_alignment("HEIGHT", Alignment::Right);

    table.add_body(vec!["Everest", "Chomolungma", "8848m"]);
    table.add_body(vec!["K2", "Karakorum No.2", "8611m"]);
    table.add_body(vec!["Kanchenjunga", "", "8505m"]);

    println!("{}", table.output());
}
