use mlua::prelude::*;

use crate::{mods::LuaRegistration, pvz::lawn_app::lawn_app::get_lawn_app, utils::Vec2};

#[derive(Debug)]
#[repr(C)]
/// 这是 `WidgetManager`
pub struct WidgetManager {
    _pad_0x0_0xE0: [u8; 0xE0 - 0x0],
    /// 0xE0 鼠标坐标
    pub mouse_pos: Vec2<i32>,
    _pad_0xE8_0x1FC: [u8; 0x1FC - 0xE8],
}
const _: () = assert!(size_of::<WidgetManager>() == 0x1FC);

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();
        let key_codes = lua.create_table()?;

        key_codes.set("MODIFIERS", -65536)?; // 用于从键值中提取修饰键的位掩码
        key_codes.set("NONE", 0)?; // 没有按键按下
        key_codes.set("L_BUTTON", 1)?; // 鼠标左键
        key_codes.set("R_BUTTON", 2)?; // 鼠标右键
        key_codes.set("LR_BUTTON", 3)?; // 左右键一起按
        key_codes.set("M_BUTTON", 4)?; // 鼠标中键（三键鼠标）
        key_codes.set("X_BUTTON_1", 5)?; // 第一个 X 鼠标键（五键鼠标）
        key_codes.set("X_BUTTON_2", 6)?; // 第二个 X 鼠标键（五键鼠标）
        key_codes.set("LMR_BUTTON", 7)?; // 左中右键一起按
        key_codes.set("BACK", 8)?; // BACKSPACE 键
        key_codes.set("TAB", 9)?; // TAB 键
        key_codes.set("LINE_FEED", 10)?; // LINEFEED 键
        key_codes.set("CLEAR", 12)?; // CLEAR 键
        key_codes.set("ENTER", 13)?; // ENTER 键
        key_codes.set("RETURN", 13)?; // RETURN 键
        key_codes.set("SHIFT_KEY", 16)?; // SHIFT 键
        key_codes.set("CONTROL_KEY", 17)?; // CTRL 键
        key_codes.set("MENU", 18)?; // ALT 键
        key_codes.set("PAUSE", 19)?; // PAUSE 键
        key_codes.set("CAPITAL", 20)?; // CAPS LOCK 键
        key_codes.set("CAPS_LOCK", 20)?; // CAPS LOCK 键
        key_codes.set("HANGUEL_MODE", 21)?; // IME Hanguel 模式键（为兼容性而保留；请使用 HangulMode）
        key_codes.set("HANGUL_MODE", 21)?; // IME Hangul 模式键
        key_codes.set("KANA_MODE", 21)?; // IME Kana 模式键
        key_codes.set("JUNJA_MODE", 23)?; // IME Junja 模式键
        key_codes.set("FINAL_MODE", 24)?; // IME Final 模式键
        key_codes.set("HANJA_MODE", 25)?; // IME Hanja 模式键
        key_codes.set("KANJI_MODE", 25)?; // IME Kanji 模式键
        key_codes.set("ESCAPE", 27)?; // ESC 键
        key_codes.set("IME_CONVERT", 28)?; // IME 转换键
        key_codes.set("IME_NONCONVERT", 29)?; // IME 非转换键
        key_codes.set("IME_ACCEPT", 30)?; // IME 接受键，取代了 IMEAceept
        key_codes.set("IME_ACEEPT", 30)?; // IME 接受键已过时，请改用 IMEAccept
        key_codes.set("IME_MODE_CHANGE", 31)?; // IME 模式更改键
        key_codes.set("SPACE", 32)?; // SPACEBAR（空格）键
        key_codes.set("PAGE_UP", 33)?; // PAGE UP（上翻页）键
        key_codes.set("PRIOR", 33)?; // PAGE UP（上翻页）键
        key_codes.set("NEXT", 34)?; // PAGE DOWN（下翻页）键
        key_codes.set("PAGE_DOWN", 34)?; // PAGE DOWN（下翻页）键
        key_codes.set("END", 35)?; // END 键
        key_codes.set("HOME", 36)?; // HOME 键
        key_codes.set("LEFT", 37)?; // LEFT ARROW（左箭头）键
        key_codes.set("UP", 38)?; // UP ARROW（上箭头）键
        key_codes.set("RIGHT", 39)?; // RIGHT ARROW（右箭头）键
        key_codes.set("DOWN", 40)?; // DOWN ARROW（下箭头）键
        key_codes.set("SELECT", 41)?; // SELECT 键
        key_codes.set("PRINT", 42)?; // PRINT 键
        key_codes.set("EXECUTE", 43)?; // EXECUTE 键
        key_codes.set("PRINT_SCREEN", 44)?; // PRINT SCREEN（打印屏幕）键
        key_codes.set("SNAPSHOT", 44)?; // PRINT SCREEN（打印屏幕）键
        key_codes.set("INSERT", 45)?; // INS（插入）键
        key_codes.set("DELETE", 46)?; // DEL（删除）键
        key_codes.set("HELP", 47)?; // HELP（帮助）键
        key_codes.set("D0", 48)?; // 0 键
        key_codes.set("D1", 49)?; // 1 键
        key_codes.set("D2", 50)?; // 2 键
        key_codes.set("D3", 51)?; // 3 键
        key_codes.set("D4", 52)?; // 4 键
        key_codes.set("D5", 53)?; // 5 键
        key_codes.set("D6", 54)?; // 6 键
        key_codes.set("D7", 55)?; // 7 键
        key_codes.set("D8", 56)?; // 8 键
        key_codes.set("D9", 57)?; // 9 键
        key_codes.set("A", 65)?; // A 键
        key_codes.set("B", 66)?; // B 键
        key_codes.set("C", 67)?; // C 键
        key_codes.set("D", 68)?; // D 键
        key_codes.set("E", 69)?; // E 键
        key_codes.set("F", 70)?; // F 键
        key_codes.set("G", 71)?; // G 键
        key_codes.set("H", 72)?; // H 键
        key_codes.set("I", 73)?; // I 键
        key_codes.set("J", 74)?; // J 键
        key_codes.set("K", 75)?; // K 键
        key_codes.set("L", 76)?; // L 键
        key_codes.set("M", 77)?; // M 键
        key_codes.set("N", 78)?; // N 键
        key_codes.set("O", 79)?; // O 键
        key_codes.set("P", 80)?; // P 键
        key_codes.set("Q", 81)?; // Q 键
        key_codes.set("R", 82)?; // R 键
        key_codes.set("S", 83)?; // S 键
        key_codes.set("T", 84)?; // T 键
        key_codes.set("U", 85)?; // U 键
        key_codes.set("V", 86)?; // V 键
        key_codes.set("W", 87)?; // W 键
        key_codes.set("X", 88)?; // X 键
        key_codes.set("Y", 89)?; // Y 键
        key_codes.set("Z", 90)?; // Z 键
        key_codes.set("L_WIN", 91)?; // 左 Windows 徽标键（Microsoft 自然键盘）
        key_codes.set("R_WIN", 92)?; // 右 Windows 徽标键（Microsoft 自然键盘）
        key_codes.set("APPS", 93)?; // 应用程序键（Microsoft 自然键盘）
        key_codes.set("SLEEP", 95)?; // 计算机睡眠键
        key_codes.set("NUM_PAD_0", 96)?; // 数字键盘上的 0 键
        key_codes.set("NUM_PAD_1", 97)?; // 数字键盘上的 1 键
        key_codes.set("NUM_PAD_2", 98)?; // 数字键盘上的 2 键
        key_codes.set("NUM_PAD_3", 99)?; // 数字键盘上的 3 键
        key_codes.set("NUM_PAD_4", 100)?; // 数字键盘上的 4 键
        key_codes.set("NUM_PAD_5", 101)?; // 数字键盘上的 5 键
        key_codes.set("NUM_PAD_6", 102)?; // 数字键盘上的 6 键
        key_codes.set("NUM_PAD_7", 103)?; // 数字键盘上的 7 键
        key_codes.set("NUM_PAD_8", 104)?; // 数字键盘上的 8 键
        key_codes.set("NUM_PAD_9", 105)?; // 数字键盘上的 9 键
        key_codes.set("MULTIPLY", 106)?; // 乘法键
        key_codes.set("ADD", 107)?; // 加法键
        key_codes.set("SEPARATOR", 108)?; // 分隔符键
        key_codes.set("SUBTRACT", 109)?; // 减法键
        key_codes.set("DECIMAL", 110)?; // 小数点键
        key_codes.set("DIVIDE", 111)?; // 除法键
        key_codes.set("F1", 112)?; // F1 键
        key_codes.set("F2", 113)?; // F2 键
        key_codes.set("F3", 114)?; // F3 键
        key_codes.set("F4", 115)?; // F4 键
        key_codes.set("F5", 116)?; // F5 键
        key_codes.set("F6", 117)?; // F6 键
        key_codes.set("F7", 118)?; // F7 键
        key_codes.set("F8", 119)?; // F8 键
        key_codes.set("F9", 120)?; // F9 键
        key_codes.set("F10", 121)?; // F10 键
        key_codes.set("F11", 122)?; // F11 键
        key_codes.set("F12", 123)?; // F12 键
        key_codes.set("F13", 124)?; // F13 键
        key_codes.set("F14", 125)?; // F14 键
        key_codes.set("F15", 126)?; // F15 键
        key_codes.set("F16", 127)?; // F16 键
        key_codes.set("F17", 128)?; // F17 键
        key_codes.set("F18", 129)?; // F18 键
        key_codes.set("F19", 130)?; // F19 键
        key_codes.set("F20", 131)?; // F20 键
        key_codes.set("F21", 132)?; // F21 键
        key_codes.set("F22", 133)?; // F22 键
        key_codes.set("F23", 134)?; // F23 键
        key_codes.set("F24", 135)?; // F24 键
        key_codes.set("NUM_LOCK", 144)?; // NUM LOCK 键
        key_codes.set("SCROLL", 145)?; // SCROLL LOCK 键
        key_codes.set("L_SHIFT_KEY", 160)?; // 左 SHIFT 键
        key_codes.set("R_SHIFT_KEY", 161)?; // 右 SHIFT 键
        key_codes.set("L_CONTROL_KEY", 162)?; // 左 CTRL 键
        key_codes.set("R_CONTROL_KEY", 163)?; // 右 CTRL 键
        key_codes.set("L_MENU", 164)?; // 左 ALT 键
        key_codes.set("R_MENU", 165)?; // 右 ALT 键
        key_codes.set("BROWSER_BACK", 166)?; // 浏览器后退键
        key_codes.set("BROWSER_FORWARD", 167)?; // 浏览器前进键
        key_codes.set("BROWSER_REFRESH", 168)?; // 浏览器刷新键
        key_codes.set("BROWSER_STOP", 169)?; // 浏览器停止键
        key_codes.set("BROWSER_SEARCH", 170)?; // 浏览器搜索键
        key_codes.set("BROWSER_FAVORITES", 171)?; // 浏览器收藏夹键
        key_codes.set("BROWSER_HOME", 172)?; // 浏览器主页键
        key_codes.set("VOLUME_MUTE", 173)?; // 静音键
        key_codes.set("VOLUME_DOWN", 174)?; // 降低音量键
        key_codes.set("VOLUME_UP", 175)?; // 增大音量键
        key_codes.set("MEDIA_NEXT_TRACK", 176)?; // 媒体下一曲键
        key_codes.set("MEDIA_PREVIOUS_TRACK", 177)?; // 媒体上一曲键
        key_codes.set("MEDIA_STOP", 178)?; // 媒体停止键
        key_codes.set("MEDIA_PLAY_PAUSE", 179)?; // 媒体播放/暂停键
        key_codes.set("LAUNCH_MAIL", 180)?; // 启动邮件键
        key_codes.set("SELECT_MEDIA", 181)?; // 选择媒体键
        key_codes.set("LAUNCH_APPLICATION_1", 182)?; // 启动应用程序一键
        key_codes.set("LAUNCH_APPLICATION_2", 183)?; // 启动应用程序二键
        key_codes.set("OEM_1", 186)?; // OEM 1 键
        key_codes.set("OEM_SEMICOLON", 186)?; // US 标准键盘上的 OEM 分号键
        key_codes.set("OEM_PLUS", 187)?; // 任何国家/地区键盘上的 OEM 加号键
        key_codes.set("OEM_COMMA", 188)?; // 任何国家/地区键盘上的 OEM 逗号键
        key_codes.set("OEM_MINUS", 189)?; // 任何国家/地区键盘上的 OEM 减号键
        key_codes.set("OEM_PERIOD", 190)?; // 任何国家/地区键盘上的 OEM 句点键
        key_codes.set("OEM_2", 191)?; // OEM 2 键
        key_codes.set("OEM_QUESTION", 191)?; // US 标准键盘上的 OEM 问号键
        key_codes.set("OEM_3", 192)?; // OEM 3 键
        key_codes.set("OEM_TILDE", 192)?; // US 标准键盘上的 OEM 波浪号键
        key_codes.set("OEM_4", 219)?; // OEM 4 键
        key_codes.set("OEM_OPEN_BRACKETS", 219)?; // US 标准键盘上的 OEM 左中括号键
        key_codes.set("OEM_5", 220)?; // OEM 5 键
        key_codes.set("OEM_PIPE", 220)?; // US 标准键盘上的 OEM 竖线键
        key_codes.set("OEM_6", 221)?; // OEM 6 键
        key_codes.set("OEM_CLOSE_BRACKETS", 221)?; // US 标准键盘上的 OEM 右中括号键
        key_codes.set("OEM_7", 222)?; // OEM 7 键
        key_codes.set("OEM_QUOTES", 222)?; // US 标准键盘上的 OEM 单引号/双引号键
        key_codes.set("OEM_8", 223)?; // OEM 8 键
        key_codes.set("OEM_102", 226)?; // OEM 102 键
        key_codes.set("OEM_BACKSLASH", 226)?; // RT 102 键键盘上的 OEM 尖括号或反斜杠键
        key_codes.set("PROCESS_KEY", 229)?; // PROCESS KEY 键
        key_codes.set("PACKET", 231)?; // 用于传递 Unicode 字符，如同它们是按键一样
        key_codes.set("ATTN", 246)?; // ATTN 键
        key_codes.set("CRSEL", 247)?; // CRSEL 键
        key_codes.set("EXSEL", 248)?; // EXSEL 键
        key_codes.set("ERASE_EOF", 249)?; // ERASE EOF 键
        key_codes.set("PLAY", 250)?; // PLAY 键
        key_codes.set("ZOOM", 251)?; // ZOOM 键
        key_codes.set("NO_NAME", 252)?; // 保留供将来使用的常量
        key_codes.set("PA1", 253)?; // PA1 键
        key_codes.set("OEM_CLEAR", 254)?; // CLEAR 键
        key_codes.set("CODE", 65535)?; // 用于从键值中提取键码的位掩码
        key_codes.set("SHIFT", 65536)?; // SHIFT 修饰键
        key_codes.set("CONTROL", 131072)?; // CTRL 修饰键
        key_codes.set("ALT", 262144)?; // ALT 修饰键

        globals.set("KeyCodes", key_codes)?;

        Ok(())
    })
}

pub fn get_widget_manager() -> LuaResult<*mut WidgetManager> {
    unsafe {
        get_lawn_app().and_then(|lawn_app| {
            if ((*lawn_app).widget_manager as u32) == 0 {
                Err(LuaError::MemoryError("WidgetManager 不可访问".to_string()))
            } else {
                Ok((*lawn_app).widget_manager)
            }
        })
    }
}

pub fn with_widget_manager<T>(f: impl FnOnce(&mut WidgetManager) -> LuaResult<T>) -> LuaResult<T> {
    get_widget_manager().and_then(|widget_manager| unsafe { f(&mut *widget_manager) })
}

impl LuaUserData for WidgetManager {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("GetMousePos", |_, _, ()| {
            with_widget_manager(|wm| Ok(wm.mouse_pos))
        });
    }
}
