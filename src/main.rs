fn help() {
    println!("You place a bet and then you get 2 random cards and you decide if you want more cards.\nIf the dealer gets more than you then you lose, but when you have more than the dealer you win.\nIf you have more than 21 you also lose.\nWhenever you win you get twice the amount of your bet.\n");
}

fn main() {
    println!("Welcome to Black jack!\n");
    println!("Type help for instructions");
    println!("Type start to start and exit to exit\n");
    
    // loop {
        let money = 50.00;
        println!("Your money: ${:.2}\n", money);
        println!("How much would you like to bet: ");

        if money == 0.00 {
            println!("Game over, You ran out of money");
        }
    }
// }