# Rust半小时教程       

原文: [A half-hour to learn Rust](https://fasterthanli.me/blog/2020/a-half-hour-to-learn-rust/)

为了熟练的掌握一门编程语言，人们不得不阅读它的大量的相关资料。但是如果你不理解这些资料介绍的内容，如何能学习更多的资料呢？
在本文中，我将尽可能多地列举Rust代码片段，并解释它们包含的关键字和符号的含义，而不是只关注Rust的一两个概念。

准备好了吗，让我们出发吧!



`let` 引入变量

```rust
let x; // declare "x"
x = 42; // assign 42 to "x"
```

也可以写成一行:

```rust
let x = 42;
```

你可以使用`:`显式地指定变量的类型，也就是类型注解：

```rust
let x: i32; // `i32` is a signed 32-bit integer
x = 42;

// there's i8, i16, i32, i64, i128
// also u8, u16, u32, u64, u128 for unsigned
```

也可以写成一行:

```rust
let x: i32 = 42;
```

如果你声明一个变量，后来再初始化它。在初始化之前使用它的话编译器会报错:

```rust
let x;
foobar(x); // error: borrow of possibly-uninitialized variable: `x`
x = 42;
```

下面的代码就可以了:

```rust
let x;
x = 42;
foobar(x); // the type of `x` will be inferred from here
```

下划线`_`是一个特殊的名字，或者更确切地说，“缺乏名字”。基本上它的意思就是扔掉一些东西:

```rust
// this does *nothing* because 42 is a constant
let _ = 42;

// this calls `get_thing` but throws away its result
let _ = get_thing();
```

以下划线开头的名称是常规名称，有一点特殊，就是如果它们未被使用的话编译器不会报警告:

```rust
// we may use `_x` eventually, but our code is a work-in-progress
// and we just wanted to get rid of a compiler warning for now.
let _x = 42;
```

变量名可以重用-它会隐藏(shadow)前一个变量:

```rust
let x = 13;
let x = x + 3;
// using `x` after that line only refers to the second `x`,
// the first `x` no longer exists.
```

Rust有[tuple类型](https://doc.rust-lang.org/std/primitive.tuple.html)，你可以把它看作有固定长度的不同类型的集合:

```rust
let pair = ('a', 17);
pair.0; // this is 'a'
pair.1; // this is 17
```

如果我们想为tuple加上类型注解，可以这么做:

```rust
let pair: (char, i32) = ('a', 17);
```

Tuple类型可以通过赋值方式进行解构(destructured)，这意味着它们被分成各自独立的字段:

```rust
let (some_char, some_int) = ('a', 17);
// now, `some_char` is 'a', and `some_int` is 17
```

当一个函数返回tuple类型的时候特别管用:

```rust
let (left, right) = slice.split_at(middle);
```

当然，解构一个tuple的时候，下划线`_`可以用来丢掉一些字段:

```rust
let (_, right) = slice.split_at(middle);
```

分号`;`放在语句(statement)的结尾:

```rust
let x = 3;
let y = 5;
let z = y + x;
```

这意味着语句可以写成多行:

```rust
let x = vec![1, 2, 3, 4, 5, 6, 7, 8]
    .iter()
    .map(|x| x + 3)
    .fold(0, |x, y| x + y);
```

(之后我们再介绍这段代码的意义)

`fn`用来声明一个函数。
下面是一个void函数:

```rust
fn greet() {
    println!("Hi there!");
}
```

下面是一个返回32位有符号的整数，箭头指示它的返回类型:

```rust
fn fair_dice_roll() -> i32 {
    4
}
```

一对大括号声明了代码块(block),块有自己的作用域:

```rust
// This prints "in", then "out"
fn main() {
    let x = "out";
    {
        // this is a different `x`
        let x = "in";
        println!(x);
    }
    println!(x);
}
```

块也是表达式，意味着它的计算结果是一个值:

```rust
// this:
let x = 42;

// is equivalent to this:
let x = { 42 };
```

块可以包括多条语句:

```rust
let x = {
    let y = 1; // first statement
    let z = 2; // second statement
    y + z // this is the *tail* - what the whole block will evaluate to
};
```

函数块的最后省略分号意味着返回这个值，例如下面两个函数功能是一样的:

```rust
fn fair_dice_roll() -> i32 {
    return 4;
}

fn fair_dice_roll() -> i32 {
    4
}
```

`if`条件也可以是表达式:

```rust
fn fair_dice_roll() -> i32 {
    if feeling_lucky {
        6
    } else {
        4
    }
}
```

`match`也是表达式:

```rust
fn fair_dice_roll() -> i32 {
    match feeling_lucky {
        true => 6,
        false => 4,
    }
}
```

点号(`.`)用来访问一个值的字段:

```rust
let a = (10, 20);
a.0; // this is 10

let amos = get_some_struct();
amos.nickname; // this is "fasterthanlime"
```

或者调用一个方法:

```rust
let nick = "fasterthanlime";
nick.len(); // this is 14
```

双冒号(`::`)类似点号但是操作的对象是命名空间。
下面的例子中`std`是一个crate(库)，`cmp`是一个模块(`module`， 源文件)，`min`是一个函数:

```rust
let least = std::cmp::min(3, 8); // this is 3
```

`use`指令将其它命名空间的名称引入到当前:

```rust
use std::cmp::min;

let least = min(7, 1); // this is 1
```

使用`use`指令的时候，大括号意味着一组名称(`glob`)。如果我们想同时引入`max`和`min`，我们可以这么做:

```rust
// this works:
use std::cmp::min;
use std::cmp::max;

// this also works:
use std::cmp::{min, max};

// this also works!
use std::{cmp::min, cmp::max};
```

通配符`*`引入命名空间下的所有的名称:

```rust
// this brings `min` and `max` in scope, and many other things
use std::cmp::*;
```

类型也是命名空间，它们的方法也可以通过普通函数一样调用:

```rust
let x = "amos".len(); // this is 4
let x = str::len("amos"); // this is also 4
```

`str`是一个基本类型(primitive type),但是在默认的命名空间下也有很多非基本类型:

```rust
// `Vec` is a regular struct, not a primitive type
let v = Vec::new();

// this is exactly the same code, but with the *full* path to `Vec`
let v = std::vec::Vec::new();
```

这是因为Rust会在每个模块开始之前插入:

```rust
use std::prelude::v1::*;
```

它重新输出了很多的符号，比如`Vec`、`String`、`Option`和`Result`。

结构体使用`struct`声明:

```rust
struct Vec2 {
    x: f64, // 64-bit floating point, aka "double precision"
    y: f64,
}
```

可以使用结构体文本初始化它们:

```rust
let v1 = Vec2 { x: 1.0, y: 3.0 };
let v2 = Vec2 { y: 2.0, x: 4.0 };
// the order does not matter, only the names do
```

有一个简洁的方式使用另外一个结构体初始化余下的字段:

```rust
let v3 = Vec2 {
    x: 14.0,
    ..v2
};
```

这被称作`struct update syntax`，只能发生在最后的位置，后面没有逗号。

注意"剩余的字段"可以是结构体的所有字段:

```rust
let v4 = Vec2 { ..v3 };
```

结构体，也像tuple类型一样，可以解构。

下面是一个合法的`let`模式:

```rust
let (left, right) = slice.split_at(middle);
```

又比如下面的代码:

```rust
let v = Vec2 { x: 3.0, y: 6.0 };
let Vec2 { x, y } = v;
// `x` is now 3.0, `y` is now `6.0`
```

抑或下面的代码:

```rust
let Vec2 { x, .. } = v;
// this throws away `v.y`
```

`let`模式也可以用在`if`中当条件:

```rust
struct Number {
    odd: bool,
    value: i32,
}

fn main() {
    let one = Number { odd: true, value: 1 };
    let two = Number { odd: false, value: 2 };
    print_number(one);
    print_number(two);
}

fn print_number(n: Number) {
    if let Number { odd: true, value } = n {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

// this prints:
// Odd number: 1
// Even number: 2
```

`match`匹配也是模式，就像`if let`:

```rust
fn print_number(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

// this prints the same as before
```

注意`match`匹配必须是详尽的，至少需要一个分支与之匹配:

```rust
fn print_number(n: Number) {
    match n {
        Number { value: 1, .. } => println!("One"),
        Number { value: 2, .. } => println!("Two"),
        Number { value, .. } => println!("{}", value),
        // if that last arm didn't exist, we would get a compile-time error
    }
}
```

如果你觉得麻烦，可以使用下划线`_`匹配所有的模式:

```rust
fn print_number(n: Number) {
    match n.value {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("{}", n.value),
    }
}
```

你可以为你的类型声明方法:

```rust
struct Number {
    odd: bool,
    value: i32,
}

impl Number {
    fn is_strictly_positive(self) -> bool {
        self.value > 0
    }
}
```

可以正常使用它们:

```rust
fn main() {
    let minus_two = Number {
        odd: false,
        value: -2,
    };
    println!("positive? {}", minus_two.is_strictly_positive());
    // this prints "positive? false"
}
```

默认变量绑定是不可变的:

```rust
fn main() {
    let n = Number {
        odd: true,
        value: 17,
    };
    n.odd = false; // error: cannot assign to `n.odd`,
                   // as `n` is not declared to be mutable
}
```

不可变的变量不能对其变量值进行修改，但是同时也不能通过赋值更改变量:

```rust
fn main() {
    let n = Number {
        odd: true,
        value: 17,
    };
    n = Number {
        odd: false,
        value: 22,
    }; // error: cannot assign twice to immutable variable `n`
}
```

`mut`允许变量可以更改:

```rust
fn main() {
    let mut n = Number {
        odd: true,
        value: 17,
    }
    n.value = 19; // all good
}
```

`trait`是多个类型拥有的共同的东西:

```rust
trait Signed {
    fn is_strictly_negative(self) -> bool;
}
```

你可以实现:

-   为任意类型实现你自己定义的trait
-   为你的类型实现任意类型的trait
-   不允许为别人的类型实现别人的trait

这被称之为`孤立规则`(orphan rules)。

下面的例子是自定义类型实现自定义trait:

```rust
impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

fn main() {
    let n = Number { odd: false, value: -44 };
    println!("{}", n.is_strictly_negative()); // prints "true"
}
```

为外部类型实现自定义trait(甚至是基本类型):

```rust
impl Signed for i32 {
    fn is_strictly_negative(self) -> bool {
        self < 0
    }
}

fn main() {
    let n: i32 = -44;
    println!("{}", n.is_strictly_negative()); // prints "true"
}
```

为自定义类型实现外部trait:

```rust
// the `Neg` trait is used to overload `-`, the
// unary minus operator.
impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Number {
        Number {
            value: -self.value,
            odd: self.odd,
        }        
    }
}

fn main() {
    let n = Number { odd: true, value: 987 };
    let m = -n; // this is only possible because we implemented `Neg`
    println!("{}", m.value); // prints "-987"
}
```

`impl`总是用来为类型实现方法的，所以在这个块中，`Self`意味着这个类型:

```rust
impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            value: -self.value,
            odd: self.odd,
        }        
    }
}
```

有些trait只是一个标记(`marker`)，它并不是指示类型要实现什么方法，而是说这种类型可以用作做特定的事情。
例如`i32`实现`Copy` trait(简单地讲，`i32`是可复制的)，所以下面的代码工作正常:

```rust
fn main() {
    let a: i32 = 15;
    let b = a; // `a` is copied
    let c = a; // `a` is copied again
}
```

下面的代码也正常:

```rust
fn print_i32(x: i32) {
    println!("x = {}", x);
}

fn main() {
    let a: i32 = 15;
    print_i32(a); // `a` is copied
    print_i32(a); // `a` is copied again
}
```

但是`Number`类型没有实现`Copy`,所以下面的代码不工作:

```rust
fn main() {
    let n = Number { odd: true, value: 51 };
    let m = n; // `n` is moved into `m`
    let o = n; // error: use of moved value: `n`
}
```

下面的代码也不行:

```rust
fn print_number(n: Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn main() {
    let n = Number { odd: true, value: 51 };
    print_number(n); // `n` is moved
    print_number(n); // error: use of moved value: `n`
}
```

但是如果`print_number`使用一个不可变的引用就可以:

```rust
fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn main() {
    let n = Number { odd: true, value: 51 };
    print_number(&n); // `n` is borrowed for the time of the call
    print_number(&n); // `n` is borrowed again
}
```

如果变量被声明为可变的，则函数参数使用可变引用也可以工作:

```rust
fn invert(n: &mut Number) {
    n.value = -n.value;
}

fn print_number(n: &Number) {
    println!("{} number {}", if n.odd { "odd" } else { "even" }, n.value);
}

fn main() {
    // this time, `n` is mutable
    let mut n = Number { odd: true, value: 51 };
    print_number(&n);
    invert(&mut n); // `n is borrowed mutably - everything is explicit
    print_number(&n);
}
```

Trait方法中的`self`参数可以使用引用，也可以使用不可变引用。

```rust
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
```



-   **self**表示调用方法的对象，作为类方法的第一个参数，类似于C++中的this。
-   **Self**表示调用者的类型

当调用trait的方法时，`receiver`隐式地被借用:

```rust
fn main() {
    let n = Number { odd: true, value: 51 };
    let mut m = n.clone();
    m.value += 100;
    
    print_number(&n);
    print_number(&m);
}
```

前面讲到过，它和下面的代码一样:

```rust
let m = n.clone();

let m = std::clone::Clone::clone(&n);
```

像`Copy`这样的Marker trait是没有方法的:

```rust
// note: `Copy` requires that `Clone` is implemented too
impl std::clone::Clone for Number {
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl std::marker::Copy for Number {}
```

现在`Clone`仍然可以使用:

```rust
fn main() {
    let n = Number { odd: true, value: 51 };
    let m = n.clone();
    let o = n.clone();
}
```

但是`Number`值不会被`move`了:

```rust
fn main() {
    let n = Number { odd: true, value: 51 };
    let m = n; // `m` is a copy of `n`
    let o = n; // same. `n` is neither moved nor borrowed.
}
```

一些trait太通用了，我们可以通过`derive`属性自动实现它们:

```rust
#[derive(Clone, Copy)]
struct Number {
    odd: bool,
    value: i32,
}

// this expands to `impl Clone for Number` and `impl Copy for Number` blocks.
```

函数可以是泛型的:

```rust
fn foobar<T>(arg: T) {
    // do something with `arg`
}
```

它们可以有多个类型参数，类型参数用在函数声明和函数体中，用来替代具体的类型:

```rust
fn foobar<L, R>(left: L, right: R) {
    // do something with `left` and `right`
}
```

类型参数通常有约束，所以你可以用它做一些额外的事情。

最简单的约束就是trait名称:

```rust
fn print<T: Display>(value: T) {
    println!("value = {}", value);
}

fn print<T: Debug>(value: T) {
    println!("value = {:?}", value);
}
```

类型约束还可以使用更长的语法:

```rust
fn print<T>(value: T)
where
    T: Display,
{
    println!("value = {}", value);
}
```

约束还可以更复杂，比如要求类型要实现多个trait:

```rust
use std::fmt::Debug;

fn compare<T>(left: T, right: T)
where
    T: Debug + PartialEq,
{
    println!("{:?} {} {:?}", left, if left == right { "==" } else { "!=" }, right);
}

fn main() {
    compare("tea", "coffee");
    // prints: "tea" != "coffee"
}
```

泛型函数可以被当作一个命名空间，包含无穷多个不同具体类型的函数。

类似`crate`、`module`和类型，泛型函数可以使用`::`导航:

```rust
fn main() {
    use std::any::type_name;
    println!("{}", type_name::<i32>()); // prints "i32"
    println!("{}", type_name::<(f64, char)>()); // prints "(f64, char)"
}
```

这被亲切地称之为[turbofish syntax](https://turbo.fish/),因为`::<>`看起来像条鱼。

结构体可以是泛型的:

```rust
struct Pair<T> {
    a: T,
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // prints "Pair<i32>"
    print_type_name(&p2); // prints "Pair<bool>"
}
```

标准库中的`Vec`(分配在堆上的数组)就是泛型实现的:

```rust
fn main() {
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}
```

谈到`Vec`,有个宏(`macro`)可以通过字面方式声明`Vec`变量:

```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![true, false, true];
    print_type_name(&v1); // prints "Vec<i32>"
    print_type_name(&v2); // prints "Vec<bool>"
}
```

类似`name!()`、`name![]`、`name!{}`都是调用宏的方式。宏会被展开成正常的代码。

事实上，`println`就是一个宏:

```rust
fn main() {
    println!("{}", "Hello there!");
}
```

它的展开代码和下面的代码功能一样:

```rust
fn main() {
    use std::io::{self, Write};
    io::stdout().lock().write_all(b"Hello there!\n").unwrap();
}
```

`panic`也是一个宏，例如`Option`既可以包含某个值，也可以不包含值。如果它不包含值，调用它的`.unwrap()`会panic:

```rust
fn main() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
}

// output: thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:378:21
```

`Option`并不是一个结构体，而是一个枚举类型(`enum`)，它包含两个值:

```rust
enum Option<T> {
    None,
    Some(T),
}

impl<T> Option<T> {
    fn unwrap(self) -> T {
        // enums variants can be used in patterns:
        match self {
            Self::Some(t) => t,
            Self::None => panic!(".unwrap() called on a None option"),
        }
    }
}

use self::Option::{None, Some};

fn main() {
    let o1: Option<i32> = Some(128);
    o1.unwrap(); // this is fine

    let o2: Option<i32> = None;
    o2.unwrap(); // this panics!
}

// output: thread 'main' panicked at '.unwrap() called on a None option', src/main.rs:11:27
```

`Result`也是一个枚举类型。它既可以包含一个正常的，也可以包含一个error:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

/*

impl<T,E> Result {
    fn unwrap(self)->T,E {
        match self {
            Self::Ok(T) => T,
            Self::Err(E) => E,
        }
    }
}

*/
```

如果包含error,调用它的`.unwrap()`也会panic。

变量绑定有`声明周期`:

```rust
fn main() {
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        println!("x = {}", x);
        // `x` stops existing
    }
    // `x` no longer exists
}
```

类似地，引用也有声明周期:

```rust
fn main() {
    // `x` doesn't exist yet
    {
        let x = 42; // `x` starts existing
        let x_ref = &x; // `x_ref` starts existing - it borrows `x`
        println!("x_ref = {}", x_ref);
        // `x_ref` stops existing
        // `x` stops existing
    }
    // `x` no longer exists
}
```

引用的声明周期不能超过它借用的变量的声明周期:

```rust
fn main() {
    let x_ref = {
        let x = 42;
        &x
    };
    println!("x_ref = {}", x_ref);
    // error: `x` does not live long enough
}
```

一个变量可以不可变地借用多次:

```rust
fn main() {
    let x = 42;
    let x_ref1 = &x;
    let x_ref2 = &x;
    let x_ref3 = &x;
    println!("{} {} {}", x_ref1, x_ref2, x_ref3);
}
```

在借用的时候，变量不能被修改:

```rust
fn main() {
    let mut x = 42;
    let x_ref = &x;
    x = 13;
    println!("x_ref = {}", x_ref);
    // error: cannot assign to `x` because it is borrowed
}
```

当不可变地借用时，不能同时可变地的借用:

```rust
fn main() {
    let mut x = 42;
    let x_ref1 = &x;
    let x_ref2 = &mut x;
    // error: cannot borrow `x` as mutable because it is also borrowed as immutable
    println!("x_ref1 = {}", x_ref1);
}
```

函数参数中的引用也有生命周期:

```rust
fn print(x: &i32) {
    // `x` is borrowed (from the outside) for the
    // entire time this function is called.
}
```

函数中的参数被调用时可以同时使用多个的生命周期:

-   所有使用这些引用的函数都是泛型的
-   声明周期也是泛型参数

生命周期的名称以`'`开始:

```rust
// elided (non-named) lifetimes:
fn print(x: &i32) {}

// named lifetimes:
fn print<'a>(x: &'a i32) {}
```

返回引用的声明周期依赖参数的声明周期:

```rust
struct Number {
    value: i32,
}

fn number_value<'a>(num: &'a Number) -> &'a i32 {
    &num.value
}

fn main() {
    let n = Number { value: 47 };
    let v = number_value(&n);
    // `v` borrows `n` (immutably), thus: `v` cannot outlive `n`.
    // While `v` exists, `n` cannot be mutably borrowed, mutated, moved, etc.
}
```

当只有一个生命周期时，它并需要被命名，所有对象都有同样的声明周期，所以下面两个函数是等价的:

```rust
fn number_value<'a>(num: &'a Number) -> &'a i32 {
    &num.value
}

fn number_value(num: &Number) -> &i32 {
    &num.value
}
```

结构体也可以通过生命周期声明为泛型，这允许它们持有引用:

```rust
struct NumRef<'a> {
    x: &'a i32,
}

fn main() {
    let x: i32 = 99;
    let x_ref = NumRef { x: &x };
    // `x_ref` cannot outlive `x`, etc.
}
```

同样的代码，增加一个函数:

```rust
struct NumRef<'a> {
    x: &'a i32,
}

fn as_num_ref<'a>(x: &'a i32) -> NumRef<'a> {
    NumRef { x: &x }
}

fn main() {
    let x: i32 = 99;
    let x_ref = as_num_ref(&x);
    // `x_ref` cannot outlive `x`, etc.
}
```

同样的代码，使用省略的(`elided`)的生命周期:

```rust
struct NumRef<'a> {
    x: &'a i32,
}

fn as_num_ref(x: &i32) -> NumRef<'_> {
    NumRef { x: &x }
}

fn main() {
    let x: i32 = 99;
    let x_ref = as_num_ref(&x);
    // `x_ref` cannot outlive `x`, etc.
}
```

`impl`块也可以使用声明周期实现泛型:

```rust
impl<'a> NumRef<'a> {
    fn as_i32_ref(&'a self) -> &'a i32 {
        self.x
    }
}

fn main() {
    let x: i32 = 99;
    let x_num_ref = NumRef { x: &x };
    let x_i32_ref = x_num_ref.as_i32_ref();
    // neither ref cannot outlive `x`
}
```

但是你同样可以使用省略的方式:

```rust
impl<'a> NumRef<'a> {
    fn as_i32_ref(&self) -> &i32 {
        self.x
    }
}
```

如果你不需要使用声明周期的名字，你甚至可以省略更多:

```rust
impl NumRef<'_> {
    fn as_i32_ref(&self) -> &i32 {
        self.x
    }
}
```

有一个特殊的声明周期，叫做`'static`，它的生命周期在整个程序运行时。

字符串字面值就是`'static`:

```rust
struct Person {
    name: &'static str,
}

fn main() {
    let p = Person {
        name: "fasterthanlime",
    };
}
```

但是`owned string`声明周期就不是`'static`的:

```rust
struct Person {
    name: &'static str,
}

fn main() {
    let name = format!("fasterthan{}", "lime");
    let p = Person { name: &name };
    // error: `name` does not live long enough
}
```

上面的例子中`name`并不是`&'static str`类型，而是`Stirng`类型。它是动态分配的，可以被释放。它的生命周期小于整个程序，尽管它是在`main`函数中。

为了在`Person`中存储一个非`'static`的字符串，你需要：

**A)** 通过声明周期声明泛型

```rust
struct Person<'a> {
    name: &'a str,
}

fn main() {
    let name = format!("fasterthan{}", "lime");
    let p = Person { name: &name };
    // `p` cannot outlive `name`
}
```

或者

**B)** 获得这个字符串的所有权

```rust
struct Person {
    name: String,
}

fn main() {
    let name = format!("fasterthan{}", "lime");
    let p = Person { name: name };
    // `name` was moved into `p`, their lifetimes are no longer tied.
}
```

在一个结构体的字面值中，如果字段名和变量相同时:

```rust
let p = Person { name: name };
```

可以简写为:

```rust
let p = Person { name };
```

Rust中很多类型都有`owned`和非`owned`变种:

-   字符串： `String`是owned, `&str`是引用
-   路径：`PathBuf`是owned, `&Path`是引用
-   集合: `Vec`是owned, `&[T]`是引用

Rust有slice - 它们是对多个连续元素的引用。
你可以借用vector的slice，例如:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let v2 = &v[2..4];
    println!("v2 = {:?}", v2);
}

// output:
// v2 = [3, 4]
```

上面并没有什么魔法。索引操作符(`foo[index]`)被`Index`和`IndexMut` trait重载。
`..`是range表示方法。 Range是标准库定义的一组结构体。
前后的索引值可以省略，默认右边的值是不包含的， 如果要包含右边的值，使用`=`，:

```rust
fn main() {
    // 0 or greater
    println!("{:?}", (0..).contains(&100)); // true
    // strictly less than 20
    println!("{:?}", (..20).contains(&20)); // false
    // 20 or less than 20
    println!("{:?}", (..=20).contains(&20)); // true
    // only 3, 4, 5
    println!("{:?}", (3..6).contains(&4)); // true
}
```

借用规则同样应用于slice:

```rust
fn tail(s: &[u8]) -> &[u8] {
  &s[1..] 
}

fn main() {
    let x = &[1, 2, 3, 4, 5];
    let y = tail(x);
    println!("y = {:?}", y);
}
```

与下面的代码相同:

```rust
fn tail<'a>(s: &'a [u8]) -> &'a [u8] {
  &s[1..] 
}
```

下面的代码是合法的:

```rust
fn main() {
    let y = {
        let x = &[1, 2, 3, 4, 5];
        tail(x)
    };
    println!("y = {:?}", y);
}
```

只不过是因为`[1, 2, 3, 4, 5]`是`'static`。下面的代码就不合法:

```rust
fn main() {
    let y = {
        let v = vec![1, 2, 3, 4, 5];
        tail(&v)
        // error: `v` does not live long enough
    };
    println!("y = {:?}", y);
}
```

这是因为`vector`分配在堆上，它有非`'static`的声明周期。

`&str`实际上是slice:

```rust
fn file_ext(name: &str) -> Option<&str> {
    // this does not create a new string - it returns
    // a slice of the argument.
    name.split(".").last()
}

fn main() {
    let name = "Read me. Or don't.txt";
    if let Some(ext) = file_ext(name) {
        println!("file extension: {}", ext);
    } else {
        println!("no file extension");
    }
}
```

借用规则同样适用:

```rust
fn main() {
    let ext = {
        let name = String::from("Read me. Or don't.txt");
        file_ext(&name).unwrap_or("")
        // error: `name` does not live long enough
    };
    println!("extension: {:?}", ext);
}
```

返回失败的函数典型地返回`Result`:

```rust
fn main() {
    let s = std::str::from_utf8(&[240, 159, 141, 137]);
    println!("{:?}", s);
    // prints: Ok("🍉")

    let s = std::str::from_utf8(&[195, 40]);
    println!("{:?}", s);
    // prints: Err(Utf8Error { valid_up_to: 0, error_len: Some(1) })
}
```

如果处理失败的时候想panic,你可以调用`.unwrap()`:

```rust
fn main() {
    let s = std::str::from_utf8(&[240, 159, 141, 137]).unwrap();
    println!("{:?}", s);
    // prints: "🍉"

    let s = std::str::from_utf8(&[195, 40]).unwrap();
    // prints: thread 'main' panicked at 'called `Result::unwrap()`
    // on an `Err` value: Utf8Error { valid_up_to: 0, error_len: Some(1) }',
    // src/libcore/result.rs:1165:5
}
```

或者调用`.expect()` panic一个定制的信息:

```rust
fn main() {
    let s = std::str::from_utf8(&[195, 40]).expect("valid utf-8");
    // prints: thread 'main' panicked at 'valid utf-8: Utf8Error
    // { valid_up_to: 0, error_len: Some(1) }', src/libcore/result.rs:1165:5
}
```

抑或使用`match`:

```rust
fn main() {
    match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e),
    }
    // prints 🍉
}
```

甚至使用`if let`:

```rust
fn main() {
    if let Ok(s) = std::str::from_utf8(&[240, 159, 141, 137]) {
        println!("{}", s);
    }
    // prints 🍉
}
```

再不济给上层抛出错误:

```rust
fn main() -> Result<(), std::str::Utf8Error> {
    match std::str::from_utf8(&[240, 159, 141, 137]) {
        Ok(s) => println!("{}", s),
        Err(e) => return Err(e),
    }
    Ok(())
}
```

常用的简洁方式是使用`?`:

```rust
fn main() -> Result<(), std::str::Utf8Error> {
    let s = std::str::from_utf8(&[240, 159, 141, 137])?;
    println!("{}", s);
    Ok(())
}
```

`*`符号常用来解引用，但是你不需要专门访问字段或者调用方法:

```rust
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 3.0 };
    let p_ref = &p;
    println!("({}, {})", p_ref.x, p_ref.y);
}

// prints `(1, 3)`
```

如果类型是可复制的(实现`Copy`),那么你可以:

```rust
struct Point {
    x: f64,
    y: f64,
}

fn negate(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.y,
    }
}

fn main() {
    let p = Point { x: 1.0, y: 3.0 };
    let p_ref = &p;
    negate(*p_ref);
    // error: cannot move out of `*p_ref` which is behind a shared reference
}
// now `Point` is `Copy`
#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn negate(p: Point) -> Point {
    Point {
        x: -p.x,
        y: -p.y,
    }
}

fn main() {
    let p = Point { x: 1.0, y: 3.0 };
    let p_ref = &p;
    negate(*p_ref); // ...and now this works
}
```

闭包(`Closure`)是实现了`Fn`、`FnMut`、`FnOnce`类型的函数。并且带有捕获的上下文。

参数是以逗号分隔的名称列表，在`|`之中。它们不需要大括号，除非你有多行语句:

```
fn for_each_planet<F>(f: F)
    where F: Fn(&'static str)
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}
 
fn main() {
    for_each_planet(|planet| println!("Hello, {}", planet));
}

// prints:
// Hello, Earth
// Hello, Mars
// Hello, Jupiter
```

借用规则同样适用:

```
fn for_each_planet<F>(f: F)
    where F: Fn(&'static str)
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}
 
fn main() {
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));
    // our closure borrows `greeting`, so it cannot outlive it
}
```

比如下面的代码就不工作:

```
fn for_each_planet<F>(f: F)
    where F: Fn(&'static str) + 'static // `F` must now have "'static" lifetime
{
    f("Earth");
    f("Mars");
    f("Jupiter");
}

fn main() {
    let greeting = String::from("Good to see you");
    for_each_planet(|planet| println!("{}, {}", greeting, planet));
    // error: closure may outlive the current function, but it borrows
    // `greeting`, which is owned by the current function
}
```

但是下面的代码就可以:

```
fn main() {
    let greeting = String::from("You're doing great");
    for_each_planet(move |planet| println!("{}, {}", greeting, planet));
    // `greeting` is no longer borrowed, it is *moved* into
    // the closure.
}
```

`FnMut`需要可变地借用去调用，所以它同时只能调用一次。

下面的代码合法：

```
fn foobar<F>(f: F)
    where F: Fn(i32) -> i32
{
    println!("{}", f(f(2))); 
}
 
fn main() {
    foobar(|x| x * 2);
}

// output: 8
```

下面的代码不合法:

```
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    println!("{}", f(f(2))); 
    // error: cannot borrow `f` as mutable more than once at a time
}
 
fn main() {
    foobar(|x| x * 2);
}
```

然后下民的代码有合法了:

```
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    let tmp = f(2);
    println!("{}", f(tmp)); 
}
 
fn main() {
    foobar(|x| x * 2);
}

// output: 8
```

之所以有`FnMut`类似，是因为有些闭包会可变地借用本地变量:

```
fn foobar<F>(mut f: F)
    where F: FnMut(i32) -> i32
{
    let tmp = f(2);
    println!("{}", f(tmp)); 
}
 
fn main() {
    let mut acc = 2;
    foobar(|x| {
        acc += 1;
        x * acc
    });
}

// output: 24
```

这些闭包不能传给期望`Fn`类型的参数:

```
fn foobar<F>(f: F)
    where F: Fn(i32) -> i32
{
    println!("{}", f(f(2))); 
}
 
fn main() {
    let mut acc = 2;
    foobar(|x| {
        acc += 1;
        // error: cannot assign to `acc`, as it is a
        // captured variable in a `Fn` closure.
        // the compiler suggests "changing foobar
        // to accept closures that implement `FnMut`"
        x * acc
    });
}
```

`FnOnce`闭包只会被调用一次。因为一些闭包会把move进来的变量在返回值中move out:

```
fn foobar<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f()); 
}
 
fn main() {
    let s = String::from("alright");
    foobar(move || s);
    // `s` was moved into our closure, and our
    // closures moves it to the caller by returning
    // it. Remember that `String` is not `Copy`.
}
```

这是自然限制的，因为`FnOnce`闭包在调用的时候需要被move:

```
fn foobar<F>(f: F)
    where F: FnOnce() -> String
{
    println!("{}", f()); 
    println!("{}", f()); 
    // error: use of moved value: `f`
}
```

而且你确保使用`move`调用闭包，它依然是非法的:

```
fn main() {
    let s = String::from("alright");
    foobar(move || s);
    foobar(move || s);
    // use of moved value: `s`
}
```

但是下面的代码就是合法的:

```
fn main() {
    let s = String::from("alright");
    foobar(|| s.clone());
    foobar(|| s.clone());
}
```

下面是有两个参数的闭包:

```
fn foobar<F>(x: i32, y: i32, is_greater: F)
    where F: Fn(i32, i32) -> bool
{
    let (greater, smaller) = if is_greater(x, y) {
        (x, y)
    } else {
        (y, x)
    };
    println!("{} is greater than {}", greater, smaller);
}
 
fn main() {
    foobar(32, 64, |x, y| x > y);
}
```

下面是忽略了这两个参数的闭包:

```
fn main() {
    foobar(32, 64, |_, _| panic!("Comparing is futile!"));
}
```

下面是一个唱“滴答”的闭包:

```
fn countdown<F>(count: usize, tick: F)
    where F: Fn(usize)
{
    for i in (1..=count).rev() {
        tick(i);
    }
}
 
fn main() {
    countdown(3, |i| println!("tick {}...", i));
}

// output:
// tick 3...
// tick 2...
// tick 1...
```

下面是一个马桶闭包:

```
fn main() {
    countdown(3, |_| ());
}
```

`|_| ()`看起来像不像一个🚽？

任意的迭代的东西都可以放在`for in`循环中。

前面我们已经看到了range，也可以使用`for in`操作`Vec`:

```
fn main() {
    for i in vec![52, 49, 21] {
        println!("I like the number {}", i);
    }
}
```

或者处理slice:

```
fn main() {
    for i in &[52, 49, 21] {
        println!("I like the number {}", i);
    }
}

// output:
// I like the number 52
// I like the number 49
// I like the number 21
```

或者是一个实际的迭代器:

```
fn main() {
    // note: `&str` also has a `.bytes()` iterator.
    // Rust's `char` type is a "Unicode scalar value"
    for c in "rust".chars() {
        println!("Give me a {}", c);
    }
}

// output:
// Give me a r
// Give me a u
// Give me a s
// Give me a t
```

即使迭代器的元素被加上了过滤、映射、展开等操作:

```
fn main() {
    for c in "sHE'S brOKen"
        .chars()
        .filter(|c| c.is_uppercase() || !c.is_ascii_alphabetic())
        .flat_map(|c| c.to_lowercase())
    {
        print!("{}", c);
    }
    println!();
}

// output: he's ok
```

你可以在一个函数中返回一个闭包:

```
fn make_tester(answer: String) -> impl Fn(&str) -> bool {
    move |challenge| {
        challenge == answer
    }
}

fn main() {
    // you can use `.into()` to perform conversions
    // between various types, here `&'static str` and `String`
    let test = make_tester("hunter2".into());
    println!("{}", test("******"));
    println!("{}", test("hunter2"));
}
```

甚至你可以move函数的参数的引用到闭包中返回:

```
fn make_tester<'a>(answer: &'a str) -> impl Fn(&str) -> bool + 'a {
    move |challenge| {
        challenge == answer
    }
}

fn main() {
    let test = make_tester("hunter2");
    println!("{}", test("*******"));
    println!("{}", test("hunter2"));
}

// output:
// false
// true
```

省略生命周期名称:

```
fn make_tester(answer: &str) -> impl Fn(&str) -> bool + '_ {
    move |challenge| {
        challenge == answer
    }
}
```

好了，半小时到了，通过本次学习，你应该能通过线上大部分的rust代码。

写rust代码和读rust又有很大的不同。一方面你不是在读一个问题的解决访问，而是正在解决它；另一方面Rust编译器会给你很大的帮助。

对于上面非法的rust代码，rustc总是会给你清楚的错误提示和富有洞察力的建议。

如果提示缺失，编译器组欢迎你提出来。

如果需要更多的Rust学习资料，你可以访问下面的资源:

-   [The Rust Book](https://doc.rust-lang.org/book/)
-   [Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)
-   [Read Rust](https://doc.rust-lang.org/stable/rust-by-example/)
-   [This Week In Rust](https://this-week-in-rust.org/)