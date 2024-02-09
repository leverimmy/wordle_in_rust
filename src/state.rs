use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize)]
pub struct State {
    pub total_rounds: usize,
    pub games: Vec<Game>,
}

impl Default for State {
    fn default() -> Self {
        State {
            total_rounds: 0,
            games: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Game {
    pub answer: String,
    pub guesses: Vec<String>,
}

impl<'de> Deserialize<'de> for State {
    fn deserialize<D>(deserializer: D) -> Result<State, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            TotalRounds,
            Games,
        }

        struct StateVisitor;

        impl<'de> serde::de::Visitor<'de> for StateVisitor {
            type Value = State;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct State")
            }

            fn visit_map<V>(self, mut map: V) -> Result<State, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut total_rounds: Option<usize> = None;
                let mut games: Option<Vec<Game>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::TotalRounds => {
                            if total_rounds.is_some() {
                                return Err(serde::de::Error::duplicate_field("total_rounds"));
                            }
                            total_rounds = Some(map.next_value()?);
                        }
                        Field::Games => {
                            if games.is_some() {
                                return Err(serde::de::Error::duplicate_field("games"));
                            }
                            games = Some(map.next_value()?);
                        }
                    }
                }

                let total_rounds = total_rounds.unwrap_or_default();
                let games = games.unwrap_or_default();

                Ok(State {
                    total_rounds,
                    games,
                })
            }
        }

        deserializer.deserialize_map(StateVisitor)
    }
}
