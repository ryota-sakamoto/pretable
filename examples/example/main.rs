extern crate pretable;

use pretable::PreTable;

fn main() {
    let mut table = PreTable::new();
    table.set_header(vec!["REPOSITORY", "TAG", "IMAGE ID", "CREATED", "SIZE"]);
    table.add_body(vec![
        "ubuntu",
        "18.04",
        "452a96d81c30",
        "6 weeks ago",
        "79.6MB",
    ]);
    table.add_body(vec![
        "ubuntu",
        "18.04",
        "452a96d81c30",
        "7 weeks ago",
        "79.6MB",
    ]);
    // table.add_body(vec!["name1dsfsdf", "vdsfdsfalue"]);
    // table.show_header(false);
    // table.is_body_split(true);
    // table.set_line_char(' ');
    // table.set_vertical_char(' ');
    // table.set_corner_char(' ');
    println!("{}", table.output());
}
