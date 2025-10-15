use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Default, Serialize, Deserialize, Debug)]
pub struct DatabaseSystembolagetProduct {
    pub product_id: String,

    pub product_number: String,

    pub product_name_bold: String,

    pub product_name_thin: Option<String>,

    pub producer_name: Option<String>,

    pub supplier_name: String,

    pub is_kosher: bool,

    pub bottle_text: String,

    pub is_organic: bool,

    pub is_sustainable_choice: bool,

    pub is_climate_smart_packaging: bool,

    pub is_ethical: bool,

    pub ethical_label: Option<String>,

    pub product_launch_date: String,

    pub is_completely_out_of_stock: bool,

    pub is_temporary_out_of_stock: bool,

    pub alcohol_percentage: String,

    pub volume: String,

    pub price: String,

    pub country: String,

    pub origin_level1: Option<String>,

    pub origin_level2: Option<String>,

    pub category_level1: Option<String>,

    pub category_level2: Option<String>,

    pub category_level3: Option<String>,

    pub category_level4: Option<String>,

    pub assortment_text: String,

    pub is_manufacturing_country: bool,

    pub is_regional_restricted: bool,

    pub packaging_level1: String,

    pub image: String,

    pub is_discontinued: bool,

    pub is_supplier_temporary_not_available: bool,

    pub sugar_content: i64,

    pub sugar_content_gram_per100ml: String,

    pub apk: String,

    pub url: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]

pub struct DatabaseSplitPerType {
    pub fast_sortiment: Vec<DatabaseSystembolagetProduct>,
    pub tillfälligt_sortiment: Vec<DatabaseSystembolagetProduct>,
    pub lokalt_och_småskaligt: Vec<DatabaseSystembolagetProduct>,
    pub säsong: Vec<DatabaseSystembolagetProduct>,
    pub webblanseringar: Vec<DatabaseSystembolagetProduct>,
    pub ordervaror: Vec<DatabaseSystembolagetProduct>,
    pub presentsortiment: Vec<DatabaseSystembolagetProduct>,
}
