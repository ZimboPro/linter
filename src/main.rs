use linter::{hn_main, oa_main};
use simplelog::{
    debug, info, warn, Color, ColorChoice, Config, ConfigBuilder, Level, LevelFilter, TermLogger,
    TerminalMode,
};

fn main() {
    println!("Hello, world!");
    let config = ConfigBuilder::new()
        .set_level_color(Level::Debug, Some(Color::Cyan))
        .set_level_color(Level::Info, Some(Color::Blue))
        .set_level_color(Level::Warn, Some(Color::Yellow))
        .set_level_color(Level::Error, Some(Color::Magenta))
        .set_level_color(Level::Trace, Some(Color::Green))
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(
        LevelFilter::Info,
        config,
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();

    // hn_main::main();
    oa_main::main();
}
