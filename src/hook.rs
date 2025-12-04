#[allow(non_snake_case)]
pub mod pvz;

use anyhow::Result;
use inventory;
use minhook::MinHook;
use std::ffi::c_void;
use tracing::info;

// const ADDR_BOARD_ADDCOIN: *mut c_void = 0x0040CB10 as _;
// type SignBoardAddCoin = extern "thiscall" fn(*mut c_void, i32, i32, u32, u32) -> *mut c_void;
// static mut ORIGINAL_BOARD_ADDCOIN: Option<SignBoardAddCoin> = None;

// extern "thiscall" fn board_add_coin(
//     board: *mut c_void,
//     x: i32,
//     y: i32,
//     coin_type: u32,
//     coin_motion: u32,
// ) -> *mut c_void {
//     let my_coin = match coin_type {
//         4 => 3,

//         _ => coin_type,
//     };

//     unsafe { ORIGINAL_BOARD_ADDCOIN.unwrap()(board, x, y, my_coin, coin_motion) }
// }

fn hook<T>(target: *mut c_void, detour: *mut c_void) -> Result<T> {
    unsafe {
        let trampoline = MinHook::create_hook(target, detour)?;

        MinHook::enable_hook(target)?;

        info!("Hooked {:#x?} -> {:#x?}", target, detour);

        Ok(std::mem::transmute_copy::<*mut c_void, T>(&trampoline))
    }
}

type HookInitFn = fn() -> Result<()>;
struct HookRegistration(HookInitFn);

inventory::collect!(HookRegistration);

pub fn init_hook() -> Result<()> {
    // pvz::init_hook()?;

    for HookRegistration(hook_init) in inventory::iter::<HookRegistration> {
        hook_init()?;
    }

    // unsafe {
    //     ORIGINAL_BOARD_ADDCOIN = Some(hook(ADDR_BOARD_ADDCOIN, board_add_coin as _)?);
    // }

    Ok(())
}
