pub fn primitive_types() {
    // 整数类型
    let integer_types = {
        let i8_var: i8 = 127;                 // -128 到 127
        let i16_var: i16 = 32767;             // -32768 到 32767
        let i32_var: i32 = 2_147_483_647;     // -2^31 到 2^31-1
        let i64_var: i64 = 9_223_372_036_854_775_807;  // -2^63 到 2^63-1
        let i128_var: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;  // -2^127 到 2^127-1
        let isize_var: isize = 123;           // 取决于系统架构（32位或64位）

        // 无符号整数
        let u8_var: u8 = 255;                 // 0 到 255
        let u16_var: u16 = 65535;             // 0 到 65535
        let u32_var: u32 = 4_294_967_295;     // 0 到 2^32-1
        let u64_var: u64 = 18_446_744_073_709_551_615;  // 0 到 2^64-1
        let u128_var: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;  // 0 到 2^128-1
        let usize_var: usize = 123;           // 取决于系统架构
        
        println!("整数类型示例：");
        println!("i8: {}, i16: {}, i32: {}", i8_var, i16_var, i32_var);
        println!("i64: {}, i128: {}, isize: {}", i64_var, i128_var, isize_var);
        println!("u8: {}, u16: {}, u32: {}", u8_var, u16_var, u32_var);
        println!("u64: {}, u128: {}, usize: {}", u64_var, u128_var, usize_var);
    };

    // 浮点数类型
    let float_types = {
        let f32_var: f32 = 3.14159;           // 单精度浮点数
        let f64_var: f64 = 3.14159265359;     // 双精度浮点数
        
        println!("\n浮点数类型示例：");
        println!("f32: {}", f32_var);
        println!("f64: {}", f64_var);
    };

    // 布尔类型
    let boolean_type = {
        let bool_var: bool = true;            // true 或 false
        
        println!("\n布尔类型示例：");
        println!("bool: {}", bool_var);
    };

    // 字符类型
    let character_type = {
        let char_var: char = 'A';             // Unicode 字符
        let emoji_char: char = '😀';          // Unicode 表情
        let chinese_char: char = '中';        // Unicode 中文字符
        
        println!("\n字符类型示例：");
        println!("char: {}, emoji: {}, chinese: {}", char_var, emoji_char, chinese_char);
    };

    // 单元类型
    let unit_type = {
        let unit_var: () = ();               // 空元组，表示无返回值
        
        println!("\n单元类型示例：");
        println!("unit: {:?}", unit_var);
    };

    // Never 类型
    let never_type = {
        fn never_returns() -> ! {
            panic!("这个函数永远不会返回");
        }
        
        println!("\nNever类型示例：");
        println!("never类型函数已定义（但不会调用）");
    };

    // 指针类型
    let pointer_types = {
        let x = 42;
        let ref_var: &i32 = &x;              // 引用
        
        let mut y = 42;
        let mut_ref_var: &mut i32 = &mut y;  // 可变引用
        println!("可变引用: {}", mut_ref_var);  // 先使用可变引用
        
        // 裸指针
        let raw_ptr: *const i32 = &x as *const i32;
        let raw_mut_ptr: *mut i32 = &mut y as *mut i32;  // 现在可以重新借用 y
        
        println!("\n指针类型示例：");
        println!("引用: {}", ref_var);
        unsafe {
            println!("裸指针: {}", *raw_ptr);
            println!("可变裸指针: {}", *raw_mut_ptr);
        }
    };
}


/**
 * 
 * isize 和 usize 是 Rust 中特殊的整数类型，它们的大小取决于程序运行的计算机架构：
isize（有符号整数）:
在 32 位系统上是 32 位（4字节），范围是 -2^31 到 2^31-1
在 64 位系统上是 64 位（8字节），范围是 -2^63 到 2^63-1
usize（无符号整数）:
在 32 位系统上是 32 位（4字节），范围是 0 到 2^32-1
在 64 位系统上是 64 位（8字节），范围是 0 到 2^64-1
为什么需要这两种类型：
平台兼容性：程序可以在不同位数的系统上运行
内存安全：用于数组索引和内存操作时能够覆盖整个地址空间
性能优化：使用系统原生字长可能获得更好的性能
使用建议：
对于集合索引和大小，使用 usize
需要表示内存中的偏移量时，使用 isize
如果确定数值范围，最好使用具体的类型（如 i32、u64 等）而不是 isize/usize
示例：检查系统架构和类型大小
 */
pub fn demonstrate_usize() {
    // 1. 数组索引
    let array = [1, 2, 3, 4, 5];
    let index: usize = 2;
    println!("数组索引: {}", array[index]);

    // 2. 集合长度
    let vector = vec![1, 2, 3];
    let length: usize = vector.len();
    println!("向量长度: {}", length);

    // 3. 内存大小计算
    let size: usize = std::mem::size_of::<i32>();
    println!("i32类型的大小: {} 字节", size);
}

pub fn demonstrate_isize() {
    // 1. 指针运算（在unsafe代码中）
    let array = [1, 2, 3, 4, 5];
    let ptr = array.as_ptr();
    let offset: isize = 2;
    
    unsafe {
        let value = *ptr.offset(offset);
        println!("指针偏移后的值: {}", value);
    }

    // 2. 表示相对位置或距离
    let position: isize = -3;
    println!("相对位置: {}", position);
}

pub fn check_size_types() {
    println!("在当前系统上：");
    println!("usize的大小: {} 字节", std::mem::size_of::<usize>());
    println!("isize的大小: {} 字节", std::mem::size_of::<isize>());
    
    // 打印最大值
    println!("usize最大值: {}", usize::MAX);
    println!("isize最大值: {}", isize::MAX);
} 
