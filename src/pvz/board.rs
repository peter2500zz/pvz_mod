
use std::ffi::c_void;

use tracing::{debug, trace};
use windows::core::BOOL;

use crate::{
    hook::pvz::board::{
            ORIGINAL_BOARD_CONSTRUCTOR, 
            ORIGINAL_BOARD_DESTRUCTOR, 
            ORIGINAL_BOARD_INIT_LEVEL, 
            ORIGINAL_BOARD_KEYDOWN
        }, 
    pvz::{
        data_array::DataArray, lawn_app::LawnApp, zombie::{self, Zombie}
    }
};

#[derive(Debug)]
#[repr(C)]
/// 这是 `Board`
pub struct Board {
    /// 0x0000 填充
    pub _pad_0x00_0x28: [u8; 0x28 - 0x00],
    /// 0x0028 窗口刷新次数
    pub refresh_count: i32,
    /// 0x002C 填充
    pub _pad_0x2c_0x30: [u8; 0x30 - 0x2C],
    /// 0x0030 画面横坐标(向左递增,正常游戏时为0)
    pub screen_x: i32,
    /// 0x0034 画面纵坐标(向下递增,正常游戏时为0)
    pub screen_y: i32,
    /// 0x0038 可点击的横坐标范围
    pub clickable_width: i32,
    /// 0x003C 可点击的纵坐标范围
    pub clickable_height: i32,
    /// 0x0040 填充
    pub _pad_0x40_0x54: [u8; 0x54 - 0x40],
    /// 0x0054 [逻辑值]true则显示画面
    pub is_show_screen: BOOL,
    /// 0x0058 [逻辑值]鼠标按下则为true(暂停时为0并不再记录)
    pub is_mouse_down: BOOL,
    /// 0x0059 [逻辑值]鼠标在画面内则为true(暂停时为0并不再记录)
    pub is_mouse_in_screen: bool,
    /// 0x005A 填充
    pub _pad_0x5a_0x8c: [u8; 0x8C - 0x5A],
    /// 0x008C [指针]=基址
    pub base_addr: *mut c_void,
    /// 0x0090 僵尸数组
    pub zombies: DataArray<Zombie>,
    /// 0x00AC 植物数组
    pub plants: DataArray<c_void>,
    /// 0x00C8 子弹数组
    pub bullets: DataArray<c_void>,
    /// 0x00E4 物品数组
    pub items: DataArray<c_void>,
    /// 0x0100 小推车数组
    pub carts: DataArray<c_void>,
    /// 0x011C 场地物品数组
    pub grid_items: DataArray<c_void>,
    /// 0x0138 [指针]鼠标相关属性
    pub mouse_attribs: *mut c_void,
    /// 0x013C [指针]鼠标额外属性
    pub mouse_extra_attribs: *mut c_void,
    /// 0x0140 [指针]文字属性
    pub text_attribs: *mut c_void,
    /// 0x0144 [指针]卡槽属性
    pub seed_bank_attribs: *mut c_void,
    /// 0x0148 [指针]Menu[按钮]属性
    pub menu_btn_attribs: *mut c_void,
    /// 0x014C [指针]LS、商店[按钮]属性(Start Onslaught)
    pub store_btn_attribs: *mut c_void,
    /// 0x0150 [逻辑值]true则鼠标不显示手型
    pub is_cursor_not_hand: BOOL,
    /// 0x0154 鼠标悬浮标签属性
    pub cursor_label_attribs: i32, // 这里未标记是指针，且大小未指定，通常是ID或Flags，暂作i32
    /// 0x0158 填充 (指针通常4字节对齐，前一个154是i32结束于158，此处无缝隙，但为确保逻辑正确检查偏移)
    // 154(4) -> 158. Next is 15C. No padding needed.
    /// 0x015C [指针]选卡界面属性（时间单位为 ms ?）
    pub seed_chooser_screen_ptr: *mut c_void,
    /// 0x0160 [指针]小游戏等属性
    pub mini_game_attribs: *mut c_void,
    /// 0x0164 [逻辑值]true则游戏暂停
    pub is_game_paused: BOOL,
    /// 0x0168~23C 场景格子类型
    pub scene_grid_types: [u8; 0x23C - 0x168],
    /// 0x023C 填充
    pub _pad_0x23c_0x240: [u8; 0x240 - 0x23C],
    /// 0x0240~314 雾的形状/墓碑形状
    pub fog_grid_shapes: [u8; 0x314 - 0x240],
    /// 0x0314 填充
    pub _pad_0x314_0x318: [u8; 0x318 - 0x314],
    /// 0x0318~4c4 墓碑的横、纵偏移
    pub grave_grid_offsets: [u8; 0x4C4 - 0x318],
    /// 0x04C4 填充
    pub _pad_0x4c4_0x4c8: [u8; 0x4C8 - 0x4C4],
    /// 0x04C8~5C0 雾的浓度
    pub fog_grid_density: [u8; 0x5C0 - 0x4C8],
    /// 0x05C0 填充
    pub _pad_0x5c0_0x5d0: [u8; 0x5D0 - 0x5C0],
    /// 0x05D0 [浮点]浓雾偏移，三叶草开雾距离
    pub fog_offset: f32,
    /// 0x05D4 浓雾倒计时
    pub fog_countdown: i32,
    /// 0x05D8~5EC 每行出怪类型
    pub spawn_row_types: [u8; 0x5EC - 0x5D8],
    /// 0x05EC 填充
    pub _pad_0x5ec_0x60c: [u8; 0x60C - 0x5EC],
    /// 0x060C~620 每行冰道坐标
    pub ice_trail_coords: [u8; 0x620 - 0x60C],
    /// 0x0620 填充
    pub _pad_0x620_0x624: [u8; 0x624 - 0x620],
    /// 0x0624~638 每行冰道消失倒计时
    pub ice_trail_countdown: [u8; 0x638 - 0x624],
    /// 0x0638 填充
    pub _pad_0x638_0x63c: [u8; 0x63C - 0x638],
    /// 0x063C~650 每行冰道粒子系统ID
    pub ice_trail_particle_id: [u8; 0x650 - 0x63C],
    /// 0x0650 填充
    pub _pad_0x650_0x6b4: [u8; 0x6B4 - 0x650],
    /// 0x06B4~54D0 出怪列表(容纳10面旗帜)
    pub wave_list: [u8; 0x54D0 - 0x6B4],
    /// 0x54D0 填充
    pub _pad_0x54d0_0x54d4: [u8; 0x54D4 - 0x54D0],
    /// 0x54D4~54F4 [1字节]出怪种类
    pub wave_types: [u8; 0x54F4 - 0x54D4],
    /// 0x54F4 填充
    pub _pad_0x54f4_0x5538: [u8; 0x5538 - 0x54F4],
    /// 0x5538 掉落阳光倒计时
    pub sun_drop_countdown: i32,
    /// 0x553C 掉落阳光计数
    pub sun_drop_count: i32,
    /// 0x5540 非0则画面立刻回到正常
    pub screen_shake_reset_flag: i32,
    /// 0x5544 画面横向震动的幅度
    pub screen_shake_x: i32,
    /// 0x5548 画面纵向震动的幅度
    pub screen_shake_y: i32,
    /// 0x554C 场景类型
    pub scene_type: i32,
    /// 0x5550 (冒险模式)当前关卡
    pub level_id_adventure: i32,
    /// 0x5554 填充 (5550+4=5554, next 5558)
    pub _pad_0x5554_0x5558: [u8; 0x5558 - 0x5554],
    /// 0x5558 鼠标X坐标
    pub mouse_x: i32,
    /// 0x555C 鼠标Y坐标
    pub mouse_y: i32,
    /// 0x5560 阳光值
    pub sun_value: i32,
    /// 0x5564 当前关卡总波数
    pub total_waves: i32,
    /// 0x5568 游戏计时(不包括选卡停留的时间)
    pub game_time_no_pause: i32,
    /// 0x556C 游戏计时(包括选卡停留的时间)
    pub game_time_total: i32,
    /// 0x5570 游戏计时(失去焦点则重新计时)
    pub game_time_focus: i32,
    /// 0x5574 产生三人组倒计时
    pub trio_spawn_countdown: i32,
    /// 0x5578 填充
    pub _pad_0x5578_0x557c: [u8; 0x557C - 0x5578],
    /// 0x557C 当前所在波数
    pub current_wave: i32,
    /// 0x5580 已刷新的波数
    pub spawned_wave: i32,
    /// 0x5584 新手教程相关的闪烁提示
    pub tutorial_flash: i32,
    /// 0x5588 填充
    pub _pad_0x5588_0x5594: [u8; 0x5594 - 0x5588],
    /// 0x5594 达到刷新条件的血量
    pub zombie_health_trigger: i32,
    /// 0x5598 本波总血量
    pub wave_total_health: i32,
    /// 0x559C 下一波僵尸倒计时
    pub next_wave_countdown: i32,
    /// 0x55A0 下一波僵尸倒计时初始值
    pub next_wave_countdown_initial: i32,
    /// 0x55A4 大波僵尸刷新倒计时
    pub huge_wave_countdown: i32,
    /// 0x55A8 填充
    pub _pad_0x55a8_0x55ec: [u8; 0x55EC - 0x55A8],
    /// 0x55EC 出现红字时为41
    pub red_text_trigger: i32,
    /// 0x55F0 填充 (41可能是1字节? 但通常这里是int，且55F1是bool)
    /// 0x55EC(4) -> 55F0. Next 55F1.
    pub _pad_0x55f0_0x55f1: [u8; 0x55F1 - 0x55F0],
    /// 0x55F1 [逻辑值]true时有铲子
    pub has_shovel: bool,
    /// 0x55F2 填充
    pub _pad_0x55f2_0x55f4: [u8; 0x55F4 - 0x55F2],
    /// 0x55F4 金钱显示倒计时
    pub coin_display_countdown: i32,
    /// 0x55F8 用于程序调试(仅英文原版有效)
    pub debug_flag: i32,
    /// 0x55FC [逻辑值]true时退出关卡
    pub is_exit_level: BOOL,
    /// 0x5600 [逻辑值]true时为过关过程
    pub is_level_completed_process: BOOL,
    /// 0x5604 退出关卡倒计时
    pub exit_level_countdown: i32,
    /// 0x5608 填充
    pub _pad_0x5608_0x560c: [u8; 0x560C - 0x5608],
    /// 0x560C [逻辑值]true时为过关状态
    pub is_level_completed_state: BOOL,
    /// 0x5610 关卡进程的进度条
    pub level_progress_bar: i32,
    /// 0x5614 填充
    pub _pad_0x5614_0x5618: [u8; 0x5618 - 0x5614],
    /// 0x5618 水面冻结倒计时
    pub pool_ice_countdown: i32,
    /// 0x561C 生存模式出怪种子,非生存模式为程序窗口打开时间
    pub survival_seed_or_uptime: i32,
    /// 0x5620 粒子系统ID
    pub particle_system_id: i32,
    /// 0x5624~5740 辣椒火焰动画,每行12个动画，共6行
    pub jalapeno_anim: [u8; 0x5740 - 0x5624],
    /// 0x5740 填充
    pub _pad_0x5740_0x5744: [u8; 0x5744 - 0x5740],
    /// 0x5744 辣椒火焰倒计时
    pub jalapeno_countdown: i32,
    /// 0x5748 [逻辑值]true则画面变白
    pub is_flash_white: BOOL,
    /// 0x574C 填充 (5748+4=574C, next 5750)
    pub _pad_0x574c_0x5750: [u8; 0x5750 - 0x574C],
    /// 0x5750 减少到0产生音效
    pub sound_effect_countdown: i32,
    /// 0x5754 点炮后30cs倒计时,倒计时期间点炮位置和准心距离不能小于100
    pub cob_cannon_countdown: i32,
    /// 0x5758 点炮位置横坐标
    pub cob_cannon_x: i32,
    /// 0x575C 点炮位置纵坐标
    pub cob_cannon_y: i32,
    /// 0x5760 填充
    pub _pad_0x5760_0x5761: [u8; 0x5761 - 0x5760],
    /// 0x5761 [逻辑值]按下mustache则为true
    pub is_mustache: bool,
    /// 0x5762 [逻辑值]按下trickedout则为true
    pub is_trickedout: bool,
    /// 0x5763 [逻辑值]按下future则为true
    pub is_future: bool,
    /// 0x5764 [逻辑值]按下pinata则为true
    pub is_pinata: bool,
    /// 0x5765 [逻辑值]按下dance则为true
    pub is_dance: bool,
    /// 0x5766 [逻辑值]按下daisies则为true
    pub is_daisies: bool,
    /// 0x5767 [逻辑值]按下sukhbir则为true
    pub is_sukhbir: bool,
    /// 0x5768 填充
    pub _pad_0x5768_0x5790: [u8; 0x5790 - 0x5768],
    /// 0x5790 游戏开始时间(距离1970-1-1 8:00的秒数)
    pub game_start_time: i32,
    /// 0x5794 填充
    pub _pad_0x5794_0x5798: [u8; 0x5798 - 0x5794],
    /// 0x5798 被吃掉的植物数
    pub plants_eaten: i32,
    /// 0x579C 被铲掉的植物数
    pub plants_shoveled: i32,
    /// 0x57A0 收取金币数
    pub coins_collected: i32,
    /// 0x57A4 收取钻石数
    pub diamonds_collected: i32,
    /// 0x57A8 收取花盆数
    pub pots_collected: i32,
    /// 0x57AC 收取巧克力数
    pub chocolates_collected: i32,
    // 57AC + 4 = 57B0 (End)
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
    unsafe {
        trace!("初始化 Board 大小 {}", size_of_val(&*this));
    }

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
            let zombie = zombie::DataArrayAlloc(
                array
            );
            zombie::ZombieInitialize(
                zombie,
                0,
                0,
                false.into(),
                0 as _,
                0
            )
        }
        90 => {
            unsafe {
                let board = &*this;
                debug!("当前阳光 {} 鼠标坐标 ({}, {})", board.sun_value, board.mouse_x, board.mouse_y);
            }
        }
        _ => (),
    }

    // 回调
    ORIGINAL_BOARD_KEYDOWN.wait()(
        this, 
        keycode
    );
}
