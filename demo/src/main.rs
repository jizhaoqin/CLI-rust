fn main() {
    let contents = std::fs::read_to_string("./file.in").unwrap();
    for line in contents.lines() {
        println!("{}", line);
    }
}
