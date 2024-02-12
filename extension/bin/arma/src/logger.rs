use arma_rs::Context;
use log::{LevelFilter, Metadata, Record};

use crate::CALLBACK_NAME;

/// Logger implementation for Arma, performs callback for each log message.
/// The callback passes the following data to Arma: `[target, level, args]`.
pub struct ArmaLogger {
    context: Context,
    level: LevelFilter,
}

impl ArmaLogger {
    /// Creates a new logger. Returns a boxed logger.
    #[must_use]
    pub fn new(context: Context, level: LevelFilter) -> Box<Self> {
        Box::new(Self { context, level })
    }

    /// Creates and Initializes the logger as the global logger.
    pub fn init(context: Context, level: LevelFilter) {
        let logger = Self::new(context, level);
        match log::set_boxed_logger(logger) {
            Ok(()) => log::set_max_level(level),
            Err(e) => error!("failed to initialize logger: {e}"),
        }
    }
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

impl simplelog::SharedLogger for ArmaLogger {
    fn level(&self) -> LevelFilter {
        self.level
    }

    fn config(&self) -> Option<&simplelog::Config> {
        None
    }

    fn as_log(self: Box<Self>) -> Box<dyn log::Log> {
        Box::new(*self)
    }
}
