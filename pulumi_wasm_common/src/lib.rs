use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

use crate::logger::Logger;
use pulumi_wasm_wit::bindings_logger;

pub use bindings_logger::*;

mod logger;

static IS_SET: AtomicBool = AtomicBool::new(false);
static LOGGER: Logger = Logger {};

pub fn setup_logger() {
    if IS_SET.swap(true, Relaxed) {
        return;
    }
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}
