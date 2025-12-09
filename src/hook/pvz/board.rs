
use std::{arch::{asm, naked_asm}, sync::OnceLock};

use super::{HookRegistration, hook};
use crate::pvz::{
    board::{
        self, 
        board::Board
    }, 
    coin::Coin, 
    lawn_app::lawn_app::LawnApp, zombie::zombie::Zombie
};

/// `Board` 构造函数的地址
const ADDR_CONSTRUCTOR: u32 = 0x00407B50;
/// `Board` 构造函数的签名
type SignConstructor = extern "thiscall" fn(
    uninit: *mut Board,
    theApp: *mut LawnApp,
) -> *mut Board;
/// `Board` 构造函数的跳板
pub static ORIGINAL_CONSTRUCTOR: OnceLock<SignConstructor> = OnceLock::new();

/// `Board` 析构函数的地址
const ADDR_DESTRUCTOR: u32 = 0x00408690;
/// `Board` 析构函数的签名
type SignDestructor = extern "thiscall" fn(
    this: *mut Board
);
/// `Board` 析构函数的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `Board::InitLevel` 的地址
const ADDR_INIT_LEVEL: u32 = 0x0040AF90;
/// `Board::InitLevel` 的签名
type SignInitLevel = extern "stdcall" fn(
    this: *mut Board
);
/// `Board::InitLevel` 的跳板
pub static ORIGINAL_INIT_LEVEL: OnceLock<SignInitLevel> = OnceLock::new();

/// `Board::AddCoin` 的地址
pub const ADDR_ADDCOIN: u32 = 0x0040CB10;
/// `Board::AddCoin` 的签名
type SignAddCoin = extern "thiscall" fn(
    this: *mut Board, 
    theX: i32, 
    theY: i32, 
    theCoinType: u32, 
    theCoinMotion: u32
) -> *mut Coin;
/// `Board::AddCoin` 的跳板
pub static ORIGINAL_ADDCOIN: OnceLock<SignAddCoin> = OnceLock::new();

/// `Board::KeyDown` 的地址
pub const ADDR_KEYDOWN: u32 = 0x0041B820;
/// `Board::KeyDown` 的签名
type SignKeyDown = extern "thiscall" fn(
    this: *mut Board, 
    keycode: i32,
);
/// `Board::KeyDown` 的跳板
pub static ORIGINAL_KEYDOWN: OnceLock<SignKeyDown> = OnceLock::new();

/// `Board::AddZombieInRow` 的地址
pub const ADDR_ADD_ZOMBIE_IN_ROW: u32 = 0x0040DDC0;
/// `Board::AddZombieInRow` 的签名
type SignAddZombieInRow = extern "stdcall" fn(
    this: *mut Board, 
    theZombieType: i32,
    theRow: i32,
    theFromWave: i32,
) -> *mut Zombie;
/// `Board::AddZombieInRow` 的跳板
pub static ORIGINAL_ADD_ZOMBIE_IN_ROW: OnceLock<SignAddZombieInRow> = OnceLock::new();

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn AddZombieInRowHelper() {
    naked_asm!(
        // 压栈 usercall 参数
        "push eax",
        // 修正参数位置
        "mov eax, [esp]",
        "xchg eax, [esp+8]",
        "xchg eax, [esp+4]",
        "mov [esp], eax",
        // 压栈 usercall 参数
        "push ebx",
        // 修正参数位置
        "mov eax, [esp]",
        "xchg eax, [esp+8]",
        "xchg eax, [esp+4]",
        "mov [esp], eax",
        // 调用 hook 函数
        "jmp {hook}",

        hook = sym board::AddZombieInRow,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn AddZombieInRowWrapper(
    this: *mut Board, 
    theZombieType: i32,
    theRow: i32,
    theFromWave: i32,
) -> *mut Zombie {
    // 获取原函数的指针
    let func = ORIGINAL_ADD_ZOMBIE_IN_ROW.wait();
    unsafe {
        asm!(
            "push ebx"
        );
        asm!(
            // 压参数
            "push {}",
            "push {}",
            in(reg) theRow,
            in(reg) theZombieType,
        );
        let zombie;
        asm!(
            // 调用原函数
            // 解指针获得真实地址
            "call dword ptr [{func}]",
            "mov {zombie}, eax",
            in("eax") this,
            in("ebx") theFromWave,
            func = in(reg) func,
            zombie = out(reg) zombie
        );
        asm!(
            "pop ebx"
        );
        zombie
    }
}

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

        let _ = ORIGINAL_ADD_ZOMBIE_IN_ROW.set(
            hook(ADDR_ADD_ZOMBIE_IN_ROW as _, AddZombieInRowHelper as _)?
        );

        Ok(())
    })
}
