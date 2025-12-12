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
            CStr::from_ptr(self.to_c_str()).to_string_lossy().to_string()
        }
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

impl Drop for MsvcString {
    fn drop(&mut self) {
        unsafe {
            type BasicString = extern "thiscall" fn(this: *mut MsvcString);
            let basic_string: BasicString = transmute(0x00404420);

            basic_string(self);
        }
    }
}
