use clap::Parser;

#[derive(Parser, Debug)]
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