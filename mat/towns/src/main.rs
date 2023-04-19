use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2, Vector2};
use speedy2d::window::{
    KeyScancode,
    ModifiersState,
    MouseButton,
    MouseScrollDistance,
    VirtualKeyCode,
    WindowHandler,
    WindowHelper,
    WindowStartupInfo
};
use speedy2d::{Graphics2D, Window};
use speedy2d::shape::Rectangle;
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Town{
    id : u32,
    name: String,
    inhabitants : u32,
    x : f32,
    y : f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
 struct Road{
    start : u32,
    end : u32,
    upgradePrice : u32,
    travelTime : u32
}

#[derive(Debug, Serialize, Deserialize)]
struct Response{
     towns: Vec<Town>,
     roads: Vec<Road>
}

impl Response{
    fn return_towns(&self) -> Vec<Town>{
        self.towns.clone()
    }
    fn return_roads(&self) -> Vec<Road>{
        self.roads.clone()
    }
}


async fn getfromapi()-> Result<Response,  Error>{
    let res: Response = Client::new()
    .get("https://maturita.delta-www.cz/prakticka/2023-map/mapData/")
    .send()
    .await?
    .json()
    .await?;
Ok((res))
}



struct MyWindowHandler {
    buttons : Vec<Vector2<f32>>,
    mouse_pos: Vector2<f32>,
    mouse_button_down: bool
}

impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

       
      
    
        for button in &self.buttons{
            // graphics.draw_rectangle(Rectangle::new(x[0].clone(),x[1].clone()), Color::BLACK);
            graphics.draw_circle((button.x, button.y), 15.0, Color::RED);
            graphics.draw_circle((button.x, button.y), 10.0, Color::BLUE);
        }
 
 
 
        // if(self.mouse_button_down){
        //     for x in &self.buttons{
        //         if self.mouse_pos.x > x[0].x as f32 && self.mouse_pos.y > x[0].y as f32 && self.mouse_pos.x < x[1].x as f32 && self.mouse_pos.y < x[1].y as f32{
        //             println!("{:?}",self.mouse_pos);
        //         }
               
        //     }

           
        // }

  

 

        helper.request_redraw();
    }



    fn on_user_event(
        &mut self,
        helper: &mut WindowHelper<()>,
        user_event: ()
    )
    {
        println!("Received user event: '{:?}'", user_event);
    }


    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: speedy2d::dimen::Vec2)
    {
        self.mouse_pos = position;
    }

  
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton)
    {
       

        if button == MouseButton::Left {
            self.mouse_button_down = true;
        }

        helper.request_redraw();
    }

    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper, button: MouseButton)
    {
      

        if button == MouseButton::Left {
            self.mouse_button_down = false;
        }

        helper.request_redraw();
    }


   // If desired, on_mouse_move(), on_key_down(), etc...
}

#[tokio::main]
async fn main(){

    
    let res: Response =  getfromapi().await.unwrap();
    // println!("{:#?}", res);

    let towns: Vec<Town> = res.return_towns();
    let roads: Vec<Road> = res.return_roads();


    let mut towncoords: Vec<Vector2<f32>> = Vec::new();
    // let mut max_x: f32 = 0.0;
    // let mut max_y : f32 = 0.0;
    for town in towns{
        // if(max_x > town.x + 15.0){
        //     max_x = town.x + 15.0;
        // }
        // if(max_y > town.y + 15.0){
        //     max_y = town.y + 15.0
        // }
        towncoords.push(Vector2::new(town.x,town.y));
    }

    //max
  
    
  

    // s.push([Vector2::new(0.0,0.0),Vector2::new(10.0,20.0)]);

    // s.push([Vector2::new(500.0,50.0),Vector2::new(600.0,90.0)]);

    let window: Window = speedy2d::Window::new_centered("Title", (900, 900)).unwrap();
    window.run_loop(MyWindowHandler{buttons: towncoords, mouse_pos: Vector2 { x: 0.0, y: 0.0 },
        mouse_button_down: false});
}
// http 

