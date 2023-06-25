use std::io::stdin;
use std::time::Duration;
use std::{thread, vec};
use colored::Colorize;

const X: [&str; 4] = [
    "     ",
    r"  \/  ",
    r"  /\  ",
    "      "];

const O: [&str; 4] = [
    " __  ",
    r" /  \ ",
    r" \__/ ",
    "      "];

const EMPTY: [&str; 4] = [
    "     ",
    "      ",
    "      ",
    "      "];

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

fn is_solved(board: [[i8; 3]; 3]) -> i8 {
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
    let mut result = vec![vec![vec![String::new(); 4]; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            for k in 0..4 {
                let mut row_seg = String::new();

                if k == 0 {
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
                thread::sleep(Duration::from_millis(20));
                print!("{}", result[i][j][k]);
                if j != 2 { print!("|"); }
            }
            println!();
        }
        if i != 2 { println!("______|______|______") }
    }
}

fn play() {
    println!("{}", "Welcome to Tic-Tac-Toe!\nPress Enter to continue...".purple());

    input!();

    let num_to_x_o: [(i8, char); 2] = [(1, 'X'), (2, 'O')];
    let mut current_pointer;

    loop {
        println!("{}", "Would you like for X or O to start the game?".bright_purple());
        input!(starting_player);

        if starting_player.to_lowercase() == String::from("x") {
            current_pointer = 0;
            break;
        } else if starting_player.to_lowercase() == String::from("o") {
            current_pointer = 1;
            break;
        }

        println!("🗿");
    }

    let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];

    while is_solved(board) == -1 {
        render_board(board);

        let message = format!("\nWhere would you like to place {}? (1-9)", num_to_x_o[current_pointer].1);
        println!("{}", message.blue());
        input!(input);

        let input_char = input.chars().next();
        let square_index;

        match input_char {
            Some(c) => square_index = match c.to_digit(10) {
                Some(0) => {
                    println!("{}", "\nThere's no square with number 0.".red());
                    thread::sleep(Duration::from_millis(1500));
                    continue;
                }
                Some(d) => { (d - 1) as usize }
                None => {
                    println!("{}", "\nPlease input a number, not some random text.".red());
                    thread::sleep(Duration::from_millis(1500));
                    continue;
                }
            },
            None => {
                println!("{}", "\nPlease input a number.".red());
                thread::sleep(Duration::from_millis(1500));
                continue;
            }
        }

        let i = square_index / 3;
        let j = square_index % 3;

        if board[i][j] == 0 {
            board[i][j] = num_to_x_o[current_pointer].0;
        } else {
            println!("{}", Colorize::red("That square is already occupied.\n"));
            thread::sleep(Duration::from_millis(1500));
            continue;
        }

        current_pointer = (current_pointer + 1) % 2;
    }

    render_board(board);
    let result = is_solved(board);

    if result == 0 {
        println!("{}", "\nDamn. It's a draw.".yellow().bold())
    } else {
        let message = format!("\nCongratulations, {} won!\n", num_to_x_o[(result - 1) as usize].1);
        println!("{}", message.green().bold());
    }

    thread::sleep(Duration::from_millis(2000));

    loop {
        println!("{}", "Would you like to play again? (Y/N)".yellow());
        input!(again);

        again = again.to_lowercase();
        if again == "y" {
            print!("{esc}c", esc = 27 as char);
            play();
        } else if again == "n" {
            break;
        } else {
            println!("{}", "Wut?".bright_cyan());
            thread::sleep(Duration::from_millis(1000));
        }
    }
}

fn main() {
    play();
}