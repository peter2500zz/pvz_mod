
use std::{ffi::c_void, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::{board::{self, Board}, lawn_app::LawnApp};

/// `Board` 构造函数的地址
const ADDR_CONSTRUCTOR: u32 = 0x00407B50 as _;
/// `Board` 构造函数的签名
type SignConstructor = extern "thiscall" fn(
    uninit: *mut Board,
    theApp: *mut LawnApp,
) -> *mut Board;
/// `Board` 构造函数的跳板
pub static ORIGINAL_CONSTRUCTOR: OnceLock<SignConstructor> = OnceLock::new();

/// `Board` 析构函数的地址
const ADDR_DESTRUCTOR: u32 = 0x00408690 as _;
/// `Board` 析构函数的签名
type SignDestructor = extern "thiscall" fn(
    this: *mut Board
);
/// `Board` 析构函数的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `Board::InitLevel` 的地址
const ADDR_INIT_LEVEL: u32 = 0x0040AF90 as _;
/// `Board::InitLevel` 的签名
type SignInitLevel = extern "stdcall" fn(
    this: *mut Board
);
/// `Board::InitLevel` 的跳板
pub static ORIGINAL_INIT_LEVEL: OnceLock<SignInitLevel> = OnceLock::new();

/// `Board::AddCoin` 的地址
pub const ADDR_ADDCOIN: u32 = 0x0040CB10 as _;
/// `Board::AddCoin` 的签名
type SignAddCoin = extern "thiscall" fn(
    this: *mut Board, 
    theX: i32, 
    theY: i32, 
    theCoinType: u32, 
    theCoinMotion: u32
) -> *mut c_void;
/// `Board::AddCoin` 的跳板
pub static ORIGINAL_ADDCOIN: OnceLock<SignAddCoin> = OnceLock::new();

/// `Board::KeyDown` 的地址
const ADDR_KEYDOWN: u32 = 0x0041B820 as _;
/// `Board::KeyDown` 的签名
type SignKeyDown = extern "thiscall" fn(
    this: *mut Board, 
    keycode: i32,
);
/// `Board::KeyDown` 的跳板
pub static ORIGINAL_KEYDOWN: OnceLock<SignKeyDown> = OnceLock::new();

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_CONSTRUCTOR.set(
            hook(ADDR_CONSTRUCTOR as _, board::Constructor as _)?
        );

        let _ = ORIGINAL_DESTRUCTOR.set(
            hook(ADDR_DESTRUCTOR as _, board::Destructor as _)?
        );

        let _ = ORIGINAL_INIT_LEVEL.set(
            hook(ADDR_INIT_LEVEL as _, board::InitLevel as _)?
        );

        let _ = ORIGINAL_ADDCOIN.set(
            hook(ADDR_ADDCOIN as _, board::AddCoin as _)?
        );

        let _ = ORIGINAL_KEYDOWN.set(
            hook(ADDR_KEYDOWN as _, board::KeyDown as _)?
        );

        Ok(())
    })
}
