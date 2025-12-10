
use std::{arch::{asm, naked_asm}, sync::OnceLock};

use crate::pvz::{lawn_app::lawn_app::LawnApp, widget_manager::{self, widget_manager::WidgetManager}};
use super::{HookRegistration, hook};


/// `WidgetManager` 构造函数的地址
const ADDR_CONSTRUCTOR: u32 = 0x005384A0 as _;
/// `WidgetManager` 构造函数的签名
type SignConstructor = extern "stdcall" fn(
    uninit: *mut WidgetManager,
    theApp: *mut LawnApp,
) -> *mut WidgetManager;
/// `WidgetManager` 构造函数的跳板
pub static ORIGINAL_CONSTRUCTOR: OnceLock<SignConstructor> = OnceLock::new();

/// `WidgetManager` 析构函数的地址
const ADDR_DESTRUCTOR: u32 = 0x00538610 as _;
/// `WidgetManager` 析构函数的签名
type SignDestructor = extern "thiscall" fn(
    this: *mut WidgetManager
);
/// `WidgetManager` 析构函数的跳板
pub static ORIGINAL_DESTRUCTOR: OnceLock<SignDestructor> = OnceLock::new();

/// `WidgetManager::KeyDown` 的地址
const ADDR_KEY_DOWN: u32 = 0x00539660 as _;
/// `WidgetManager::KeyDown` 是 `usercall` ，签名没啥意义
type SignKeydown = fn(
    this: *mut WidgetManager,
    key: i32,
);
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

        Ok(())
    })
}
