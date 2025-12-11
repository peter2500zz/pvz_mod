use std::sync::OnceLock;

use crate::{hook::{HookRegistration, hook}, pvz::graphics::{self, graphics::Graphics}};


/// `Graphics::Create` 的地址
pub const ADDR_CREATE: u32 = 0x00586C30;
/// `Graphics::Create` 的签名
type SignCreate = extern "stdcall" fn(
    this: *mut Graphics,
) -> *mut Graphics;
/// `Graphics::Create` 的跳板
pub static ORIGINAL_CREATE: OnceLock<SignCreate> = OnceLock::new();

/// `Graphics::Destructor` 的地址
pub const ADDR_DESTRUCTOR: u32 = 0x00586B10;
/// `Graphics::Destructor` 的签名
type SignDestructor = extern "thiscall" fn(
    this: *mut Graphics,
);
/// `Graphics::Destructor` 的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `Graphics::DrawRect` 的地址
pub const ADDR_DRAW_RECT: u32 = 0x00586DE0;

/// `Graphics::FillRect` 的地址
pub const ADDR_FILL_RECT: u32 = 0x00586D50;

/// `Graphics::SetColor` 的地址
pub const ADDR_SET_COLOR: u32 = 0x00586CC0;

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_CREATE.set(
            hook(ADDR_CREATE as _, graphics::Create as _)?
        );

        let _ = ORIGINAL_DESTRUCTOR.set(
            hook(ADDR_DESTRUCTOR as _, graphics::Destructor as _)?
        );

        Ok(())
    })
}

