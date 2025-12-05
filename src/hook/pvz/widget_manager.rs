
use std::{arch::{asm, naked_asm}, ffi::c_void, sync::OnceLock};

use crate::pvz::{lawn_app::LawnApp, widget_manager::{self, WidgetManager}};
use super::{HookRegistration, hook};


/// `WidgetManager` 构造函数的地址
const ADDR_WIDGET_MANAGER_CONSTRUCTOR: *mut c_void = 0x005384A0 as _;
/// `WidgetManager` 构造函数的签名
type SignWidgetManagerConstructor = extern "stdcall" fn(
    uninit: *mut WidgetManager,
    theApp: *mut LawnApp,
) -> *mut WidgetManager;
/// `WidgetManager` 构造函数的跳板
pub static ORIGINAL_WIDGET_MANAGER_CONSTRUCTOR: OnceLock<SignWidgetManagerConstructor> = OnceLock::new();

/// `WidgetManager` 析构函数的地址
const ADDR_WIDGET_MANAGER_DESTRUCTOR: *mut c_void = 0x00538610 as _;
/// `WidgetManager` 析构函数的签名
type SignWidgetManagerDestructor = extern "thiscall" fn(
    this: *mut WidgetManager
);
/// `WidgetManager` 析构函数的跳板
pub static ORIGINAL_WIDGET_MANAGER_DESTRUCTOR: OnceLock<SignWidgetManagerDestructor> = OnceLock::new();

/// `WidgetManager::KeyDown` 的地址
const ADDR_WIDGET_MANAGER_KEY_DOWN: *mut c_void = 0x00539660 as _;
/// `WidgetManager::KeyDown` 是 `usercall` ，签名没啥意义
type SignWidgetManagerKeydown = fn(
    this: *mut WidgetManager,
    key: i32,
);
/// `WidgetManager::KeyDown` 的跳板
static ORIGINAL_WIDGET_MANAGER_KEY_DOWN: OnceLock<SignWidgetManagerKeydown> = OnceLock::new();

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
            "mov ecx, {key}",
            "mov eax, {this}",
            // 调用原函数
            // 注意 OnceLock 存储的是指向原函数的指针，这里解一次指针
            "call dword ptr [{func}]",
            // 提取返回值
            "mov {result}, eax",

            key = in(reg) key,
            this = in(reg) this,
            func = in(reg) ORIGINAL_WIDGET_MANAGER_KEY_DOWN.wait(),
            result = out(reg) result,
        );
        result as _
    }
}

inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_WIDGET_MANAGER_CONSTRUCTOR.set(
            hook(ADDR_WIDGET_MANAGER_CONSTRUCTOR, widget_manager::Constructor as _)?
        );

        let _ = ORIGINAL_WIDGET_MANAGER_DESTRUCTOR.set(
            hook(ADDR_WIDGET_MANAGER_DESTRUCTOR, widget_manager::Destructor as _)?
        );

        let _ = ORIGINAL_WIDGET_MANAGER_KEY_DOWN.set(
            hook(ADDR_WIDGET_MANAGER_KEY_DOWN, KeyDownHelper as _)?
        );

        Ok(())
    })
}
