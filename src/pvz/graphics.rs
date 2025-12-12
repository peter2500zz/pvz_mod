use std::arch::{asm, naked_asm};

use crate::{
    hook::pvz::graphics::{
        ADDR_DRAW_RECT, ADDR_FILL_RECT, ADDR_SET_COLOR, ORIGINAL_CREATE, ORIGINAL_DESTRUCTOR,
    },
    pvz::graphics::graphics::{Color, Font, Graphics},
    utils::{Rect2, msvc_string::MsvcString},
};

pub mod graphics;

pub extern "stdcall" fn Create(g: *mut Graphics) -> *mut Graphics {
    ORIGINAL_CREATE.wait()(g)
}

pub extern "thiscall" fn Destructor(g: *mut Graphics) {
    ORIGINAL_DESTRUCTOR.wait()(g)
}

pub fn SetColor(g: *mut Graphics, color: &Color) {
    unsafe {
        asm!(
            "call {func}",
            // 1. 输入绑定：直接告诉 Rust 把变量放在特定寄存器里
            // 这样 Rust 就不需要先分配随机寄存器再让你 mov 了
            in("eax") color, // color 是引用，传入的是地址，正是 EAX 需要的
            in("ecx") g,     // g 是指针，传入的是地址，正是 ECX 需要的
            func = in(reg) ADDR_SET_COLOR,

            // 2. 关键修复：告诉 Rust 我们破坏了什么
            // "clobber_abi" 告诉编译器："我进行了一个标准的函数调用"
            // 编译器会自动处理所有易失性寄存器（EAX, ECX, EDX）和标志位的保存/恢复。
            clobber_abi("C"),

            // 如果你的 Rust 版本较老不支持 clobber_abi，可以用下面的写法替代：
            // out("eax") _, // 函数返回值通常在 eax，会被覆盖
            // out("ecx") _, // 也就是告诉 Rust：别指望这些寄存器里的值能活下来
            // options(nostack) // 如果这个调用不涉及 Rust 栈操作
        );
    }
}

pub fn DrawRect(g: *mut Graphics, rect: Rect2<i32>) {
    unsafe {
        asm!(
            // 1. 压栈参数 (注意：汇编 push 顺序通常是反向的，从右到左)
            "push {height}",
            "push {width}",
            "push {y}",
            "push {x}",

            // 2. 调用函数
            "call {func}",

            // 3. 【非常重要】栈平衡修正
            // 如果目标函数是 stdcall (它自己 ret 16)，则删除下面这行。
            // 如果目标函数是 cdecl (像 C 语言默认那样)，你必须加上这行！
            // 如果你不确定，看 IDA/Ghidra 里该函数结尾是 ret 还是 ret 0x10。
            // "add esp, 16",  <-- 如果崩溃，尝试取消注释这行

            // 变量绑定
            x = in(reg) rect.position.x,
            y = in(reg) rect.position.y,
            width = in(reg) rect.size.x,
            height = in(reg) rect.size.y,

            // 将 g 放入 eax (根据你的代码逻辑)
            in("eax") g,

            // 函数地址放入寄存器 (让编译器选一个通用的，或者你指定 edx)
            func = in(reg) ADDR_DRAW_RECT,

            // 标记 ABI 破坏 (保护 eax, ecx, edx, flags)
            clobber_abi("C"),
        );
    }
}

pub fn FillRect(g: *mut Graphics, rect: Rect2<i32>) {
    unsafe {
        asm!(
            // 1. 压栈参数 (注意：汇编 push 顺序通常是反向的，从右到左)
            "push {height}",
            "push {width}",
            "push {y}",
            "push {x}",

            // 2. 调用函数
            "call {func}",

            // 变量绑定
            x = in(reg) rect.position.x,
            y = in(reg) rect.position.y,
            width = in(reg) rect.size.x,
            height = in(reg) rect.size.y,

            in("eax") g,

            func = in(reg) ADDR_FILL_RECT,

            // 标记 ABI 破坏
            clobber_abi("C"),
        );
    }
}

#[unsafe(naked)]
pub extern "stdcall" fn TodDrawStringWrapped(
    g: *mut Graphics,
    theText: *const MsvcString,
    theRect: *const Rect2<i32>,
    theFont: *mut Font,
    theColor: *const Color,
    theJustification: i32,
) {
    naked_asm!(
        // 函数序言 (prologue)
        "push ebp",      // 保存旧的基址指针
        "mov ebp, esp",  // 设置新的栈帧
        "sub esp, 0x10", // 为局部变量分配栈空间 (可根据需要调整大小)
        // 保存需要使用的寄存器
        "push ebx",
        "push esi",
        "push edi",
        // ===== 函数体在这里 =====
        // 参数访问:
        // [ebp + 8]  = g
        // [ebp + 12] = theText
        // [ebp + 16] = theRect
        // [ebp + 20] = theFont
        // [ebp + 24] = theColor
        // [ebp + 28] = theJustification
        "push [ebp + 28]",
        "mov esi, [ebp + 24]",
        "push [ebp + 20]",
        "mov ebx, [ebp + 16]",
        "mov edx, [ebp + 12]",
        "push [ebp + 8]",
        "mov eax, 0x0051A040",
        "call eax",
        "add esp, 0xC",
        // ===== 函数体结束 =====

        // 函数尾声 (epilogue)
        "pop edi", // 恢复寄存器
        "pop esi",
        "pop ebx",
        "mov esp, ebp", // 恢复栈指针
        "pop ebp",      // 恢复基址指针
        "ret 0x18",     // stdcall: 返回并清理 6 个参数 (6 * 4 = 0x18 字节)
    )
}
