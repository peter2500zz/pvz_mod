pub mod pvz;

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
