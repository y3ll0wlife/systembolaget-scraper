pub fn generate_scraping_urls() -> Vec<String> {
    let mut urls: Vec<String> = Vec::new();

    const URL: &str = "https://api-extern.systembolaget.se/sb-api-ecommerce";
    const PATH: &str = "v1/productsearch/search";

    let url: String = format!(
        "{}/{}?size=30&sortBy=Score&sortDirection=Ascending",
        URL, PATH
    );

    let params = vec![
        "Fast%20sortiment",
        "Tillf%C3%A4lligt%20sortiment",
        "Lokalt%20%26%20Sm%C3%A5skaligt",
        "S%C3%A4song",
        "Webblanseringar",
        "Ordervaror",
        "Presentsortiment",
    ];

    for param in params {
        match param {
            "Ordervaror" => {
                urls.push(format!("{}&assortmentText={}&price.max=250", url, param));
                urls.push(format!(
                    "{}&assortmentText={}&price.max=500&price.min=251",
                    url, param
                ));
                urls.push(format!("{}&assortmentText={}&price.min=501", url, param));
            }
            _ => urls.push(format!("{}&assortmentText={}", url, param)),
        }
    }

    urls
}
