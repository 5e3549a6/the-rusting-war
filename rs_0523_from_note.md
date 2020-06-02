​	





enum:  

```rust
//处理IP地址的方法
enum IP { //枚举IP地址的类型
	V4,
	V6,
}
struct IPAddr { //使用结构体对IP的储存方式进行解析
	kind: IP,
	address: String,
}
let home = IPAddr { //ipv4的储存信息
    kind: IP:: V4,
    address: String::from("127.0.0.1"),
};
let loopback = IPAddr { //ipv6的储存信息
    kind:IP::V6,
    address: String::from("::1"),
};

//以上代码更简洁的表示法
enum IP {
    V4(String), //直接将数据附加到每个成员上,这样就不需要一个额外的结构体了
    V6(String),
}
let home = IP::V4(String::from("127.0.0.1"));
let loopback = IP::V6(String::from("::1"));
```



match:  

```rust
enum Num {
    one,
    two,
    three,
    four,
}
fn value_of_num(num: Num)->u32 {
    match num {
        Num:: one => 1,
        Num:: two => {
            println!("the value of two is {}",two);
            2
        },
        Num:: three => 3,
        Num:: four => 4,
    }
}
let value = value_of_num(2); //如果调用函数,它会像if表达式一样工作

//绑定值的模式
#[derive(Debug)]
enum UsState {
	Alabama,
    Alaska,
    //..etc
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_of_cents(coin: Coin)->u32 {
    match coin {
        Coin:: Penny => 1,
        Coin:: Nickel => 5,
        Coin:: Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        },
    }
}


//匹配Option<T>
fn add(x: Option<i32>)->Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = add(five);
let none = add(None);


```



if let:

```rust
# let U = Some(0u8);
if let Some(3) = U{
	println!("three")
}

//if let 控制流

//不使用if let
//不使用if let

#[derive(Debug)]
enum Us{
	Alabama,
	Alaska,
}

enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter(Us),
}
let coin = Coin::Penny;
let mut count = 0;
match coin{
	Coin::Quarter(state) => println！("State quarter from {:?}",state),
	_ => count += 1,
}

//使用if let

//使用if let
#[derive(Debug)]

enum U {
    A1,
    A2,
}

enum Coin {
    P,
    N,
    D,
    Quarter(U),
}

let coin = Coin::P;
let mut count = 0;
if let Coin::Quarter(state) = coin{
    println!("State quarter from {:?}",state);
}else{
    count += 1;
}

```



mod: 

-   如果有一个叫做foo的模块没有子模块,应该将foo的声明放入叫做foo.rs的文件中
-   如果一个叫做foo的模块拥有子模块,应该将foo的声明放入叫做foo/mod.rs的文件中













































