use std::io::{self, *};
use rand::seq::SliceRandom;

#[derive(Debug)]
struct Card {
    number: String,  // Number or Letter (R for Reverse, W for Wild, etc)
    color: Color,  // Color
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
}

impl Color {
    fn as_char(&self) -> char {
        match self {
            Color::Red => '🟥',
            Color::Blue => '🟦',
            Color::Yellow => '🟨',
            Color::Green => '🟩',
            Color::Black => '⬛',
        }
    }
}

struct Hand {
    cards: Vec<Card>
}

struct Player {
    name: String,
    hand: Hand
}

struct Game {
    players: Vec<Player>,
    current_card: Option<Card>,
    deck: Vec<Card>,
    current_player: usize,
}

impl Game {
    fn new(num_players: usize) -> Game {
        let mut players = Vec::new();
        for i in 0..num_players {
            let player = Player {
                name: format!("Player {}", i + 1),
                hand: Hand { cards: Vec::new() },
            };
            players.push(player);
        }
    
        let mut deck = Vec::new();
        Game::initialize_deck(&mut deck);
    
        let mut game = Game {
            players,
            current_card: None,  // Start the game with a card from the deck
            deck,
            current_player: 0,
        };
    
        game.shuffle_and_deal();
    
        game
    }    

    fn shuffle_and_deal(&mut self) {
        self.deck.shuffle(&mut rand::thread_rng());
        for player in &mut self.players {
            for _ in 0..7 {
                let card = self.deck.pop().unwrap();
                player.hand.cards.push(card);
            }
        }
        self.current_card = Some(self.deck.pop().unwrap());
    }

    fn initialize_deck(deck: &mut Vec<Card>) {
        deck.clear();
        let colors = [Color::Red, Color::Blue, Color::Yellow, Color::Green];
    
        for color in colors.iter() {
            deck.push(Card { number: "0".to_string(), color: *color});
            for num in 1..=9 {
                for _i in 0..2 {
                    deck.push(Card { number: num.to_string(), color: *color});
                }
            }
    
            for _i in 0..2 {
                deck.push(Card { number: "Draw_2".to_string(), color: *color});
                deck.push(Card { number: "Reverse".to_string(), color: *color});
                deck.push(Card { number: "Skip".to_string(), color: *color});
            }
            deck.push(Card { number: "Wild_+4".to_string(), color: Color::Black});
            deck.push(Card { number: "Wild".to_string(), color: Color::Black});
        }
        deck.shuffle(&mut rand::thread_rng());
    }

    #[allow(dead_code)]
    fn print_all_hands(&self) {
        for player in &self.players {
            println!("{}'s hand:", player.name);
            for card in &player.hand.cards {
                print!("|{}{}|  ", card.number, card.color.as_char());
            } println!();
            println!();  // Print an empty line for better readability
        }
    }

    fn next_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    fn print_current_hand(&self) {
        let player = &self.players[self.current_player];
        println!("{}'s hand:", player.name);
        for card in &player.hand.cards {
            print!("|{}{}|  ", card.number, card.color.as_char());
        }
        println!();
    }
       
}

fn main() {
    let num_players: usize = loop {
        print!("How many players? (2-10): ");
        io::stdout().flush().unwrap();

        match get_user_input().trim().parse() {
            Ok(value) => {
                if value >= 2 && value <= 10 {
                    break value;
                } println!("You can only have 2-10 players.");
            }
            Err(_) => println!("Invalid Input."),
        }
    };

    let mut game = Game::new(num_players);
    
    for _i in 0..num_players {
        println!();
        game.print_current_hand();
        game.next_turn();
        println!("Next Player");
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input
}