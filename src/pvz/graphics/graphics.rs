use std::ptr;

use mlua::prelude::*;
use tracing::*;

use crate::{
    mods::LuaRegistration, pvz::graphics::{Create, Destructor, DrawRect, FillRect, SetColor}, utils::Vec2
};

#[repr(C)]
pub struct DDImage {}

#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Color {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
    pub alpha: i32,
}

impl Color {
    pub fn new(r: i32, g: i32, b: i32, a: i32) -> Self {
        Self { red: r, green: g, blue: b, alpha: a }
    }
}

impl FromLua for Color {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let Some(color) = value.as_userdata() {
            Ok(Self::new(
                color.get("red")?,
                color.get("green")?,
                color.get("blue")?,
                color.get("alpha")?,
            ))
        } else {
            Err(LuaError::ToLuaConversionError {
                from: value.to_string()?,
                to: "Color",
                message: None,
            })
        }
    }
}

inventory::submit! {
    LuaRegistration(|lua| {
        let globals = lua.globals();

        globals.set("Color", Color::default())?;

        Ok(())
    })
}

impl LuaUserData for Color {
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("red", |_, this| {
            Ok(this.red)
        });
        fields.add_field_method_set("red", |_, this, value| {
            this.red = value;
            Ok(())
        });

        fields.add_field_method_get("green", |_, this| {
            Ok(this.green)
        });
        fields.add_field_method_set("green", |_, this, value| {
            this.green = value;
            Ok(())
        });

        fields.add_field_method_get("blue", |_, this| {
            Ok(this.blue)
        });
        fields.add_field_method_set("blue", |_, this, value| {
            this.blue = value;
            Ok(())
        });

        fields.add_field_method_get("alpha", |_, this| {
            Ok(this.alpha)
        });
        fields.add_field_method_set("alpha", |_, this, value| {
            this.alpha = value;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("New", |_, (red, green, blue, alpha)| {
            Ok(Color {
                red,
                green,
                blue,
                alpha,
            })
        });
    }
}

#[repr(C)]
pub struct Font {}

#[repr(C)]
pub struct Graphics {
    _pad_0x0_0x4: [u8; 0x4 - 0x0],
    /// 0x4 DDImage
    pub ddimage: *mut DDImage,
    /// 0x8 偏移
    pub trans: Vec2<i32>,
    _pad_0x10_0x30: [u8; 0x30 - 0x10],
    /// 0x30
    pub color: Color,
    /// 0x40 字体
    pub font: *mut Font,
    /// 0x44
    pub draw_mode: *mut i32,
    _pad_0x48_0x68: [u8; 0x68 - 0x48],
}
const _: () = assert!(size_of::<Graphics>() == 0x68);

impl Clone for Graphics {
    fn clone(&self) -> Self {
        // debug!("自动构造");
        unsafe { ptr::read(Create(self as *const Self as *mut Self)) }
    }
}

impl Drop for Graphics {
    fn drop(&mut self) {
        // debug!("自动析构");
        Destructor(self)
    }
}

impl LuaUserData for Graphics {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("SetColor", |_, this, color: Color| {
            SetColor(this as *const _ as *mut _, &color);

            Ok(())
        });

        methods.add_method("DrawRect", |_, this, rect| {
            DrawRect(this as *const _ as *mut _, rect);

            Ok(())
        });

        methods.add_method("FillRect", |_, this, rect| {
            FillRect(this as *const _ as *mut _, rect);

            Ok(())
        });
    }
}
