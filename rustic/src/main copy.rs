use reqwest;
use serde;
use serde::Serialize;
use serde::Deserialize;

// pub type Response = Vec<Product>;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Product{
    id: u32,
    title : String,
    description : String,
    price: u32,
    discountPercentage : f64,
    rating : f32,
    stock : i32,
    brand : String,
    category : String,
    thumbnail: String,
    images: Vec<String>
}



// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
    // chaining .await will yield our query result
    // let res = reqwest::get("https://dummyjson.com/products/1").await.unwrap();
    let res = reqwest::get("https://dummyjson.com/products/1").await.unwrap();
    // let json = res.text().await;
    let json = res.json::<Product>().await.unwrap();
    println!("{:?}",json);
}