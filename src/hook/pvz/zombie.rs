
use std::{arch::{asm, naked_asm}, ffi::c_void, sync::OnceLock};

use windows::core::BOOL;

use crate::{pause, pvz::{lawn_app::LawnApp, widget_manager::{self, WidgetManager}}};
use super::{HookRegistration, hook};

/// 僵尸
type Zombie = c_void;

/// `DataArray::DataArrayAlloc` 构造函数的地址
const ADDR_DATA_ARRAY_ALLOC: *mut c_void = 0x0041DDA0 as _;
/// `DataArray::DataArrayAlloc` 构造函数的签名
type SignDataArrayAlloc = fn(
    this: *mut c_void,
) -> *mut Zombie;
/// `DataArray::DataArrayAlloc` 构造函数的跳板
static mut ORIGINAL_DATA_ARRAY_ALLOC: Option<SignDataArrayAlloc> = None;

/// `DataArray::DataArrayAlloc` 的 hook 函数
pub extern "stdcall" fn data_array_alloc(
    this: *mut c_void,
) -> *mut Zombie {
    tracing::trace!("alloc zombie");
    DataArrayAllocWrapper(this)
}

/// 从非标准调用中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn DataArrayAllocHelper() {
    naked_asm!(
        // 压栈 esi 作为参数
        "push esi",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret",

        // 传入 hook 函数符号地址
        hook = sym data_array_alloc,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn DataArrayAllocWrapper(
    this: *mut c_void, 
) -> *mut Zombie {
    unsafe {
        let result;
        asm!(
            // 保存 esi
            "push esi",
            // 把参数放入原函数期望的寄存器中
            "mov esi, {this}",
            // 调用原函数
            "call {func}",
            // 恢复 esi
            "pop esi",
            // 提取返回值
            "mov {result}, eax",

            this = in(reg) this,
            func = in(reg) ORIGINAL_DATA_ARRAY_ALLOC.unwrap(),
            result = out(reg) result,
        );
        result
    }
}

use anyhow::Result;
use minhook::MinHook;

/// `Zombie::ZombieInitialize` 的地址
const ADDR_ZOMBIE_ZOMBIE_INITIALIZE: *mut c_void = 0x00522580 as _;
/// `Zombie::ZombieInitialize` 的签名
/// 
/// 仅标注用
type SignZombieZombieInitialize = fn(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
);
/// `Zombie::ZombieInitialize` 的跳板
static mut ORIGINAL_ZOMBIE_ZOMBIE_INITIALIZE: Option<SignZombieZombieInitialize> = None;

/// hook 函数
pub extern "stdcall" fn zombie_zombie_initialize(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
) {
    tracing::trace!("初始化 行 {} 类型 {} 来自第 {} 波", theRow, theZombieType, theFromWave);
    ZombieInitializeWrapper(
        this,
        theRow,
        theZombieType,
        theVariant,
        theParentZombie,
        theFromWave
    );
}

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn ZombieInitializeHelper() {
    naked_asm!(
        // 压栈 usercall 参数
        "push eax",
        // 修正参数位置
        "mov eax, [esp]",
        "xchg eax, [esp+8]",
        "xchg eax, [esp+4]",
        "mov [esp], eax",
        // 调用 hook 函数
        "jmp {hook}",

        hook = sym zombie_zombie_initialize,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn ZombieInitializeWrapper(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
) {
    unsafe {
        asm!(
            // 调用原函数
            "push {}",
            "push {}",
            "push {}",
            "push {}",
            "push {}",
            "call dword ptr {func}",
            in(reg) theFromWave,
            in(reg) theParentZombie,
            in(reg) theVariant.0,
            in(reg) theZombieType,
            in(reg) this,
            in("eax") theRow,
            func = sym ORIGINAL_ZOMBIE_ZOMBIE_INITIALIZE,
        );
    }
}

/// 下钩子
pub fn init_hooks() -> Result<()> {
    unsafe {
        let trampoline = MinHook::create_hook(
            ADDR_ZOMBIE_ZOMBIE_INITIALIZE, 
            ZombieInitializeHelper as _
        )?;

        ORIGINAL_ZOMBIE_ZOMBIE_INITIALIZE = Some(std::mem::transmute(trampoline));

        MinHook::enable_all_hooks()?;
    }

    Ok(())
}

inventory::submit! {
    HookRegistration(|| {
        // let _ = ORIGINAL_WIDGET_MANAGER_CONSTRUCTOR.set(
        //     hook(ADDR_WIDGET_MANAGER_CONSTRUCTOR, widget_manager::Constructor as _)?
        // );

        // let _ = ORIGINAL_WIDGET_MANAGER_DESTRUCTOR.set(
        //     hook(ADDR_WIDGET_MANAGER_DESTRUCTOR, widget_manager::Destructor as _)?
        // );

        // let _ = ORIGINAL_WIDGET_MANAGER_KEY_DOWN.set(
        //     hook(ADDR_WIDGET_MANAGER_KEY_DOWN, KeyDownHelper as _)?
        // );

        unsafe {
            unsafe {
        let trampoline = MinHook::create_hook(
            ADDR_DATA_ARRAY_ALLOC, 
            DataArrayAllocHelper as _
        )?;

        ORIGINAL_DATA_ARRAY_ALLOC = Some(std::mem::transmute(trampoline));

        MinHook::enable_all_hooks()?;
    }
            init_hooks()?;
            
        }

        Ok(())
    })
}
