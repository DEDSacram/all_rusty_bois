use std::{path::{PathBuf, self}};




// use libraries_examples::prelude::*;

// #[pollster::main]
// async fn main() {


    // let future = async {
    //     let file = AsyncFileDialog::new()
    //         .add_filter("text", &["txt", "rs"])
    //         .add_filter("rust", &["rs", "toml"])
    //         .set_directory("/")
    //         .pick_file()
    //         .await;

    //     let data = file.unwrap().read().await;
    // };
    // my_fut.await;
// }

// #[pollster::main]
// async fn main() {
//     let my_fut = async {};

//     my_fut.await;
// }


// async fn async_pickread_file() -> Result<rfd::FileHandle, String> {
//     let future = async {
//         let file = AsyncFileDialog::new()
//             .add_filter("text", &["txt", "rs"])
//             .add_filter("rust", &["rs", "toml"])
//             .set_directory("/")
//             .pick_file()
//             .await;
    
//         // let data = file.unwrap().read().await;
//         match(file){
//             Some(T)=> return Ok((T)),
//             None => return Err(("Invalid path".to_owned()))
//         }
//         // return data
//     };
//     future.await
// }

//async choose
// async fn async_pickread_file() -> Result<rfd::FileHandle,()> {
//     let future = async {
//         let file = AsyncFileDialog::new()
//             .add_filter("text", &["txt", "rs"])
//             .add_filter("rust", &["rs", "toml"])
//             .set_directory("/")
//             .pick_file()
//             .await;
    
//         // let data = file.unwrap().read().await;
//         match(file){
//             Some(T)=> return Ok((T)),
//             None => return Err(())
//         }
//         // return data
//     };
//     future.await
// }

use rfd::FileDialog;
fn pickread_file() -> Result<PathBuf,()>{

let files = FileDialog::new()
    .add_filter("text", &["txt", "rs"])
    .add_filter("rust", &["rs", "toml"])
    .set_directory("/")
    .pick_file();

    match files {
        Some(p) => Ok(p),
        None => Err(())
    }

 
}

fn checkok(ch : Result<PathBuf,()>) -> PathBuf{

if ch.is_err() {
    println!("Pick failed, try again");
    return checkok(pickread_file());
}

ch.unwrap()
}

fn main() {
    //async pick
    // let my_fut = async_pickread_file();
    // let file = pollster::block_on(my_fut).ok();
    // println!("{:?}",file);

  
   
    //normal pick file

    let path = checkok(pickread_file()) ;

 


    println!("Heeloo");
    

}