//cool array
// let mut array :[[i32;5];5] = [[3;5];5]; 5x5 filled with 3s

//references
// let mut num1 : i32 = 1;
// let mut a = &mut num1;
// let mut b = &mut *a;
// *b = 4;
// println!("{}",a);
// *a = 3;
// println!("{}",a);

//infinite loop
// let mut counter = 0;

// let result = loop {
//     counter += 1;

//     if counter == 10 {
//                  pass to var
//         break counter * 2;
//     }
// };

// println!("The result is {result}");

// breaking out of loop using loop labels
// let mut count = 0;
// 'counting_up: loop {
//     println!("count = {count}");
//     let mut remaining = 10;

//     loop {
//         println!("remaining = {remaining}");
//         if remaining == 9 {
//             break;
//         }
//         if count == 2 {
//             break 'counting_up;
//         }
//         remaining -= 1;
//     }

//     count += 1;
// }
// println!("End count = {count}");

// change var in function;
// let mut s = String::from("hello");

// change(&mut s);
// println!("{s}");
// 

// fn change(some_string: &mut String) {
// some_string.push_str(", world");
// }

//Not a problem
// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// println!("{} and {}", r1, r2);
// // variables r1 and r2 will not be used after this point

// let r3 = &mut s; // no problem
// println!("{}", r3);

//Problem
// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // Big problem
// println!("{} and {}", r1, r2,r3);

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// fn no_dangle() -> String {
//     let s = String::from("hello");

//     s
// }

        //index and value  iter() returns each element in a collection   enumerate wraps the result of iter and returns each element as part of a tuple instead. 
        // The first element of the tuple returned from enumerate is the index, and the second element is a reference to the element
// for (i, &item) in bytes.iter().enumerate() {}

// the same
// let s = String::from("hello");

// let len = s.len();

// let slice = &s[3..len];
// let slice = &s[3..];
// println!("{}",&slice)

//same thing but more general

// let a = [1, 2, 3, 4, 5];

// let slice = &a[1..3];

// assert_eq!(slice, &[2, 3]);

// Same thing
// let user2 = User {
//     active: user1.active,
//     username: user1.username,
//     email: String::from("another@example.com"),
//     sign_in_count: user1.sign_in_count,
// };

// let user2 = User {
//     email: String::from("another@example.com"),
//     ..user1
// };

//tuple struct
// struct Color(i32, i32, i32);

// let black = Color(0, 0, 0);

// struct AlwaysEqual;
// let subject = AlwaysEqual;

// fn main() {
//     let rect1 = (30, 50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rect1)
//     );
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {:?}", rect1);
// }

// Listing 5-12: Adding the attribute to derive the Debug trait and printing the Rectangle instance using debug formatting

// when to use in implementation
// We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership
// We just want to read the data in the struct, not write to it. 
// If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter.
// Having a method that takes ownership of the instance by using just self as the first parameter is rare; 
// this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.

// Here’s how it works: when you call a method with object.something(), 
// Rust automatically adds in &, &mut, or * so object matches the signature of the method. 
// In other words, the following are the same:

// p1.distance(&p2);
// (&p1).distance(&p2);

// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     //Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
//     //  These are often called new
//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }

// let rect1 = Rectangle {
//     width: 30,
//     height: 50,
// };
// let rect2 = Rectangle {
//     width: 10,
//     height: 40,
// };
// let rect3 = Rectangle {
//     width: 60,
//     height: 45,
// };
// println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
// println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

// enum IpAddrKind {
//     V4,
//     V6,
// }

// Initializization
// let four = IpAddrKind::V4;
// let six = IpAddrKind::V6;

// function that accepts both
// fn route(ip_kind: IpAddrKind) {}

// Option, which is another enum defined by the standard library. 
// The Option type encodes the very common scenario in which a value could be something or it could be nothing.
//Standard definition for Option
// enum Option<T> {
//     None,
//     Some(T),
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// #[derive(Debug)] // so we can inspect the state in a minute
// enum UsState {
//     Alabama,
//     Alaska,
//     // --snip--
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         }
//     }
// }
// At that point, the binding for state will be the value UsState::Alaska

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

// other => move_player(other) can be used in a match case to cover everything else
// If wont have to use the catch-all value, we can change our code to use _ which doesnt bind to the value instead of the variable named other as such:
// let dice_roll = 9;
//     match dice_roll {
//         3 => add_fancy_hat(),
//         7 => remove_fancy_hat(),
//         _ => reroll(),
//     }

//     fn add_fancy_hat() {}
//     fn remove_fancy_hat() {}
//     fn reroll() {}

// Are the same
// let config_max = Some(3u8);
// match config_max {
//     Some(max) => println!("The maximum is configured to be {}", max),
//     _ => (),
// }

// let config_max = Some(3u8);
// if let Some(max) = config_max {
//     println!("The maximum is configured to be {}", max);
// }

//crates
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }


// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

//calling
// let mut meal = back_of_house::Breakfast::summer("Rye");
// // Change our mind about what bread we'd like
// meal.toast = String::from("Wheat");
// println!("I'd like {} toast please", meal.toast);

// use applies only to local context if another module is added and something linking to the use statement it wont work
// use can also be aliased

//Same thing
// use std::cmp::Ordering;
// use std::io;

// use std::{cmp::Ordering, io};

//Vectors

// Macro detects type
// let v: Vec<i32> = Vec::new();
// let v = vec![1, 2, 3];

// Two ways to do get element
// let v = vec![1, 2, 3, 4, 5];

// let third: &i32 = &v[2];
// println!("The third element is {third}");

// let third: Option<&i32> = v.get(2);
// match third {
//     Some(third) => println!("The third element is {third}"),
//     None => println!("There is no third element."),
// }

// Fortunately, the variants of an enum are defined under the same enum type, 
// so when we need one type to represent elements of different types, we can define and use an enum!
// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

// let row = vec![
//     SpreadsheetCell::Int(3),
//     SpreadsheetCell::Text(String::from("blue")),
//     SpreadsheetCell::Float(10.12),
// ];

// let s1 = String::from("tic");
// let s2 = String::from("tac");
// let s3 = String::from("toe");

// let s = format!("{s1}-{s2}-{s3}");

// String .chars() .bytes()

//hashmap
// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

//retrieving by key
// let team_name = String::from("Blue");
// let score = scores.get(&team_name).copied().unwrap_or(0);

//iterating
// use std::collections::HashMap;

// let mut scores = HashMap::new();

// scores.insert(String::from("Blue"), 10);
// scores.insert(String::from("Yellow"), 50);

// Enum Entry checks if key isnt already present if not insert 50
// scores.entry(String::from("Blue")).or_insert(50);

// for (key, value) in &scores {
//     println!("{key}: {value}");
// }

// Updating a value based on old one
// let text = "hello world wonderful world";

// let mut map = HashMap::new();

// for word in text.split_whitespace() {
//     let count = map.entry(word).or_insert(0);
//     *count += 1;
// }

// println!("{:?}", map);
fn main() {
   
}

