
//N1
// println!("Hello, world!");
//N2
//    input.chars().rev().collect::<String>()

use time::PrimitiveDateTime as DateTime;
pub fn after(start: DateTime) -> DateTime {
    // start.checked_add(1000000i32.seconds());
    // start.checked_add(1.days(),None);
    start.checked_add(2_i32.days());
 
    return start;
  //  unimplemented!("What time is a gigasecond later than {start}");
}
fn main() {
   
}
