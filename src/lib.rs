use serde::{Deserialize, Serialize};
use warp::reply::{html, Html};
use sailfish::TemplateOnce;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Render {
    fn render(self) -> Html<String>;
}

impl<T: TemplateOnce> Render for T {
    fn render(self) -> Html<String> {
        html(self.render_once().unwrap())
    }
}

pub mod template {
    use super::*;
    
    #[derive(TemplateOnce)]
    #[template(path = "index.html")]
    pub struct Index {
        pub products: Vec<Product>
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rating {
    pub rate: f32,
    pub count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub category: String,
    pub image: String,
    pub rating: Rating,
}

pub async fn fetch_products() -> Result<Vec<Product>> {
    Ok(reqwest::get("https://fakestoreapi.com/products")
       .await?
       .json::<Vec<Product>>()
       .await?)
}
