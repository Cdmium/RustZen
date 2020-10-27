trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> self::Output;
}

// // 3-28
// use std::ops::Add;
// impl Add<u64> for u32 {
//     type Output = u64;
//     fn add(self, other: u64) -> Self::Output {
//         (self as u64) + other
//     }
// }
// fn main() {
//     let a = 1u32;
//     let b = 2u64;
//     println!("{}", a + b);
// }

// // trait add 3-22
// trait Add<RHS, Output> {
//     fn my_add(self, rhs: RHS) -> Output;
// }
// impl Add<i32, i32> for i32 {
//     fn my_add(self, rhs: i32) -> i32 {
//         self + rhs
//     }
// }
// impl Add<u32, i32> for u32 {
//     fn my_add(self, rhs: u32) -> i32 {
//         (self + rhs) as i32
//     }
// }
// fn main() {
//     let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
//     let x: i32 = a.my_add(b);
//     let y: i32 = c.my_add(d);
//     println!("{} {}", x, y);
// }

// // listing 3-21
// #[derive(Debug, PartialEq)]
// struct Foo(i32);
// #[derive(Debug, PartialEq)]
// struct Bar(i32, i32);
// trait Inst {
//     fn new(i: i32) -> Self;
// }
// impl Inst for Foo {
//     fn new(i: i32) -> Foo {
//         Foo(i)
//     }
// }
// impl Inst for Bar {
//     fn new(i: i32) -> Bar {
//         Bar(i, i + 10)
//     }
// }
// fn foobar<T: Inst>(i: i32) -> T {
//     T::new(i)
// }
// fn main() {
//     let f: Foo = foobar(10);
//     println!("{:?}", f);
//     let b: Bar = foobar(20);
//     println!("{:?}", b);
// }
// // monomorphization function
// fn foo_1(x: i32) -> i32 {
//     return x;
// }
// fn foo_2(x: &str) -> &str {
//     return x;
// }
// fn main() {
//     foo_1(1);
//     foo_2("2");
// }

// // generic structure
// #[derive(Debug, PartialEq)]
// struct Point<T> {x: T, y: T}
// impl<T> Point<T> {
//     fn new(x: T, y: T) -> Self {
//         Point { x, y }
//     }
// }
// fn main() {
//     let point1 = Point::new(1, 2);
//     let point2 = Point::new("1", "2");
//     assert_eq!(point1, Point { x: 1, y: 2 });
//     assert_eq!(point2, Point { x: "1", y: "2" });
// }

// // generic function
// fn foo<T>(x: T) -> T {
//     return x;
// }
// fn main() {
//     assert_eq!(foo(1), 1);
//     assert_eq!(foo("hello"), "hello");
// }

// fn main() {
//     let a = 0;
//     let a_pos = a.is_positive();
// }

// fn main() {
//     let x = "1";
//     assert_eq!(x.parse::<i32>().unwrap(), 1);
// }

// fn main() {
//     let x = "1";
//     let int_x: i32 = x.parse().unwrap();
//     assert_eq!(int_x, 1);
// }

// // 类型推导
// fn sum(a: u32, b: i32) -> u32 {
//     a + (b as u32)
// }
// fn main() {
//     let a = 1;
//     let b = 2;
//     assert_eq!(sum(a, b), 3);
//     let elem = 5u8;
//     let mut vec = Vec::new();
//     vec.push(elem);
//     assert_eq!(vec, [5]);
// }

// // Vec<()>
// fn main() {
//     let v: Vec<()> = vec![(); 10];
//     for i in v {
//         println!{"{:?}", i};
//     }
// }

// // ZST
// enum Void {}
// struct Foo;
// struct Baz {
//     foo: Foo,
//     qux: (),
//     baz: [u8; 0],
// }
// fn main() {

// }


// // size_of &[u32; 5] &[32]
// fn main() {
//     println!("{}", std::mem::size_of::<&[u32; 5]>());
//     println!("{}", std::mem::size_of::<&[u32]>());
// }


// // &mut [u32]
// fn reset(arr: &mut [u32]) {
//     arr[0] = 5;
//     arr[1] = 4;
//     arr[2] = 3;
//     arr[3] = 2;
//     arr[4] = 1;
//     println!("array length {:?}", arr.len());
//     println!("reset array {:?}", arr);
// }
// fn main() {
//     let mut arr = [1, 2, 3, 4, 5];
//     println!("reset before: origin array {:?}", arr);
//     {
//         let mut_arr: &mut[u32] = &mut arr;
//         reset(mut_arr);
//     }
//     println!("reset after: origin array {:?}", arr);
// }

// // [u32; 5]
// fn reset(mut arr: [u32; 5]) {
//     arr[0] = 5;
//     arr[1] = 4;
//     arr[2] = 3;
//     arr[3] = 2;
//     arr[4] = 1;
//     println!("reset str {:?}", arr);
// }
// fn main(){
//     let arr: [u32; 5] = [1, 2, 3, 4, 5];
//     reset(arr);
//     println!("origin are {:?}", arr);
// }

// // &str
// fn main(){
//     let str = "Hello Rust";
//     let ptr = str.as_ptr();
//     let len = str.len();
//     println!("{:p}", ptr);
//     println!("{:?}", len);
// }

// // main Result<T, E>
// use std::fs::File;
// fn main() -> Result<(), std::io::Error> {
//     let f = File::open("bar.txt")?;
//     Ok(())
// }

// // Result<T, E>
// fn main() {
//     let x: Result<i32, &str> = Ok(-3);
//     assert_eq!(x.is_ok(), true);
//     let x: Result<i32, &str> = Err("Some error message");
//     assert_eq!(x.is_ok(), false);
// }

// // Debug trait
// use std::fmt::*;
// struct Point {
//     x: i32,
//     y: i32,
// }
// impl Debug for Point {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
//     }
// }
// fn main() {
//     let origin = Point { x: 0, y: 0 };
//     println!("The origin is: {:?}", origin);
// }

// // trait
// struct Duck;
// struct Pig;
// trait Fly {
//     fn fly(&self) -> bool;
// }
// impl Fly for Duck {
//     fn fly(&self) -> bool {
//         return true;
//     }
// }
// impl Fly for Pig {
//     fn fly(&self) -> bool {
//         return false;
//     }
// }
// fn fly_static<T: Fly>(s: T) -> bool {
//     s.fly()
// }
// fn fly_dyn(s: &dyn Fly) -> bool {
//     s.fly()
// }
// fn main() {
//     let pig = Pig;
//     assert_eq!(fly_static::<Pig>(pig), false);
//     let duck = Duck;
//     assert_eq!(fly_static::<Duck>(duck), true);
//     assert_eq!(fly_dyn(&Pig), false);
//     assert_eq!(fly_dyn(&Duck), true);
// }

// // Option<T>
// use std::fmt::Debug;
// fn match_option<T: Debug>(o: Option<T>) {
//     match o {
//         Some(i) => println!("{:?}", i),
//         None => println!("nothing"),
//     }
// }
// fn main() {
//     let a: Option<i32> = Some(3);
//     let b: Option<&str> = Some("hello");
//     let c: Option<char> = Some('A');
//     let d: Option<u32> = None;
//     match_option(a);
//     match_option(b);
//     match_option(c);
//     match_option(d);
// }

// // Box<T>
// fn main() {
//     #[derive(Debug, PartialEq)]
//     struct Point {
//         x: f64,
//         y: f64,
//     }
//     let box_point = Box::new(Point { x: 0.0, y: 0.0 });
//     let unbox_point: Point = *box_point;
//     assert_eq!(unbox_point, Point { x: 0.0, y: 0.0 });
// }

// // BinaryHeap<T>
// use std::collections::BinaryHeap;
// fn main() {
//     let mut heap = BinaryHeap::new();
//     assert_eq!(heap.peek(), None);
//     let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45];
//     for &i in arr.iter() {
//         heap.push(i);
//     }
//     assert_eq!(heap.peek(), Some(&93));
//     println!("{:?}", heap);
// }

// // HashSet<K> BTreeSet<K>
// use std::collections::HashSet;
// use std::collections::BTreeSet;
// fn main() {
//     let mut hbooks = HashSet::new();
//     let mut bbooks = BTreeSet::new();
//     hbooks.insert("A Song of Ice and Fire");
//     hbooks.insert("The Emerald City");
//     hbooks.insert("The Odyssey");
//     if !hbooks.contains("The Emerald City") {
//         println!("We have {} books, but The Emerald City ain't one.", hbooks.len());
//     }
//     println!("{:?}", hbooks);
//     bbooks.insert("A Song of Ice and Fire");
//     bbooks.insert("The Emerald City");
//     bbooks.insert("The Odyssey");
//     println!("{:?}", bbooks);
// }
// // HashMap<K, V> BTreeMap<K, V>
// use std::collections::BTreeMap;
// use std::collections::HashMap;
// fn main() {
//     let mut hmap = HashMap::new();
//     let mut bmap = BTreeMap::new();
//     hmap.insert(3, "c");
//     hmap.insert(1, "a");
//     hmap.insert(2, "b");
//     hmap.insert(5, "e");
//     hmap.insert(4, "d");
//     bmap.insert(3, "c");
//     bmap.insert(1, "a");
//     bmap.insert(2, "b");
//     bmap.insert(5, "e");
//     bmap.insert(4, "d");
//     println!("{:?}", hmap);
//     println!("{:?}", bmap);
// }

// // LinkedList<T>
// use std::collections::LinkedList;
// fn main() {
//     let mut list1 = LinkedList::new();
//     list1.push_back('a');
//     let mut list2 = LinkedList::new();
//     list2.push_back('b');
//     list2.push_back('c');
//     list1.append(&mut list2);
//     println!("{:?}", list1);
//     println!("{:?}", list2);
//     list1.pop_front();
//     println!("{:?}", list1);
//     list1.push_front('e');
//     println!("{:?}", list1);
// }

// // VecDeque<T>
// use std::collections::VecDeque;
// fn main() {
//     let mut buf = VecDeque::new();
//     buf.push_front(1);
//     buf.push_front(2);
//     assert_eq!(buf.get(0), Some(&2));
//     assert_eq!(buf.get(1), Some(&1));
//     buf.push_back(3);
//     buf.push_back(4);
//     buf.push_back(5);

// }

// // Vec<T>
// fn main() {
//     let mut v1 = vec![];
//     v1.push(1);
//     v1.push(2);
//     v1.push(3);
//     assert_eq!(v1, [1,2,3]);
//     assert_eq!(v1[1], 2);
//     let mut v2 = vec![0, 10];
//     let mut v3 = Vec::new();
//     v3.push(4);
//     v3.push(5);
//     v3.push(6);
// }

// fn main() {
//     let s :&Option<String> = &Some("hello".to_string());
//     match s {
//         Some(s) => println!("s is {}", s),
//         _ => (),
//     }
// }

// enum Option {
//     Some(i32),
//     None,
// }
// fn main() {
//     let s = Some(32);
//     let num = s.unwrap();
//     match s {
//         Some(n) => println!("num is: {}", n),
//         None => (),
//     }
// }

// // 带参数枚举体
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }
// fn main() {
//     let x: fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
//     let y: fn(String) -> IpAddr = IpAddr::V6;
//     let home = IpAddr::V4(127, 0, 0, 1);
// }

// // 类C枚举体
// enum Color {
//     Red = 0xff0000,
//     Green = 0x00ff00,
//     Blue = 0x0000ff,
// }
// fn main() {
//     println!("roses are #{:06x}", Color::Red as i32);
//     println!("violets are #{:06x}", Color::Blue as i32);
// }

// // 无参数枚举体
// enum Number {
//     Zero,
//     One,
//     Two,
// }
// fn main() {
//     let a = Number::One;
//     match a {
//         Number::Zero => println!("{}", 0),
//         Number::One => println!("{}", 1),
//         Number::Two => println!("{}", 2),
//     }
// }

// struct Empty;
// fn main() {
//     let x = Empty;
//     println!("{:p}", &x);
//     let y = x;
//     println!("{:p}", &y);
//     let z = Empty;
//     println!("{:p}", &z);
//     assert_eq!((..), std::ops::RangeFull);
// }

// struct Interger(u32);
// type Int = u32;
// fn main() {
//     let int = Interger(10);
//     assert_eq!(int.0, 10);
//     let int: Int = 10;
//     assert_eq!(int, 10);
// }
// #[derive(Debug, PartialEq)]
// struct People {
//     name: &'static str,
//     gender: u32,
// }

// impl People {
//     fn new(name: &'static str, gender: u32) -> Self {
//         return People { name, gender };
//     }

//     fn name(&self) {
//         println!("name: {:?}", self.name);
//     }

//     fn set_name(&mut self, name: &'static str) {
//         self.name = name;
//     }

//     fn gender(&self) {
//         let gender = if self.gender == 1 {"boy"} else {"girl"};
//         println!("gender: {:?}", gender);
//     }
// }

// fn main() {
//     let alex = People::new("Alex", 1);
//     alex.name();
//     alex.gender();
//     assert_eq!(alex, People { name: "Alex", gender: 1});
//     let mut alice = People::new("Alice", 0);
//     alice.name();
//     alice.gender();
//     assert_eq!(alice, People { name: "Alice", gender: 0});
//     alice.set_name("Rose");
//     alice.name();
// }
