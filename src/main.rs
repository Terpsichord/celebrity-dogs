mod dog;
mod game;
mod setup;

use crate::game::Game;
use crate::setup::Setup;
use iced::widget::{text, button, column};
use iced::{Element, Task};

pub fn main() -> iced::Result {
    iced::application(App::title, App::update, App::view)
        .run_with(App::new)
}

enum View {
    Menu,
    Error(ErrorScreen),
    Setup(Setup),
    Game(Game),
}

struct App {
    view: View,
}

struct ErrorScreen {
    display_message: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug)]
enum Message {
    SetupMessage(setup::Message),
    GameMessage(game::Message),
    PlayPressed,
    Quit,
}

// enum Window {
//     Main,
//     Error,
//     Unknown,
// }

// #[derive(Clone, Debug)]
// struct ErrorWindow {
//     id: window::Id,
//     display_message: String,
//     // TODO: add Vec<Buttons> for possible actions in response to the error
// }

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                view: View::Menu,
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self.view {
            View::Menu => "In Menu",
            View::Game(_) | View::Setup(_) => "Playing",
            View::Error(_) => "Something went wrong"
        };
        format!("Celebrity Dogs - {}", subtitle)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetupMessage(setup_message) => self.update_setup(setup_message),
            Message::GameMessage(game_message) => self.update_game(game_message),
            Message::PlayPressed => {
                let (setup, task) = Setup::new();
                self.view = View::Setup(setup);
                task.map(Message::SetupMessage)
            }
            Message::Quit => iced::exit(),
        }
    }

    fn update_setup(&mut self, message: setup::Message) -> Task<Message> {
        use setup::Action;

        if let View::Setup(setup) = &mut self.view {
            match setup.update(message) {
                Action::None => {},
                Action::Error(error) => {
                    let error_screen = ErrorScreen {
                        // TODO: make error message more human friendly
                        display_message: error.to_string(),
                    };
                    self.view = View::Error(error_screen);
                },
                Action::StartGame {
                    user_pile,
                    computer_pile,
                } => {
                    let (game, task) = Game::new(user_pile, computer_pile);
                    self.view = View::Game(game);

                    return task.map(Message::GameMessage)
                }
            };
        }
        Task::none()
    }

    fn update_game(&mut self, message: game::Message) -> Task<Message> {
        use game::Action;

        if let View::Game(game) = &mut self.view {
            return match game.update(message) {
                Action::None => Task::none(),
                Action::Task(task) => task.map(Message::GameMessage),
                Action::GameOver { winner } => {
                    todo!("game over, {} won", winner)
                }
            };
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        match &self.view {
            View::Menu => column![
                button("Play").on_press(Message::PlayPressed),
                button("Quit").on_press(Message::Quit),
            ]
                .into(),
            View::Error(error) => column![
                text(&error.display_message),
                button("Ok").on_press(Message::Quit),
            ].into(),
            View::Setup(setup) => setup.view().map(Message::SetupMessage),
            View::Game(game) => game.view().map(Message::GameMessage),
        }
    }
}
