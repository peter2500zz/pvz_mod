use std::sync::OnceLock;

use crate::{
    hook::{HookRegistration, hook},
    pvz::{
        graphics::graphics::Graphics,
        widget_container::{self, ModalFlags, WidgetContainer},
    },
};

/// `WidgetContainer::DrawAll` 的地址
pub const ADDR_DRAW_ALL: u32 = 0x00538240;
/// `WidgetContainer::DrawAll` 的签名
type SignDrawAll =
    extern "thiscall" fn(this: *mut WidgetContainer, theFlags: *mut ModalFlags, g: *mut Graphics);
/// `WidgetContainer::DrawAll` 的跳板
pub static ORIGINAL_DRAW_ALL: OnceLock<SignDrawAll> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_DRAW_ALL.set(
            hook(ADDR_DRAW_ALL as _, widget_container::DrawAll as _)?
        );

        Ok(())
    })
}
