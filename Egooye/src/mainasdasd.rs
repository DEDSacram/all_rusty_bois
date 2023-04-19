

// use async_std::channel::{Sender, Receiver, bounded};
// use async_std::task::{spawn, JoinHandle};

// async fn sendreq(tx: Sender<u32>) -> Result<(), Box<dyn std::error::Error + Send>> {
//     tx.send(25).await;
//     Ok(())
// }

// async fn run() -> Result<(), Box<dyn std::error::Error + Send>> {
//     let (tx, mut rx): (Sender<u32>, Receiver<u32>) = bounded(1);
//     let handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send>>> = spawn(async move {
//         sendreq(tx).await
//     });
//     let received = rx.recv().await.unwrap();
//     println!("Received message: {}", received);
//     let _ = handle.await?;
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error + Send>> {
//     async_std::task::block_on(run())?;
//     while true {
        
//         println!("XD");
//         std::thread::sleep(std::time::Duration::from_secs(10));
//     }

//     Ok(())
// }

// another as passing 

// use async_std::channel::{Sender, Receiver, bounded};
// use async_std::task::{spawn, JoinHandle};

// async fn sendreq(tx: Sender<u32>) -> Result<(), Box<dyn std::error::Error + Send>> {
//     tx.send(25).await;
//     Ok(())
// }

// async fn run(tx: Sender<u32>) -> Result<(), Box<dyn std::error::Error + Send>> {

//     let handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send>>> = spawn(async move {
//         sendreq(tx).await
//     });
//     // let received = rx.recv().await.unwrap();
//     // println!("Received message: {}", received);
//     let _ = handle.await?;
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error + Send>> {
//     let (tx, mut rx): (Sender<u32>, Receiver<u32>) = bounded(1);
//     async_std::task::block_on(run(tx.clone()))?;
//     while true {
//         println!("{:?}",rx.try_recv());
        
//         println!("XD");
//         std::thread::sleep(std::time::Duration::from_secs(10));
//         async_std::task::block_on(run(tx.clone()))?;
//     }

//     Ok(())
// }


// Vectors

// use async_std::channel::{Sender, Receiver, bounded};
// use async_std::task::{spawn, JoinHandle};
// use std::sync::Arc;



// async fn sendreq(tx: Arc<Sender<Product>>) -> Result<(), Box<dyn std::error::Error + Send>> {
//     tx.send(vec![25, 30, 35]).await;
//     Ok(())
// }

// async fn run(tx: Arc<Sender<Product>>) -> Result<(), Box<dyn std::error::Error + Send>> {
//     let handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send>>> = spawn(async move {
//         sendreq(tx.clone()).await
//     });
//     let _ = handle.await?;
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error + Send>> {
//     let (tx, mut rx): (Sender<Product>, Receiver<Product>) = bounded(1);
//     let tx_arc = Arc::new(tx);
//     async_std::task::block_on(run(tx_arc.clone()))?;
//     while true {
//         println!("{:?}", rx.try_recv());
//         std::thread::sleep(std::time::Duration::from_secs(10));
//         async_std::task::block_on(run(tx_arc.clone()))?;
//     }
//     Ok(())
// }


// //good
// use async_std::channel::{Sender, Receiver, bounded};
// use async_std::task::{spawn, JoinHandle};
// use std::sync::Arc;

// //api
// use serde;
// use serde::Serialize;
// use serde::Deserialize;
// use surf;

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// struct Product {
//     id: u32,
//     title : String,
//     description : String,
//     price: u32,
//     discountPercentage : f64,
//     rating : f32,
//     stock : i32,
//     brand : String,
//     category : String,
//     thumbnail: String,
//     images: Vec<String>
// }

// async fn sendreq(tx: Arc<Sender<Product>>) -> Result<(), Box<dyn std::error::Error + Send>> {
//     let mut res = surf::get("https://dummyjson.com/products/1").await.unwrap();
//     let json = res.body_json::<Product>().await.unwrap();
    
//     tx.send(json).await;
//     Ok(())
// }

// async fn run(tx: Arc<Sender<Product>>) -> Result<(), Box<dyn std::error::Error + Send>> {
//     let handle: JoinHandle<Result<(), Box<dyn std::error::Error + Send>>> = spawn(async move {
//         sendreq(tx.clone()).await
//     });
//     let _ = handle.await?;
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error + Send>> {
//     let (tx, mut rx): (Sender<Product>, Receiver<Product>) = bounded(1);
//     let tx_arc = Arc::new(tx);
//     async_std::task::block_on(run(tx_arc.clone()))?;
//     while true {
//         println!("{:?}", rx.try_recv());
//         std::thread::sleep(std::time::Duration::from_secs(10));
//         async_std::task::block_on(run(tx_arc.clone()))?;
//     }
//     Ok(())
// }


// fn Rotate2D(array: &Vec<Vec<i32>>, n: usize) ->  Vec<Vec<i32>> {

//     let mut ret = vec![vec![0;n];5];
    
//     for y in 0..ret.len(){
//         for x in 0..ret.len(){
//             ret[x][y] = ret[n - y - 1][x];
//         }
//     }
//     ret
// }


// fn main(){

//     let array = vec![
//         vec![11, 12, 13, 14],
//         vec![21, 22, 23, 24],
//         vec![31, 32, 33, 34]];

//     for y in 0..array.len(){
//         for x in 0..array[y].len(){

//         }
//     }

//     Rotate2D(&array, array.len());

// }
