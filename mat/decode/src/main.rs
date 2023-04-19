// decoding


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


// working with a file
use std::io;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;


struct MyWindowHandler {
    buttons : Vec<[Vector2<f32>; 2]>,
    bits : Vec<bool>,
    mouse_pos: Vector2<f32>,
    mouse_button_down: bool
}


fn read_and_convert() -> io::Result<Vec<bool>> {
    let f = File::open("./file/squares.dat")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;

    let x = 0b_01101010_u8;

    // Read.
    let mut bits: Vec<bool> = Vec::new();
    for value in buffer {
        print!("{:?} ",value);
        // convert byte to bites
        for i in 0..8 {
            let mask = 1 << i;
            let bit_is_set = (mask & value) > 0;
            bits.push(bit_is_set);
        }
    }
    
    Ok((bits))
}


impl WindowHandler for MyWindowHandler
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

       
      
        let mut counter = 0;
        for x in &self.buttons{
            if(self.bits[counter]){
                graphics.draw_rectangle(Rectangle::new(x[0].clone(),x[1].clone()), Color::BLACK);
            }else{
                graphics.draw_rectangle(Rectangle::new(x[0].clone(),x[1].clone()), Color::WHITE);
            }
         
            counter += 1;
        }
 
        if(self.mouse_button_down){
            for x in &self.buttons{
                if self.mouse_pos.x > x[0].x as f32 && self.mouse_pos.y > x[0].y as f32 && self.mouse_pos.x < x[1].x as f32 && self.mouse_pos.y < x[1].y as f32{
                    // println!("{:?}",self.mouse_pos);
                }
               
            }

           
        }
        helper.request_redraw();
    }



    fn on_start(
        &mut self,
        helper: &mut WindowHelper<()>,
        info: speedy2d::window::WindowStartupInfo
    )
    {
    }

    fn on_user_event(
        &mut self,
        helper: &mut WindowHelper<()>,
        user_event: ()
    )
    {
        // println!("Received user event: '{:?}'", user_event);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: speedy2d::dimen::UVec2)
    {
    }

    fn on_mouse_grab_status_changed(
        &mut self,
        helper: &mut WindowHelper<()>,
        mouse_grabbed: bool
    )
    {
    }

    fn on_fullscreen_status_changed(
        &mut self,
        helper: &mut WindowHelper<()>,
        fullscreen: bool
    )
    {
    }

    fn on_scale_factor_changed(
        &mut self,
        helper: &mut WindowHelper<()>,
        scale_factor: f64
    )
    {
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

    fn on_mouse_wheel_scroll(
        &mut self,
        helper: &mut WindowHelper<()>,
        distance: speedy2d::window::MouseScrollDistance
    )
    {
    }

    fn on_key_down(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode
    )
    {
    }

    fn on_key_up(
        &mut self,
        helper: &mut WindowHelper<()>,
        virtual_key_code: Option<speedy2d::window::VirtualKeyCode>,
        scancode: speedy2d::window::KeyScancode
    )
    {
    }

    fn on_keyboard_char(
        &mut self,
        helper: &mut WindowHelper<()>,
        unicode_codepoint: char
    )
    {
    }

    fn on_keyboard_modifiers_changed(
        &mut self,
        helper: &mut WindowHelper<()>,
        state: speedy2d::window::ModifiersState
    )
    {
    }

   // If desired, on_mouse_move(), on_key_down(), etc...
}


fn main(){
    let w = 640;
    let h = 640;



    let bits = read_and_convert().unwrap();
    let rows = ((bits.len() / 35) as f32).floor();

    let maxiteration = (rows * 35.0) as usize;

    //size
    let rectwidth: f32 = (w as f32) / 35.0;
    let rectheight: f32 = (h as f32)/rows;

    

    let mut s: Vec<[Vector2<f32>; 2]> = Vec::new();
    s.push([Vector2::new(0.0,0.0),Vector2::new(10.0,20.0)]);
    s.push([Vector2::new(500.0,50.0),Vector2::new(600.0,90.0)]);

    for y in 0..rows as i32{
        for x in 0..35 as i32{
            s.push([Vector2::new(rectwidth*(x as f32),rectheight*(y as f32)),Vector2::new(rectwidth*(x as f32) + rectwidth,rectheight*(y as f32) + rectheight)]);
        }
    }

    let window = speedy2d::Window::new_centered("Title", (w, h)).unwrap();
    window.run_loop(MyWindowHandler{buttons: s,bits : bits, mouse_pos: Vector2 { x: 0.0, y: 0.0 },
        mouse_button_down: false});
}
