use rand::Rng;
use std::ops::RangeInclusive;
use std::{io, process::exit};

fn random_number(startrange: usize, maxrange: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(RangeInclusive::new(startrange, maxrange));
    return num;
}

fn game() {
    let money = 50.00;
    let mut betstring = String::new();
    println!("Your money: ${:.2}\n", money);
    println!("How much would you like to bet: ");
    io::stdin()
        .read_line(&mut betstring)
        .expect("Failed to read line");

    let bet = match betstring.trim().parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    match (bet * 100.0) as i64 {
        0 => {
            println!("Your bet cannot be $0.00");
            return;
        }
        bet if bet > (money * 100.0) as i64 => {
            println!("Your bet is too large");
            println!("You currently have ${}", money);
        }
        _ => {
            println!("Bet set of ${}\nLets begin!", bet)
        }
    }

    if money == 0.00 {
        println!("Game over, You ran out of money");
        return;
    }

    let suits = ["hearts ", "spades ", "clubs ", "diamonds "];
    let rank = [
        "ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "jack", "queen", "king",
    ];

    let cards = [format!("{}{}", suits[random_number(0, 3)], rank[random_number(0, 12)]), format!("{}{}", suits[random_number(0, 3)], rank[random_number(0, 12)])];

    for item in cards.iter() {
        println!("{}", item);
    }

}

fn help() {
    println!("You place a bet and then you get 2 random cards and you decide if you want more cards.\nIf the dealer gets more than you then you lose, but when you have more than the dealer you win.\nIf you have more than 21 you also lose.\nWhenever you win you get twice the amount of your bet.");
}

fn main() {
    println!("Welcome to Black jack!\n");
    println!("Type help for instructions");
    println!("Type start to play and exit to quit the game");

    loop {
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("Failed to read line");

        match action.trim().to_lowercase().as_str() {
            "start" => {
                game();
            }
            "exit" => {
                println!("Goodbye have a nice day!");
                exit(0);
            }
            "help" => {
                help();
            }
            _ => {
                println!("Invalid command");
            }
        }
    }
}
