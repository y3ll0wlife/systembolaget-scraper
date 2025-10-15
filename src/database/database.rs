use super::models::{ DatabaseSplitPerType, DatabaseSystembolagetProduct };
use crate::systembolaget::SystembolagetProduct;
use sqlx::migrate::MigrateDatabase;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::{ SqliteConnectOptions, SqliteQueryResult };
use sqlx::{ ConnectOptions, Sqlite, SqlitePool };
use std::env;
use std::str::FromStr;
use tracing::{ info, warn };

pub struct Database {
    connection: PoolConnection<Sqlite>,
}

#[allow(dead_code)]
impl Database {
    pub async fn init() -> Self {
        let db_url = &env::var("DATABASE_URL").expect("failed to find 'DATABASE_URL' in env");

        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            info!("Creating database {}", db_url);
            match Sqlite::create_database(db_url).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            warn!("Database with url '{}' already exists", &db_url);
        }

        let options = SqliteConnectOptions::from_str(&db_url)
            .expect("failed to create options")
            .disable_statement_logging()
            .clone();

        let pool = SqlitePool::connect_with(options).await.expect("failed to connect to database");
        let conn = pool.acquire().await.expect("failed to create connection");

        Database { connection: conn }
    }

    pub async fn create_tables(&mut self) -> Result<SqliteQueryResult, sqlx::Error> {
        let query =
            r#"
        CREATE TABLE IF NOT EXISTS products (
            product_id TEXT NOT NULL PRIMARY KEY, 
            product_number TEXT NOT NULL,
            product_name_bold TEXT NOT NULL, 
            product_name_thin TEXT, 
            producer_name TEXT, 
            supplier_name TEXT, 
            is_kosher INTEGER NOT NULL, 
            bottle_text TEXT NOT NULL, 
            is_organic INTEGER NOT NULL, 
            is_sustainable_choice INTEGER NOT NULL, 
            is_climate_smart_packaging INTEGER NOT NULL, 
            is_ethical INTEGER NOT NULL, ethical_label TEXT, 
            product_launch_date TEXT NOT NULL, 
            is_completely_out_of_stock INTEGER NOT NULL, 
            is_temporary_out_of_stock INTEGER NOT NULL, 
            alcohol_percentage TEXT NOT NULL,
            volume TEXT NOT NULL, 
            price TEXT NOT NULL, 
            country TEXT NOT NULL, 
            origin_level1 TEXT, 
            origin_level2 TEXT, 
            category_level1 TEXT, 
            category_level2 TEXT, 
            category_level3 TEXT, 
            category_level4 TEXT, 
            assortment_text TEXT NOT NULL, 
            is_manufacturing_country INTEGER NOT NULL, 
            is_regional_restricted INTEGER NOT NULL, 
            packaging_level1 TEXT, 
            image TEXT, 
            is_discontinued INTEGER NOT NULL, 
            is_supplier_temporary_not_available INTEGER NOT NULL, 
            sugar_content INTEGER NOT NULL, 
            sugar_content_gram_per100ml TEXT NOT NULL, 
            apk TEXT NOT NULL,
            url TEXT NOT NULL
        );
       "#;
        sqlx::query(query).execute(&mut self.connection).await
    }

    pub async fn fetch_all_products(
        &mut self
    ) -> Result<Vec<DatabaseSystembolagetProduct>, sqlx::Error> {
        sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>("SELECT * FROM products")
            .fetch_all(&mut self.connection).await
    }

    pub async fn fetch_all_products_category(
        &mut self
    ) -> Result<DatabaseSplitPerType, sqlx::Error> {
        let fast_sortiment = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Fast sortiment'"
            )
            .fetch_all(&mut self.connection).await?;

        let tillfälligt_sortiment = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Tillfälligt sortiment'"
            )
            .fetch_all(&mut self.connection).await?;

        let lokalt_och_småskaligt = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Lokalt & Småskaligt'"
            )
            .fetch_all(&mut self.connection).await?;

        let säsong = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Säsong'"
            )
            .fetch_all(&mut self.connection).await?;

        let webblanseringar = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Webblanseringar'"
            )
            .fetch_all(&mut self.connection).await?;

        let ordervaror = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Ordervaror'"
            )
            .fetch_all(&mut self.connection).await?;

        let presentsortiment = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                "SELECT * FROM products WHERE assortment_text = 'Presentsortiment'"
            )
            .fetch_all(&mut self.connection).await?;

        Ok(DatabaseSplitPerType {
            fast_sortiment,
            lokalt_och_småskaligt,
            ordervaror,
            presentsortiment,
            säsong,
            tillfälligt_sortiment,
            webblanseringar,
        })
    }

    pub async fn insert_product(
        &mut self,
        product: &SystembolagetProduct
    ) -> Result<(), sqlx::Error> {
        let _tmp = sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                r#"
            INSERT OR REPLACE INTO products (
                product_id,
                product_number,
                product_name_bold,
                product_name_thin,
                producer_name,
                supplier_name,
                is_kosher,
                bottle_text,
                is_organic,
                is_sustainable_choice,
                is_climate_smart_packaging,
                is_ethical,
                product_launch_date,
                is_completely_out_of_stock,
                is_temporary_out_of_stock,
                alcohol_percentage,
                volume,
                price,
                country,
                origin_level1,
                origin_level2,
                category_level1,
                category_level2,
                category_level3,
                category_level4,
                assortment_text,
                is_manufacturing_country,
                is_regional_restricted,
                packaging_level1,
                image,
                is_discontinued,
                is_supplier_temporary_not_available,
                sugar_content,
                sugar_content_gram_per100ml,
                apk,
                url
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36)
            RETURNING *
                        "#
            )
            .bind(&product.product_id)
            .bind(&product.product_number)
            .bind(&product.product_name_bold)
            .bind(&product.product_name_thin)
            .bind(&product.producer_name)
            .bind(&product.supplier_name)
            .bind(&product.is_kosher)
            .bind(&product.bottle_text)
            .bind(&product.is_organic)
            .bind(&product.is_sustainable_choice)
            .bind(&product.is_climate_smart_packaging)
            .bind(&product.is_ethical)
            .bind(&product.product_launch_date)
            .bind(&product.is_completely_out_of_stock)
            .bind(&product.is_temporary_out_of_stock)
            .bind(&product.alcohol_percentage)
            .bind(&product.volume)
            .bind(&product.price)
            .bind(&product.country)
            .bind(&product.origin_level1)
            .bind(&product.origin_level2)
            .bind(&product.category_level1)
            .bind(&product.category_level2)
            .bind(&product.category_level3)
            .bind(&product.category_level4)
            .bind(&product.assortment_text)
            .bind(&product.is_manufacturing_country)
            .bind(&product.is_regional_restricted)
            .bind(&product.packaging_level1)
            .bind(&product.get_image())
            .bind(&product.is_discontinued)
            .bind(&product.is_supplier_temporary_not_available)
            .bind(&product.sugar_content)
            .bind(&product.sugar_content_gram_per100ml)
            .bind(&product.calculate_apk())
            .bind(&product.get_url())
            .fetch_one(&mut self.connection).await?;

        Ok(())
    }

    pub async fn get_top_products_by_apk(
        &mut self,
        limit: i64
    ) -> Result<Vec<DatabaseSystembolagetProduct>, sqlx::Error> {
        sqlx
            ::query_as::<_, DatabaseSystembolagetProduct>(
                r#"
        SELECT
            product_id,
            product_number,
            product_name_bold,
            product_name_thin,
            producer_name,
            supplier_name,
            is_kosher,
            bottle_text,
            is_organic,
            is_sustainable_choice,
            is_climate_smart_packaging,
            is_ethical,
            ethical_label,
            product_launch_date,
            is_completely_out_of_stock,
            is_temporary_out_of_stock,
            alcohol_percentage,
            volume,
            price,
            country,
            origin_level1,
            origin_level2,
            category_level1,
            category_level2,
            category_level3,
            category_level4,
            assortment_text,
            is_manufacturing_country,
            is_regional_restricted,
            packaging_level1,
            image,
            is_discontinued,
            is_supplier_temporary_not_available,
            sugar_content,
            sugar_content_gram_per100ml,
            apk,
            url
        FROM products
        ORDER BY CAST(apk AS REAL) DESC
        LIMIT ?
        "#
            )
            .bind(limit)
            .fetch_all(&mut self.connection).await
    }

    pub fn products_to_markdown_table(
        &self,
        products: Vec<DatabaseSystembolagetProduct>
    ) -> String {
        let mut markdown = String::from(
            "| APK (ml/kr) | Product Name | Price (kronor) | Alcohol Percentage | Volume (ml) | Category | Link |\n"
        );
        markdown.push_str(
            "| :---------- | :----------- | :------------- | :----------------- | :---------- | :------- | :--- |\n"
        );

        for product in products {
            let category = product.category_level2
                .as_deref()
                .or(product.category_level1.as_deref())
                .unwrap_or("N/A");

            markdown.push_str(
                &format!(
                    "| {} | {} | {} | {} | {} | {} | {} |\n",
                    product.apk,
                    product.product_name_bold,
                    product.price,
                    product.alcohol_percentage,
                    product.volume,
                    category,
                    product.url
                )
            );
        }

        markdown
    }
}
