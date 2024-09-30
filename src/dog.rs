use serde::Deserialize;
use std::cmp;
use strum::{Display, EnumIter};

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct Dog {
    name: String,
    exercise: u8,
    intelligence: u8,
    friendliness: u8,
    drool: u8,
}

#[derive(Clone, Copy, PartialEq, Debug, EnumIter, Display)]
pub enum Attribute {
    Exercise,
    Intelligence,
    Friendliness,
    Drool,
}

impl Dog {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_attr(&self, attr: Attribute) -> u8 {
        match attr {
            Attribute::Exercise => self.exercise,
            Attribute::Intelligence => self.intelligence,
            Attribute::Friendliness => self.friendliness,
            Attribute::Drool => self.drool,
        }
    }

    pub fn cmp_attr(&self, other: &Dog, attr: Attribute) -> cmp::Ordering {
        if attr == Attribute::Drool {
            // Drool is reversed as the player with the lower drool wins
            self.drool.cmp(&other.drool).reverse()
        } else {
            self.get_attr(attr).cmp(&other.get_attr(attr))
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn parse_dog_toml() {
        let toml_str = "
name = \"Annie the Afghan Hound\"
exercise = 4
intelligence = 15
friendliness = 6
drool = 1";
        let dog = Dog {
            name: "Annie the Afghan Hound".to_string(),
            exercise: 4,
            intelligence: 15,
            friendliness: 6,
            drool: 1,
        };
        assert_eq!(Ok(dog), toml::from_str(toml_str));
    }

    #[test]
    fn parse_deck() {
        let dogs_toml = "\
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
        let deck = toml::from_str(dogs_toml);

        assert_eq!(Ok(crate::setup::Deck {
            dogs: vec![
                Dog {
                    name: "Annie the Afghan Hound".to_string(),
                    exercise: 4,
                    intelligence: 15,
                    friendliness: 6,
                    drool: 1,
                },
                Dog {
                    name: "Bertie the Boxer".to_string(),
                    exercise: 5,
                    intelligence: 60,
                    friendliness: 9,
                    drool: 3,
                },
            ]
        }), deck);
    }

    #[test]
    fn parse_deck_into_vec() {
        let dogs_toml = "\
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
        let parse_result: Result<Vec<Dog>, _> = toml::from_str(dogs_toml);
        dbg!(&parse_result);

        assert!(parse_result.is_err());
    }
}
