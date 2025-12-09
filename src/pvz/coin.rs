
use std::ffi::c_void;
use mlua::prelude::*;
use tracing::trace;
use windows::core::BOOL;

use crate::{
    add_field_mut, hook::pvz::coin::{
        CoinInitializeWrapper, 
        DataArrayAllocWrapper
    }, mods::LuaRegistration, pvz::data_array::DataArray
};


#[repr(C)]
pub struct Coin {
    /// 0x0 基址
    pub base_address: *mut c_void,
    /// 0x4 当前游戏信息和对象
    pub current_game_info: *mut c_void,
    /// 0x8 图像X坐标变化量
    pub image_x_offset: i32,
    /// 0xC 图像Y坐标变化量
    pub image_y_offset: i32,
    /// 0x10 判定宽度
    pub hit_box_width: i32,
    /// 0x14 判定高度
    pub hit_box_height: i32,
    /// 0x18 true则物品隐形
    pub is_invisible: BOOL,
    /// 0x1C 所在行数
    pub row: i32,
    /// 0x20 图层
    pub layer: i32,
    /// 0x24 X坐标
    pub x: f32,
    /// 0x28 Y坐标
    pub y: f32,
    /// 0x2C X坐标变化量
    pub velocity_x: f32,
    /// 0x30 Y坐标变化量
    pub velocity_y: f32,
    /// 0x34 大小
    pub scale: f32,
    /// 0x38 true则物品消失
    pub is_disappearing: BOOL,
    /// 0x3C true则物品消失
    pub is_disappearing2: BOOL,
    /// 0x40 收集后变为[24]
    pub collected_x: i32,
    /// 0x44 收集后变为[28]
    pub collected_y: i32,
    /// 0x48 物品要移动到的Y坐标
    pub target_y: i32,
    /// 0x4C 物品已存在时间
    pub time_alive: i32,
    /// 0x50 true则被收集
    pub is_collected: BOOL,
    /// 0x54 消失计时
    pub disappear_countdown: i32,
    /// 0x58 物品类型
    pub coin_type: i32,
    /// 0x5C 物品运动状态
    pub coin_motion_type: i32,
    /// 0x60 动画附件ID
    pub attachment_id: i32,
    /// 0x64 目标距离
    pub target_distance: i32,
    /// 0x68 植物卡牌类型
    pub plant_type: i32,
    /// 0x6C~0xC0 花园盆栽内容
    pub potted_plant_data: [u8; 0x55],
    _padding_c1: [u8; 7],
    /// 0xC8 true则有光环
    pub has_glow: bool,
    /// 0xC9 [C8]光环
    pub glow: bool,
    /// 0xCA 是否落地
    pub is_on_ground: bool,
    _padding_cb: [u8; 1],
    /// 0xCC 落下时间
    pub fall_time: i32,
    _padding_d0: [u8; 4],
    /// 0xD4 物品ID(结构为[序列号,编号],序列号与编号各占2字节)
    pub coin_id: i32,
}

inventory::submit! {
    LuaRegistration(|lua| {
        let global = lua.globals();
        let coin_types = lua.create_table()?;

        coin_types.set("SILVER_COIN", 1)?;  // 银币
        coin_types.set("GOLD_COIN", 2)?;  // 金币
        coin_types.set("DIAMOND", 3)?;  // 钻石
        coin_types.set("SUN", 4)?;  // 太阳
        coin_types.set("SMALL_SUN", 5)?;  // 小太阳
        coin_types.set("LARGE_SUN", 6)?;  // 大太阳
        coin_types.set("SEED_PACKET", 7)?;  // 植物卡片
        coin_types.set("TROPHY", 8)?;  // 奖杯
        coin_types.set("SHOVEL", 9)?;  // 铲子
        coin_types.set("ALMANAC", 10)?;  // 图鉴
        coin_types.set("KEY", 11)?;  // 钥匙
        coin_types.set("VASE", 12)?;  // 花瓶
        coin_types.set("WATERING_CAN", 13)?;  // 洒水壶
        coin_types.set("SANDWICH", 14)?;  // 三明治
        coin_types.set("NOTE", 15)?;  // 便条/遗书
        coin_types.set("VANISH_PLACEHOLDER", 16)?;  // 立即消失(占位)
        coin_types.set("SEEDLING_GIFT", 17)?;  // 花苗礼盒
        coin_types.set("COIN_BAG", 18)?;  // 金币袋
        coin_types.set("GIFT_BOX_PERSISTENT", 19)?;  // 礼盒(不消失)
        coin_types.set("COIN_BAG_PERSISTENT", 20)?;  // 金币袋(不消失)
        coin_types.set("SILVER_TROPHY", 21)?;  // 银奖杯
        coin_types.set("GOLD_TROPHY", 22)?;  // 金奖杯
        coin_types.set("CHOCOLATE", 23)?;  // 巧克力
        coin_types.set("CHOCOLATE_PERSISTENT", 24)?;  // 巧克力(不消失)
        coin_types.set("GIFT_BOX_MINI_GAMES", 25)?;  // 礼品盒(小游戏)
        coin_types.set("GIFT_BOX_PUZZLE", 26)?;  // 礼品盒(解密模式)
        coin_types.set("GIFT_BOX_SURVIVAL", 27)?;  // 礼品盒(生存模式)

        global.set("CoinTypes", coin_types)?;

        let coin_motions = lua.create_table()?;

        coin_motions.set("DROP_FROM_XY", 0)?;  // 从坐标落下
        coin_motions.set("SLOW_DROP_FROM_XY", 1)?;  // 从坐标缓慢落下
        coin_motions.set("POP_FROM_BACK", 2)?;  // 从后方跳出
        coin_motions.set("FAST_POP_FROM_BACK", 3)?;  // 从后方快速跳出
        coin_motions.set("COLLECT_IMMEDIATELY", 4)?;  // 直接收集
        coin_motions.set("AUTO_COLLECT_LATER", 5)?;  // 稍后自动收集
        coin_motions.set("POP_FROM_RIGHT", 6)?;  // 从屏幕右侧蹦出
        coin_motions.set("SPAWN_IN_SEED_SLOT", 7)?;  // 在卡槽栏生成

        global.set("CoinMotions", coin_motions)?;

        Ok(())
    })
}

impl LuaUserData for Coin {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // // x
        // fields.add_field_method_get("x", |_, this| Ok(this.x));
        // fields.add_field_method_set("x", |_, this, val| Ok(this.x = val));
        add_field_mut!(fields, "x", x);
        add_field_mut!(fields, "y", y);
        add_field_mut!(fields, "coin_type", coin_type);

    }
}

/// `DataArray::DataArrayAlloc` 的 hook 函数
pub extern "stdcall" fn DataArrayAlloc(
    this: *mut DataArray<Coin>,
) -> *mut Coin {
    trace!("alloc coin");
    DataArrayAllocWrapper(this)
}

pub extern "stdcall" fn CoinInitialize(
    this: *mut Coin,
    theCoinMotion: i32,
    theCoinType: i32,
    theX: i32,
    theY: i32,
) {
    // 

    // trace!("初始化 类型 {} 运动方式 {} 位置 ({}, {})", args.theCoinType, args.theCoinMotion, args.theX, args.theY);
    CoinInitializeWrapper(
        this,
        theX,
        theY,
        theCoinType,
        theCoinMotion,
    );


    // // callback_mut(ADDR_COIN_INITIALIZE, );

    // unsafe {
    //     let coin = &mut (*args.this);

    //     callback(ADDR_COIN_INITIALIZE, coin);

    // }
}
