
use std::ffi::c_void;

use tracing::trace;

use crate::{
    hook::pvz::{board::{
        ORIGINAL_BOARD_CONSTRUCTOR, 
        ORIGINAL_BOARD_DESTRUCTOR, 
        ORIGINAL_BOARD_INIT_LEVEL, ORIGINAL_BOARD_KEYDOWN
    }, zombie::{data_array_alloc, zombie_zombie_initialize}}, pvz::lawn_app::LawnApp
};


#[derive(Debug)]
#[repr(C)]
/// 这是 `Board`
pub struct Board {
    _pad: [u8; 0x57B0],  
}

/// 这是 `Board` 的构造函数
pub extern "thiscall" fn Constructor(
    uninit: *mut Board, 
    theApp: *mut LawnApp
) -> *mut Board {
    trace!("构造 Board");

    let this = ORIGINAL_BOARD_CONSTRUCTOR.wait()(
        uninit,
        theApp
    );

    trace!("地址 {:#x?}", this);

    this
}

/// 这是 `Board` 的析构函数
pub extern "thiscall" fn Destructor(
    this: *mut Board
) {
    trace!("析构 Board");

    ORIGINAL_BOARD_DESTRUCTOR.wait()(this);
}

/// `Board` 的初始化函数
/// 
/// 初始化关卡信息，设定关卡背景、出怪、初始阳光、浓雾坐标等基础数据及卡槽和部分关卡的固定选卡
pub extern "stdcall" fn InitLevel(
    this: *mut Board
) {
    trace!("初始化 Board");

    ORIGINAL_BOARD_INIT_LEVEL.wait()(this);
}

/// `Board::KeyDown` 的 hook 函数
pub extern "thiscall" fn KeyDown(
    this: *mut Board, 
    keycode: i32, 
) {
    trace!("Board({:#x?}) 按下 {:#x}", this, keycode);

    match keycode {
        65 => {
            let array = ((this as u32) + 0x90) as *mut c_void;
            let zombie = data_array_alloc(
                array
            );
            zombie_zombie_initialize(
                zombie,
                0,
                0,
                false.into(),
                0 as _,
                0
            )
        }
        _ => (),
    }

    // 回调
    ORIGINAL_BOARD_KEYDOWN.wait()(
        this, 
        keycode
    );
}
