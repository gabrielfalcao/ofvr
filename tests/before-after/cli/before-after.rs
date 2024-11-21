fn main() {
    let argv = dbg!(std::env::args().collect::<Vec<String>>());
    println!("{}", argv[1..].join(" "));
}
