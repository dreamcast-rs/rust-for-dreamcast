fn main() {
    println!("Hello, world from Rust! - romdisk example");

    println!("{}", std::fs::read_to_string("/rd/text.txt").expect("Could not read file /rd/text.txt!"));
}
