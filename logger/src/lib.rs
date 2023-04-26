#![allow(improper_ctypes_definitions)]
#![allow(dead_code)]

pub use log::{trace, debug, info, warn, error, LevelFilter};
use crate::structs::Logger;
use anyhow::{Context, Result};

#[cfg(feature = "templates")]
pub mod templates {
    use std::fs::File;
    use std::io::stdout;
    use std::path::Path;
    use log::LevelFilter;
    use crate::structs::Logger;
    use anyhow::{Context, Result};

    pub extern fn get_console_logger(max_level: LevelFilter) -> Logger {
        Logger::new(
            stdout(),
            max_level,
        )
    }

    pub extern fn get_file_logger<P: AsRef<Path>>(max_level: LevelFilter, path: P) -> Result<Logger> {
        let file = if path.as_ref().exists() {
            File::open(path)
        } else {
            File::create(path)
        }.context("Couldn´t open or crate file for file logger!")?;

        Ok(Logger::new(
            file,
            max_level
        ))
    }

    pub extern fn init_file_logger<P: AsRef<Path>>(max_level: LevelFilter, path: P) -> Result<()> {
        let logger = Box::new(get_file_logger(max_level, path)?);
        log::set_boxed_logger(logger).context("Failed to set logger!")?;
        log::set_max_level(max_level);
        Ok(())
    }

    pub extern fn init_console_logger(max_level: LevelFilter) -> Result<()> {
        let logger = Box::new(get_console_logger(max_level));
        log::set_boxed_logger(logger).context("Failed to set logger!")?;
        log::set_max_level(max_level);
        Ok(())
    }
}

pub extern fn init_custom_logger(logger: Logger) -> Result<()> {
    let max_level = logger.max_level;
    let logger = Box::new(logger);
    log::set_boxed_logger(logger).context("Failed to set logger!")?;
    log::set_max_level(max_level);
    Ok(())
}

pub mod structs {
    use std::io::{stderr, Write};
    use std::sync::{Arc, Mutex};
    use log::{LevelFilter, Log, Metadata, Record};

    pub struct Logger {
        pub(crate) max_level: LevelFilter,
        pub(crate) writer: Arc<Mutex<dyn Write + Send>>
    }

    impl Logger {
        pub fn new<W: Write + Send + 'static>(writer: W, max_level: LevelFilter) -> Self {
            Self {
                max_level,
                writer: Arc::new(Mutex::new(writer))
            }
        }
    }

    const PREFIX: &[&str; 5] = &["[ERROR]", " [WARN]", " [INFO]", "[DEBUG]", "[TRACE]"];
    const PREFIX_START_VALUE: usize = 1;

    impl Log for Logger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= self.max_level
        }

        fn log(&self, record: &Record) {
            if !self.enabled(record.metadata()) { return }

            let message: String = format!("{}: {}", PREFIX[record.metadata().level() as usize - PREFIX_START_VALUE], record.args());

            let mut writer = self.writer.lock().unwrap();
            let mut worked = writer.write_all(message.as_bytes()).is_ok();
            worked &= writer.write_all(b"\n").is_ok();
            if !worked {
                stderr().write_all(b"Failed to log to desired destination!").expect("FATAL: Failed to write to stderr!");
            }
        }
        fn flush(&self) {}
    }
}