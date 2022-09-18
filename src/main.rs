use log::{LevelFilter, SetLoggerError};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode, ConfigBuilder};

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    setup_logging().expect("The logger couldn't be setup.");
    log::info!("started djfy.");



    Ok(())
}

fn setup_logging() -> Result<(), SetLoggerError> {
    let level_filter = LevelFilter::Debug;

    let config = ConfigBuilder::new()
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        level_filter,
        config,
        TerminalMode::Mixed,
        ColorChoice::Always,
    )])
}