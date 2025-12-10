use std::ptr;

use mlua::prelude::*;

use crate::{mods::LuaRegistration, pvz::{
    board::{
        AddCoin, 
        AddZombieInRow, 
        ArgsAddCoin, 
        ArgsAddZombieInRow
    }, 
    data_array::DataArray, 
    lawn_app::lawn_app::get_lawn_app, 
    zombie::zombie::Zombie
}};


// inventory::submit! {
//     LuaRegistration(|lua| {

//         let globals = lua.globals();

//         let log_table = lua.create_table()?;

//         globals.set("Log", log_table)?;

//         Ok(())
//     })
// }

#[derive(Debug)]
#[repr(C)]
/// 这是 `Board`
pub struct Board {
    _pad_0x0_0x58: [u8; 0x58 - 0x0],
    /// 0x58 鼠标是否按下（暂停不再记录）
    pub mouse_pressing: bool,
    _pad_0x59_0x90: [u8; 0x90 - 0x59],
    /// 0x90 僵尸数据
    pub zombies: DataArray<Zombie>,
    _pad_0xAC_0x5560: [u8; 0x5560 - 0xAC],
    /// 0x5560 阳光值
    pub sun_value: i32,
    _pad_0x5564_0x57B0: [u8; 0x57B0 - 0x5564],
}
const _: () = assert!(size_of::<Board>() == 0x57B0);

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();
        let mouse_codes = lua.create_table()?;

        mouse_codes.set("L_CLICK", 1)?; // 鼠标左键
        mouse_codes.set("L_DOUBLE_CLICK", 2)?; // 鼠标左键双击
        mouse_codes.set("R_CLICK", -1)?; // 鼠标右键
        mouse_codes.set("R_DOUBLE_CLICK", -2)?; // 鼠标右键双击
        mouse_codes.set("M_CLICK", 3)?; // 鼠标中键

        globals.set("MouseCodes", mouse_codes)?;

        Ok(())
    })
}


pub fn get_board() -> Option<*mut Board> {
    unsafe {
        get_lawn_app().and_then(|lawn_app| {
            if ((*lawn_app).board as u32) == 0 {
                None
            } else {
                Some((*lawn_app).board)
            }
        })
    }
}

pub fn with_board<T>(f: impl FnOnce(&mut Board) -> T) -> LuaResult<T> {
    get_board()
        .map(|board| unsafe { f(&mut *board) })
        .ok_or_else(|| LuaError::MemoryError("Board 不可访问".to_string()))
}

impl LuaUserData for Board {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("MousePressing", |_, _, ()| {
            with_board(|board| board.mouse_pressing)
        });

        methods.add_method("SetSun", |_, _, value: i32| {
            with_board(|board| board.sun_value = value)
        });

        methods.add_method("AddZombie", |_, _, (zombie_type, row, from_wave)| {
            with_board(|board| {
                let zombie = AddZombieInRow(ArgsAddZombieInRow {
                    theZombieType: zombie_type,
                    theFromWave: from_wave,
                    this: board,
                    theRow: row,
                });

                unsafe {
                    ptr::read(zombie)
                }
            })
        });

        methods.add_method("AddCoin", |_, _, (pos, theCoinType, theCoinMotion)| {
            with_board(|board| {
                let coin = AddCoin(
                    board,
                    ArgsAddCoin {
                        pos,
                        theCoinType: theCoinType,
                        theCoinMotion: theCoinMotion,
                    }
                );

                unsafe {
                    ptr::read(coin)
                }
            })
        });
    }

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("sun", |_, _| {
            with_board(|board| board.sun_value)
        });

        fields.add_field_method_set("sun", |_, _, value| {
            with_board(|board| board.sun_value = value)
        });
    }
}
