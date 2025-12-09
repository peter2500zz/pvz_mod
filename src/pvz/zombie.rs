pub mod zombie;

use mlua::prelude::*;
use tracing::trace;
use windows::core::BOOL;

use crate::{
    add_callback, 
    add_field_mut, 
    hook::pvz::zombie::{
        ADDR_UPDATE, 
        ADDR_ZOMBIE_INITIALIZE, 
        DataArrayAllocWrapper, 
        UpdateWrapper, 
        ZombieInitializeWrapper
    }, 
    mods::callback::{
        POST, 
        PRE, 
        callback_data
    }, 
    pvz::{
        data_array::DataArray, 
        zombie::zombie::Zombie
    }
};




/// `DataArray::DataArrayAlloc` 的 hook 函数
pub extern "stdcall" fn DataArrayAlloc(
    this: *mut DataArray<Zombie>,
) -> *mut Zombie {
    trace!("alloc zombie");
    DataArrayAllocWrapper(this)
}

#[repr(C)]
pub struct ArgsZombieInitialize {
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
}

impl LuaUserData for ArgsZombieInitialize {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        add_field_mut!(fields, "row", theRow);
        add_field_mut!(fields, "zombie_type", theZombieType);
        
        add_field_mut!(fields, "from_wave", theFromWave);
    }
}

pub extern "stdcall" fn ZombieInitialize(
    args: ArgsZombieInitialize
) {
    let mut args = args;
    callback_data(PRE | ADDR_ZOMBIE_INITIALIZE, &mut args);

    trace!("初始化 行 {} 类型 {} 来自第 {} 波", args.theRow, args.theZombieType, args.theFromWave);
    ZombieInitializeWrapper(
        args.this,
        args.theRow,
        args.theZombieType,
        args.theVariant,
        args.theParentZombie,
        args.theFromWave
    );
    unsafe {
        let zombie = &mut *args.this;

        callback_data(POST | ADDR_ZOMBIE_INITIALIZE, zombie);
    }
}
add_callback!("AT_NEW_ZOMBIE", PRE | ADDR_ZOMBIE_INITIALIZE);
add_callback!("AT_ZOMBIE_INIT", POST | ADDR_ZOMBIE_INITIALIZE);


pub extern "stdcall" fn Update(
    this: *mut Zombie,
) {
    unsafe {
        let zombie = &mut *this;

        callback_data(PRE | ADDR_UPDATE, zombie);
    }
    UpdateWrapper(this);
}
add_callback!("AT_ZOMBIE_UPDATE", PRE | ADDR_UPDATE);

