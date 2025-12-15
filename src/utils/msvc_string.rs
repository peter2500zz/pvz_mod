use std::{
    ffi::{CStr, CString, c_char},
    mem::transmute,
};

#[repr(C)]
pub struct MsvcString {
    _pad: [u8; 0x1C],
}

impl MsvcString {
    pub fn new() -> Self {
        unsafe {
            let mut this = Self {
                _pad: std::mem::zeroed(),
            };

            type BasicString = extern "thiscall" fn(this: *mut MsvcString) -> *mut MsvcString;
            let basic_string: BasicString = transmute(0x00404400);

            basic_string(&mut this);

            this
        }
    }

    pub fn to_c_str(&self) -> *const c_char {
        unsafe {
            type ToCStr = extern "thiscall" fn(this: *const MsvcString) -> *const c_char;
            let to_c_str: ToCStr = transmute(0x004042D0);

            to_c_str(self)
        }
    }

    pub fn to_string(&self) -> String {
        unsafe {
            CStr::from_ptr(self.to_c_str())
                .to_string_lossy()
                .to_string()
        }
    }

    /// 以 _Right 字符串的一个指定的子字符串赋值 this
    ///
    /// # 参数
    /// - `right`: 源字符串的引用
    /// - `roff`: 子字符串在源字符串中的起始偏移量
    /// - `count`: 子字符串的长度,当为 u32::MAX 时不限制长度
    ///
    /// # 返回值
    /// 返回 self 的可变引用
    pub fn assign(&mut self, right: &MsvcString, roff: u32, count: u32) -> &mut Self {
        unsafe {
            // 注意:参数顺序是倒序的
            // C++ 签名: assign(unsigned int _Count, unsigned int _Roff, const std::string& _Right, ecx = this)
            // thiscall 约定: ecx = this, 其他参数从右到左入栈
            type Assign = extern "thiscall" fn(
                this: *mut MsvcString,
                right: *const MsvcString,
                roff: u32,
                count: u32,
            ) -> *mut MsvcString;

            let assign: Assign = std::mem::transmute(0x00403E20);

            assign(self, right, roff, count);

            self
        }
    }

    /// 便捷方法:从指定偏移量开始复制整个子字符串(不限制长度)
    pub fn assign_from(&mut self, right: &MsvcString, roff: u32) -> &mut Self {
        self.assign(right, roff, u32::MAX)
    }

    /// 便捷方法:完整复制另一个字符串
    pub fn assign_all(&mut self, right: &MsvcString) -> &mut Self {
        self.assign(right, 0, u32::MAX)
    }
}

impl From<*const c_char> for MsvcString {
    fn from(value: *const c_char) -> Self {
        unsafe {
            let mut this = Self {
                _pad: std::mem::zeroed(),
            };

            type BasicString =
                extern "thiscall" fn(this: *mut MsvcString, ptr: *const c_char) -> *mut MsvcString;
            let basic_string: BasicString = transmute(0x00404450);

            basic_string(&mut this, value);

            this
        }
    }
}

impl From<&str> for MsvcString {
    fn from(value: &str) -> Self {
        unsafe {
            let cstr = CString::from_vec_unchecked(value.as_bytes().to_vec());

            MsvcString::from(cstr.as_ptr())
        }
    }
}

impl From<String> for MsvcString {
    fn from(value: String) -> Self {
        MsvcString::from(value.as_str())
    }
}

impl Drop for MsvcString {
    fn drop(&mut self) {
        unsafe {
            type BasicString = extern "thiscall" fn(this: *mut MsvcString);
            let basic_string: BasicString = transmute(0x00404420);

            basic_string(self);
        }
    }
}
