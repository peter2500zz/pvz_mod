use std::ffi::c_void;
use windows::core::BOOL;
use mlua::prelude::*;

use crate::{
    pvz::{
        board::board::get_board, 
        data_array::HasId
    }, utils::{Rect2, Vec2}
};

#[repr(C)]
#[derive(Debug)]
pub struct Zombie {
    /// 0x0 基址
    pub base_address: *mut c_void,
    /// 0x4 当前游戏信息和对象
    pub current_game_info: *mut c_void,
    /// 0x8 贴图尺寸
    pub image_rect: Rect2<i32>,
    /// 0x18 为false时隐形
    pub is_visible: BOOL,
    /// 0x1C 所在行数
    pub row: i32,
    /// 0x20 图像图层
    pub image_layer: i32,
    /// 0x24 僵尸类型
    pub zombie_type: i32,
    /// 0x28 僵尸状态
    pub zombie_state: i32,
    /// 0x2C 坐标
    pub pos: Vec2<f32>,
    /// 0x34 相对速度/僵尸水族馆中表示合速度
    pub velocity_x: f32,
    /// 0x38 不断增大直到大于[40]*[44]的值变回0
    pub animation_counter: i32,
    /// 0x3C 僵尸喊brians的倒计时初始值范围[500,1500)
    pub brains_countdown_initial: i32,
    /// 0x40 无动画抖动帧计时
    pub frame_timer: i32,
    /// 0x44 无动画抖动帧倒计时
    pub frame_countdown: i32,
    /// 0x48 无动画抖动帧索引
    pub frame_index: i32,
    /// 0x4C 上一个无动画抖动帧索引
    pub last_frame_index: i32,
    /// 0x50 是否有舌头
    pub has_tongue: bool,
    /// 0x51 啃食时为true
    pub is_eating: bool,
    _padding_52: [u8; 2],
    /// 0x54 僵尸闪光倒计时
    pub flash_countdown: i32,
    /// 0x58 2类饰品发光倒计时
    pub accessory2_glow_countdown: i32,
    /// 0x5C 2类饰品抖动倒计时
    pub accessory2_shake_countdown: i32,
    /// 0x60 僵尸已存在时间
    pub time_alive: i32,
    /// 0x64 僵尸运动状态
    pub movement_state: i32,
    /// 0x68 属性倒计时
    pub attribute_countdown: i32,
    /// 0x6C 出生波数(从0开始)/站立状态,为-2、-3时静止,-4时向上(对于选卡界面的僵尸)
    pub spawn_wave: i32,
    /// 0x70 0时不掉落物品
    pub drop_item_flag: i32,
    /// 0x74 僵尸消失倒计时
    pub disappear_countdown: i32,
    /// 0x78 冰车正在死亡则为 true（篮球有效吗？不明）
    pub is_dying: BOOL,
    /// 0x7C 爬的梯子所在列
    pub ladder_col: i32,
    /// 0x80 蹦极僵尸所在列/僵王最左侧蹦极所在列/僵王砸车左上角格子所在列
    pub bungee_col: i32,
    /// 0x84 纵向偏移(僵尸实际y等价于[纵坐标]-[纵向偏移])
    pub y_offset: f32,
    /// 0x88 蹦极是否被保护伞挡住
    pub is_blocked_by_umbrella: BOOL,
    /// 0x8C 中弹判定
    pub hitbox_rect: Rect2<i32>,
    /// 0x9C 攻击判定
    pub attackbox_rect: Rect2<i32>,
    /// 0xAC 减速倒计时
    pub slow_countdown: i32,
    /// 0xB0 黄油固定倒计时
    pub butter_countdown: i32,
    /// 0xB4 冻结倒计时
    pub freeze_countdown: i32,
    /// 0xB8 被魅惑则为true
    pub is_charmed: bool,
    /// 0xB9 被吹走则为true
    pub is_blown_away: bool,
    /// 0xBA 非濒死状态则为true
    pub is_not_dying: bool,
    /// 0xBB 没断手则为true
    pub has_arm: bool,
    /// 0xBC 存在手持物,雪人向左走则为true
    pub has_item_or_yeti_left: bool,
    /// 0xBD 在水中则为true
    pub in_water: bool,
    /// 0xBE 上梯子时的影子跟随
    pub ladder_shadow_follow: u8,
    /// 0xBF 吃到大蒜则为true
    pub ate_garlic: bool,
    /// 0xC0 吃完大蒜倒计时
    pub garlic_countdown: i32,
    /// 0xC4 1类饰品(0没有1路障2铁桶3橄榄球4矿工帽7雪橇车8坚果9高坚果)
    pub helmet_type: i32,
    /// 0xC8 当前本体血量
    /// 
    /// `0x0052A52D` 如果血量小于血量上限/3且非濒死，死亡
    pub body_hp: i32,
    /// 0xCC 本体血量上限
    pub body_max_hp: i32,
    /// 0xD0 1类饰品当前血量
    pub helmet_hp: i32,
    /// 0xD4 1类饰品血量上限
    pub helmet_hp_max: i32,
    /// 0xD8 2类饰品
    pub shield_type: i32,
    /// 0xDC 2类饰品当前血量
    pub shield_hp: i32,
    /// 0xE0 2类饰品血量上限
    pub shield_hp_max: i32,
    /// 0xE4 气球当前血量
    pub balloon_hp: i32,
    /// 0xE8 气球血量上限
    pub balloon_hp_max: i32,
    /// 0xEC 消失则为true
    pub is_dead: BOOL,
    /// 0xF0 雪橇队领头僵尸ID/舞王ID
    pub bobsled_leader_id: i32,
    /// 0xF4 雪橇队第二只僵尸/舞王第一只伴舞ID/僵王第一只蹦极ID
    pub bobsled_member2_id: i32,
    /// 0xF8 雪橇队第三只僵尸/舞王第二只伴舞ID/僵王第二只蹦极ID
    pub bobsled_member3_id: i32,
    /// 0xFC 雪橇队第四只僵尸/舞王第三只伴舞ID/僵王第三只蹦极ID
    pub bobsled_member4_id: i32,
    /// 0x100 舞王第四只伴舞ID
    pub dancer_member4_id: i32,
    /// 0x104 haveUniqueSample(翻译不过来摆烂了)
    pub have_unique_sample: BOOL,
    /// 0x108 粒子坐标
    pub particle_pos: Vec2<i32>,
    /// 0x110 受到子弹攻击的动画附件ID
    pub bullet_hit_attachment_id: i32,
    /// 0x114 僵王放僵尸倒计时/舞王召唤倒计时/僵尸水族馆生产阳光倒计时/篮球剩余数量
    pub special_countdown: i32,
    /// 0x118 僵尸本体动画ID
    pub body_anim_id: i32,
    /// 0x11C 大小
    pub scale: f32,
    /// 0x120 [+84]变化量(僵尸纵向偏移速度)/僵尸水族馆中表示角度(弧度制)
    pub y_offset_velocity: f32,
    /// 0x124 僵王运动速度
    pub gargantuar_velocity: i32,
    /// 0x128 蹦极手里的植物
    pub bungee_grabbed_plant: i32,
    /// 0x12C 僵王根据损伤度判断是否放蹦极或砸车
    pub boss_damage_threshold: i32,
    /// 0x130 僵王放僵尸的行数/僵王砸车左上角格子所在的行数/僵王跺脚偏上行的行数
    pub boss_spawn_row: i32,
    /// 0x134 僵王放蹦极或砸车倒计时
    pub boss_action_countdown: i32,
    /// 0x138 僵王跺脚倒计时
    pub boss_stomp_countdown: i32,
    /// 0x13C 僵王伸头倒计时
    pub boss_head_countdown: i32,
    /// 0x140 僵王冰火球动画ID(没有冰火球时为0)
    pub boss_fireball_anim_id: i32,
    /// 0x144 植物僵尸头部动画ID/旗帜僵尸旗帜动画ID
    pub head_anim_id: i32,
    /// 0x148 僵王冰火球的行数
    pub boss_fireball_row: i32,
    /// 0x14C 球的类型,冰球则为0
    pub fireball_type: i32,
    /// 0x150 被小推车碾压时动画ID
    pub squashed_anim_id: i32,
    /// 0x154 上一次穿过传送门的X坐标
    pub last_portal_x: i32,
    /// 0x158 僵尸ID(结构为[序列号,编号],序列号与编号各占2字节)
    id: i32,
}
const _: () = assert!(size_of::<Zombie>() == 0x15C);

impl HasId for Zombie {
    fn id(&self) -> i32 {
        self.id
    }
}

/// 尝试通过索引从 Board 中的 zombies 对象池中获取僵尸指针
/// 
/// 如果无法访问僵尸会返回 None
pub fn get_zombie(id: i32) -> Option<*mut Zombie> {
    get_board().and_then(|board| unsafe {
        (*board).zombies.get_mut(id).map(|zombie| zombie as *mut Zombie)
    })
}

/// 尝试通过索引从 Board 中的 zombies 对象池中获取僵尸并执行操作
/// 
/// 如果无法访问僵尸会返回错误
pub fn with_zombie<T>(id: i32, f: impl FnOnce(&mut Zombie) -> T) -> LuaResult<T> {
    get_zombie(id)
        .map(|zombie| unsafe { f(&mut *zombie) })
        .ok_or_else(|| LuaError::MemoryError(format!("Zombie({}) 不可访问", id)))
}

impl LuaUserData for Zombie {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 如果僵尸被从内存里清理掉了，就给 false
        methods.add_method("IsValid", |_, this, ()| {
            Ok(get_zombie(this.id()).is_some())
        });

        methods.add_method("GetPos", |_, this, ()| {
            with_zombie(this.id(), |zombie| {
                zombie.pos
            })
        });

        methods.add_method("SetPos", |_, this, pos| {
            with_zombie(this.id(), |zombie| {
                zombie.pos = pos;
            })
        });
    }

    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // 由于 UserData 给 Lua 后会锁定状态，虽然对大多数字段不合适，但 this.id 不变，太棒了
        fields.add_field_method_get("body_hp", |_, this| {
            with_zombie(this.id(), |zombie| {
                zombie.body_hp
            })
        });


    }
}


