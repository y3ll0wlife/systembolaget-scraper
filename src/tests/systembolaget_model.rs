use crate::systembolaget::{Image, Metadata, SystembolagetProduct, SystembolagetSearchResponse};

fn generate_systembolaget_product_struct() -> SystembolagetProduct {
    SystembolagetProduct {
        product_id: String::from("24700361"),
        product_number: String::from("5617615"),
        product_name_bold: String::from("Kopparberg Cocktail Collection"),
        product_name_thin: Some(String::from("Strawberry Cosmo")),
        producer_name: None,
        supplier_name: Some(String::from("56176")),
        is_kosher: false,
        bottle_text: String::from("Burk"),
        is_organic: false,
        is_sustainable_choice: false,
        is_climate_smart_packaging: false,
        is_ethical: false,
        ethical_label: None,
        product_launch_date: String::from("2022-04-20T00:00:00"),
        is_completely_out_of_stock: false,
        is_temporary_out_of_stock: false,
        alcohol_percentage: 5.0,
        volume: 330.0,
        price: 15.9,
        country: String::from("Sverige"),
        origin_level1: None,
        origin_level2: None,
        category_level1: Some(String::from("Öl")),
        category_level2: Some(String::from("Ljus lager")),
        category_level3: Some(String::from("Pilsner - tysk stil")),
        category_level4: None,
        assortment_text: String::from("Ordervaror"),
        is_manufacturing_country: true,
        is_regional_restricted: false,
        packaging_level1: Some(String::from("Burk")),
        images: vec![Image {
            image_url: String::from(
                "https://product-cdn.systembolaget.se/productimages/32002009/32002009",
            ),
        }],
        is_discontinued: false,
        is_supplier_temporary_not_available: false,
        sugar_content: 0,
        sugar_content_gram_per100ml: 0.0,
    }
}

fn generate_systembolaget_struct() -> SystembolagetSearchResponse {
    SystembolagetSearchResponse {
        products: vec![generate_systembolaget_product_struct()],
        metadata: Metadata {
            doc_count: 0,
            next_page: -1,
        },
    }
}

#[test]
fn systembolaget_struct() {
    generate_systembolaget_struct();
}

#[test]
fn systembolaget_struct_apk_calcution() {
    let mut product = generate_systembolaget_product_struct();

    // example from wikipedia, swedish bar
    product.alcohol_percentage = 5.0;
    product.volume = 400.0;
    product.price = 50.0;
    assert_eq!(product.calculate_apk(), 0.4);

    product.alcohol_percentage = 5.0;
    product.volume = 500.0;
    product.price = 60.0;
    assert_eq!(product.calculate_apk(), 0.42);

    product.alcohol_percentage = 12.0;
    product.volume = 120.0;
    product.price = 60.0;
    assert_eq!(product.calculate_apk(), 0.24);

    product.alcohol_percentage = 40.0;
    product.volume = 40.0;
    product.price = 80.0;
    assert_eq!(product.calculate_apk(), 0.2);

    // example from wikipedia from systembolaget
    product.alcohol_percentage = 2.8;
    product.volume = 3000.0;
    product.price = 21.50;
    assert_eq!(product.calculate_apk(), 3.91);

    product.alcohol_percentage = 5.2;
    product.volume = 500.0;
    product.price = 10.9;
    assert_eq!(product.calculate_apk(), 2.39);

    product.alcohol_percentage = 7.5;
    product.volume = 500.0;
    product.price = 14.9;
    assert_eq!(product.calculate_apk(), 2.52);

    product.alcohol_percentage = 13.0;
    product.volume = 3000.0;
    product.price = 165.0;
    assert_eq!(product.calculate_apk(), 2.36);

    product.alcohol_percentage = 40.0;
    product.volume = 700.0;
    product.price = 199.0;
    assert_eq!(product.calculate_apk(), 1.41);
}

#[test]
fn systembolaget_struct_image() {
    let product = generate_systembolaget_product_struct();

    assert_eq!(product.get_image(), Some(String::from("https://product-cdn.systembolaget.se/productimages/32002009/32002009_400.png?q=75&w=2000")));
}
