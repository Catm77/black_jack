use std::{io, time::Duration, thread};
use rand::Rng;

#[derive(Debug, Clone)]
enum Suit
{
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[derive(Debug, Clone)]
enum Value
{
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value
{
    fn value(&self) -> u32
    {
        match self
        {
               Value::Ace => 11,
               Value::Two => 2,
               Value::Three => 3,
               Value::Four => 4,
               Value::Five => 5,
               Value::Six => 6,
               Value::Seven => 7,
               Value::Eight => 8,
               Value::Nine => 9,
               Value::Ten | Value::Jack | Value::Queen | Value::King => 10,
        }
    }
}

#[derive(Debug, Clone)]
struct Card
{
    suit: Suit,
    value: Value,
}

impl Card 
{
    fn new() -> Self
    {
        let mut rng = rand::rng();
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let values = [Value::Ace, Value::Two, Value::Three, Value::Four, Value::Five,
        Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen,
        Value::King];

        let suit = suits[rng.random_range(0..suits.len())].clone();
        let value = values[rng.random_range(0..values.len())].clone();

        Card {suit, value}
    }
}

fn dealer_cards() -> u32
{
    let mut cards: Vec<Card> = Vec::new();
    let mut total = 0;
    let mut aces = 0;

    for _ in 0..2 
    {
        let card = Card::new();

        cards.push(card.clone());
        total += card.value.value();

        if let Value::Ace = card.value 
        {
            aces += 1;
        }
    }

    while total > 21 && aces > 0 
    {
        total -= 10;
        aces -= 1;
    }

    while total < 17
    {
        let card = Card::new();

        cards.push(card.clone());
        total += card.value.value();

        if let Value::Ace = card.value 
        {
            aces += 1;
        }

        while total > 21 && aces > 0 
        {
            total -= 10;
            aces -= 1;
        }
    }

    total
}

fn player_cards() -> u32
{
    let mut cards: Vec<Card> = Vec::new();
    let mut total = 0;
    let mut aces = 0;

    for _ in 0..2 
    {
        let card = Card::new();

        cards.push(card.clone());
        total += card.value.value();

        if let Value::Ace = card.value 
        {
            aces += 1;
        }
    }

    while total > 21 && aces > 0 
    {
        total -= 10;
        aces -= 1;
    }

    loop 
    {
        println!("Your cards are: {:?}", cards);
        println!("Current total: {}", total);

        let mut card_input:String = String::new();
        
        println!("Do you want to hit or stand?");

        io::stdin()
        .read_line(&mut card_input)
        .expect("Failed to read line");

        let card_input = card_input.to_lowercase();

        let card_input = card_input.trim();

        if card_input == "stand"
        {
            break;
        }
        else if card_input == "hit" 
        {
            let card = Card::new();
            cards.push(card.clone());
            total += card.value.value();

            if let Value::Ace = card.value 
            {
                aces += 1;
            }

            while total > 21 && aces > 0 
            {
                total -= 10;
                aces -= 1;
            }

            if total > 21 
            {
                println!("Bust! Total: {}", total);
                break;
            }
        }
        else
        {
            println!("Invalid input, input hit or stand");
        }
    }

    total
}

fn game_loop()
{
    println!("Starting game . . .");
    thread::sleep(Duration::from_secs(2));
    
    let mut cash:i32 = 1000;

    let mut wins:i32 = 0;
    let mut loses:i32 = 0;

    println!("Welcome to the CATsino!");

    'game_menu: loop
    {
        println!("Current cash: {}", cash);

        let mut game_menu_input:String = String::new();

        println!("What would you like to do? \n\
        1.Play a round of BlackJack. \n\
        2.Check your scores. \n\
        3.Exit the Catsino");

        io::stdin()
        .read_line(&mut game_menu_input)
        .expect("Failed to read line");

        let game_menu_input = game_menu_input.trim();

        match game_menu_input
        {
            "1" => 
            {
                println!("Starting Match");

                thread::sleep(Duration::from_secs(2));

                'game_loop: loop
                {
                    let mut bet_amount:String = String::new();

                    println!("Place your bet (Integers only)");

                    io::stdin()
                    .read_line(&mut bet_amount)
                    .expect("Failed to read line");

                    let bet_amount:i32 = bet_amount.parse::<i32>().unwrap();

                    let player_card_result:u32 = player_cards();

                    //player card loop ends

                    //dealers turn logic



                    //dealers turn logic ends

                    //win or lose calculation

                    //win or lose calculation ends

                    let mut continue_input:String = String::new();

                    println!("What do you want to do? \n\
                    1.Continue playing. \n\
                    2.Stop playing.");

                    io::stdin()
                    .read_line(&mut continue_input)
                    .expect("Failed to read line");

                    let continue_input = continue_input.trim();

                    if continue_input == "1"
                    {
                        continue;
                    }
                    else if continue_input == "2"
                    {
                        break 'game_loop;
                    }
                    else
                    {
                        println!("Invalid input continuing automatically");

                        thread::sleep(Duration::from_secs(1));
                    }
                }
            }
            "2" => 
            {
                println!("Scores: \n\
                ========== \n\
                Wins: {} \n\
                Loses: {} \n\
                ==========",
                wins,
                loses);

                thread::sleep(Duration::from_secs(10));
            }
            "3" => 
            {
                break 'game_menu;
            }
            _ => 
            {
                println!("Invalid input please try again (Tip input the number of the action)");
            }
        }
    }
}

fn main() 
{
    println!("Welcome to Black Jack gambler!");
    thread::sleep(Duration::from_secs(1));

    loop 
    {
        let mut start_input: String = String::new();

        println!("Would you like to start? \n\
        1.Start \n\
        2.Exit");

        io::stdin()
        .read_line(&mut start_input)
        .expect("Failed to read line");

        let start_input = start_input.trim();

        match start_input 
        {
            "1" => 
            {
                game_loop();
            }

            "2" => 
            {
                break;
            }

            _ => 
            {
                println!("Invalid input please try again.(Input the number of the action)");
            }
        }
    }
}
