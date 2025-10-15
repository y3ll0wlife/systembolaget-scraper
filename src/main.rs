mod database;
mod systembolaget;
#[cfg(test)]
mod tests;
mod utils;

use database::database::Database;
use dotenv::dotenv;
use std::env;
use systembolaget::SystembolagetSearchResponse;
use tokio::{ main };
use tracing::{ error, info };
use tracing_subscriber;
use std::fs;
use crate::utils::generate_scraping_urls;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let client = reqwest::Client::new();
    let mut database = Database::init().await;
    database.create_tables().await?;

    let urls = generate_scraping_urls();

    info!("Scanning {} urls for products", urls.len());

    for url in urls {
        let mut page = 0;

        loop {
            let fetch_url = format!("{}&page={}", &url, page);

            let resp = client
                .get(&fetch_url)
                .header(
                    "Ocp-Apim-Subscription-Key",
                    &env
                        ::var("SYSTEMBOLAGET_API_KEY")
                        .expect("'SYSTEMBOLAGET_API_KEY' is not configured")
                )
                .send().await?;

            let response = resp.json::<SystembolagetSearchResponse>().await?;

            info!("Page {} have {} products", page, response.products.len());

            for product in response.products {
                match database.insert_product(&product).await {
                    Ok(_) => (),
                    Err(error) => {
                        error!("Failed to insert product {:#?} into database", &product);
                        error!("URL: {}", &fetch_url);
                        error!("Error {:#?}", &error);
                    }
                }
            }

            if response.metadata.next_page == -1 {
                info!("Scanned last page {}, moving to next url", page);
                break;
            }

            page += 1;
        }
    }

    let products = database.fetch_all_products().await?;
    let products_split = database.fetch_all_products_category().await?;

    info!("Database got {} different products", products.len());
    info!("Fast sortiment: {}", products_split.fast_sortiment.len());
    info!("Tillfälligt sortiment: {}", products_split.tillfälligt_sortiment.len());
    info!("Lokalt & Småskaligt: {}", products_split.lokalt_och_småskaligt.len());
    info!("Säsong: {}", products_split.säsong.len());
    info!("Webblanseringar: {}", products_split.webblanseringar.len());
    info!("Ordervaror: {}", products_split.ordervaror.len());
    info!("Presentsortiment: {}", products_split.presentsortiment.len());

    let products = database.get_top_products_by_apk(50).await?;
    let markdown_table = database.products_to_markdown_table(products);
    fs::write("products.md", &markdown_table)?;

    Ok(())
}
