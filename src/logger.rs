use arma_rs::Context;
use log::{Level, Metadata, Record};

use crate::CALLBACK_NAME;

/// Logger implementation for Arma, performs callback for each log message
pub struct ArmaLogger {
    level: Level,
    context: Context,
}

impl log::Log for ArmaLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let _ = self.context.callback_data(
                CALLBACK_NAME,
                "log",
                vec![
                    record.target().to_string(),
                    record.level().to_string(),
                    record.args().to_string(),
                ],
            );
        }
    }

    fn flush(&self) {}
}

/// Initializes the logger for Arma
pub fn init(context: Context, level: Level) {
    let logger = ArmaLogger { level, context };
    match log::set_boxed_logger(Box::new(logger)) {
        Ok(_) => log::set_max_level(level.to_level_filter()),
        Err(e) => error!("failed to initialize logger: {e}"),
    }
}
