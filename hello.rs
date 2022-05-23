fn main() {
    let mut line = String::new();
    println!("Enter your move: ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Move was: {}", line);
    println!("no of bytes read , {}", b1);
}
