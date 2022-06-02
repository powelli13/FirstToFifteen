fn display_board(board: [i32; 9]) {
    println!("Available numbers are: ");

    for item in board.iter() {
        let x: &i32 = item;
        println!("{}", x);
    }
}

fn main() {
    let mut board: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    display_board(board);

    let mut line = String::new();
    println!("Enter your move: ");

    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Move was: {}", line);
    println!("no of bytes read , {}", b1);
}
