//泛型

// fn add<T>(a:T, b:T) -> T {   
//     a + b
// }


fn add<T: std::ops::Add<Output = T>>(a:T, b:T) -> T { //使用范型T 表示 但是还要额外限制，因为不是所有类型可以相加
    a + b
}

//: std::ops::Add<Output = T>  解析 : 

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f64: {}", add(1.23, 1.23));
}