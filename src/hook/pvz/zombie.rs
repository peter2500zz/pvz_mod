
use std::{arch::{asm, naked_asm}, ffi::c_void, sync::OnceLock};
use windows::core::BOOL;

use crate::pvz::zombie::{self, Zombie};
use super::{HookRegistration, hook};

/// `DataArray::DataArrayAlloc` 构造函数的地址
const ADDR_DATA_ARRAY_ALLOC: u32 = 0x0041DDA0 as _;
/// `DataArray::DataArrayAlloc` 构造函数的签名
type SignDataArrayAlloc = fn(
    this: *mut c_void,
) -> *mut Zombie;
/// `DataArray::DataArrayAlloc` 构造函数的跳板
static ORIGINAL_DATA_ARRAY_ALLOC: OnceLock<SignDataArrayAlloc> = OnceLock::new();

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
        hook = sym zombie::DataArrayAlloc,
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
            "call [{func}]",
            // 恢复 esi
            "pop esi",
            // 提取返回值
            "mov {result}, eax",

            this = in(reg) this,
            func = in(reg) ORIGINAL_DATA_ARRAY_ALLOC.wait(),
            result = out(reg) result,
        );
        result
    }
}

/// `Zombie::ZombieInitialize` 的地址
const ADDR_ZOMBIE_INITIALIZE: u32 = 0x00522580 as _;
/// `Zombie::ZombieInitialize` 的签名
/// 
/// 仅标注用
type SignZombieInitialize = fn(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
);
/// `Zombie::ZombieInitialize` 的跳板
static ORIGINAL_ZOMBIE_INITIALIZE: OnceLock<SignZombieInitialize> = OnceLock::new();

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

        hook = sym zombie::ZombieInitialize,
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
    // 获取原函数的指针
    let func = ORIGINAL_ZOMBIE_INITIALIZE.wait();
    unsafe {
        asm!(
            // 压参数
            "push {}",
            "push {}",
            "push {}",
            "push {}",
            "push {}",
            in(reg) theFromWave,
            in(reg) theParentZombie,
            in(reg) theVariant.0,
            in(reg) theZombieType,
            in(reg) this,
        );
        asm!(
            // 调用原函数
            // 解指针获得真实地址
            "call dword ptr [{func}]",
            in("eax") theRow,
            func = in(reg) func,
        );
    }
}


inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_DATA_ARRAY_ALLOC.set(
            hook(ADDR_DATA_ARRAY_ALLOC as _, DataArrayAllocHelper as _)?
        );

        let _ = ORIGINAL_ZOMBIE_INITIALIZE.set(
            hook(ADDR_ZOMBIE_INITIALIZE as _, ZombieInitializeHelper as _)?
        );

        Ok(())
    })
}
