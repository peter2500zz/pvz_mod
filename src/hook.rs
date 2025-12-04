use anyhow::Result;
use minhook::MinHook;
use std::ffi::c_void;
const BOARD_ADDCOIN: *mut c_void = 0x0040CB10 as _;
type BoardAddCoin = extern "thiscall" fn(*mut c_void, i32, i32, u32, u32) -> *mut c_void;

// 保存原始函数地址
static mut ORIGINAL_BOARD_ADDCOIN: Option<BoardAddCoin> = None;

extern "thiscall" fn board_add_coin(
    board: *mut c_void,
    x: i32,
    y: i32,
    coin_type: u32,
    coin_motion: u32,
) -> *mut c_void {
    let my_coin = match coin_type {
        4 => 3,

        _ => coin_type,
    };

    unsafe { ORIGINAL_BOARD_ADDCOIN.unwrap()(board, x, y, my_coin, coin_motion) }
}

pub fn init_hooks() -> Result<()> {
    unsafe {
        let trampoline = MinHook::create_hook(BOARD_ADDCOIN, board_add_coin as _)?;

        ORIGINAL_BOARD_ADDCOIN = Some(std::mem::transmute(trampoline));

        MinHook::enable_all_hooks()?;
    }

    Ok(())
}
