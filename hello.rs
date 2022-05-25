// what is the Rust casing convention?
fn display_board() {
    println!("Available numbers are: ");
    // TODO make this an array and comma separate it
    println!("1, 2, 3, 4, 5, 6, 7, 8, 9");
}

fn main() {
    display_board();
    let mut line = String::new();
    println!("Enter your move: ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Move was: {}", line);
    println!("no of bytes read , {}", b1);
}
