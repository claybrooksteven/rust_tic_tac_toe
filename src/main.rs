extern crate rand;
use std::io;
use rand::seq::IteratorRandom;
use std::convert::TryInto;

fn main() {
    println!("Input the character you would like to use\n(Input cannot be '_') -> ");
    let mut player_char = String::new();
    match io::stdin().read_line(&mut player_char) {
        Ok(bytes_read) => {
            if bytes_read != 3 {//chars are always 3 bytes
                println!("A CHARACTER. Not a buncha gobbledygook. Oh, you thought you got to play after pulling something like that? Uh, no.");
                std::process::exit(-1)
            }
        }
        Err(err) => {
            println!("Welp, something else is screwed, {err} namely. Gotta bail.");
            std::process::exit(1);
        }
    }
    if player_char.starts_with('_') {
        println!("Nice buddy, real smart, well now you don't get to play.\nThat's it, the program just terminates. Bye.");
        std::process::exit(2);
    }
    println!("Input the character the system will use\n(Input cannot be '_' or your character) -> ");
    let mut system_char = String::new();
    match io::stdin().read_line(&mut system_char) {
        Ok(bytes_read) => {
            if bytes_read != 3 {
                println!("A CHARACTER. Not a buncha gobbledygook. Oh, you thought you got to play after pulling something like that? Uh, no");
                std::process::exit(-2);
            }
        }
        Err(err) => {
            println!("Welp, something else is screwed, {err} namely. Gotta bail.");
            std::process::exit(4);
        }
    }
    if system_char.starts_with('_') || system_char == player_char {
        println!("Well, we were off to a good start, at least.\nYou don't get to play now, by the way. Bye.");
        std::process:: exit(3);
    }
    execute_game(player_char.as_str().trim(), system_char.as_str().trim());
}

fn execute_game(player: &str, system: &str) {
    let mut board_positions:Vec<&str> = Vec::from(["_", "_", "_", "_", "_", "_", "_", "_", "_"]);
    println!("Surprise, the system has the first move.");
    board_positions[4] = system;
    let mut available_move_list: Vec<i32> = Vec::new();
    loop {
        if check_for_draw(&board_positions, player, system) {
            println!("Nobody won. Great.");
            print_current_board(&board_positions);
            break;
        } else {
            print_current_board(&board_positions);
            let mut player_move = String::new();
            println!("Input the number of the cell in which to place your character -> ");
            match io::stdin().read_line(&mut player_move) {
                Ok(..) => {
                    let player_selection_result = player_move.trim().parse::<usize>();
                    if player_selection_result.is_ok() {
                        let player_selection = player_selection_result.ok().unwrap();
                        if board_positions.len() >= player_selection && board_positions[player_selection - 1] == "_" {
                            match &player_selection {
                                1 => {board_positions[0] = player;}
                                2 => {board_positions[1] = player;}
                                3 => {board_positions[2] = player;}
                                4 => {board_positions[3] = player;}
                                5 => {board_positions[4] = player;}
                                6 => {board_positions[5] = player;}
                                7 => {board_positions[6] = player;}
                                8 => {board_positions[7] = player;}
                                9 => {board_positions[8] = player;}
                                _ => {std::process::exit(313);}
                            }
                            if check_for_win(&board_positions, player) {
                                println!("Player has won!");
                                print_current_board(&board_positions);
                                std::process::exit(0);
                            }
                            for (index, element) in board_positions.iter().enumerate() {
                                if element == &"_" {
                                    available_move_list.push((index + 1).try_into().unwrap());
                                }
                            }
                            let mut rng = rand::thread_rng();
                            let system_move: &i32 = available_move_list.iter().choose(&mut rng).unwrap();
                            match system_move {
                                1 => {board_positions[0] = system;}
                                2 => {board_positions[1] = system;}
                                3 => {board_positions[2] = system;}
                                4 => {board_positions[3] = system;}
                                5 => {board_positions[4] = system;}
                                6 => {board_positions[5] = system;}
                                7 => {board_positions[6] = system;}
                                8 => {board_positions[7] = system;}
                                9 => {board_positions[8] = system;}
                                _ => {std::process::exit(314)}
                            }
                            available_move_list.clear();
                            if check_for_win(&board_positions, system) {
                                println!("System has won!");
                                print_current_board(&board_positions);
                                std::process::exit(0);
                            }
                        } else {
                            println!("That either isn't a move possible in this board or it's already been taken. Try again.")
                        }
                    }
                }
                Err(err) => {
                    println!("Welp, something else is screwed, {err} namely. Gotta bail.");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn print_current_board(positions: &Vec<&str>) {
    println!("{}|{}|{}\n{}|{}|{}\n{}|{}|{}", if positions[0] == "_" {"1"} else {positions[0]}, if positions[1] == "_" {"2"} else {positions[1]}, if positions[2] == "_" {"3"} else {positions[2]}, if positions[3] == "_" {"4"} else {positions[3]}, if positions[4] == "_" {"5"} else {positions[4]}, if positions[5] == "_" {"6"} else {positions[5]}, if positions[6] == "_" {"7"} else {positions[6]}, if positions[7] == "_" {"8"} else {positions[7]}, if positions[8] == "_" {"9"} else {positions[8]});
}

fn check_for_draw(positions: &Vec<&str>, player: &str, system: &str) -> bool {
    return !check_for_win(positions, player) && !check_for_win(positions, system) && !positions.into_iter().any(|x| *x == "_");
}

fn check_for_win(positions: &Vec<&str>, character: &str) -> bool {
    return (positions[0] == character && positions[1] == character && positions[2] == character) ||
        (positions[3] == character && positions[4] == character && positions[5] == character) ||
        (positions[6] == character && positions[7] == character && positions[8] == character) ||
        (positions[0] == character && positions[3] == character && positions[6] == character) ||
        (positions[1] == character && positions[4] == character && positions[7] == character) ||
        (positions[2] == character && positions[5] == character && positions[8] == character) ||
        (positions[0] == character && positions[4] == character && positions[8] == character) ||
        (positions[2] == character && positions[4] == character && positions[6] == character);
}