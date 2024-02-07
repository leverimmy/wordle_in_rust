use clap::Parser;
use serde::Deserialize;

#[derive(Parser, Debug, Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'w', long = "word", conflicts_with = "random", group = "word_mode")]
    pub word: Option<String>,

    #[arg(short = 'r', long = "random", group = "random_mode", default_value_t = false)]
    pub random: bool,

    #[arg(short = 'D', long = "difficult", default_value_t = false)]
    pub difficult: bool,

    #[arg(short = 't', long = "stats", default_value_t = false)]
    pub stats: bool,

    #[arg(short = 'd', long = "day", conflicts_with = "word")]
    pub day: Option<usize>,

    #[arg(short = 's', long = "seed", conflicts_with = "word")]
    pub seed: Option<u64>,

    #[arg(short = 'f', long = "final-set")]
    pub final_set: Option<String>,

    #[arg(short = 'a', long = "acceptable-set")]
    pub acceptable_set: Option<String>,

    #[arg(short = 'S', long = "state")]
    pub state: Option<String>,

    #[arg(short = 'c', long = "config")]
    pub config: Option<String>,
}

impl Cli {
    pub fn merge(&mut self, from: Cli) {
        if self.word == None {
            self.word = from.word;
        }
        if self.random == false {
            self.random = from.random;
        }
        if self.difficult == false {
            self.difficult = from.difficult;
        }
        if self.stats == false {
            self.stats = from.stats;
        }
        if self.day == None {
            self.day = from.day;
        }
        if self.seed == None {
            self.seed = from.seed;
        }
        if self.final_set == None {
            self.final_set = from.final_set;
        }
        if self.acceptable_set == None {
            self.acceptable_set = from.acceptable_set;
        }
        if self.state == None {
            self.state = from.state;
        }
        if self.config == None {
            self.config = from.config;
        }
    }
}