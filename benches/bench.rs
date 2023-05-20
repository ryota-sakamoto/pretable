#![feature(test)]

extern crate test;

fn generate_test_table(n: u64) -> pretable::PreTable {
    let numbers = (0..n).map(|i| (i + 1).to_string()).collect::<Vec<_>>();

    let mut table = pretable::PreTable::new();
    table.set_header(vec!["i"]);
    for n in numbers.iter() {
        table.add_body(vec![n]);
    }

    table
}

#[bench]
fn bench_output_100(b: &mut test::Bencher) {
    let table = generate_test_table(100);
    b.iter(|| table.output());
}

#[bench]
fn bench_output_1000(b: &mut test::Bencher) {
    let table = generate_test_table(1_000);
    b.iter(|| table.output());
}

#[bench]
fn bench_output_10000(b: &mut test::Bencher) {
    let table = generate_test_table(10_000);
    b.iter(|| table.output());
}

#[bench]
fn bench_output_100000(b: &mut test::Bencher) {
    let table = generate_test_table(100_000);
    b.iter(|| table.output());
}
