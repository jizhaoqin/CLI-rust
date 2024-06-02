fn main() {
    eprintln!("Hello, world!");
    println!("test");
    let str = "hello".to_string();
    println!("{}", str);
    println!("{str}");
    let array = vec![1, 2, 3, 4, 1];
    println!("{:?}", &array[..5]);
    println!()
}
