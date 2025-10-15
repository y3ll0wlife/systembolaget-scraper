use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "imageUrl")]
    pub image_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystembolagetSearchResponse {
    pub products: Vec<SystembolagetProduct>,
    pub metadata: Metadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Metadata {
    #[serde(rename = "docCount")]
    pub doc_count: usize,

    #[serde(rename = "nextPage")]
    pub next_page: i16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystembolagetProduct {
    #[serde(rename = "productId")]
    pub product_id: String,

    #[serde(rename = "productNumber")]
    pub product_number: String,

    #[serde(rename = "productNameBold")]
    pub product_name_bold: String,

    #[serde(rename = "productNameThin")]
    pub product_name_thin: Option<String>,

    #[serde(rename = "producerName")]
    pub producer_name: Option<String>,

    #[serde(rename = "supplierName")]
    pub supplier_name: Option<String>,

    #[serde(rename = "isKosher")]
    pub is_kosher: bool,

    #[serde(rename = "bottleText")]
    pub bottle_text: String,

    #[serde(rename = "isOrganic")]
    pub is_organic: bool,

    #[serde(rename = "isSustainableChoice")]
    pub is_sustainable_choice: bool,

    #[serde(rename = "isClimateSmartPackaging")]
    pub is_climate_smart_packaging: bool,

    #[serde(rename = "isEthical")]
    pub is_ethical: bool,

    // if is_ethical is true this has a value
    #[serde(rename = "ethicalLabel")]
    pub ethical_label: Option<String>,

    #[serde(rename = "productLaunchDate")]
    pub product_launch_date: String,

    #[serde(rename = "isCompletelyOutOfStock")]
    pub is_completely_out_of_stock: bool,

    #[serde(rename = "isTemporaryOutOfStock")]
    pub is_temporary_out_of_stock: bool,

    #[serde(rename = "alcoholPercentage")]
    pub alcohol_percentage: f64,

    pub volume: f64,

    // price in Swedish kronor
    pub price: f64,

    pub country: String,

    #[serde(rename = "originLevel1")]
    pub origin_level1: Option<String>,

    #[serde(rename = "originLevel2")]
    pub origin_level2: Option<String>,

    #[serde(rename = "categoryLevel1")]
    pub category_level1: Option<String>,

    #[serde(rename = "categoryLevel2")]
    pub category_level2: Option<String>,

    #[serde(rename = "categoryLevel3")]
    pub category_level3: Option<String>,

    #[serde(rename = "categoryLevel4")]
    pub category_level4: Option<String>,

    #[serde(rename = "assortmentText")]
    pub assortment_text: String,

    #[serde(rename = "isManufacturingCountry")]
    pub is_manufacturing_country: bool,

    #[serde(rename = "isRegionalRestricted")]
    pub is_regional_restricted: bool,

    #[serde(rename = "packagingLevel1")]
    pub packaging_level1: Option<String>,

    pub images: Vec<Image>,

    #[serde(rename = "isDiscontinued")]
    pub is_discontinued: bool,

    #[serde(rename = "isSupplierTemporaryNotAvailable")]
    pub is_supplier_temporary_not_available: bool,

    #[serde(rename = "sugarContent")]
    pub sugar_content: i64,

    #[serde(rename = "sugarContentGramPer100ml")]
    pub sugar_content_gram_per100ml: f64,
}

impl SystembolagetProduct {
    // calculate the alkohol per krona
    // https://sv.wikipedia.org/wiki/Alkohol_per_krona
    pub fn calculate_apk(&self) -> f64 {
        let he = (self.alcohol_percentage / 100.0) * self.volume;

        format!("{:.2}", he / self.price)
            .parse()
            .expect("failed to convert apk value to f64")
    }

    pub fn get_image(&self) -> Option<String> {
        let first_image = self.images.first();
        match first_image {
            Some(img) => Some(format!("{}_400.png?q=75&w=2000", img.image_url)),
            None => None,
        }
    }

    pub fn get_url(&self) -> String {
        let category = &self
            .category_level1
            .clone()
            .expect("failed to get category")
            .replace(" &", "")
            .replace(' ', "-")
            .replace('Ö', "o")
            .to_ascii_lowercase();

        format!(
            "https://www.systembolaget.se/produkt/{}/{}/",
            &category, &self.product_number
        )
    }
}
