// HashSet
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
