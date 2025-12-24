use anyhow::Result;
use config::load_config;
use serde::Deserialize;
use tracing::debug;
use windows::Win32::System::Console::AllocConsole;

use crate::{CONFIG, pvz::lawn_app::lawn_app::with_lawn_app, save::PROFILE_MANAGER};

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

pub fn tigger_handler(flag: String) {
    unsafe {
        match flag.as_str() {
            "save slot" => {
                let _ = with_lawn_app(|lawn_app| {
                    debug!("{}", (*lawn_app.player_info).save_slot);

                    Ok(())
                });
            }

            "win" => {
                let _ = with_lawn_app(|lawn_app| {
                    debug!("pre: {}", (*(*lawn_app).board).is_winning);
                    (*(*lawn_app).board).is_winning = true;
                    debug!("tiggerred win: {}", (*(*lawn_app).board).is_winning);
                    debug!("pre: {}", (*(*lawn_app).board).is_win);
                    (*(*lawn_app).board).is_win = true;
                    debug!("tiggerred win: {}", (*(*lawn_app).board).is_win);

                    Ok(())
                });
            }

            "data" => {
                if let Ok(pm) = PROFILE_MANAGER.lock() {
                    debug!("{:#?}", pm);
                }
            }

            _ => {
                debug!("无效调试标志");
            }
        }
    }
}
