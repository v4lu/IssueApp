use log::{set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::sync::Once;

static LOGGER: CustomLogger = CustomLogger;
static INIT: Once = Once::new();

struct CustomLogger;

impl Log for CustomLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let color = match record.level() {
                Level::Error => "\x1b[31m", // Red
                Level::Warn => "\x1b[33m",  // Yellow
                Level::Info => "\x1b[32m",  // Green
                Level::Debug => "\x1b[34m", // Blue
                Level::Trace => "\x1b[35m", // Purple
            };
            let reset = "\x1b[0m";
            println!(
                "{}{}{} [{}] - {}",
                color,
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                reset,
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn setup_custom_logger() -> Result<(), SetLoggerError> {
    INIT.call_once(|| {
        set_logger(&LOGGER).unwrap();
        set_max_level(LevelFilter::Info);
    });
    Ok(())
}
