pub fn tuple_examples() {
    // 1. 基本元组声明和初始化
    let tuple: (i32, f64, &str) = (42, 3.14, "hello");
    println!("基本元组: {:?}", tuple);

    // 2. 类型推断的元组
    let inferred_tuple = (1, "rust", true);
    println!("类型推断元组: {:?}", inferred_tuple);

    // 3. 单元元组
    let unit_tuple = ();
    println!("单元元组: {:?}", unit_tuple);

    // 4. 元组解构
    let (x, y, z) = tuple;
    println!("\n元组解构:");
    println!("x: {}, y: {}, z: {}", x, y, z);

    // 5. 通过索引访问元组元素
    println!("\n索引访问:");
    println!("第一个元素: {}", tuple.0);
    println!("第二个元素: {}", tuple.1);
    println!("第三个元素: {}", tuple.2);

    // 6. 嵌套元组
    let nested_tuple = ((1, 2), (3, 4), (5, 6));
    println!("\n嵌套元组: {:?}", nested_tuple);
    println!("访问嵌套元素: {}", nested_tuple.0.1);  // 访问第一个内部元组的第二个元素

    // 7. 可变元组
    let mut mutable_tuple = (1, 2, 3);
    println!("\n修改前: {:?}", mutable_tuple);
    mutable_tuple.0 = 10;
    println!("修改后: {:?}", mutable_tuple);

    // 8. 元组作为函数返回值
    fn return_tuple() -> (i32, String, bool) {
        (42, String::from("Rust"), true)
    }
    let returned_tuple = return_tuple();
    println!("\n函数返回的元组: {:?}", returned_tuple);

    // 9. 部分解构
    let tuple = (1, 2, 3, 4, 5);
    let (first, second, ..) = tuple;  // 只解构前两个元素
    println!("\n部分解构: first={}, second={}", first, second);

    // 10. 元组切片（不支持，但可以转换后切片）
    let tuple_vec: Vec<_> = vec![(1, 2), (3, 4), (5, 6)];
    let slice = &tuple_vec[1..];
    println!("\n元组数组切片: {:?}", slice);

    // 11. 元组比较
    let tuple1 = (1, "hello");
    let tuple2 = (1, "hello");
    let tuple3 = (2, "hello");
    println!("\n元组比较:");
    println!("tuple1 == tuple2: {}", tuple1 == tuple2);
    println!("tuple1 == tuple3: {}", tuple1 == tuple3);

    // 12. 元组用作复合键
    use std::collections::HashMap;
    let mut tuple_map = HashMap::new();
    tuple_map.insert((1, "hello"), "value");
    println!("\n元组作为HashMap键: {:?}", tuple_map);

    // 13. 元组结构体
    struct Point(i32, i32);
    let point = Point(10, 20);
    println!("\n元组结构体: ({}, {})", point.0, point.1);
} 