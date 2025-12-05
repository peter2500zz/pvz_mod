
use std::{ffi::c_void, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::{board::{self, Board}, lawn_app::LawnApp};

/// `Board` 构造函数的地址
const ADDR_BOARD_CONSTRUCTOR: *mut c_void = 0x00407B50 as _;
/// `Board` 构造函数的签名
type SignBoardConstructor = extern "thiscall" fn(
    uninit: *mut Board,
    theApp: *mut LawnApp,
) -> *mut Board;
/// `Board` 构造函数的跳板
pub static ORIGINAL_BOARD_CONSTRUCTOR: OnceLock<SignBoardConstructor> = OnceLock::new();

/// `Board` 析构函数的地址
const ADDR_BOARD_DESTRUCTOR: *mut c_void = 0x00408690 as _;
/// `Board` 析构函数的签名
type SignBoardDestructor = extern "thiscall" fn(
    this: *mut Board
);
/// `Board` 析构函数的跳板
pub static ORIGINAL_BOARD_DESTRUCTOR: OnceLock<SignBoardDestructor> = OnceLock::new();

/// `Board::InitLevel` 的地址
const ADDR_BOARD_INIT_LEVEL: *mut c_void = 0x0040AF90 as _;
/// `Board::InitLevel` 的签名
type SignBoardInitLevel = extern "stdcall" fn(
    this: *mut Board
);
/// `Board::InitLevel` 的跳板
pub static ORIGINAL_BOARD_INIT_LEVEL: OnceLock<SignBoardInitLevel> = OnceLock::new();

/// `Board::KeyDown` 的地址
const ADDR_BOARD_KEYDOWN: *mut c_void = 0x0041B820 as _;
/// `Board::KeyDown` 的签名
type SignBoardKeyDown = extern "thiscall" fn(
    this: *mut Board, 
    keycode: i32,
);
/// `Board::KeyDown` 的跳板
pub static ORIGINAL_BOARD_KEYDOWN: OnceLock<SignBoardKeyDown> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_BOARD_CONSTRUCTOR.set(
            hook(ADDR_BOARD_CONSTRUCTOR, board::Constructor as _)?
        );

        let _ = ORIGINAL_BOARD_DESTRUCTOR.set(
            hook(ADDR_BOARD_DESTRUCTOR, board::Destructor as _)?
        );

        let _ = ORIGINAL_BOARD_INIT_LEVEL.set(
            hook(ADDR_BOARD_INIT_LEVEL, board::InitLevel as _)?
        );

        let _ = ORIGINAL_BOARD_KEYDOWN.set(
            hook(ADDR_BOARD_KEYDOWN, board::KeyDown as _)?
        );

        Ok(())
    })
}
