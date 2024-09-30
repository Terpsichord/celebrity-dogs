use std::collections::VecDeque;
use std::ops::RangeInclusive;
use std::time::Duration;

use iced::widget::{button, column, row, text, Column, Space, container, Container, center};
use iced::{widget, Element, Length, Task};
use rand::seq::IteratorRandom;
use rand::{thread_rng, Rng};
use strum::{Display, IntoEnumIterator};

use crate::dog::{self, Dog};

pub enum Action {
    None,
    Task(Task<Message>),
    GameOver { winner: Player },
}

#[derive(Clone, Debug)]
pub enum Message {
    AttributeSelected(dog::Attribute),
    NextRound,
}

#[derive(Clone, Copy, Display, PartialEq)]
pub enum Player {
    User,
    Computer,
}

pub struct Game {
    user_pile: VecDeque<Dog>,
    computer_pile: VecDeque<Dog>,
    winner: Option<(Player, dog::Attribute)>,
    previous_winner: Player,
}

impl Game {
    const COMPUTER_THINK_TIME: RangeInclusive<Duration> =
        Duration::from_secs(3)..=Duration::from_secs(7);

    pub fn new(
        user_pile: impl IntoIterator<Item = Dog>,
        computer_pile: impl IntoIterator<Item = Dog>,
    ) -> (Self, Task<Message>) {
        (
            Self {
                user_pile: VecDeque::from_iter(user_pile),
                computer_pile: VecDeque::from_iter(computer_pile),
                // starts as Player::User as the user goes first
                previous_winner: Player::User,
                winner: None,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::AttributeSelected(attr) => {
                let winner = if self
                    .user_card()
                    .expect("user pile empty")
                    .cmp_attr(self.computer_card().expect("computer pile empty"), attr)
                    .is_ge()
                {
                    Player::User
                } else {
                    Player::Computer
                };
                self.winner = Some((winner, attr))
            }
            Message::NextRound => {
                self.previous_winner = self.winner.expect("expected winner to be Some").0;
                self.winner = None;

                let (winning_pile, losing_pile) = match self.previous_winner {
                    Player::User => (&mut self.user_pile, &mut self.computer_pile),
                    Player::Computer => (&mut self.computer_pile, &mut self.user_pile),
                };
                winning_pile.rotate_right(1);
                winning_pile.push_back(
                    losing_pile
                        .pop_front()
                        .expect("loser's pile is already empty"),
                );

                if let Some(winner) = self.overall_winner() {
                    return Action::GameOver { winner };
                }

                if self.previous_winner == Player::Computer {
                    return Action::Task(Task::future(Self::computer_choose_attr()));
                }
            }
        }
        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        let user_card: Element<Message> = match self.user_card() {
            Some(dog) => Self::view_card(dog, self.previous_winner == Player::User && self.winner.is_none()).into(),
            None => widget::text("User deck not loaded").into(),
        };

        let computer_card: Element<Message> = match self.winner.and(self.computer_card()) {
            Some(dog) => Self::view_card(dog, false).into(),
            None => Space::new(Length::Fill, Length::Fill).into(),
        };

        let cards = row![user_card, computer_card].spacing(60);

        let column = if let Some((winner, attr)) = self.winner {
            column![
                text(format!("The {} has better {}. {} wins!", winner.to_string().to_lowercase(), attr.to_string().to_lowercase(), winner)),
                cards,
                button("Next round").on_press(Message::NextRound),
            ]
        } else {
            let message = if self.previous_winner == Player::User {
                "Choose an attribute:"
            } else {
                "Computer is choosing an attribute..."
            };

            column![text(message), cards]
        };

        center(column).into()
    }

    async fn computer_choose_attr() -> Message {
        let computer_think_time = { thread_rng().gen_range(Self::COMPUTER_THINK_TIME) };
        async_std::task::sleep(computer_think_time).await;
        Message::AttributeSelected(dog::Attribute::iter().choose(&mut thread_rng()).unwrap())
    }

    fn view_card(dog: &Dog, user_choice: bool) -> Container<Message> {
        let attr_strings =
            dog::Attribute::iter().map(|attr| format!("{}: {}", attr, dog.get_attr(attr)));

        let column = Column::with_capacity(dog::Attribute::iter().len() + 1)
            .push(Element::from(text(dog.name())));

        let column = if user_choice {
            column.extend(attr_strings.zip(dog::Attribute::iter()).map(|(s, attr)| {
                button(text(s))
                    .style(button::text)
                    .on_press(Message::AttributeSelected(attr))
                    .into()
            }))
        } else {
            column.extend(attr_strings.map(text).map(Element::from))
        };

        container(column).width(Length::Fixed(300.0)).height(Length::Fixed(400.0)).style(container::bordered_box).padding(20)
    }

    fn user_card(&self) -> Option<&Dog> {
        self.user_pile.front()
    }

    fn computer_card(&self) -> Option<&Dog> {
        self.computer_pile.front()
    }

    fn overall_winner(&self) -> Option<Player> {
        if self.computer_pile.is_empty() {
            Some(Player::User)
        } else if self.user_pile.is_empty() {
            Some(Player::Computer)
        } else {
            None
        }
    }
}
