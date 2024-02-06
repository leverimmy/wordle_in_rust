use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'w', long = "word")]
    word: Option<String>,

    #[arg(short = 'r', long = "random")]
    random: Option<bool>,

    #[arg(short = 'D', long = "difficult")]
    difficult: Option<bool>,

    #[arg(short = 't', long = "stats")]
    stats: Option<bool>,

    #[arg(short = 'd', long = "day")]
    day: Option<u32>,

    #[arg(short = 's', long = "seed")]
    seed: Option<u64>,

    #[arg(short = 'f', long = "final-set")]
    final_set: Option<String>,

    #[arg(short = 'a', long = "acceptable-set")]
    acceptable_set: Option<String>,

    #[arg(short = 'S', long = "state")]
    state: Option<String>,

    #[arg(short = 'c', long = "config")]
    config: Option<String>,
}

pub struct Config {
    pub random: bool,
    difficult: bool,
    stats: bool,
    pub day: u32,
    pub seed: u64,
    final_set: String,
    acceptable_set: String,
    state: String,
    pub word: String
}

impl Cli {
    pub fn get(&self) -> Config {
        let config = Config { word: (
            match &self.word {
                Some(word) => word.to_string(),
                None => "".to_string(),
            }
        ), random: (
            match self.random {
                Some(flag) => flag,
                None => false,
            }
        ), difficult: (
            match self.difficult {
                Some(flag) => flag,
                None => false
            }
        ), stats: (
            match self.stats {
                Some(flag) => flag,
                None => false,
            }
        ), day: (
            match self.day {
                Some(day) => day,
                None => 1,
            }
        ), seed: (
            match self.seed {
                Some(seed) => seed,
                None => 19260817,
            }
        ), final_set: (
            match &self.final_set {
                Some(dir) => dir.to_string(),
                None => "./final_set.txt".to_string(),
            }
        ), acceptable_set: (
            match &self.acceptable_set {
                Some(dir) => dir.to_string(),
                None => "./acceptable_set.txt".to_string(),
            }
        ), state: (
            match &self.state {
                Some(dir) => dir.to_string(),
                None => "./state.json".to_string(),
            }
        ) };
        assert!(true/* day in range */);
        config
    }
}