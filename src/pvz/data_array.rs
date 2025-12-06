use std::os::raw::c_char;


#[repr(C)]
#[derive(Debug)]
pub struct DataArray<T> {
    pub Block: *mut T,
    /// 数组大小
    pub BlockSize: i32,
    /// 数量上限
    pub MaxCount: i32,
    /// 下一个编号
    pub FreeListHead: i32,
    /// 当前数量
    pub Size: i32,
    /// 序列号
    pub IDCounter: i32,
    /// 文本指针
    pub DebugName: *const c_char,
}
