pub mod zombie;

use tracing::trace;
use windows::core::BOOL;

use crate::{
    add_callback, 
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
    // trace!("alloc zombie");
    DataArrayAllocWrapper(this)
}


pub extern "stdcall" fn ZombieInitialize(
    this: *mut Zombie,
    theRow: i32,
    theZombieType: i32,
    theVariant: BOOL,
    theParentZombie: *mut Zombie,
    theFromWave: i32,
) {

    // trace!("初始化 行 {} 类型 {} 来自第 {} 波", theRow, theZombieType, theFromWave);
    ZombieInitializeWrapper(
        this,
        theRow,
        theZombieType,
        theVariant,
        theParentZombie,
        theFromWave
    );
    unsafe {
        let zombie = &mut *this;

        callback_data(POST | ADDR_ZOMBIE_INITIALIZE, zombie);
        trace!("初始化僵尸 {:#x?}", this);
    }
}
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
