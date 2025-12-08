
use std::{arch::{asm, naked_asm}, sync::OnceLock};

use crate::pvz::{coin::{self, Coin}, data_array::DataArray};
use super::{HookRegistration, hook};

/// `DataArray<Coin>::DataArrayAlloc` 构造函数的地址
const ADDR_DATA_ARRAY_ALLOC: u32 = 0x0041E040;
/// `DataArray<Coin>::DataArrayAlloc` 构造函数的签名
type SignDataArrayAlloc = fn(
    this: *mut DataArray<Coin>,
) -> *mut Coin;
/// `DataArray<Coin>::DataArrayAlloc` 构造函数的跳板
static ORIGINAL_DATA_ARRAY_ALLOC: OnceLock<SignDataArrayAlloc> = OnceLock::new();

/// 从非标准调用中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn DataArrayAllocHelper() {
    naked_asm!(
        // 压栈 edi 作为参数
        "push edi",
        // 调用 hook 函数
        "call {hook}",
        // 返回
        "ret",

        // 传入 hook 函数符号地址
        hook = sym coin::DataArrayAlloc,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn DataArrayAllocWrapper(
    this: *mut DataArray<Coin>, 
) -> *mut Coin {
    unsafe {
        let result;
        asm!(
            // 保存 esi
            "push edi",
            // 把参数放入原函数期望的寄存器中
            "mov edi, {this}",
            // 调用原函数
            "call [{func}]",
            // 恢复 esi
            "pop edi",
            // 提取返回值
            "mov {result}, eax",

            this = in(reg) this,
            func = in(reg) ORIGINAL_DATA_ARRAY_ALLOC.wait(),
            result = out(reg) result,
        );
        result
    }
}

/// `Coin::CoinInitialize` 的地址
pub const ADDR_COIN_INITIALIZE: u32 = 0x0042FF60;
/// `Coin::CoinInitialize` 的签名
/// 
/// 仅标注用
type SignCoinInitialize = fn(
    this: *mut Coin,
    theX: i32,
    theY: i32,
    theCoinType: i32,
    theCoinMotion: i32,
);
/// `Coin::CoinInitialize` 的跳板
static ORIGINAL_COIN_INITIALIZE: OnceLock<SignCoinInitialize> = OnceLock::new();

/// 从 `usercall` 中提取参数的辅助函数
#[unsafe(naked)]
extern "stdcall" fn CoinInitializeHelper() {
    naked_asm!(
        // 压栈 usercall 参数
        "push eax",
        // 修正参数位置
        "mov eax, [esp]",
        "xchg eax, [esp+8]",
        "xchg eax, [esp+4]",
        "mov [esp], eax",
        // 压栈 usercall 参数
        "push ecx",
        // 修正参数位置
        "mov ecx, [esp]",
        "xchg ecx, [esp+8]",
        "xchg ecx, [esp+4]",
        "mov [esp], ecx",
        // 调用 hook 函数
        "jmp {hook}",

        hook = sym coin::CoinInitialize,
    )
}

/// 回调辅助函数
pub extern "stdcall" fn CoinInitializeWrapper(
    this: *mut Coin,
    theX: i32,
    theY: i32,
    theCoinType: i32,
    theCoinMotion: i32,
) {
    // 获取原函数的指针
    let func = ORIGINAL_COIN_INITIALIZE.wait();
    unsafe {
        asm!(
            // 压参数
            "push {}",
            "push {}",
            "push {}",
            in(reg) theY,
            in(reg) theX,
            in(reg) this,
        );
        asm!(
            // 调用原函数
            // 解指针获得真实地址
            "call dword ptr [{func}]",
            in("eax") theCoinType,
            in("ecx") theCoinMotion,
            func = in(reg) func,
        );
    }
}


inventory::submit! {
    HookRegistration(|| {
        let _ = ORIGINAL_DATA_ARRAY_ALLOC.set(
            hook(ADDR_DATA_ARRAY_ALLOC as _, DataArrayAllocHelper as _)?
        );

        let _ = ORIGINAL_COIN_INITIALIZE.set(
            hook(ADDR_COIN_INITIALIZE as _, CoinInitializeHelper as _)?
        );

        Ok(())
    })
}
