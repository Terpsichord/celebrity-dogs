use crate::dog::Dog;
use async_std::{fs, io};
use iced::{
    widget::{button, checkbox, column, text, text_input},
    Element, Task,
};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, mem};
use serde::Deserialize;
use thiserror::Error;

#[derive(Default)]
pub struct Setup {
    deck: Deck,
    deck_size_input: String,
    invalid_input: bool,
    shuffle: bool,
}


#[derive(Deserialize, Clone, Debug, Default, PartialEq)]
pub(crate) struct Deck {
    pub(crate) dogs: Vec<Dog>,
}


pub enum Action {
    None,
    Error(Error),
    StartGame {
        user_pile: Vec<Dog>,
        computer_pile: Vec<Dog>,
    },
}

#[derive(Clone, Error, Debug)]
pub enum Error {
    #[error("failed to read deck file")]
    LoadDeck(#[from] Arc<io::Error>),
    #[error("invalid deck format {0}")]
    DeckFormat(#[from] toml::de::Error),
}

#[derive(Clone, Debug)]
pub enum Message {
    DeckLoaded(Result<Deck, Error>),
    InputChanged(String),
    InputSubmitted,
    ShuffleToggled(bool),
    StartPressed,
}

impl Setup {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self::default(),
            Task::perform(Self::load_deck(), Message::DeckLoaded),
        )
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::DeckLoaded(Ok(deck)) => self.deck = deck,
            Message::DeckLoaded(Err(error)) => return Action::Error(error),
            Message::InputChanged(input) => self.deck_size_input = input,
            Message::InputSubmitted => {
                self.invalid_input = self.deck_size_input.parse::<usize>().is_err()
            }
            Message::ShuffleToggled(toggled) => self.shuffle = toggled,
            Message::StartPressed => match self.deck_size_input.parse() {
                Ok(deck_size) => {
                    let (user_pile, computer_pile) =
                        Self::create_piles(mem::take(&mut self.deck.dogs), deck_size, self.shuffle);
                    return Action::StartGame {
                        user_pile,
                        computer_pile,
                    };
                }
                Err(_) => self.invalid_input = true,
            },
        }
        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        let content = column![
            text("How many cards in the deck would you like to play with?"),
            text_input("6", &self.deck_size_input)
                .on_input(Message::InputChanged)
                .on_submit(Message::InputSubmitted),
            checkbox("Shuffle deck?", self.shuffle).on_toggle(Message::ShuffleToggled),
            button("Start Game").on_press(Message::StartPressed),
        ];

        if self.invalid_input {
            content
                .push(
                    text("Invalid input. Please enter a valid number of cards.")
                        .color([255.0, 0.0, 0.0]),
                )
                .into()
        } else {
            content.into()
        }
    }

    fn create_piles(mut deck: Vec<Dog>, deck_size: usize, shuffle: bool) -> (Vec<Dog>, Vec<Dog>) {
        dbg!(&deck);
        if shuffle {
            deck.partial_shuffle(&mut thread_rng(), deck_size);
        }
        // Split the deck in 2
        let new_pile = deck.split_off(deck_size / 2);
        (deck, new_pile)
    }

    async fn load_deck() -> Result<Deck, Error> {
        let contents = fs::read_to_string(Self::deck_path())
            .await
            .map_err(Arc::new)?;

        let deck = toml::from_str(&contents)?;

        Ok(deck)
    }

    pub fn deck_path() -> PathBuf {
        env::current_dir().unwrap_or_default().join("dogs.toml")
    }
}

mod tests {
    use super::*;


    #[test]
    fn parse_deck_str() {
        let deck_str = "\
[[dogs]]
name = \"Annie the Afghan Hound\"
exercise = 4
intelligence = 15
friendliness = 6
drool = 1


[[dogs]]
name = \"Bertie the Boxer\"
exercise = 5
intelligence = 60
friendliness = 9
drool = 3";
        let deck: Result<Deck, _> = toml::from_str(deck_str);
        assert!(deck.is_ok());
    }

    #[test]
    fn parse_deck_from_path() {
        let contents = std::fs::read_to_string(Setup::deck_path()).expect(&format!("failed to read from deck path {:?}", Setup::deck_path()));
        let _: Deck = toml::from_str(&contents).expect("failed to parse deck");
    }
}
