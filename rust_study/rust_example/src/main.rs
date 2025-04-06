// 导入模块
mod datatype;
use datatype::primitive_types::*;
use datatype::array_types::*;
use datatype::string_types::string_examples;
use datatype::tuple_types::tuple_examples;

fn main() {
    println!("Hello, world!");
    primitive_types();
    demonstrate_usize();
    demonstrate_isize();
    check_size_types();
    array_examples();
    string_examples();
    tuple_examples();
}
