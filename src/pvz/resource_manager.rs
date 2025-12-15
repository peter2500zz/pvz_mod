use std::arch::asm;

use crate::{hook::pvz::resource_manager::ADDR_LOAD_FONT, pvz::graphics::graphics::Font, utils::msvc_string::MsvcString};


#[repr(C)]
pub struct ResourceManager {
    _pad_0x0_0xAC: [u8; 0xAC - 0x0],
    /// 0xAC 使用系统字体标志
    pub use_system_font: bool,
    _pad_0xB1_0xCC: [u8; 0xCC - 0xAD],
}
const _: () = assert!(size_of::<ResourceManager>() == 0xCC);


pub fn LoadFont(
    this: *mut ResourceManager,
    theName: &MsvcString
) -> Option<*mut Font> {
    unsafe {
        let result: *mut Font;

        asm!(
            "push {this}",

            "call {func}",

            in("ecx") theName,
            this = in(reg) this,
            func = in(reg) ADDR_LOAD_FONT,
            lateout("eax") result,

            clobber_abi("C")
        );

        if result as u32 != 0 {
            Some(result)
        } else {
            None
        }
    }
}
