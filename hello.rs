fn display_game_rules() {
    println!("Two players alternate selecting numbers between 1 and 9.");
    println!("Once a number has is selected by a player it cannot be put back or selected again.");
    println!("The first player to have the sum of their selected numbers equal 15 wins.");
}

fn display_game_end() {
    println!("Thanks for playing!");
}

fn display_board(board: [i32; 9]) {
    println!("Available numbers are: ");

    for item in board.iter() {
        if *item != 0 {
            print!("{} ", item);
        }
    }

    println!();
}

fn get_player_selection() {
    let mut line = String::new();
    println!("Enter your move: ");

    let _b = std::io::stdin().read_line(&mut line).unwrap();

    // TODO probably don't want to unwrap here in case it is bad data we should retry
    let num: i32 = line.trim().parse::<i32>().unwrap();
    println!("Move was: {}", num);
}

fn play_game() {
    let board: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    display_board(board);

    get_player_selection();
}

fn main() {
    display_game_rules();
    play_game();
    display_game_end();
}
