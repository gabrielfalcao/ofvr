fn main() {
    let argv = std::env::args().collect::<Vec<String>>();
    println!("{}", argv[1..].join(" "));
}
