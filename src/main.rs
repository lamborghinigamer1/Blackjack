use rand::Rng;
use std::ops::RangeInclusive;
use std::{io, process::exit};

fn random_number(startrange: usize, maxrange: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(RangeInclusive::new(startrange, maxrange));
    return num;
}

fn sortcard(card: &str, cardvalue: i32) -> i32 {
    let mut cardvalue = cardvalue;
    match card {
        "ace" => {
            if cardvalue > 11 {
                cardvalue += 1;
            } else {
                cardvalue += 11;
            }
        }
        "2" => {
            cardvalue += 2;
        }
        "3" => {
            cardvalue += 3;
        }
        "4" => {
            cardvalue += 4;
        }
        "5" => {
            cardvalue += 5;
        }
        "6" => {
            cardvalue += 6;
        }
        "7" => {
            cardvalue += 7;
        }
        "8" => {
            cardvalue += 8;
        }
        "9" => {
            cardvalue += 9;
        }
        _ => {
            cardvalue += 10;
        }
    }
    cardvalue
}

fn game(money: f64) {
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

    let suits = ["heart ", "spades ", "clubs ", "diamonds "];
    let rank = [
        "ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "jack", "queen", "king",
    ];

    let cards = [
        format!(
            "{}{}",
            suits[random_number(0, 3)],
            rank[random_number(0, 12)]
        ),
        format!(
            "{}{}",
            suits[random_number(0, 3)],
            rank[random_number(0, 12)]
        ),
    ];

    let mut cardvalue = 0;

    for fullcard in cards.iter() {
        println!("{}", fullcard);

        let card = fullcard.split(' ').last().unwrap();

        cardvalue = sortcard(card, cardvalue);
    }
    println!("The value total value = {}\n", cardvalue);

    println!("What would you like to?\n");
    println!("1 Stand");
    println!("2 Hit");
    println!("3 Double bet and get 1 card");
    let mut choosestring = String::new();
    io::stdin()
        .read_line(&mut choosestring)
        .expect("Failed to read line");
    let choose = match choosestring.trim().parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };
    match choose {
        1 => {
            println!("Standing");
        }
        2 => {
            println!("Hitting");
        }
        3 => {
            println!("Doubling");
        }
        _ => {
            println!("Invalid option");
        }
    }
}

fn help() {
    println!("You place a bet and then you get 2 random cards and you decide if you want more cards.\nIf the dealer gets more than you then you lose, but when you have more than the dealer you win.\nIf you have more than 21 you also lose.\nWhenever you win you get twice the amount of your bet.");
}

fn main() {
    let money = 50.00;
    let mut continuescript = false;

    println!("Welcome to Black jack!\n");
    println!("Type help for instructions");
    println!("Type start to play and exit to quit the game");

    loop {
        let mut action = String::new();

        if continuescript == false {
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            match action.trim().to_lowercase().as_str() {
                "start" => {
                    game(money);
                    continuescript = true;
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
        } else {
            println!("Would you like to continue? Yes or No?");
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            match action.trim().to_lowercase().as_str() {
                "yes" => {
                    game(money);
                }
                "y" => {
                    game(money);
                }
                "ye" => {
                    game(money);
                }
                "affirmative" => {
                    game(money);
                }
                "no" => {
                    println!("Goodbye have a nice day!");
                    exit(0);
                }
                "n" => {
                    println!("Goodbye have a nice day!");
                    exit(0);
                }
                "negative" => {
                    println!("Goodbye have a nice day!");
                    exit(0);
                }
                "nein" => {
                    println!("Goodbye have a nice day!");
                    exit(0);
                }
                _ => {
                    println!("Invalid command");
                }
            }
        }
    }
}
