use clap::{AppSettings, Clap};

const BUZZ_VERSION: &str = "0.0.1";
const BUZZ_AUTHOR: &str = "Bugout (engineering@bugout.dev)";

/// Runs a given set of patterns on a stream of tags and only prints the matches
#[derive(Clap)]
#[clap(version = BUZZ_VERSION, author = BUZZ_AUTHOR)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Args {
    /// Path to file containing buzz patterns to match in the input stream
    #[clap(short, long)]
    patterns: String,
    tags: Vec<String>,
}

fn main() {
    let args: Args = Args::parse();
    println!("Patterns: {}, tags: {:?}", args.patterns, args.tags);
}