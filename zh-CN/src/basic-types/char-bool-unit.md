# 字符、布尔、单元类型

### 字符
1. 🌟
```rust, editable

use std::mem::size_of_val;
fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2),4); 

    println!("Success!")
} 
```

2. 🌟
```rust, editable

fn main() {
    let c1 = '中';
    print_char(c1);
} 

fn print_char(c : char) {
    println!("{}", c);
}
```

### 布尔
3. 🌟
```rust, editable

// make  println! work
fn main() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!")
    }
} 
```

4. 🌟
```rust, editable

fn main() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success!")
}
```


### 单元类型
5. 🌟🌟
```rust,editable

// 让代码工作，但不要修改 `implicitly_ret_unit` !
fn main() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will returen a ()")
}

// 不要使用下面的函数，它只用于演示！
fn explicitly_ret_unit() -> () {
    println!("I will returen a ()")
}
```

6. 🌟🌟 单元类型占用的内存大小是多少？
```rust,editable

// 让代码工作：修改 `assert!` 中的 `4` 
use std::mem::size_of_val;
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!")
}
```

> 你可以在[这里](https://github.com/sunface/rust-by-practice)找到答案(在 solutions 路径下) 