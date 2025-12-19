use std::marker::PhantomData;
use std::os::raw::c_char;

pub trait HasId {
    const NAMESPACE: &'static str;

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

    /// 获取可变指针
    pub fn get_ptr(&mut self, id: i32) -> Option<*mut T> {
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

            return Some(element_ptr);
        }
    }
}

// ==========================================================================
// 不可变迭代器 (Immutable Iterator)
// ==========================================================================

pub struct DataArrayIter<'a, T: 'a + HasId> {
    array: &'a DataArray<T>,
    current_index: isize,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: HasId> Iterator for DataArrayIter<'a, T> {
    // 这里返回引用 &T，这是 Rust 中“安全的指针”
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            // 安全检查：block 为空则直接结束
            if self.array.block.is_null() {
                return None;
            }

            // 循环直到找到一个有效元素或到达最大使用索引
            while self.current_index < self.array.max_used_index as isize {
                let index = self.current_index;
                self.current_index += 1;

                // 获取当前位置的指针
                let ptr = self.array.block.offset(index);
                let element = &*ptr;
                let id = element.id();

                // 核心存活校验逻辑：
                // 1. ID 不能为 0
                // 2. ID 解析出的索引 (id & 0xFFFF) 必须等于当前内存位置 (index)
                //    如果不相等，说明这个位置的数据是陈旧的，或者被用于 FreeList 了
                if id != 0 && (id & 0xFFFF) as isize == index {
                    return Some(element);
                }
            }
        }
        None
    }
}

// ==========================================================================
// 可变迭代器 (Mutable Iterator)
// ==========================================================================

pub struct DataArrayIterMut<'a, T: 'a + HasId> {
    array: &'a mut DataArray<T>,
    current_index: isize,
    _marker: PhantomData<&'a mut T>,
}

impl<'a, T: HasId> Iterator for DataArrayIterMut<'a, T> {
    // 返回可变引用 &mut T
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.array.block.is_null() {
                return None;
            }

            while self.current_index < self.array.max_used_index as isize {
                let index = self.current_index;
                self.current_index += 1;

                // 注意：这里需要将 array.block 视为可变指针
                let ptr = self.array.block.offset(index);
                // 临时转为不可变引用来检查 ID (避免过早的可变借用问题，虽然在这里是指针操作)
                let id = (*ptr).id();

                if id != 0 && (id & 0xFFFF) as isize == index {
                    // 确认存活后，返回可变引用
                    return Some(&mut *ptr);
                }
            }
        }
        None
    }
}

// ==========================================================================
// 扩展 DataArray 方法
// ==========================================================================

impl<T: HasId> DataArray<T> {
    /// 创建安全的不可变迭代器
    pub fn iter(&'_ self) -> DataArrayIter<'_, T> {
        DataArrayIter {
            array: self,
            current_index: 0,
            _marker: PhantomData,
        }
    }

    /// 创建安全的可变迭代器
    pub fn iter_mut(&'_ mut self) -> DataArrayIterMut<'_, T> {
        DataArrayIterMut {
            array: self,
            current_index: 0,
            _marker: PhantomData,
        }
    }

    /// 如果你非常坚持要“裸指针”迭代器（不建议在 Safe Rust 中直接用），
    /// 可以使用 iter_mut().map(|x| x as *mut T) 或者参考下面的做法：
    pub fn iter_ptr(&mut self) -> impl Iterator<Item = *mut T> + '_ {
        self.iter_mut().map(|r| r as *mut T)
    }
}

// 为了让 for 循环能直接使用 &array
impl<'a, T: HasId> IntoIterator for &'a DataArray<T> {
    type Item = &'a T;
    type IntoIter = DataArrayIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// 为了让 for 循环能直接使用 &mut array
impl<'a, T: HasId> IntoIterator for &'a mut DataArray<T> {
    type Item = &'a mut T;
    type IntoIter = DataArrayIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
