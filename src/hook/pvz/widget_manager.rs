use std::{
    arch::{asm, naked_asm},
    sync::{
        OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
};

use super::{HookRegistration, hook};
use crate::pvz::{
    lawn_app::lawn_app::LawnApp,
    widget_manager::{self, PostDrawScreen, widget_manager::WidgetManager},
};

/// `WidgetManager` 构造函数的地址
pub const ADDR_CONSTRUCTOR: u32 = 0x005384A0;
/// `WidgetManager` 构造函数的签名
type SignConstructor =
    extern "stdcall" fn(uninit: *mut WidgetManager, theApp: *mut LawnApp) -> *mut WidgetManager;
/// `WidgetManager` 构造函数的跳板
pub static ORIGINAL_CONSTRUCTOR: OnceLock<SignConstructor> = OnceLock::new();

/// `WidgetManager` 析构函数的地址
pub const ADDR_DESTRUCTOR: u32 = 0x00538610;
/// `WidgetManager` 析构函数的签名
type SignDestructor = extern "thiscall" fn(this: *mut WidgetManager);
/// `WidgetManager` 析构函数的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `WidgetManager::KeyDown` 的地址
pub const ADDR_KEY_DOWN: u32 = 0x00539660;
/// `WidgetManager::KeyDown` 是 `usercall` ，签名没啥意义
type SignKeydown = fn(this: *mut WidgetManager, key: i32);
/// `WidgetManager::KeyDown` 的跳板
static ORIGINAL_KEY_DOWN: OnceLock<SignKeydown> = OnceLock::new();

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "C" fn KeyDownHelper() {
    naked_asm!(
        // 压栈 usercall 参数
        "push ecx",
        "push eax",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret",

        hook = sym widget_manager::KeyDown,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn KeyDownWrapper(this: *mut WidgetManager, key: i32) -> u8 {
    unsafe {
        let result: u32;
        asm!(
            // 把参数放入原函数期望的寄存器中
            // 函数不清栈就得我们清了
            "mov ecx, {key}",
            "mov eax, {this}",
            // 调用原函数
            // 注意 OnceLock 存储的是指向原函数的指针，这里解一次指针
            "call dword ptr [{func}]",
            // 提取返回值
            "mov {result}, eax",

            key = in(reg) key,
            this = in(reg) this,
            func = in(reg) ORIGINAL_KEY_DOWN.wait(),
            result = out(reg) result,
        );
        result as _
    }
}

/// `WidgetManager::KeyUp` 的地址
pub const ADDR_KEY_UP: u32 = 0x005396A0;
/// `WidgetManager::KeyUp` 是 `usercall` ，签名没啥意义
type SignKeyUp = fn(this: *mut WidgetManager, key: i32);
/// `WidgetManager::KeyUp` 的跳板
static ORIGINAL_KEY_UP: OnceLock<SignKeyUp> = OnceLock::new();

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "C" fn KeyUpHelper() {
    naked_asm!(
        // 压栈 usercall 参数
        "push ecx",
        "push eax",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret",

        hook = sym widget_manager::KeyUp,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn KeyUpWrapper(this: *mut WidgetManager, key: i32) -> u8 {
    unsafe {
        let result: u32;
        asm!(
            // 把参数放入原函数期望的寄存器中
            // 函数不清栈就得我们清了
            "mov ecx, {key}",
            "mov eax, {this}",
            // 调用原函数
            // 注意 OnceLock 存储的是指向原函数的指针，这里解一次指针
            "call dword ptr [{func}]",
            // 提取返回值
            "mov {result}, eax",

            key = in(reg) key,
            this = in(reg) this,
            func = in(reg) ORIGINAL_KEY_DOWN.wait(),
            result = out(reg) result,
        );
        result as _
    }
}

/// 诡异包装 用于函数中劫持 g
pub const ADDR_POST_DRAW_SCREEN: u32 = 0x0053910A;
static ORIGINAL_POST_DRAW_SCREEN: AtomicUsize = AtomicUsize::new(0);

#[unsafe(naked)]
pub extern "thiscall" fn PostDrawScreenHelper() {
    naked_asm!(
        "push ecx",
        "call {hook}",
        "pop ecx",
        // 1. 读取存放的原始函数地址到 EDX
        // AtomicUsize 在内存中就是单纯的 usize 数据，可以直接读
        "mov edx, [{func}]",

        // 2. 直接跳转到 EDX 中存储的地址
        // 注意：这里没有方括号 []
        "jmp edx",

        hook = sym PostDrawScreen,
        func = sym ORIGINAL_POST_DRAW_SCREEN
    );
}

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_CONSTRUCTOR.set(
            hook(ADDR_CONSTRUCTOR as _, widget_manager::Constructor as _)?
        );

        let _ = ORIGINAL_DESTRUCTOR.set(
            hook(ADDR_DESTRUCTOR as _, widget_manager::Destructor as _)?
        );

        let _ = ORIGINAL_KEY_DOWN.set(
            hook(ADDR_KEY_DOWN as _, KeyDownHelper as _)?
        );

        let _ = ORIGINAL_KEY_UP.set(
            hook(ADDR_KEY_UP as _, KeyUpHelper as _)?
        );

        let _ = ORIGINAL_POST_DRAW_SCREEN.store(
            hook(ADDR_POST_DRAW_SCREEN as _, PostDrawScreenHelper as _)?, Ordering::SeqCst
        );

        Ok(())
    })
}
