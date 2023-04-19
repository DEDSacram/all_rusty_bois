

use std::time::{SystemTime, UNIX_EPOCH, Duration};

// testing only
use std::{thread, time};

#[derive(Eq, PartialEq)]
pub enum State{
    Started,
    Paused,
    Stopped
}
struct Stopwatch{
    current : Duration,
    state : State,
    time : Option<SystemTime>,
    stop_points: Vec<u128>
}
impl Stopwatch{
    fn new() -> Self{
        Stopwatch { current: Duration::from_millis(0),state: State::Stopped, time: None, stop_points: Vec::new() }
    }

    //States Writes
    fn IsStarted(&self)-> bool{
        if matches!(&self.state,&State::Started){
            return true
        }
        false
    }
    fn IsPaused(&self)->bool{
        if matches!(self.state,State::Paused) {
            return true
        }
        false
    }
    fn SplitTime(&self)-> Vec<u128>{
        if(self.stop_points.is_empty()){
            println!("Nejsou tu žádné")
        }
        self.stop_points.clone()
    }
    fn getTime(&self) -> u128{
        if !matches!(self.state, State::Started){
            return self.current.as_millis()
        }
        (self.current + self.difference(self.time)).as_millis()
    }

    pub fn FormatMillis(millis : u128) -> String{
        let mut seconds = ((millis / 1000) as f64).floor();
        let mut minutes = ((seconds / 60.0) as f64).floor();
        let mut hours = ((minutes / 60.0) as f64).floor();
      
        seconds = seconds % 60.0;
        minutes = minutes % 60.0;
        hours = hours % 24.0;
        String::from(format!("{:0>2}:{:0>2}:{:0>2}.{:0>3}",hours,minutes,seconds,millis % 1000))
    }


    // Control 
    fn Start(&mut self) -> bool{
        // return if running
        if matches!(self.state,State::Started) {
            return false
        }
        // change state set time now
        if matches!(self.state,State::Paused) {
            self.state = State::Started;
            self.time = Some(SystemTime::now());
            return true
        }
    
        self.state = State::Started;
        self.time = Some(SystemTime::now());
        self.stop_points = Vec::new();
        true
    }

    fn Pause(&mut self) -> bool{
        if !matches!(self.state,State::Started){
            return false
        }
        self.state = State::Paused;
        self.current += self.difference(self.time);
        true
    }
    fn Stop(&mut self) -> bool{
        if matches!(self.state,State::Stopped) {
            return false
        }
        self.state = State::Stopped;
        self.current += self.difference(self.time);
        self.time = None;
        true
    }
    fn Add_SplitTime(&mut self)-> u128{
        let point = self.getTime();
        self.stop_points.push(point);
        point
    }
    

    fn difference(&self,before: Option<SystemTime>) -> Duration{
        let duration_start =  before.unwrap().duration_since(UNIX_EPOCH).expect("Time before");
    
        let duration_end = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time now");
        let time = duration_end-duration_start;
        time
    }

 

}



fn main() {
    // let mut timer = Stopwatch::new();
    // timer.Start();
    // thread::sleep(time::Duration::from_millis(100));
    // println!("{}",timer.getTime());
    // timer.Add_SplitTime();
    // thread::sleep(time::Duration::from_millis(100));
    // timer.Add_SplitTime();
    // println!("{}",timer.getTime());

    // for x in timer.SplitTime(){
    //     print!("{}",x);
    // }

    let mut sw = Stopwatch::new();
    sw.Start();

    thread::sleep(time::Duration::from_millis(1200));

    let time1 = sw.getTime();

    let time_formatted = Stopwatch::FormatMillis(time1);

    println!("formatted time {}",time_formatted);

    println!("Time elapsed in millis:{time1}");
    thread::sleep(time::Duration::from_millis(800));
    sw.Add_SplitTime();
    for x in &sw.SplitTime(){
        println!("{:?} ",x);
    }
    thread::sleep(time::Duration::from_millis(500));
    sw.Pause();

    let time2 = sw.getTime();

   
    println!("{time2}");






  
    // since.as_millis()

//     let start = SystemTime::now();
//     let end = SystemTime::now();
//     let current = start
//     .duration_since(UNIX_EPOCH)
//     .expect("Time went backwards");

//     let before = end
//     .duration_since(UNIX_EPOCH)
//     .expect("Time went backwards");
// println!("{:?}", before-current);

//     println!("Hello, world!");
}
