#[cfg(test)]
mod tests;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use firecrawl::{
    crawl::{CrawlOptions, CrawlScrapeFormats, CrawlScrapeOptions},
    scrape::{ScrapeFormats, ScrapeOptions, ExtractOptions},
    FirecrawlApp,
};
use serde::Deserialize;
use std::env;

// Request structs
#[derive(Deserialize)]
struct ScrapeRequest {
    url: String,
    formats: Option<Vec<String>>,
    schema: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct CrawlRequest {
    url: String,
    limit: Option<i32>,
    formats: Option<Vec<String>>,
}

// Convert string format to ScrapeFormats
fn parse_formats(formats: &[String]) -> Vec<ScrapeFormats> {
    formats
        .iter()
        .filter_map(|f| match f.to_uppercase().as_str() {
            "MARKDOWN" => Some(ScrapeFormats::Markdown),
            "HTML" => Some(ScrapeFormats::HTML),
            "EXTRACT" => Some(ScrapeFormats::Extract),
            _ => None,
        })
        .collect()
}

// Convert string format to CrawlScrapeFormats
fn parse_crawl_formats(formats: &[String]) -> Vec<CrawlScrapeFormats> {
    formats
        .iter()
        .filter_map(|f| match f.to_uppercase().as_str() {
            "MARKDOWN" => Some(CrawlScrapeFormats::Markdown),
            "HTML" => Some(CrawlScrapeFormats::HTML),
            _ => None,
        })
        .collect()
}
async fn scrape(
    app: web::Data<FirecrawlApp>,
    req: web::Json<ScrapeRequest>,
) -> impl Responder {
    let formats = req
        .formats
        .as_ref()
        .map(|f| parse_formats(f))
        .unwrap_or_else(|| vec![ScrapeFormats::Markdown]);

    // Log the formats being used
    println!("Parsed formats: {:?}", formats); // Added log statement

    let mut all_formats = formats.clone();
    let mut extract_option = None;

    if let Some(schema) = req.schema.clone() {
        extract_option = Some(ExtractOptions {
            schema: schema.into(),
            ..Default::default()
        });
        all_formats.push(ScrapeFormats::Extract);
    }

    let options = ScrapeOptions {
        formats: all_formats.into(),
        extract: extract_option,
        ..Default::default()
    };

    println!("Scraping URL: {}", req.url);
    match app.scrape_url(&req.url, options).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn crawl(
    app: web::Data<FirecrawlApp>,
    req: web::Json<CrawlRequest>,
) -> impl Responder {
    let formats = req
        .formats
        .as_ref()
        .map(|f| parse_crawl_formats(f))
        .unwrap_or_else(|| vec![CrawlScrapeFormats::Markdown]);

    let crawl_options = CrawlOptions {
        scrape_options: CrawlScrapeOptions {
            formats: formats.into(),
            ..Default::default()
        }
        .into(),
        limit: req.limit.map(|l| l as u32).unwrap_or(100).into(),
        ..Default::default()
    };

    match app.crawl_url(&req.url, crawl_options).await {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables from .env file
    dotenv::dotenv().ok();
    env_logger::init();

    // Get API key from environment variable
    let api_key = env::var("FIRECRAWL_API_KEY").expect("FIRECRAWL_API_KEY must be set");
    
    // Initialize FirecrawlApp
    let app = FirecrawlApp::new(&api_key).expect("Failed to initialize FirecrawlApp");
    let app_data = web::Data::new(app);

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(web::resource("/scrape").route(web::post().to(scrape)))
            .service(web::resource("/crawl").route(web::post().to(crawl)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
