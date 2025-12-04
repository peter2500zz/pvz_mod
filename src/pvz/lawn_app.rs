pub mod loading;

use tracing::{debug, trace};

use crate::hook::pvz::lawn_app::{
    ORIGINAL_LAWNAPP_CONSTRUCTOR, 
    ORIGINAL_LAWNAPP_DESTRUCTOR, 
    ORIGINAL_LAWNAPP_INIT, 
    ORIGINAL_LAWNAPP_LOST_FOCUS
};

#[derive(Debug)]
#[repr(C)]
/// 这是 `LawnApp`
/// 
/// 手动管理生命周期并不好玩，孩子们
pub struct LawnApp {
    _pad: [u8; 0x8C8],  
}

/// 这是 `LawnApp` 的构造函数
pub extern "stdcall" fn Constructor(
    uninit: *mut LawnApp
) -> *mut LawnApp {
    trace!("构造 LawnApp");

    let this = ORIGINAL_LAWNAPP_CONSTRUCTOR.wait()(
        uninit
    );

    trace!("地址 {:#x?}", this);

    this
}

/// 这是 `LawnApp` 的析构函数
pub extern "thiscall" fn Destructor(
    this: *mut LawnApp
) {
    trace!("析构 LawnApp");

    ORIGINAL_LAWNAPP_DESTRUCTOR.wait()(
        this
    );
}

/// `LawnApp` 的初始化函数
/// 
/// 包括读取设定数据及存档、加载资源、创建标题界面及初始化游戏内的各个系统等
pub extern "thiscall" fn Init(
    this: *mut LawnApp
) {
    trace!("初始化 LawnApp");

    ORIGINAL_LAWNAPP_INIT.wait()(
        this
    );
}

/// 程序窗口失去焦点
/// 
/// 如果能暂停且没有启用作弊会暂停，除此之外没有别的作用
pub extern "thiscall" fn LostFocus(
    this: *mut LawnApp
) {
    debug!("游戏失去焦点");

    let _ = this;
    let _ = ORIGINAL_LAWNAPP_LOST_FOCUS;
    // 仙布暂停
    // ORIGINAL_LAWNAPP_LOST_FOCUS.wait()(
    //     this
    // );

}
