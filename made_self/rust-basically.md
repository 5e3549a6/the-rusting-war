Rust primer



# 数据类型

![2020-05-30 11-49-30 的屏幕截图](/home/john/图片/2020-05-30 11-49-30 的屏幕截图.png)







### Structs

Rust 结构体可以使用 `struct Name { field1: T1, field2: T2 [, ...] }` 的形式声明。 T1,T2 表示类型，实例化一个struct也用类似的语法，没有struct关键字，比如
`Point { x: 1.0, y: 2.0 }` 。
 Rust 的结构体和C的非常类似，甚至内存布局也一样，所以可以从C程序读取Rust的struct。
 使用`mypoint.x`的形式访问struct的值。

```
struct Point {
    x: f64,
    y: f64
}
```

如果struct的实例是可变的，此实例的字段也都是可变的。
 比如

```
let mut mypoint = Point { x: 1.0, y: 1.0 }; // 可变
let origin = Point { x: 0.0, y: 0.0 }; // 不可变

mypoint.y += 1.0; // `mypoint`  是可变的，字段也是
origin.y += 1.0; // ERROR: assigning to immutable field
```

`match` 模式匹配也可以匹配struct，基本的语法是
`Name { fieldname: pattern, ... }`:

```
match mypoint {
    Point { x: 0.0, y: yy } => println!("{}", yy),
    Point { x: xx,  y: yy } => println!("{} {}", xx, yy)
}
```

在对struct的模式匹配中，你可能不需要所有的字段，可以使用 `..` （ `Name { field1, .. }`）忽略其他的字段。例如：

```
match mypoint {
    Point { x, .. } => println!("{}", x)
}
```

### Enums 枚举

一个简单的枚举类型定义：

```
enum Direction {
    North,
    East,
    South,
    West
}
```

其中North 是 0, East 是 1, South 是 2, and West 是 3. 默认值是先前的值加一。
 使用`as` 可以把North 转为int

```
println!( "{:?} => {}", North, North as int );
```

指定常量值：

```
enum Color {
  Red = 0xff0000,
  Green = 0x00ff00,
  Blue = 0x0000ff
}
```

更复杂的情况：

```
enum Shape {
    Circle(Point, f64),
    Rectangle(Point, Point)
}
```

`Circle` 包含了一个`Point` 结构体和一个f64，`Rectangle`包含了两个`Point`结构体。
 声明定义了类型`Shape`，`Shape` 指的是一个·`Shape` 类型外加两个函数`Circle` 和`Rectangle` 用来构造该类型的值。
 创建新的Circle，这样写`Circle(Point { x: 0.0, y: 0.0 }, 10.0)`
 当然也可以进行模式匹配，访问枚举实例值的唯一方式是解构。例如：

```
use std::f64;
fn area(sh: Shape) -> f64 {
    match sh {
        Circle(_, size) => f64::consts::PI * size * size,
        Rectangle(Point { x, y }, Point { x: x2, y: y2 }) => (x2 - x) * (y2 - y)
    }
}
```

使用`_`忽略个别字段，忽略所有的字段使用`Circle(..)`。
 枚举的模式匹配：

```
fn point_from_direction(dir: Direction) -> Point {
    match dir {
        North => Point { x:  0.0, y:  1.0 },
        East  => Point { x:  1.0, y:  0.0 },
        South => Point { x:  0.0, y: -1.0 },
        West  => Point { x: -1.0, y:  0.0 }
    }
}
```

枚举变量也可以是struct，例如：

```
use std::f64;
enum Shape {
    Circle { center: Point, radius: f64 },
    Rectangle { top_left: Point, bottom_right: Point }
}
fn area(sh: Shape) -> f64 {
    match sh {
        Circle { radius: radius, .. } => f64::consts::PI * square(radius),
        Rectangle { top_left: top_left, bottom_right: bottom_right } => {
            (bottom_right.x - top_left.x) * (top_left.y - bottom_right.y)
        }
    }
}
```

### Tuples 

元组（Tuples）有点类似struct，不过元组的字段是没有名字的。你也不能通过点来访问字段。
 元组可以有任意个元素，没有0个元素的情况（`()` 如果你喜欢，可以当作空元组）。

```
let mytup: (int, int, f64) = (10, 20, 30.0);
match mytup {
  (a, b, c) => info!("{}", a + b + (c as int))
}
```

### Tuple structs

Rust 提供了`tuple structs`，元组和结构体的结合。`tuple structs` 是有名字的（Foo(1, 2)的类型和Bar(1, 2)的不一样）。`tuple structs`的字段没有名字。

例如：

```
struct MyTup(int, int, f64);
let mytup: MyTup = MyTup(10, 20, 30.0);
match mytup {
  MyTup(a, b, c) => info!("{}", a + b + (c as int))
}
```

有个特例是只有一个字段的`tuple structs` ，可以称作`newtypes` （类似Haskell里的"newtype"特性）。用来创建新的类型，而不是一种类型的别名而已。

```
struct GizmoId(int);
```

像这种类型定义对于区分基础类型一样，但是用途不一样的数据是很有用的。

```
struct Inches(int);
struct Centimeters(int);
```

上边的定义可以以一个简单的方式避免混淆不同单元的数字。整型值可以用模式匹配：

​                        

```rust
let length_with_unit = Inches(10);
let Inches(integer_length) = length_with_unit;
println!("length is {} inches", integer_length);
```



