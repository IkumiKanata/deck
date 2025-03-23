use rand::{rngs::ThreadRng, seq::SliceRandom};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Deck {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let values = [
            "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace",
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = ThreadRng::default();
        let mut cards = self.cards.clone();
        cards.shuffle(&mut rng);
        self.cards = cards;
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    println!("Here is your deck: {:#?}", deck);
}
