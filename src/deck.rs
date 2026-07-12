// Data model: Deck, Card, CardStatus, Suit, Rank.
// Each deck contains 54 cards (52 standard + 2 Jokers).

use serde::{Deserialize, Serialize};

pub const TOTAL_CARDS: usize = 54;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
    Joker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rank {
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
    RedJoker,
    BlackJoker,
}

/// A single playing card (immutable reference data).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Card {
    /// Human-readable display string for the card.
    pub fn display(&self) -> String {
        match self.rank {
            Rank::RedJoker => "★ Red Joker".into(),
            Rank::BlackJoker => "☆ Black Joker".into(),
            _ => {
                let rank_str = match self.rank {
                    Rank::Ace => "A",
                    Rank::Two => "2",
                    Rank::Three => "3",
                    Rank::Four => "4",
                    Rank::Five => "5",
                    Rank::Six => "6",
                    Rank::Seven => "7",
                    Rank::Eight => "8",
                    Rank::Nine => "9",
                    Rank::Ten => "10",
                    Rank::Jack => "J",
                    Rank::Queen => "Q",
                    Rank::King => "K",
                    _ => unreachable!(),
                };
                let suit_str = match self.suit {
                    Suit::Spades => "♠",
                    Suit::Hearts => "♥",
                    Suit::Diamonds => "♦",
                    Suit::Clubs => "♣",
                    _ => unreachable!(),
                };
                format!("{}{}", rank_str, suit_str)
            }
        }
    }
}

/// The collection status of a single card.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CardStatus {
    pub collected: bool,
}

/// Generate the standard 54-card reference deck.
pub fn make_reference_cards() -> [Card; TOTAL_CARDS] {
    let mut cards = Vec::with_capacity(TOTAL_CARDS);
    for &suit in &[Suit::Spades, Suit::Hearts, Suit::Clubs, Suit::Diamonds] {
        for &rank in &[
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ] {
            cards.push(Card { suit, rank });
        }
    }
    cards.push(Card { suit: Suit::Joker, rank: Rank::RedJoker });
    cards.push(Card { suit: Suit::Joker, rank: Rank::BlackJoker });
    cards.try_into().expect("should be exactly 54 cards")
}

/// A named deck with collection status for all 54 cards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck {
    pub id: u64,
    pub name: String,
    pub cards: Vec<CardStatus>,
}

impl Deck {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            cards: vec![CardStatus { collected: false }; TOTAL_CARDS],
        }
    }

    pub fn total_cards(&self) -> usize {
        TOTAL_CARDS
    }

    pub fn collected_count(&self) -> usize {
        self.cards.iter().filter(|c| c.collected).count()
    }

    pub fn toggle_card(&mut self, index: usize) {
        if index < self.cards.len() {
            self.cards[index].collected = !self.cards[index].collected;
        }
    }

    pub fn collect_all(&mut self) {
        for c in self.cards.iter_mut() {
            c.collected = true;
        }
    }

    pub fn uncollect_all(&mut self) {
        for c in self.cards.iter_mut() {
            c.collected = false;
        }
    }

    pub fn is_all_collected(&self) -> bool {
        self.collected_count() == TOTAL_CARDS
    }
}
