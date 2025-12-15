use anyhow::Result;
use serde::Deserialize;
use windows::Win32::System::Console::AllocConsole;

use crate::{CONFIG, config::load_config};

#[derive(Deserialize)]
struct Config {
    show_terminal: Option<bool>,
}

pub fn alloc_console() -> Result<()> {
    unsafe {
        if let Some(show_terminal) = load_config::<Config>(CONFIG).show_terminal
            && show_terminal
        {
            AllocConsole()?;
        }

        Ok(())
    }
}
