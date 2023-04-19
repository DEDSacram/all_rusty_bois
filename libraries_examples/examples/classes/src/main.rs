
// trait Quack {
//     fn quack(&self);
// }

// struct Duck ();

// impl Quack for Duck {
//     fn quack(&self) {
//         println!("quack!");
//     }
// }

// struct RandomBird {
//     is_a_parrot: bool
// }

// impl Quack for RandomBird {
//     fn quack(&self) {
//         if ! self.is_a_parrot {
//             println!("quack!");
//         } else {
//             println!("squawk!");
//         }
//     }
// }

trait Geometry{
 
    fn print_edges(&self) -> i32;
}

#[derive(Debug)]
struct Triangle{
    edges: i32
}
#[derive(Debug)]
struct Rectangle{
    edges:i32
}

impl Geometry for Rectangle{
    fn print_edges(&self) -> i32{
       return self.edges
    }
}
impl Geometry for Triangle{
    fn print_edges(&self) -> i32{
        return self.edges
     }
}

fn func(geo : &impl Geometry){
    geo.print_edges();
}
fn func2(geo : &dyn Geometry){
    geo.print_edges();
}

fn main(){
  func(&Triangle{edges : 5
});
func2(&Triangle{edges : 5
});

let mut vec: Vec<Box<dyn Geometry>> = vec![Box::new(Triangle{edges : 5}),Box::new(Rectangle{edges : 6})];

for (index,value) in vec.iter().enumerate(){
   println!("index : {} - value : {}",index,value.print_edges());
}
}