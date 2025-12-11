pub mod data_array;
#[allow(unused)]
pub mod delta_mgr;

use mlua::prelude::*;
use std::fmt::Debug;

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

        globals.set("Vec2", Vec2::<f64>::default())?;
        globals.set("Rect2", Rect2::<f64>::default())?;

        Ok(())
    })
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2<T: Sized + FromLua + Into<f64>> {
    pub x: T,
    pub y: T,
}

impl<T: FromLua + Into<f64> + Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn as_f64(&self) -> Vec2<f64> {
        Vec2 {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

// 关键点：
// 1. T 必须是 Copy + 'static + Send + Sync，这是 UserData 的基础要求。
// 2. 使用 for<'lua> 语法 (HRTB) 约束 T 必须能转换为 Lua 值 (IntoLua) 和从 Lua 值转换 (FromLua)。
// 3. Debug 约束用于实现 __tostring。
impl<T> LuaUserData for Vec2<T>
where
    T: Copy + 'static + Send + Sync + Debug + Into<f64>,
    for<'lua> T: FromLua + IntoLua,
{
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.x));
        fields.add_field_method_set("x", |_, this, value| {
            this.x = value;
            Ok(())
        });

        fields.add_field_method_get("y", |_, this| Ok(this.y));
        fields.add_field_method_set("y", |_, this, value| {
            this.y = value;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // 实现 __tostring，方便在 Lua 中 print(v)
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!("Vec2({:?}, {:?})", this.x, this.y))
        });

        // 向量加法
        methods.add_method("Add", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            Ok(Vec2 {
                x: a.x + b.x,
                y: a.y + b.y,
            })
        });

        // 向量减法
        methods.add_method("Sub", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            Ok(Vec2 {
                x: a.x - b.x,
                y: a.y - b.y,
            })
        });

        // 标量乘法
        methods.add_method("Mul", |_, this, scalar: f64| {
            let v = this.as_f64();
            Ok(Vec2 {
                x: v.x * scalar,
                y: v.y * scalar,
            })
        });

        // 标量除法
        methods.add_method("Div", |_, this, scalar: f64| {
            let v = this.as_f64();
            Ok(Vec2 {
                x: v.x / scalar,
                y: v.y / scalar,
            })
        });

        // 点积（内积）
        methods.add_method("Dot", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            Ok(a.x * b.x + a.y * b.y)
        });

        // 向量长度（模）
        methods.add_method("Length", |_, this, ()| {
            let v = this.as_f64();
            Ok((v.x * v.x + v.y * v.y).sqrt())
        });

        // 向量长度的平方（避免开方运算，性能更好）
        methods.add_method("LengthSquared", |_, this, ()| {
            let v = this.as_f64();
            Ok(v.x * v.x + v.y * v.y)
        });

        // 归一化（返回单位向量）
        methods.add_method("Normalize", |_, this, ()| {
            let v = this.as_f64();
            let len = (v.x * v.x + v.y * v.y).sqrt();
            if len == 0.0 {
                Ok(Vec2 { x: 0.0, y: 0.0 })
            } else {
                Ok(Vec2 {
                    x: v.x / len,
                    y: v.y / len,
                })
            }
        });

        // 计算两点之间的距离
        methods.add_method("Distance", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            Ok((dx * dx + dy * dy).sqrt())
        });

        // 计算两点之间距离的平方
        methods.add_method("DistanceSquared", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            let dx = a.x - b.x;
            let dy = a.y - b.y;
            Ok(dx * dx + dy * dy)
        });

        // 线性插值（t 为 0 到 1 之间的值）
        methods.add_method("Lerp", |_, this, (other, t): (Vec2<T>, f64)| {
            let a = this.as_f64();
            let b = other.as_f64();
            Ok(Vec2 {
                x: a.x + (b.x - a.x) * t,
                y: a.y + (b.y - a.y) * t,
            })
        });

        // 计算向量的角度（弧度）
        methods.add_method("Angle", |_, this, ()| {
            let v = this.as_f64();
            Ok(v.y.atan2(v.x))
        });

        // 计算从当前点到另一点的角度
        methods.add_method("AngleTo", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            Ok((b.y - a.y).atan2(b.x - a.x))
        });

        // 旋转向量（angle 为弧度）
        methods.add_method("Rotate", |_, this, angle: f64| {
            let v = this.as_f64();
            let cos = angle.cos();
            let sin = angle.sin();
            Ok(Vec2 {
                x: v.x * cos - v.y * sin,
                y: v.x * sin + v.y * cos,
            })
        });

        // 叉积（2D 叉积返回标量，表示 z 分量）
        methods.add_method("Cross", |_, this, other: Vec2<T>| {
            let a = this.as_f64();
            let b = other.as_f64();
            Ok(a.x * b.y - a.y * b.x)
        });

        // 反射向量（根据法线反射）
        methods.add_method("Reflect", |_, this, normal: Vec2<T>| {
            let v = this.as_f64();
            let n = normal.as_f64();
            let dot = v.x * n.x + v.y * n.y;
            Ok(Vec2 {
                x: v.x - 2.0 * dot * n.x,
                y: v.y - 2.0 * dot * n.y,
            })
        });

        methods.add_function("New", |_, (x, y)| Ok(Vec2::<f64>::new(x, y)));

        methods.add_function("Zero", |_, ()| Ok(Vec2::<f64>::default()));
    }
}

impl<T: FromLua + Into<f64> + Copy> FromLua for Vec2<T> {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let Some(vec2) = value.as_userdata() {
            Ok(Self::new(vec2.get("x")?, vec2.get("y")?))
        } else {
            Err(LuaError::ToLuaConversionError {
                from: value.to_string()?,
                to: "Vec2",
                message: None,
            })
        }
    }
}

// ==========================================
// Rect2 实现
// ==========================================

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Rect2<T: Sized + FromLua + Into<f64>> {
    pub position: Vec2<T>,
    pub size: Vec2<T>,
}

impl<T> Rect2<T>
where
    T: FromLua + Into<f64> + Copy,
{
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        Self {
            position: Vec2::new(x, y),
            size: Vec2::new(width, height),
        }
    }

    pub fn as_f64(&self) -> Rect2<f64> {
        Rect2 {
            position: self.position.as_f64(),
            size: self.size.as_f64(),
        }
    }
}

impl<T: FromLua + Into<f64> + Copy> FromLua for Rect2<T> {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        if let Some(rect2) = value.as_userdata() {
            Ok(Self::new(
                rect2.get("x")?,
                rect2.get("y")?,
                rect2.get("width")?,
                rect2.get("height")?,
            ))
        } else {
            Err(LuaError::ToLuaConversionError {
                from: value.to_string()?,
                to: "Rect2",
                message: None,
            })
        }
    }
}

// Rect2 的 UserData 实现
// 这里的约束条件与 Vec2 相同，因为 Rect2 内部包含 Vec2
impl<T: FromLua + Into<f64>> LuaUserData for Rect2<T>
where
    T: Copy + 'static + Send + Sync + Debug,
    for<'lua> T: FromLua + IntoLua,
{
    fn add_fields<F: LuaUserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("x", |_, this| Ok(this.position.x));
        fields.add_field_method_set("x", |_, this, value| {
            this.position.x = value;
            Ok(())
        });

        fields.add_field_method_get("y", |_, this| Ok(this.position.y));
        fields.add_field_method_set("y", |_, this, value| {
            this.position.y = value;
            Ok(())
        });

        fields.add_field_method_get("width", |_, this| Ok(this.size.x));
        fields.add_field_method_set("width", |_, this, value| {
            this.size.x = value;
            Ok(())
        });

        fields.add_field_method_get("height", |_, this| Ok(this.size.y));
        fields.add_field_method_set("height", |_, this, value| {
            this.size.y = value;
            Ok(())
        });

        fields.add_field_method_get("pos", |_, this| Ok(this.position.clone()));
        fields.add_field_method_set("pos", |_, this, value| {
            this.position = value;
            Ok(())
        });

        fields.add_field_method_get("size", |_, this| Ok(this.size.clone()));
        fields.add_field_method_set("size", |_, this, value| {
            this.size = value;
            Ok(())
        });
    }

    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        // Rect2 的字符串表示
        methods.add_meta_method(LuaMetaMethod::ToString, |_, this, ()| {
            Ok(format!(
                "Rect2(Pos:({:?}, {:?}), Size:({:?}, {:?}))",
                this.position.x, this.position.y, this.size.x, this.size.y
            ))
        });

        methods.add_method("Contains", |_, this, pos: Vec2<T>| {
            let pos_f64 = pos.as_f64();
            let this_f64 = this.as_f64();

            Ok(pos_f64.x >= this_f64.position.x
                && pos_f64.x <= this_f64.position.x + this_f64.size.x
                && pos_f64.y >= this_f64.position.y
                && pos_f64.y <= this_f64.position.y + this_f64.size.y)
        });

        methods.add_function("New", |_, (x, y, w, h)| {
            Ok(Rect2 {
                position: Vec2::<f64>::new(x, y),
                size: Vec2::<f64>::new(w, h),
            })
        });

        methods.add_function("Zero", |_, ()| {
            Ok(Rect2 {
                position: Vec2::<f64>::default(),
                size: Vec2::<f64>::default(),
            })
        });
    }
}
