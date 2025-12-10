use std::fmt::Debug;

use mlua::prelude::*;

use crate::mods::LuaRegistration;


#[macro_export]
macro_rules! pause {
    () => {
        use std::io::{self, Read};
        let _ = io::stdin().read(&mut [0u8]);
    };
    ($($args:tt)*) => {
        use std::io::{self, Read};
        println!($($args)*);
        let _ = io::stdin().read(&mut [0u8]);
    };
}

#[macro_export]
macro_rules! add_field_mut {
    ($fields:expr, $name:literal, $field:ident) => {
        $fields.add_field_method_get($name, |_, this| Ok(this.$field));
        $fields.add_field_method_set($name, |_, this, val| Ok(this.$field = val));
    };

    // 支持多个字段
    ($fields:expr, $( $name:literal => $field:ident ),* $(,)?) => {
        $(
            $fields.add_field_method_get($name, |_, this| Ok(this.$field));
            $fields.add_field_method_set($name, |_, this, val| Ok(this.$field = val));
        )*
    };
}

#[macro_export]
macro_rules! add_field {
    ($fields:expr, $name:literal, $field:ident) => {
        $fields.add_field_method_get($name, |_, this| Ok(this.$field));
    };

    // 支持多个字段
    ($fields:expr, $( $name:literal => $field:ident ),* $(,)?) => {
        $(
            $fields.add_field_method_get($name, |_, this| Ok(this.$field));
        )*
    };
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        let new_vec2 = lua.create_function(|_, (x, y)| {
            Ok(Vec2::<f64>::new(x, y))
        })?;

        let new_rect2 = lua.create_function(|_, (x, y, width, height)| {
            Ok(Rect2::<f64>::new(x, y, width, height))
        })?;

        globals.set("NewVec2", new_vec2)?;
        globals.set("NewRect2", new_rect2)?;

        Ok(())
    })
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: Sized + FromLua> {
    pub x: T,
    pub y: T,
}

impl<T: FromLua> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 关键点：
// 1. T 必须是 Copy + 'static + Send + Sync，这是 UserData 的基础要求。
// 2. 使用 for<'lua> 语法 (HRTB) 约束 T 必须能转换为 Lua 值 (IntoLua) 和从 Lua 值转换 (FromLua)。
// 3. Debug 约束用于实现 __tostring。
impl<T> LuaUserData for Vec2<T>
where
    T: Copy + 'static + Send + Sync + Debug,
    for<'lua> T: FromLua + IntoLua,
{
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // Getter: x
        fields.add_field_method_get("x", |_, this| Ok(this.x));

        // Getter: y
        fields.add_field_method_get("y", |_, this| Ok(this.y));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 实现 __tostring，方便在 Lua 中 print(v)
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("Vec2({:?}, {:?})", this.x, this.y))
        });
    }
}

impl<T: FromLua> FromLua for Vec2<T> {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let Some(vec2) = value.as_userdata() {
            Ok(Self::new(vec2.get("x")?, vec2.get("y")?))
        } else {
            Err(LuaError::ToLuaConversionError { from: value.to_string()?, to: "Vec2", message: None })
        }
    }
}

// ==========================================
// Rect2 实现
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rect2<T: Sized + FromLua> {
    pub position: Vec2<T>,
    pub size: Vec2<T>,
}

impl<T> Rect2<T> 
where T: FromLua
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }
}

impl<T: FromLua> FromLua for Rect2<T> {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let Some(rect2) = value.as_userdata() {
            Ok(Self::new(rect2.get("x")?, rect2.get("y")?, rect2.get("width")?, rect2.get("height")?))
        } else {
            Err(LuaError::ToLuaConversionError { from: value.to_string()?, to: "Rect2", message: None })
        }
    }
}

// Rect2 的 UserData 实现
// 这里的约束条件与 Vec2 相同，因为 Rect2 内部包含 Vec2
impl<T: FromLua> LuaUserData for Rect2<T>
where
    T: Copy + 'static + Send + Sync + Debug,
    for<'lua> T: FromLua + IntoLua,
{
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        // --- 复合属性访问 ---
        
        // 获取 position (返回一个新的 Vec2 UserData)
        fields.add_field_method_get("position", |_, this| Ok(this.position));

        // 获取 size
        fields.add_field_method_get("size", |_, this| Ok(this.size));

        // --- 便捷属性 (快捷访问 x, y, width, height) ---
        
        // x
        fields.add_field_method_get("x", |_, this| Ok(this.position.x));

        // y
        fields.add_field_method_get("y", |_, this| Ok(this.position.y));

        // width
        fields.add_field_method_get("width", |_, this| Ok(this.size.x));

        // height
        fields.add_field_method_get("height", |_, this| Ok(this.size.y));
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Rect2 的字符串表示
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("Rect2(Pos:({:?}, {:?}), Size:({:?}, {:?}))", this.position.x, this.position.y, this.size.x, this.size.y))
        });
        
        // 示例：可以添加计算面积的方法
        // 注意：这需要 T 支持乘法，这里为了演示简化，暂不添加额外的 Math Trait Bounds
    }
}
