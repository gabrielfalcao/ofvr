fn main() {
    let argv: Vec<String> = std::env::args().collect();
    println!("{}", argv[1..].join(" "));
}
