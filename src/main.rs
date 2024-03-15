use std::io::stdin;
use std::time::Duration;
use std::{thread, vec};
use colored::Colorize;

const X: [&str; 3] = [
    "     ",
    r"  \/  ",
    r"  /\  "];

const O: [&str; 3] = [
    " __  ",
    r" /  \ ",
    r" \__/ "];

const EMPTY: [&str; 3] = [
    "     ",
    "      ",
    "      "];

const NUM_TO_PLAYER: [char; 2] = ['X', 'O'];
const POST_INVALID_INPUT_DELAY: Duration = Duration::from_millis(1500);
const POST_VICTORY_DELAY: Duration = Duration::from_millis(2000);

macro_rules! input {
    () => {
        stdin().read_line(&mut String::new()).expect("Failed to read line!");
    };
    ($var:ident) => {
        let mut $var = String::new();
        stdin().read_line(&mut $var).expect("Failed to read line!");
        $var = $var.trim().to_string();
    };
}

macro_rules! check_winner {
    ($square1:expr, $square2:expr, $square3:expr) => {
        if $square1 == $square2 && $square2 == $square3 && $square1 != 0 {
            return $square1 as i8
        }
    };
}

/// Returns the status of the board.
/// -1: Game is still ongoing
/// 0: Draw
/// 1: X won
/// 2: O won
fn board_status(board: [[i8; 3]; 3]) -> i8 {
    let mut has_empty = false;

    for i in 0..3 {
        let row = board[i];

        if row.contains(&0) { has_empty = true }

        check_winner!(row[0], row[1], row[2]);
        check_winner!(board[0][i], board[1][i], board[2][i]);
    }

    for i in [0, 2] {
        check_winner!(board[0][0 + i], board[1][1], board[2][2 - i]);
    }

    if has_empty { -1 } else { 0 }
}

fn render_board(board: [[i8; 3]; 3]) {
    println!();
    // Outer array represents the 3 rows of squares on the board, 
    // inner array represents the 3 columns of squares in each row and
    // the innermost array represents the 3 rows of each square
    let mut result = vec![vec![vec![String::new(); 3]; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                let mut row_seg = String::new();

                if k == 0 {
                    // Add the square number (1 - 9)
                    row_seg += &*(i * 3 + j + 1).to_string();
                }

                match board[i][j] {
                    0 => row_seg += EMPTY[k],
                    1 => row_seg += X[k],
                    2 => row_seg += O[k],
                    _ => ()
                }

                result[i][j][k] = row_seg;
            }
        }
    }

    for i in 0..3 {
        for k in 0..3 {
            for j in 0..3 {
                thread::sleep(Duration::from_millis(10));
                print!("{}", result[i][j][k]);
                if j != 2 { print!("|"); }
            }
            println!();
        }
        if i != 2 { println!("______|______|______") }
    }
}

fn initialize_game() -> usize {
    println!("{}", "Welcome to Tic-Tac-Toe!\nPress Enter to continue...".purple());
    input!();

    let starting_player_index;

    loop {
        println!("{}", "Would you like for X or O to start the game?".bright_purple());
        input!(starting_player);

        starting_player_index = match starting_player.to_lowercase().as_str() {
            "x" => 0,
            "o" => 1,
            _ => {
                println!("🗿");
                continue;
            }
        };

        break;
    }

    starting_player_index
}

fn validate_choice(input_char: Option<char>) -> Option<usize> {
    match input_char {
        Some(c) => {
            match c.to_digit(10) {
                Some(0) => {
                    println!("{}", "\nThere's no square with number 0.".red());
                    thread::sleep(POST_INVALID_INPUT_DELAY);
                    None
                }
                Some(d) => Some((d - 1) as usize),
                None => {
                    println!("{}", "\nPlease input a number, not some random text.".red());
                    thread::sleep(POST_INVALID_INPUT_DELAY);
                    None
                }
            }
        }
        None => {
            println!("{}", "\nPlease input a number.".red());
            thread::sleep(POST_INVALID_INPUT_DELAY);
            None
        }
    }
}

fn play() {
    'game: loop {
        let mut current_player_index = initialize_game();
        let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
        let mut status = board_status(board);
        render_board(board);
        
        while status == -1 {
            let message = format!("\nWhere would you like to place {}? (1-9)", NUM_TO_PLAYER[current_player_index]);
            println!("{}", message.blue());
            input!(input);

            let input_char = input.chars().next();
            let square_index = match validate_choice(input_char) {
                None => continue,
                Some(n) => n
            };

            let i = square_index / 3;
            let j = square_index % 3;

            if board[i][j] == 0 {
                board[i][j] = (current_player_index + 1) as i8;
            } else {
                println!("{}", Colorize::red("That square is already occupied.\n"));
                thread::sleep(POST_INVALID_INPUT_DELAY);
                continue;
            }

            current_player_index = (current_player_index + 1) % 2;
            render_board(board);
            status = board_status(board);
        }

        if status == 0 {
            println!("{}", "\nDamn. It's a draw.\n".yellow().bold())
        } else {
            let message = format!("\nCongratulations, {} won!\n", NUM_TO_PLAYER[(status - 1) as usize]);
            println!("{}", message.green().bold());
        }

        thread::sleep(POST_VICTORY_DELAY);

        loop {
            println!("{}", "Would you like to play again? (Y/N)".yellow());
            input!(again);

            again = again.to_lowercase();
            if again == "y" {
                print!("{esc}c", esc = 27 as char);
                continue 'game;
            } else if again == "n" {
                return;
            } else {
                println!("{}", "Wut?".bright_cyan());
                thread::sleep(POST_INVALID_INPUT_DELAY);
            }
        }
    }
}

fn main() {
    play();
}