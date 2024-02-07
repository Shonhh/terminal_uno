use std::io::{self, Write};
use std::fmt;
use rand::seq::SliceRandom;
use eframe::{egui::{self, RichText}, epaint::Color32};

#[derive(Debug)]
struct Card {
    number: String,
    color: Color,
}

struct Deck { cards: Vec<Card> }

impl Deck {
    fn new() -> Deck { 
        let mut deck = Deck { cards: Vec::new() };
        deck.initialize();
        deck.shuffle();
        deck
    }

    fn initialize(&mut self) {
        self.cards.clear();
        let colors = [Color::Red, Color::Blue, Color::Yellow, Color::Green];

        for color in colors.iter() {
            self.cards.push(Card { number: "0".to_string(), color: *color});
            for num in 1..=9 {
                self.cards.extend((0..2).map(|_| Card { number: num.to_string(), color: *color}));
            }

            self.cards.extend((0..2).map(|_| Card { number: "Draw_2".to_string(), color: *color}));
            self.cards.extend((0..2).map(|_| Card { number: "Reverse".to_string(), color: *color}));
            self.cards.extend((0..2).map(|_| Card { number: "Skip".to_string(), color: *color}));

            self.cards.push(Card { number: "Wild_+4".to_string(), color: Color::Black});
            self.cards.push(Card { number: "Wild".to_string(), color: Color::Black});
        }
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut rand::thread_rng());
    }

    fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        self.cards.drain(..num_cards).collect()
    }
}

#[derive(Debug, Copy, Clone)]
enum Color {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
}

#[derive(Debug)]
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
    deck: Deck,
    current_player: usize,
}

impl Game {
    fn new(num_players: usize) -> Game {
        let players = (0..num_players).map(|i| Player {
            name: format!("Player {}", i + 1),
            hand: Hand { cards: Vec::new() },
        }).collect();

        let mut deck = Deck::new();

        let mut game = Game {
            players,
            current_card: None,
            deck,
            current_player: 0,
        };

        game.deal_cards();
        game
    }    

    fn deal_cards(&mut self) {
        for player in &mut self.players {
            player.hand = Hand { cards: self.deck.deal(7) };
        }
    }

    // fn shuffle_deck_from_discard() {}

    fn next_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }
}

fn main() -> eframe::Result<()> {
    env_logger::init(); 

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([380.0, 262.0]),
        ..Default::default()
    };

    let num_players = get_num_players();

    let mut game = Game::new(num_players);
    
    println!("Next Player");

    eframe::run_simple_native("Uno", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is Uno");
            ui.vertical(|ui| {
                for player_index in 0..game.players.len() {
                    ui.horizontal(|ui| {
                        let hand = get_current_hand(&game);
                        let player = &game.players[player_index];
                        display_player_hand(ui, player, &hand);
                    });
                    game.next_turn();
                } 
            })
        });
    })
}

fn display_player_hand(ui: &mut egui::Ui, player: &Player, hand: &[String]) {
    ui.label(format!("{}:", player.name));
    for (i, card) in hand.iter().enumerate() {
        let card_color = match player.hand.cards[i].color {
            Color::Red => Color32::WHITE,
            Color::Blue => Color32::WHITE,
            Color::Yellow => Color32::BLACK,
            Color::Green => Color32::BLACK,
            Color::Black => Color32::WHITE,
        };

        let color = match player.hand.cards[i].color {
            Color::Red => Color32::RED,
            Color::Blue => Color32::BLUE,
            Color::Yellow => Color32::YELLOW,
            Color::Green => Color32::GREEN,
            Color::Black => Color32::BLACK,
        };

        ui.label(
            RichText::new(card)
                .background_color(color)
                .color(card_color)
        );
    }                
}

fn get_current_hand(game: &Game) -> Vec<String> {
    let player = &game.players[game.current_player];
    player.hand.cards.iter().map(|card| format!("|{}|", card.number)).collect()
}

fn get_num_players() -> usize {
    loop {
        print!("How many players? (2-10): ");
        io::stdout().flush().unwrap();

        match get_user_input().trim().parse() {
            Ok(value) => {
                if value >= 2 && value <= 10 {
                    break value;
                } 
                println!("You can only have 2-10 players.");
            }
            Err(_) => println!("Invalid Input."),
        }
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input
}