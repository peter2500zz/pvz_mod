#[allow(non_snake_case)]
pub mod pvz;

use anyhow::Result;
use inventory;
use minhook::MinHook;
use std::ffi::c_void;
use tracing::{
    trace,
    error
};

// const ADDR_BOARD_ADDCOIN: *mut c_void = 0x0040CB10 as _;
// type SignBoardAddCoin = extern "thiscall" fn(*mut c_void, i32, i32, u32, u32) -> *mut c_void;
// static mut ORIGINAL_BOARD_ADDCOIN: Option<SignBoardAddCoin> = None;

// extern "thiscall" fn board_add_coin(
//     board: *mut c_void,
//     x: i32,
//     y: i32,
//     coin_type: u32,
//     coin_motion: u32,
// ) -> *mut c_void {
//     let my_coin = match coin_type {
//         4 => 3,

//         _ => coin_type,
//     };

//     unsafe { ORIGINAL_BOARD_ADDCOIN.unwrap()(board, x, y, my_coin, coin_motion) }
// }

fn hook<F>(target: *mut c_void, detour: *mut c_void) -> Result<F> {
    unsafe {
        let trampoline = MinHook::create_hook(target, detour);

        match &trampoline {
            Ok(trampoline) => {
                trace!("Hook {:#x?} -> {:#x?}", target, detour);

                Ok(std::mem::transmute_copy::<*mut c_void, F>(trampoline))
            },
            Err(e) => {
                error!("Hook {:#x?} 时出现错误: {}", target, e);

                trampoline?;
                unreachable!()
            }
        }
    }
}

fn hook_api<F, T>(module_name: T, proc_name: T, detour: *mut c_void) -> Result<F> 
where
    T: AsRef<str> + Clone,
{
    unsafe {
        let trampoline = MinHook::create_hook_api(
            module_name.clone(),
            proc_name.clone(), 
            detour
        )?;

        // MinHook::enable_hook(trampoline)?;

        trace!("Hook API {}::{} -> {:#x?}", module_name.as_ref(), proc_name.as_ref(), detour);

        Ok(std::mem::transmute_copy::<*mut c_void, F>(&trampoline))
    }
}

type HookInitFn = fn() -> Result<()>;
struct HookRegistration(HookInitFn);

inventory::collect!(HookRegistration);

pub fn init_hook() -> Result<()> {
    // pvz::init_hook()?;

    for HookRegistration(hook_init) in inventory::iter::<HookRegistration> {
        hook_init()?;
    }

    unsafe {
        MinHook::enable_all_hooks()?;
    }

    // unsafe {
    //     ORIGINAL_BOARD_ADDCOIN = Some(hook(ADDR_BOARD_ADDCOIN, board_add_coin as _)?);
    // }

    Ok(())
}
