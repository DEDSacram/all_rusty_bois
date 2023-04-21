use std::io;
use std::io::BufRead;

use std::io::BufReader;
use std::fs::File;

use std::fs;


struct Maze{
    name: String,
    description: String,
    grid : Vec<Vec<String>>
}

fn read_and_convert() -> io::Result<Vec<Maze>> {

    //path
    let paths = fs::read_dir("./maze").unwrap();

    let mut mazes: Vec<Maze> = Vec::new();

    for path in paths {
        let unwrapped_pth = path.unwrap();
        let f: File = File::open(unwrapped_pth.path())?;
        let reader: BufReader<File> = BufReader::new(f);
        let mut divided: Vec<Vec<String>> = vec![];    
        // println!("Name: {}", path.unwrap().path().display());
        for line in reader.lines() {
            let ln = line?;
            let parts = ln.split("").map(|x| x.to_owned());
            let mut arr = parts.collect::<Vec<String>>();
            arr.pop();
            arr.remove(0);
            divided.push(arr);
        }
        
        let name = unwrapped_pth.file_name().to_str().unwrap().to_owned();
        let description = divided.remove(0);

        mazes.push(Maze { name: name, description: description.join(""), grid: divided })
    }
    Ok(mazes)
}

fn check_around(y: usize,x : usize,grid : &Vec<Vec<String>>) -> (Option<String>, [bool; 4]){
              // up down left right
              let mut crossroad: [bool; 4] = [false;4];

              let mut current = Some(grid[y][x].clone());
              if(y > 0){
                  //check up
                  if !(grid[y-1][x] == "#"){
                      crossroad[0] = true;
                  }
                  }
  
              if(y < grid.len()){
                    //check down
                  if !(grid[y+1][x] == "#"){
                      crossroad[1] = true;
                  }
              };
       
              if(x > 0){
                      //check left
                  if !(grid[y][x-1] == "#"){
                      crossroad[2] = true;
                  }
              }
              if(x < grid[y].len()){
               //check right
              if !(grid[y][x+1] == "#"){
                  crossroad[3] = true;
              }
              }
    (current,crossroad)
}
fn main() {
    let each: Vec<Maze> = read_and_convert().unwrap();

    for maze in each{
        println!("filename : {}",maze.name);
        println!("Goal {}", maze.description);

        let grid = maze.grid;
        //entrance
        let mut entrance : Option<usize> = None;
        for y in 0..grid.len() {
          if !(grid[y][0] == "#"){
            entrance = Some(y);
          }
        }
        println!("Entrance on index {}",entrance.unwrap());

        for x in 0..grid.len(){
            for y in 0..grid[x].len(){
                print!("{}",grid[x][y])
            }
            println!();
        }
        // check all
        
        let mut found_str :Vec<(u32,String)> = Vec::new();

        let mut cost = 0;

        if entrance.is_none(){
            panic!("no entrance");
        }
        

        let mut y = entrance.unwrap();
        let mut x = 0;

        loop {
            let (letter,crossroad) = check_around(y, x, &grid);
            // println!("{}",letter.unwrap());
            if crossroad.iter().filter(|ele| ele == &&true).collect::<Vec<&bool>>().len() == 0{
                println!("End of road");
                break;
            }else{
                // for x in 0..crossroad.len(){
                //     if(crossroad[x] == true){
                //     }
                // }
            }
            cost+=1;
        }
    }

}
