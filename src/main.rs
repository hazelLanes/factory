use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

fn main() {
    // Set up the logger so I may provide insight UwU
    TermLogger::init(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap();
}
