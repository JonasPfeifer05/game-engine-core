#![allow(improper_ctypes_definitions)]
#![allow(dead_code)]

use std::path::Path;
pub use log::{trace, debug, info, warn, error, LevelFilter};
use crate::structs::Logger;
use anyhow::{Context, Result};

#[cfg(feature = "templates")]
pub mod templates {
    use std::fs::File;
    use std::io::stdout;
    use std::path::Path;
    use log::{LevelFilter};
    use crate::structs::Logger;
    use anyhow::{Context, Result};

    impl Logger {
        pub extern fn simple_console_logger(max_level: LevelFilter) -> Logger {
            Logger::new(
                vec![(max_level, Box::new(stdout()))],
                max_level,
            )
        }

        pub extern fn simple_file_logger<P: AsRef<Path>>(max_level: LevelFilter, path: P) -> Result<Logger> {
            let file = if path.as_ref().exists() {
                File::open(path)
            } else {
                File::create(path)
            }.context("Couldn´t open or crate file for file logger!")?;

            Ok(Logger::new(
                vec![(max_level, Box::new(file))],
                max_level
            ))
        }

        pub extern fn extended_console_logger(master_max_level: LevelFilter, console_max_level: LevelFilter) -> Logger {
            Logger::new(
                vec![(console_max_level, Box::new(stdout()))],
                master_max_level,
            )
        }

        pub extern fn extended_file_logger<P: AsRef<Path>>(master_max_level: LevelFilter, file_max_level: LevelFilter, path: P) -> Result<Logger> {
            let file = if path.as_ref().exists() {
                File::open(path)
            } else {
                File::create(path)
            }.context("Couldn´t open or crate file for file logger!")?;

            Ok(Logger::new(
                vec![(file_max_level, Box::new(file))],
                master_max_level
            ))
        }
    }

    impl Logger {
        pub extern fn add_console_writer(&mut self, max_level: LevelFilter) {
            self.add_general_writer(
                (max_level, Box::new(stdout()))
            );
        }

        pub extern fn add_file_writer<P: AsRef<Path>>(&mut self, max_level: LevelFilter, path: P) -> Result<()> {
            let file = if path.as_ref().exists() {
                File::open(path)
            } else {
                File::create(path)
            }.context("Couldn´t open or crate file for file logger!")?;

            self.add_general_writer((max_level, Box::new(file)));

            Ok(())
        }
    }
}

#[cfg(feature = "templates")]
pub extern fn init_simple_file_logger<P: AsRef<Path>>(max_level: LevelFilter, path: P) -> Result<()> {
    let logger = Box::new(Logger::simple_file_logger(max_level, path)?);
    log::set_boxed_logger(logger).context("Failed to set logger!")?;
    log::set_max_level(max_level);
    Ok(())
}

#[cfg(feature = "templates")]
pub extern fn init_simple_console_logger(max_level: LevelFilter) -> Result<()> {
    let logger = Box::new(Logger::simple_console_logger(max_level));
    log::set_boxed_logger(logger).context("Failed to set logger!")?;
    log::set_max_level(max_level);
    Ok(())
}

pub extern fn init_general_logger(logger: Logger) -> Result<()> {
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

    type LogWriter = (LevelFilter, Box<(dyn Write + Send + 'static)>);

    pub struct Logger {
        pub(crate) max_level: LevelFilter,
        pub(crate) writers: Arc<Mutex<Vec<LogWriter>>>
    }

    impl Logger {
        pub extern fn new(writers: Vec<LogWriter>, master_max_level: LevelFilter) -> Self {
            Self {
                max_level: master_max_level,
                writers: Arc::new(Mutex::new(writers))
            }
        }

        pub extern fn add_general_writer(&mut self, writer: LogWriter) {
            self.writers.lock().unwrap().push(writer);
        }

        pub(crate) fn individual_enabled(&self, metadata: &Metadata, max_level: LevelFilter) -> bool {
            metadata.level() <= max_level
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
                if !self.individual_enabled(record.metadata(), writer.0) { return }

                let mut worked = writer.1.write_all(message.as_bytes()).is_ok();
                worked &= writer.1.write_all(b"\n").is_ok();
                if !worked {
                    stderr().write_all(b"Failed to log to desired destination!").expect("FATAL: Failed to write to stderr!");
                }
            });
        }

        fn flush(&self) {}
    }
}