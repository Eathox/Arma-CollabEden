#![deny(clippy::all, clippy::nursery)]
#![warn(clippy::pedantic, clippy::unwrap_used)]
#![allow(clippy::similar_names, clippy::module_name_repetitions)]
#![warn(missing_docs)]

//! Arma 3 extension used for networking in Eden Editor.

#[allow(unused_imports)]
#[macro_use]
extern crate log;

mod callback;
mod logger;

use callback::CALLBACK_NAME;
pub use logger::ArmaLogger;

use arma_rs::{arma, Context, Extension};
use simplelog::{ColorChoice, CombinedLogger, TermLogger, TerminalMode};

#[arma]
fn init() -> Extension {
    let ext = Extension::build()
        .version(std::env!("CARGO_PKG_VERSION").to_string())
        .freeze_state()
        .finish();
    init_logger(ext.context());

    std::thread::spawn(|| loop {
        debug!("Test");
        std::thread::sleep(std::time::Duration::from_millis(1000));
    });
    ext
}

#[allow(clippy::vec_init_then_push)]
fn init_logger(ctx: Context) {
    let log_level = if cfg!(debug_assertions) {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Warn
    };

    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();
    loggers.push(ArmaLogger::new(ctx, log_level));

    #[cfg(debug_assertions)]
    loggers.push(TermLogger::new(
        log_level,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Never,
    ));

    CombinedLogger::init(loggers)
        .expect("Logger initialization should only happen on extension creation");
}
