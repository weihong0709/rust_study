pub fn string_examples() {
    // 1. 字符串字面量（字符串切片）
    let string_literal: &str = "Hello, world!";
    println!("字符串字面量: {}", string_literal);

    // 2. String类型（可增长的字符串）
    let mut owned_string = String::from("Hello");
    owned_string.push_str(", Rust!");  // 追加字符串
    println!("可变字符串: {}", owned_string);

    // 3. 字符串创建的多种方式
    let string_from_literal = String::from("直接从字面量创建");
    let string_to_owned = "通过to_owned创建".to_owned();
    let string_to_string = "通过to_string创建".to_string();
    
    // 4. 字符串拼接
    let hello = String::from("Hello, ");
    let world = String::from("World!");
    // 使用 + 运算符
    let hello_world = hello + &world;  // hello被移动
    // 使用format!宏
    let formatted = format!("{} {}", "Hello", "Rust!");

    // 5. 字符串切片
    let slice = &owned_string[0..5];  // 获取前5个字节
    println!("字符串切片: {}", slice);

    // 6. 字符串遍历
    println!("\n字符遍历：");
    for c in "你好，世界".chars() {
        print!("{} ", c);
    }
    println!();

    // 7. 字节遍历
    println!("\n字节遍历：");
    for b in "Hello".bytes() {
        print!("{} ", b);
    }
    println!();

    // 8. 字符串长度
    let chinese = "你好世界";
    println!("\n字符串长度：");
    println!("字节长度: {}", chinese.len());  // 字节长度
    println!("字符数量: {}", chinese.chars().count());  // 字符数量

    // 9. 字符串查找和替换
    let text = String::from("Hello, world!");
    println!("\n查找和替换：");
    println!("包含'world': {}", text.contains("world"));
    println!("替换后: {}", text.replace("world", "Rust"));

    // 10. 字符串分割
    let text = "a,b,c,d";
    println!("\n字符串分割：");
    for part in text.split(',') {
        print!("{} ", part);
    }
    println!();

    // 11. 字符串转换
    let num = 42;
    let num_as_string = num.to_string();
    let parsed_num: i32 = "42".parse().unwrap();
    println!("\n字符串转换：");
    println!("数字到字符串: {}", num_as_string);
    println!("字符串到数字: {}", parsed_num);

    // 12. 字符串大小写转换
    let text = "Hello, World!";
    println!("\n大小写转换：");
    println!("全部大写: {}", text.to_uppercase());
    println!("全部小写: {}", text.to_lowercase());

    // 13. 字符串修剪
    let text = "   Hello, World!   ";
    println!("\n字符串修剪：");
    println!("去除两端空格: '{}'", text.trim());
    println!("去除左侧空格: '{}'", text.trim_start());
    println!("去除右侧空格: '{}'", text.trim_end());
} 