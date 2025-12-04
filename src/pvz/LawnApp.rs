use tracing::info;

use crate::hook::pvz::LawnApp::{
    ORIGINAL_LAWNAPP_CONSTRUCTOR, 
    ORIGINAL_LAWNAPP_DESTRUCTOR
};

#[derive(Debug)]
#[repr(C)]
pub struct LawnApp {
    _pad: [u8; 0x8C8],
}

pub extern "stdcall" fn Constructor(uninit: *mut LawnApp) -> *mut LawnApp {
    info!("构造");

    ORIGINAL_LAWNAPP_CONSTRUCTOR.wait()(uninit)
}

pub extern "thiscall" fn Destructor(this: *mut LawnApp) {
    info!("析构");

    ORIGINAL_LAWNAPP_DESTRUCTOR.wait()(this)
}
