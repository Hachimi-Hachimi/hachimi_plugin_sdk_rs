use std::ffi::CString;

use log::{Level, Log, Metadata, Record, SetLoggerError};

use crate::api::HachimiApi;

pub struct HachimiLogger {
    api: HachimiApi,
    pub level: Level
}

impl Log for HachimiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let target = if record.target().len() > 0 {
            record.target()
        }
        else {
            record.module_path().unwrap_or_default()
        };
        let message = record.args().to_string();

        let Ok(target_c_str) = CString::new(target) else {
            return;
        };
        let Ok(message_c_str) = CString::new(message) else {
            return;
        };
        // log level values are compatible
        let level = record.metadata().level() as usize as i32;

        unsafe {
            (self.api.vtable().log)(level, target_c_str.as_ptr(), message_c_str.as_ptr());
        }
    }

    fn flush(&self) {}
}

pub fn init(api: HachimiApi, level: Level) -> Result<(), SetLoggerError> {
    let logger = HachimiLogger { api, level };
    match log::set_boxed_logger(Box::new(logger)) {
        Ok(()) => {
            log::set_max_level(level.to_level_filter());
            Ok(())
        }
        Err(e) => Err(e),
    }
}