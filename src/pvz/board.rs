pub mod board;

use mlua::prelude::*;
use tracing::*;

use crate::{
    add_callback, add_field_mut,
    hook::pvz::board::{
        ADDR_ADD_ZOMBIE_IN_ROW, ADDR_ADDCOIN, ADDR_KEYDOWN, ADDR_MOUSE_DOWN, ADDR_MOUSE_UP,
        ADDR_UPDATE, AddZombieInRowWrapper, ORIGINAL_ADDCOIN, ORIGINAL_CONSTRUCTOR,
        ORIGINAL_DESTRUCTOR, ORIGINAL_INIT_LEVEL, ORIGINAL_KEYDOWN, ORIGINAL_MOUSE_DOWN,
        ORIGINAL_MOUSE_UP, ORIGINAL_UPDATE,
    },
    mods::callback::{POST, PRE, callback, callback_data},
    pvz::{board::board::Board, coin::Coin, lawn_app::lawn_app::LawnApp, zombie::zombie::Zombie},
    utils::{Vec2, delta_mgr::get_delta_mgr},
};

/// 这是 `Board` 的构造函数
pub extern "thiscall" fn Constructor(uninit: *mut Board, theApp: *mut LawnApp) -> *mut Board {
    trace!("构造 Board");

    let this = ORIGINAL_CONSTRUCTOR.wait()(uninit, theApp);

    trace!("地址 {:#x?}", this);

    this
}

/// 这是 `Board` 的析构函数
pub extern "thiscall" fn Destructor(this: *mut Board) {
    trace!("析构 Board");

    ORIGINAL_DESTRUCTOR.wait()(this);
}

/// `Board` 的初始化函数
///
/// 初始化关卡信息，设定关卡背景、出怪、初始阳光、浓雾坐标等基础数据及卡槽和部分关卡的固定选卡
pub extern "stdcall" fn InitLevel(this: *mut Board) {
    unsafe {
        trace!("初始化 Board 大小 {}", size_of_val(&*this));
    }

    ORIGINAL_INIT_LEVEL.wait()(this);
}

#[repr(C)]
pub struct ArgsAddCoin {
    pos: Vec2<i32>,
    theCoinType: u32,
    theCoinMotion: u32,
}

impl LuaUserData for ArgsAddCoin {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetPos", |_, this, ()| Ok(this.pos));

        methods.add_method_mut("SetPos", |_, this, pos| Ok(this.pos = pos));

        methods.add_method("GetCoinType", |_, this, ()| Ok(this.theCoinType));

        methods.add_method_mut("SetCoinType", |_, this, coin_type| {
            Ok(this.theCoinType = coin_type)
        });

        methods.add_method("GetCoinMotion", |_, this, ()| Ok(this.theCoinMotion));

        methods.add_method_mut("SetCoinMotion", |_, this, coin_motion| {
            Ok(this.theCoinMotion = coin_motion)
        });
    }
}

/// 在游戏中生成掉落物的函数
pub extern "thiscall" fn AddCoin(this: *mut Board, args: ArgsAddCoin) -> *mut Coin {
    let mut args = args;

    callback_data(PRE | ADDR_ADDCOIN, &mut args);
    trace!(
        "产生掉落物 {} at {:?} with motion {}",
        args.theCoinType, args.pos, args.theCoinMotion
    );

    let coin = ORIGINAL_ADDCOIN.wait()(this, args.pos, args.theCoinType, args.theCoinMotion);

    coin
}
add_callback!("AT_NEW_COIN", PRE | ADDR_ADDCOIN);

/// `Board::KeyDown` 的 hook 函数
pub extern "thiscall" fn KeyDown(this: *mut Board, keycode: i32) {
    trace!("Board({:#x?}) 按下 {:#x}", this, keycode);
    callback(PRE | ADDR_KEYDOWN, keycode);

    // 回调
    ORIGINAL_KEYDOWN.wait()(this, keycode);
}
add_callback!("AT_BOARD_KEY_DOWN", PRE | ADDR_KEYDOWN);

#[repr(C)]
pub struct ArgsAddZombieInRow {
    theZombieType: i32,
    theFromWave: i32,
    this: *mut Board,
    theRow: i32,
}

impl LuaUserData for ArgsAddZombieInRow {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_field_mut!(fields, "row", theRow);
        add_field_mut!(fields, "zombie_type", theZombieType);
        add_field_mut!(fields, "from_wave", theFromWave);
    }
}

pub extern "stdcall" fn AddZombieInRow(args: ArgsAddZombieInRow) -> *mut Zombie {
    let mut args = args;
    callback_data(PRE | ADDR_ADD_ZOMBIE_IN_ROW, &mut args);
    trace!(
        "在第 {} 波 行 {} 生成僵尸 类型 {}",
        args.theFromWave, args.theRow, args.theZombieType
    );

    AddZombieInRowWrapper(args.this, args.theZombieType, args.theRow, args.theFromWave)
}
add_callback!("AT_NEW_ZOMBIE", PRE | ADDR_ADD_ZOMBIE_IN_ROW);

/// 关卡内鼠标点击
pub extern "thiscall" fn MouseDown(this: *mut Board, pos: Vec2<i32>, theClickCount: i32) {
    trace!("Board({:#x?}) 在 {:?} 点击 {}", this, pos, theClickCount);
    callback(PRE | ADDR_MOUSE_DOWN, (theClickCount, pos));

    ORIGINAL_MOUSE_DOWN.wait()(this, pos, theClickCount)
}
add_callback!("AT_BOARD_MOUSE_DOWN", PRE | ADDR_MOUSE_DOWN);

/// 关卡内鼠标松开
pub extern "thiscall" fn MouseUp(this: *mut Board, pos: Vec2<i32>, theClickCount: i32) {
    trace!("Board({:#x?}) 在 {:?} 松开 {}", this, pos, theClickCount);
    callback(PRE | ADDR_MOUSE_UP, (theClickCount, pos));

    ORIGINAL_MOUSE_UP.wait()(this, pos, theClickCount)
}
add_callback!("AT_BOARD_MOUSE_UP", PRE | ADDR_MOUSE_UP);

/// 关卡更新
pub extern "thiscall" fn Update(this: *mut Board) {
    let delta = get_delta_mgr().update_delta("Board");
    ORIGINAL_UPDATE.wait()(this);
    callback(ADDR_UPDATE, delta);
}
add_callback!("AT_BOARD_UPDATE", POST | ADDR_UPDATE);
