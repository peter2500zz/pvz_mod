pub mod board;

use tracing::{trace};
use windows::core::BOOL;

use crate::{
    add_callback, hook::pvz::board::{
        ADDR_KEYDOWN, ORIGINAL_ADDCOIN, ORIGINAL_CONSTRUCTOR, ORIGINAL_DESTRUCTOR, ORIGINAL_INIT_LEVEL, ORIGINAL_KEYDOWN
    }, mods::callback::callback, pvz::{
        board::board::Board, 
        coin::Coin, 
        data_array::DataArray, 
        lawn_app::lawn_app::LawnApp, 
        zombie::zombie::Zombie
    }
};

/// 这是 `Board` 的构造函数
pub extern "thiscall" fn Constructor(
    uninit: *mut Board, 
    theApp: *mut LawnApp
) -> *mut Board {
    trace!("构造 Board");

    let this = ORIGINAL_CONSTRUCTOR.wait()(
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

    ORIGINAL_DESTRUCTOR.wait()(this);
}

/// `Board` 的初始化函数
/// 
/// 初始化关卡信息，设定关卡背景、出怪、初始阳光、浓雾坐标等基础数据及卡槽和部分关卡的固定选卡
pub extern "stdcall" fn InitLevel(
    this: *mut Board
) {
    unsafe {
        trace!("初始化 Board 大小 {}", size_of_val(&*this));
    }

    ORIGINAL_INIT_LEVEL.wait()(this);
}

/// 在游戏中生成掉落物的函数
pub extern "thiscall" fn AddCoin(
    this: *mut Board, 
    theX: i32, 
    theY: i32, 
    theCoinType: u32, 
    theCoinMotion: u32
) -> *mut Coin {
    trace!("产生掉落物 {} at ({}, {}) with motion {}", theCoinType, theX, theY, theCoinMotion);
    // let (
    //     theX,
    //     theY,
    //     theCoinType,
    //     theCoinMotion
    // ) = callback(ADDR_ADDCOIN, (
    //     theX,
    //     theY,
    //     theCoinType,
    //     theCoinMotion
    // ));

    let coin = ORIGINAL_ADDCOIN.wait()(
        this, 
        theX, 
        theY, 
        theCoinType, 
        theCoinMotion
    );

    coin
}

/// `Board::KeyDown` 的 hook 函数
pub extern "thiscall" fn KeyDown(
    this: *mut Board, 
    keycode: i32, 
) {
    trace!("Board({:#x?}) 按下 {:#x}", this, keycode);

    callback(ADDR_KEYDOWN, keycode);

    match keycode {
        65 => {
            let array = ((this as u32) + 0x90) as *mut DataArray<Zombie>;
            // let zombie = zombie::DataArrayAlloc(
            //     array
            // );
            // zombie::ZombieInitialize(
            //     zombie,
            //     0,
            //     0,
            //     false.into(),
            //     0 as _,
            //     0
            // )
        }
        _ => (),
    }

    // 回调
    ORIGINAL_KEYDOWN.wait()(
        this, 
        keycode
    );
}
add_callback!("AT_BOARD_KEYDOWN", ADDR_KEYDOWN);
