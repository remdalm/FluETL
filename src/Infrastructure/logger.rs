use std::env;

use log::LevelFilter;

use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::{
    roll::fixed_window::FixedWindowRoller, trigger::size::SizeTrigger,
};
use log4rs::append::rolling_file::RollingFileAppender;

use log4rs::encode::pattern::PatternEncoder;

use log4rs::config::{Appender, Root};
use log4rs::Config;

use super::InfrastructureError;

fn make() -> Result<Config, InfrastructureError> {
    let log_directory =
        env::var("LOG_DIRECTORY").map_err(InfrastructureError::EnvVarError)?;

    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".to_string());

    let log_line_pattern = "{d(%Y-%m-%d %H:%M:%S)} | {({l}):5.5} | {f}:{L} — {m}{n}";

    let trigger_size = 2000000_u64; //2MB
    let trigger = Box::new(SizeTrigger::new(trigger_size));

    let roller_pattern_path = format!("{}/step_{}.gz", log_directory, "{}");
    let roller_pattern = roller_pattern_path.as_str();
    let roller_count = 10;
    let roller_base = 1;
    let roller = Box::new(
        FixedWindowRoller::builder()
            .base(roller_base)
            .build(roller_pattern, roller_count)
            .unwrap(),
    );

    let compound_policy = Box::new(CompoundPolicy::new(trigger, roller));

    let file_logger = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(log_line_pattern)))
        .build(format!("{}/step.log", log_directory), compound_policy)
        .unwrap();

    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file_logger", Box::new(file_logger)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file_logger")
                .build(log_level_from_string(log_level)),
        )
        .unwrap();
    Ok(config)
}

pub fn init() {
    let make_result = make();
    if make_result.is_err() {
        println!("Failed to initialize logger: {:?}", make_result);
        return;
    }
    log4rs::init_config(make_result.unwrap()).unwrap();
}

fn log_level_from_string(level: String) -> LevelFilter {
    match level.as_str() {
        "TRACE" => LevelFilter::Trace,
        "DEBUG" => LevelFilter::Debug,
        "INFO" => LevelFilter::Info,
        "WARN" => LevelFilter::Warn,
        "ERROR" => LevelFilter::Error,
        _ => LevelFilter::Info,
    }
}
