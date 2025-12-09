use std::os::raw::c_char;


pub trait HasId {
    /// 获取对象的当前 ID
    fn id(&self) -> i32;
}

#[repr(C)]
#[derive(Debug)]
pub struct DataArray<T> {
    /// +00: 数据块指针
    pub block: *mut T,
    /// +04: 当前已分配过的最大索引位置 (Watermark / Next New Index)
    /// 汇编中的 a2[1]
    pub max_used_index: i32,
    /// +08: 数组最大容量上限 (Capacity)
    /// 汇编中的 a2[2]
    pub max_capacity: i32,
    /// +0C: 空闲链表头 (FreeListHead)
    pub free_list_head: i32,
    /// +10: 当前存活元素数量 (Count)
    pub size: i32,
    /// +14: 下一个 ID 的版本序列号 (Sequence)
    pub id_counter: i32,
    /// +18: 调试名称
    pub debug_name: *const c_char,
}

impl<T: HasId> DataArray<T> {
    /// 复刻 DataArray_Zombie_::DataArrayTryToGet
    /// 
    /// # Logic
    /// int __fastcall TryToGet(int id, DataArray *this)
    /// {
    ///   if ( id && (unsigned int)(unsigned __int16)id < this->max_capacity )
    ///     return this->block[(unsigned __int16)id].id != id ? 0 : &this->block[(unsigned __int16)id];
    ///   else
    ///     return 0;
    /// }
    pub fn get(&self, id: i32) -> Option<&T> {
        // 汇编: if ( a1 && ... )
        // ID 为 0 永远是无效的
        if id == 0 {
            return None;
        }

        // 汇编: (unsigned __int16)a1
        // 提取低 16 位作为索引
        let index = (id & 0xFFFF) as isize;

        unsafe {
            // 汇编: ... < a2[2]
            // 检查索引是否超过了数组的“最大容量”（偏移 0x08）
            // 注意：这里比较的是 MaxCapacity，而不是 Size 或 MaxUsedIndex
            if index >= self.max_capacity as isize {
                return None;
            }
            
            // 额外安全检查：防止 block 为空
            if self.block.is_null() {
                return None;
            }

            // 计算目标地址
            // 汇编: *a2 + 348 * (unsigned __int16)a1
            let element_ptr = self.block.offset(index);
            
            // ---------------------------------------------------------
            // 核心逻辑: 版本号校验
            // 汇编: ... + 344) != a1 ? 0 : ...
            // 读取内存中的 ID，必须与请求的 ID 完全一致（包括高位的序列号）
            // 如果不一致，说明该位置的僵尸已经死了，或者被复用给了另一个僵尸
            // ---------------------------------------------------------
            if (*element_ptr).id() != id {
                return None;
            }

            return Some(&*element_ptr);
        }
    }

    /// 获取可变引用 (逻辑同上)
    pub fn get_mut(&mut self, id: i32) -> Option<&mut T> {
        if id == 0 {
            return None;
        }

        let index = (id & 0xFFFF) as isize;

        unsafe {
            if index >= self.max_capacity as isize {
                return None;
            }

            if self.block.is_null() {
                return None;
            }

            let element_ptr = self.block.offset(index);

            // 核心校验：只有 ID 完全匹配才返回
            if (*element_ptr).id() != id {
                return None;
            }

            return Some(&mut *element_ptr);
        }
    }
}
