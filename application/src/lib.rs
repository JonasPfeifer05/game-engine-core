use anyhow::{bail, Result};
use chrono::{Datelike, Timelike};
use winit::platform::run_return::EventLoopExtRunReturn;
use logger::LevelFilter;
use crate::error::ApplicationError;

pub use logger::{error, warn, info, debug, trace};
use crate::window::event_loop::Platform;

mod error;
mod window;

pub struct ApplicationConfig {
    pub start_pos_x: u32,
    pub start_pos_y: u32,
    pub start_width: u32,
    pub start_height: u32,
}

pub struct Application {
    pub platform: Platform
}

static mut ALREADY_INITIALIZED: bool = false;

impl Application {
    pub unsafe fn new(config: ApplicationConfig) -> Result<Self> {
        if ALREADY_INITIALIZED { bail!(ApplicationError::AlreadyInitialized) }
        ALREADY_INITIALIZED = false;

        let current_date = chrono::Utc::now();
        let time_string = format!("{}-{}-{}_{}-{}-{}", current_date.year(), current_date.month(), current_date.day(), current_date.hour(), current_date.minute(), current_date.second());
        logger::templates::init_file_logger(LevelFilter::Trace, format!("./core/application/resources/logging/{time_string}_log.txt"))?;

        debug!("Test log!");
        Ok(Self { platform: Default::default() })
    }

    pub fn run(&mut self) {
        info!("Starting window platform!");
        trace!("Starting platform!");
        self.platform.start();
        trace!("Platform stopped!")
    }
}