# Rust：Trait



## **1、Trait是什么？**

一个Trait描述了一种抽象接口（找不到很合适的词），这个抽象接口可以被类型继承。`Trait`只能由三部分组成（可能只包含部分）：

-   functions（方法）
-   types（类型）
-   constants（常量）

所有的`Trait`都定义了一个隐含类型`Self`，其指向实现该`Trait`的类型。`Traits`可能也包含额外的类型参数，这些类型参数（包括`Self`），与往常一样可能受到其他`Traits`等的约束。

类型需要通过独立的`implementations`去实现不同的`Trait`。

Items associated with a trait do not need to be defined in the trait, but they may be.（不知道怎么翻译合适？意会意会，Emm）。如果`Trait`提供了定义，该定义即为任何实现它的类型的默认行为（如果对应类型没有override的话）；如果`Trait`未提供定义，则任何实现它的类型都必须提供一个定义。

### **2、`Self`与`self`**

`Self`：实现`Trait`的类型的别名

```
self`：方法参数  `fn f(self) {}`，等价于`fn f(self: Self) {}
```

同理有：

`&self`等价于`self: &Self`，

```
&mut self`等价于`self: &mut Self
```

### **3、默认实现 + 无Override**

```rust
 trait Hello {
     fn say_hi(&self) {
         println!("hi");
     }
 }
 
 struct Student {}
 impl Hello for Student {}
 struct Teacher {}
 impl Hello for Teacher {}
 
 fn main() {
     let s = Student {};
     s.say_hi();
     let t = Teacher {};
     t.say_hi();
 }
```

### **默认实现 + Override**

```rust
 trait Hello {
     fn say_hi(&self) {
         println!("hi");
     }
 }
 
 struct Nobody {}
 impl Hello for Nobody {}
 struct Teacher {}
 impl Hello for Teacher {
     fn say_hi(&self) {
         println!("hi, I'm teacher Lee.");
     }
 }
 
 fn main() {
     let n = Nobody {};
     n.say_hi();
     let t = Teacher {};
     t.say_hi();
 }
```

### **无默认实现**

```rust
 trait Hello {
     fn say_hi(&self);
 }
 
 struct Student {}
 impl Hello for Student {
     fn say_hi(&self) {
         println!("hi, I'm Jack.");
     }
 }
 struct Teacher {}
 impl Hello for Teacher {
     fn say_hi(&self) {
         println!("hi, I'm teacher Lee.");
     }
 }
 
 fn main() {
     let s = Student {};
     s.say_hi();
     let t = Teacher {};
     t.say_hi();
 }
```

### **4、孤儿规则（Orphan rule）**

如果我们有如下定义，会是什么样呢？

```rust
 use std::ops::Add;
 
 impl Add<i32> for i32 {
     type Output = i32;
     fn add(self, other: i32) -> Self::Output {
         self - other
     }
 }
 
 fn main() {}
```

会发生编译错误，其中一个错误为：

```bash
 error[E0117]: only traits defined in the current crate can be implemented for arbitrary types
  --> src\main.rs:3:1
   |
 3 | impl Add<i32> for i32 {
   | ^^^^^--------^^^^^---
   | |    |            |
   | |    |            `i32` is not defined in the current crate
   | |    `i32` is not defined in the current crate
   | impl doesn't use only types from inside the current crate
   |
   = note: define and implement a trait or new type instead
```

这个就是孤儿规则在起作用：当你为某类型实现某 `trait` 的时候，该类型或者`trait`至少有一个是在当前 crate 中定义的，你不能为第三方的类型实现第三方的 `trait` 。

### **5、Trait与泛型**

```rust
 trait Hello<T> {
     fn world<T>(&self, i: T) -> String;
 }
```

上述就是一个简单的携带泛型T的`trait`

### **6、Trait用途一：接口抽象**

```rust
 trait Hello {
     fn say_hi(&self) {
         println!("hi");
     }
 }
 
 struct Student {}
 impl Hello for Student {}
 
 fn main() {
     let s = Student {};
     s.say_hi();
 }
```

### **7、Trait用途二：泛型约束**

比如要编写一个函数，但是只能对指定类型生效，这个时候可以借助`Trait`作为泛型的约束，称之为`Trait Bound`（`Trait`限定），当然`Trait Bound`可以同时存在多个，用`"+"`连接

```rust
 trait Run {
 }
 trait Eat {
 }
 #[derive(Debug)]
 struct Horse {
 }
 impl Run for Horse {
 }
 impl Eat for Horse {
 }
 fn demo<T: Run + Eat>(x: T) {}
 //如下方式是等价的
 //fn demo<T>(x: T) where T: Run + Eat {}
```

### **8、Trait用途三：作为抽象类型 - Trait Object（Trait对象，动态分发）**

当我们只关心某个类型是否实现了特定trait，而不关注其具体类型的时候，就该`Trait Object`（Trait对象）登场了，请看下面两个例子的差别，前者使用泛型+Trait bound，后者使用Trait对象

泛型模式：

```rust
 trait Run {}
 struct Human {}
 impl Run for Human {}
 struct Cat {}
 impl Run for Cat {}
 fn demo<T>(x: Vec<Box<T>>) where T: Run {}
 fn main() {
     let mut v = vec![];
     v.push(Box::new(Human {}));
     //v.push(Box::new(Cat {}));  //这行会导致编译失败
     demo(v);
 }
```

Trait对象模式：

```rust
 trait Run {}
 struct Human {}
 impl Run for Human {}
 struct Cat {}
 impl Run for Cat {}
 fn demo(x: Vec<Box<dyn Run>>) {}
 fn main() {
     let mut v: Vec<Box<dyn Run>> = vec![];
     v.push(Box::new(Human {}));
     v.push(Box::new(Cat {}));
     demo(v);
 }
```

`Trait object`的本质是指针，它可以指向不同的类型，指向的具体类型不同，调用的方法也不同。

### **对象安全**

一个`Trait Object`是指实现了一组`Traits`的某个类型的不确定值，这组`Traits`由对象安全的`Trait`以及`auto traits`组成，即一个`Trait`是对象安全的，如果满足：

-   `Trait`本身是没有`Self: Sized`约束
-   所有方法都是`Object Safe`（对象安全）的

所有方法都必须满足如下约束才能成为是`Object Safe`的

-   函数有`Self: Sized`约束，
-   -------------或者满足如下所有条件---------------
-   函数不能有泛型参数
-   第一个参数必须为`Self`类型或者可以解引用为`Self`的类型（目前包含`self` `&self` `&mut self` `self:: Box`）
-   其他参数或者返回值均不能使用`Self`类型

![img](https://pic1.zhimg.com/80/v2-f2bab8e000c259e2c80305e7c36f6130_720w.jpg)

**9、Trait用途四：作为抽象类型 - impl Trait**

`impl TraitName`可以作为返回值类型（更准确的说，`impl TraitName`并不是一个类型），也可以作为函数参数，2018版本，`impl Trait`只能出现在这两个位置。该使用方式在Closure和Iterator中十分有用，以后介绍；

```rust
 trait Run {
 }
 struct Human {
 }
 impl Run for Human {
 }
 fn demo() -> impl Run {
     Human {}
 }
 fn main() {
 }
```

特别需要指出的是，函数的不同分支的返回值需要为同一个具体类型，如下方式将会编译失败。

```rust
 trait Run {
 }
 struct Human {
 }
 impl Run for Human {
 }
 struct Cat {
 }
 impl Run for Cat {
 }
 fn demo(x: i32) -> impl Run {
     if x > 0 {
         Human {}
     } else {
         Cat {}
     }
 }
 fn main() {
 }
```

### **10、Trait中的所有权**

`Trait`方法如果接受了 `self` 类型的参数，则会消耗类型的值自身，比如

```rust
 trait Run {
     fn run_and_die(self);
 }
 #[derive(Debug)]
 struct Horse {
 
 }
 impl Run for Horse {
     fn run_and_die(self) {
 
     }
 }
 fn main() {
     let h = Horse {};
     h.run_and_die();
     //println!("{:?}", h);  //这行会导致失败，因为run_and_die会消耗自己
 }
```

### **11、Derive**

编译器允许你通过 `#[derive]` 属性自动实现一些`Trait`，这些`Trait`包含：

-   比较相关的：`Eq`，`PartialEq`，`Ord`，`PartialOrd`
-   `Clone`，经由&T创建T
-   `Copy`，实现T的复制语义
-   `Hash`，计算&T的哈希值
-   `Default`，创建数据类型的空实例
-   `Debug`，使得可以用 `{:?}`格式化T

你既可以通过`#[derive]`自动实现这些`Trait`，也可以自己手动去impl这些`Trait`

```rust
 #[derive(Debug,Copy, Clone)]
 struct Person {}
```

### **12、Unsafe Trait**

带有`unsafe`关键字定义的`Trait`，使用`unsafe Trait` 是`safe`的，一个类型实现`unsafe Trait`的时候，必须使用`unsafe impl`作为前缀，比如`Send`和`Sync`是`unsafe`的，则impl这两个`Trait`如下

```rust
 unsafe impl Send for Student {}
```

### **13、常见的Trait**

a. `Deref`，`DerefMut` ： 可以用来重载 `*`操作符，也可以用来  `method resolution` 以及 `deref coercions`（解引用转换）

b. `Drop`： 用于解构，在一个类型的变量被销毁前执行

c. `Copy` ：如果类型实现了这个`Trait`，则该类型的值会使用拷贝语义替代移动语义，并避免所有权的变更。一个类型实现`Copy`有两个前提条件：1、这个类型不能实现`Drop`；2、这个类型的所有字段都必须为`Copy`的。

编译器会为以下类型自动实现`Copy` ，除此之外的类型如果想要`impl Copy`必须先`impl Clone`：

-   Numeric types
-   `char`，`bool` 以及 `!`
-   由`Copy`类型组成的`Tuples`
-   由`Copy`类型组成的`Arrays`
-   `Shared references`（共享引用/借用）
-   `Raw pointers`（裸指针）

d. `Clone`：`Copy`的超集，它也是需要编译器生成`implementations`，编译器会为以下类型自动实现`Clone`：

-   内置实现了`Copy`的类型
-   由`Clone`类型组成的`Tuples`
-   由`Clone`类型组成的`Arrays`

e. `Send`：表明一个类型的值是否可以安全的在线程间传递。

f. `Sync`：表明一个类型的值是否可以安全的在线程间共享。

g. `Sized`：编译器可确定大小的类型

i. `Unsize`：编译器无法确定大小的类型

### **14、Operator Traits**

```
std::cmp`中定义的`Traits
std::ops`中定义的`Traits
```

可以用来重载操作符，slice索引表达式（indexing expressions） 以及 调用表达式（call expressions）

### **15、Auto traits**

```
Send`，`Sync`，`UnwindSafe`，`RefUnwindSafe
```

### **16、消除Trait歧义（Disambiguating overlapping traits）**

假设为某一个类型，实现了两个`Trait`，这两个`Trait`刚好有同名的method，这个时候如何区分呢？

```rust
 trait TraitOne {
     fn action(&self) {
         println!("action of trait one!")
     }
 }
 trait TraitTwo {
     fn action(&self) {
         println!("action of trait two!");
     }
 }
 struct Person {}
 impl TraitOne for Person {}
 impl TraitTwo for Person {}
 fn main() {
     let p = Person {};
     <Person as TraitOne>::action(&p);
     <Person as TraitTwo>::action(&p);
     //如下方式会编译失败
     //p.action();
 }
```