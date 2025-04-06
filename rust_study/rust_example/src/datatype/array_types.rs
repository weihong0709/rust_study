pub fn array_examples() {
    // 1. 基本数组定义和初始化
    let fixed_array: [i32; 5] = [1, 2, 3, 4, 5];  // 显式类型标注
    let inferred_array = [1, 2, 3, 4, 5];         // 类型推断
    let repeated_array = [0; 5];                   // [0, 0, 0, 0, 0]

    // 2. 多维数组
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    // 3. 数组访问
    let first_element = fixed_array[0];
    let last_element = fixed_array[fixed_array.len() - 1];

    // 4. 数组切片
    let slice: &[i32] = &fixed_array[1..4];  // [2, 3, 4]
    let full_slice: &[i32] = &fixed_array;   // 全数组切片

    // 5. 数组方法使用
    println!("数组基本信息：");
    println!("长度: {}", fixed_array.len());
    println!("占用字节数: {}", std::mem::size_of_val(&fixed_array));
    println!("第一个元素: {}", first_element);
    println!("最后一个元素: {}", last_element);

    // 6. 数组遍历
    println!("\n不同的遍历方式：");
    
    // for 循环遍历元素
    print!("普通for循环: ");
    for element in fixed_array.iter() {
        print!("{} ", element);
    }
    println!();

    // 带索引的遍历
    print!("带索引遍历: ");
    for (index, &value) in fixed_array.iter().enumerate() {
        print!("{}:{} ", index, value);
    }
    println!();

    // 7. 数组操作
    let mut mutable_array = [1, 2, 3, 4, 5];
    
    // 修改元素
    mutable_array[2] = 10;
    
    // 数组方法
    println!("\n数组操作：");
    println!("是否包含3: {}", mutable_array.contains(&3));
    println!("查找10的位置: {:?}", mutable_array.iter().position(|&x| x == 10));

    // 8. 数组排序
    let mut numbers = [5, 2, 8, 1, 9];
    numbers.sort();
    println!("\n排序后: {:?}", numbers);

    // 9. 安全访问
    let safe_access = fixed_array.get(3);  // 返回 Option<&T>
    match safe_access {
        Some(value) => println!("\n安全访问索引3: {}", value),
        None => println!("\n索引越界！"),
    }

    // 10. 数组比较
    let array1 = [1, 2, 3];
    let array2 = [1, 2, 3];
    let array3 = [1, 2, 4];
    
    println!("\n数组比较：");
    println!("array1 == array2: {}", array1 == array2);
    println!("array1 == array3: {}", array1 == array3);

    // 11. 使用切片进行部分操作
    fn print_slice(slice: &[i32]) {
        println!("切片内容: {:?}", slice);
    }

    let full_array = [1, 2, 3, 4, 5];
    println!("\n切片操作：");
    print_slice(&full_array);      // 传递整个数组作为切片
    print_slice(&full_array[1..4]); // 传递部分数组作为切片

    // 12. 数组与Vec的转换
    let array_to_convert = [1, 2, 3, 4, 5];
    let vec_from_array: Vec<i32> = array_to_convert.to_vec();
    let array_from_vec: [i32; 5] = vec_from_array.clone().try_into().unwrap();  // 使用 clone() 避免移动

    println!("\n数组与Vec转换：");
    println!("Vec: {:?}", vec_from_array);
    println!("Array: {:?}", array_from_vec);
} 