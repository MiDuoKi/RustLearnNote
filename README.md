### RUST 基础学习笔记

学习资料 https://course.rs/first-try/cargo.html


#### 01_hello_world-新建工程编写第一行代码



命令  `cargo new projectName`

代码

```rust
fn main() {
    let a  = "hello world";
    println!("{}",a);
}
```

进入工程根目录 执行 `cargo run`

或者等价的 先 build 出执行文件 再运行

```
cargo build
 .\target\debug\hello_rust.exe
```

如果要编译 release版本 添加参数 `--release`

单独执行代码合规检测 使用 `cargo check`

配置清单文件 
 
- Cargo.toml 项目数据描述文件 用户修改
- Cargo.lock 项目依赖详细清单 不用修改，根据Cargo.toml自动生成

>什么情况下该把 Cargo.lock 上传到 git 仓库里？很简单，当你的项目是一个可运行的程序时，就上传 Cargo.lock，如果是一个依赖库项目，那么请把它添加到 .gitignore 中


核心的如何写依赖库

三种形式:1.官方仓库 2.git仓库 3.本地仓库

```
[dependencies]
rand = "0.3"
hammer = { version = "0.5.0"}
color = { git = "https://github.com/bjz/color-rs" }
geometry = { path = "crates/geometry" }
```


#### 02_variables-变量的定义和使用



let 修饰变量 定义不可变变量并可以绑定

```
let a = 6
a = 7
```


新增mut关键字变为可变变量

```
let mut a = 6

a = 7
```

下划线修饰变量名 可以忽略未使用变量检查

比如 _x 可以通过变量不使用检查  

变量解构-自动匹配类型

let (a, mut b): (bool,bool) = (true, false);

切分赋值

```
struct Struct {
    e: i32
}

fn main() {
    let (a, b, c, d, e);

    (a, b) = (1, 2); // a = 1 b = 2
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    [c, .., d, _] = [1, 2, 3, 4, 5];      // c = 3 d = 4
    Struct { e, .. } = Struct { e: 5 };  //e = 5 

    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
```

常量 const 修饰 值的类型必须标注

const MAX_POINTS: u32 = 100_000;

变量名遮蔽 

Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的
