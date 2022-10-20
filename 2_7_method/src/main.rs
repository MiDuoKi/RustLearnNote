//方法和结构体方法

#[derive(Debug)]    //debug打印
struct Rectangle {  //定义结构体
    width: u32,
    height: u32,
}

impl Rectangle {  //定义结构体方法 使用 impl 关键字关联


    fn new(w: u32, h: u32) -> Rectangle {  //关联函数 不引用 &self 这个函数作为构造函数使用
        Rectangle { width: w, height: h }
    }

    fn area(&self) -> u32 {  //直接使用self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少(会导致前面的Rectangle失效)    &self 不可变租用 &mut self 表示可变借用
        self.width * self.height
    }
}

fn main() {

    // 结构体构造
    let rect1 = Rectangle::new(30,80);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() //结构体方法调用
    );

    println!("{}", rect1.width); //如果方法直接使用self 这里出错 因为rect1已经失效
}


//枚举可以实现方法
#![allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}

//特征也可以实现方法