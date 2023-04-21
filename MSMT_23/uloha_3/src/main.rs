
use std::f32;
use std::fs::File;
use std::path;
use std::path::Path;

use std::fs;


use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Pixels;
use image::Pixel;
use image::Rgba;

fn init(input_path: &str) {



}

fn main() {
    let paths = fs::read_dir("./pictures").unwrap();

    let mut v : Vec<Vec<DynamicImage>>= Vec::new();
    //one
    for dir in paths{
        let pict = fs::read_dir(dir.unwrap().path()).unwrap();

        // each piece
        let mut p : Vec<DynamicImage> = Vec::new();
        for pic in pict{
            let path_to_piece = pic.unwrap().path();
            let mut img = image::open(path_to_piece.as_path()).unwrap();
            p.push(img);
        }
        v.push(p);
    }

    // process each image
 
    for img in v[0].clone(){
        let img_width = img.dimensions().0;
        let img_height = img.dimensions().1;

        let mut pix : Vec<Vec<Rgba<u8>>> = Vec::new();
        let mut l : Vec<Rgba<u8>> = Vec::new();
        let mut count: u32 = 1;
        for p in img.pixels() {
            if count == p.0{
                count = 1;
                l = Vec::new();
            }
            l.push(p.2);
             println!("{:?}",p.2.channels());
            count+=1;
            }
    }


 
   
}