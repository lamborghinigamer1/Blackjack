use rand::Rng;
use std::ops::RangeInclusive;
use std::{io, process::exit};

fn checkmoney(money: f64) -> bool {
    let mut lose = false;
    if money < 0.01 {
        lose = true;
    }
    lose
}

fn random_number(startrange: usize, maxrange: usize) -> usize {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(RangeInclusive::new(startrange, maxrange));
    return num;
}

fn sortcard(card: &str, cardvalue: i32) -> i32 {
    let mut cardvalue = cardvalue;
    match card {
        "ace" => {
            if cardvalue >= 11 {
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

fn revealdealer(dealercards: [String; 2], dealervalue: i32) {
    for dealercard in dealercards.iter() {
        println!("{}", dealercard);
    }
    println!("Dealers value = {}", dealervalue);
}

fn stand(cardvalue: i32, dealervalue: i32, bet: f64, mut money: f64) -> f64 {
    if dealervalue <= 21 && cardvalue < dealervalue {
        println!("Dealer wins!");
    } else {
        println!("You win!");
        money += bet * 2.0;
    }
    money
}

fn game(mut money: f64) -> f64 {
    println!("Your money: ${:.2}\n", money);
    println!("How much would you like to bet: ");
    let mut betstring = String::new();
    let mut bet: f64;

    loop {
        betstring.clear();
        io::stdin()
            .read_line(&mut betstring)
            .expect("Failed to read line");
        bet = match betstring.trim().parse::<f64>() {
            Ok(n) => n,
            Err(_) => 0.0,
        };
        match (bet * 100.0) as i64 {
            0 => {
                println!("Your bet cannot be $0.00");
                continue;
            }
            bet if bet > (money * 100.0) as i64 => {
                println!("Your bet is too large");
                println!("You currently have ${:.2}", money);
                continue;
            }
            _ => {
                println!("Bet set of ${:.2}\nLets begin!\n", bet);
                money -= bet;
                money = (money * 100.0).round() / 100.0;
                break;
            }
        }
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

    let dealercards = [
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
    println!("Dealer cards:\n\n???");
    println!("{}\n", dealercards[1]);

    println!("Your cards:\n");

    let mut dealervalue = 0;
    let mut cardvalue = 0;

    for fullcard in cards.iter() {
        println!("{}", fullcard);

        let card = fullcard.split(' ').last().unwrap();

        cardvalue = sortcard(card, cardvalue);
    }

    for fulldealercard in dealercards.iter() {
        let dealercard = fulldealercard.split(' ').last().unwrap();
        dealervalue = sortcard(dealercard, dealervalue)
    }

    println!("The value total value = {}\n", cardvalue);

    println!("What would you like to?\n");
    println!("1 Stand");
    println!("2 Hit");
    println!("3 Double bet and get 1 card");

    loop {
        let mut choosestring = String::new();
        io::stdin()
            .read_line(&mut choosestring)
            .expect("Failed to read line");
        let choose = match choosestring.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        };
        match choose {
            1 => {
                revealdealer(dealercards, dealervalue);
                money = stand(cardvalue, dealervalue, bet, money);
                break;
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
    money = (money * 100.0).round() / 100.0;
    money
}

fn help() {
    println!("\nYou place a bet and then you get 2 random cards and you decide if you want more cards.\nIf the dealer gets more than you then you lose, but when you have more than the dealer you win.\nIf you have more than 21 you also lose.\nWhenever you win you get twice the amount of your bet.\n");
}

fn main() {
    let mut continuescript = false;
    let mut money: f64 = 0.00;

    println!("Welcome to Black jack!\n");
    println!("Type help for instructions");
    println!("Type start to play and exit to quit the game");

    loop {
        let mut action = String::new();

        if continuescript == false {
            money += 50.00;
            io::stdin()
                .read_line(&mut action)
                .expect("Failed to read line");
            match action.trim().to_lowercase().as_str() {
                "start" => {
                    money = game(money);
                    continuescript = true;
                    println!("Your money: ${:.2}\n", money);
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
                "yes" | "y" | "ye" | "affirmative" => {
                    money = game(money);
                    println!("Your money: ${:.2}\n", money);
                }
                "no" | "n" | "negative" | "nein" => {
                    println!("Goodbye have a nice day!");
                    exit(0);
                }
                _ => {
                    println!("Invalid command");
                }
            }
        }
        if checkmoney(money) == true {
            println!("You lose! You ran out of money :(\n");
            continuescript = false;
            println!(
                "Type start to play again.\nExit to quit the game\nHelp to get game instructions."
            );
        }
    }
}
