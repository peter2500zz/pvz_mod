pub mod lawn_app;
pub mod loading;

use tracing::{debug, info, trace};

use crate::{hook::pvz::lawn_app::{
    ORIGINAL_CONSTRUCTOR, 
    ORIGINAL_DESTRUCTOR, 
    ORIGINAL_INIT, 
    ORIGINAL_LOST_FOCUS
}, pvz::lawn_app::lawn_app::LawnApp};

/// 这是 `LawnApp` 的构造函数
pub extern "stdcall" fn Constructor(
    uninit: *mut LawnApp
) -> *mut LawnApp {
    trace!("构造 LawnApp");

    // unsafe {
        // info!("{:?}", (0x6A9EC0 as *mut usize).is_null());
        // info!("{:?}", *(0x6A9EC0 as *mut usize));
    // }

    let this = ORIGINAL_CONSTRUCTOR.wait()(
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

    ORIGINAL_DESTRUCTOR.wait()(
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

    ORIGINAL_INIT.wait()(
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
    let _ = ORIGINAL_LOST_FOCUS;
    // 仙布暂停
    // ORIGINAL_LAWNAPP_LOST_FOCUS.wait()(
    //     this
    // );

}
