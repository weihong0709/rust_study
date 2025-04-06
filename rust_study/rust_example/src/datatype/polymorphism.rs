// 特征（Trait）定义 - 实现接口多态
trait Animal {
    fn make_sound(&self) -> String;
    
    // 带默认实现的方法
    fn description(&self) -> String {
        String::from("这是一个动物")
    }
}

// 结构体实现特征 - 接口多态的具体实现
struct Dog {
    name: String
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("汪汪!")
    }
    
    // 覆盖默认实现
    fn description(&self) -> String {
        format!("这是一只名叫{}的狗", self.name)
    }
}

struct Cat {
    name: String
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        String::from("喵喵!")
    }
}

// 泛型 - 参数化多态
struct Container<T> {
    value: T
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get_value(&self) -> &T {
        &self.value
    }
}

// 特征对象 - 动态分发
fn print_animal_sound(animal: &dyn Animal) {
    println!("动物叫声: {}", animal.make_sound());
}

// 泛型约束 - 静态分发
fn print_animal_info<T: Animal>(animal: &T) {
    println!("动物信息: {}", animal.description());
}

fn demonstrate_polymorphism() {
    // 接口多态示例
    let dog = Dog { name: String::from("旺财") };
    let cat = Cat { name: String::from("咪咪") };
    
    println!("=== 接口多态 ===");
    println!("狗: {}", dog.make_sound());
    println!("猫: {}", cat.make_sound());
    
    // 泛型多态示例
    println!("\n=== 参数化多态（泛型）===");
    let int_container = Container::new(42);
    let str_container = Container::new(String::from("Hello"));
    println!("整数容器: {}", int_container.get_value());
    println!("字符串容器: {}", str_container.get_value());
    
    // 动态分发示例
    println!("\n=== 动态分发（特征对象）===");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(dog),
        Box::new(cat)
    ];
    
    for animal in animals.iter() {
        print_animal_sound(animal.as_ref());
    }
    
    // 静态分发示例
    println!("\n=== 静态分发（泛型约束）===");
    let new_dog = Dog { name: String::from("小黑") };
    let new_cat = Cat { name: String::from("小白") };
    print_animal_info(&new_dog);
    print_animal_info(&new_cat);
}
