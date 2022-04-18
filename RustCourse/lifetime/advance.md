# advance

1
```rust
/* Annotate struct with lifetime:
1. `r` and `s` must has different lifetimes
2. lifetime of `s` is bigger than that of 'r'
*/
struct DoubleRef<'a, T> {
    r: &'a T,
    s: &'a T,
}
fn main() {
    println!("Success!")
}
```

2
```rust
/* Adding trait bounds to make it work */
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a: 'b, 'b> ImportantExcerpt<'a> {
    fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    println!("Success!")
}
```

3
```rust
/* Adding trait bounds to make it work */
fn f<'a: 'b, 'b>(x: &'a i32, mut y: &'b i32) {
    y = x;
    let r: &'b &'a i32 = &&0;
}

fn main() {
    println!("Success!")
}
```

4
```rust
/* Adding HRTB to make it work!*/
fn call_on_ref_zero<'a, F>(f: F)
where
    F: Fn(&'a i32),
{
    let zero: &'a i32 = &0;
    f(zero);
}

fn call_on_ref_zero<F>(f: F)
where
    for<'a> F: Fn(&'a i32),
{
    let zero = 0;
    f(&zero);
}

fn main() {
    println!("Success!")
}
```

5
```rust
/* Make it work by reordering some code */
fn main() {
    let mut data = 10;
    let ref1 = &mut data;
    let ref2 = &mut *ref1;

    *ref2 += 2;
    *ref1 += 1;

    println!("{}", data);
}
```

6
```rust
/* Make it work */
struct Interface<'a, 'b: 'a> {
    manager: &'a mut Manager<'b>,
}

impl<'a, 'b: 'a> Interface<'a, 'b> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

fn main() {
    let mut list = List {
        manager: Manager { text: "hello" },
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    use_list(&list);
}

fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
```