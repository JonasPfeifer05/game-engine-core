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
            vec![Box::new(stdout())],
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
            vec![Box::new(file)],
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
        pub(crate) writers: Arc<Mutex<Vec<Box<(dyn Write + Send + 'static)>>>>
    }

    impl Logger {
        pub fn new(writers: Vec<Box<(dyn Write + Send + 'static)>>, max_level: LevelFilter) -> Self {
            Self {
                max_level,
                writers: Arc::new(Mutex::new(writers))
            }
        }

        pub fn add_writer(&mut self, writer: Box<(dyn Write + Send + 'static)>) {
            self.writers.lock().unwrap().push(writer);
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

            let mut writers = self.writers.lock().unwrap();
            writers.iter_mut().for_each(|writer| {
                let mut worked = writer.write_all(message.as_bytes()).is_ok();
                worked &= writer.write_all(b"\n").is_ok();
                if !worked {
                    stderr().write_all(b"Failed to log to desired destination!").expect("FATAL: Failed to write to stderr!");
                }
            });
        }
        fn flush(&self) {}
    }
}