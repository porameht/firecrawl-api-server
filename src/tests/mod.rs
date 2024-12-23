use actix_web::{test, web, App};
use firecrawl::FirecrawlApp;
use serde_json::json;
use std::env;

use crate::{crawl, scrape};

fn setup_test_app() -> impl Fn(&mut web::ServiceConfig) + Clone {
    // Load test environment - do this once at the start
    if dotenv::from_filename(".env").is_err() {
        panic!("Failed to load .env file. Make sure it exists in the project root.");
    }
    
    let api_key = match env::var("FIRECRAWL_API_KEY") {
        Ok(key) => key,
        Err(_) => panic!("FIRECRAWL_API_KEY not found in .env file"),
    };
    
    let app = FirecrawlApp::new(&api_key).expect("Failed to initialize FirecrawlApp");
    let app_data = web::Data::new(app);

    move |cfg: &mut web::ServiceConfig| {
        cfg.app_data(app_data.clone())
            .service(web::resource("/scrape").route(web::post().to(scrape)))
            .service(web::resource("/crawl").route(web::post().to(crawl)));
    }
}

#[actix_web::test]
async fn test_scrape_endpoint() {
    let app = test::init_service(
        App::new().configure(setup_test_app())
    ).await;

    // Example request body
    println!("Example request for /scrape:");
    println!("{}", serde_json::to_string_pretty(&json!({
        "url": "https://example.com",
        "formats": ["markdown", "html"]
    })).unwrap());

    let req = test::TestRequest::post()
        .uri("/scrape")
        .set_json(json!({
            "url": "https://example.com",
            "formats": ["markdown", "html"]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    
    if !status.is_success() {
        let error_body = test::read_body(resp).await;
        println!("Error response: {}", String::from_utf8_lossy(&error_body));
        panic!("Request failed with status: {}", status);
    }

    let body: serde_json::Value = test::read_body_json(resp).await;
    
    // Example response body
    println!("\nExample response from /scrape:");
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
    
    assert!(body.get("markdown").is_some() || body.get("html").is_some());
}

#[actix_web::test]
async fn test_crawl_endpoint() {
    let app = test::init_service(
        App::new().configure(setup_test_app())
    ).await;

    // Example request body
    println!("Example request for /crawl:");
    println!("{}", serde_json::to_string_pretty(&json!({
        "url": "https://example.com",
        "limit": 1,
        "formats": ["markdown"]
    })).unwrap());

    let req = test::TestRequest::post()
        .uri("/crawl")
        .set_json(json!({
            "url": "https://example.com",
            "limit": 1,
            "formats": ["markdown"]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    
    if !status.is_success() {
        let error_body = test::read_body(resp).await;
        println!("Error response: {}", String::from_utf8_lossy(&error_body));
        panic!("Request failed with status: {}", status);
    }

    let body: serde_json::Value = test::read_body_json(resp).await;
    
    // Example response body
    println!("\nExample response from /crawl:");
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
    
    assert!(body.get("creditsUsed").is_some());
    assert!(body.get("data").is_some());
}

#[actix_web::test]
async fn test_scrape_with_extract() {
    let app = test::init_service(
        App::new().configure(setup_test_app())
    ).await;

    let schema = json!({
        "type": "object",
        "properties": {
            "title": {"type": "string"},
            "content": {"type": "string"}
        },
        "required": ["title", "content"]
    });

    // Example request body
    println!("Example request for /scrape with extract:");
    println!("{}", serde_json::to_string_pretty(&json!({
        "url": "https://example.com",
        "formats": ["markdown", "extract"],
        "schema": schema
    })).unwrap());

    let req = test::TestRequest::post()
        .uri("/scrape")
        .set_json(json!({
            "url": "https://example.com",
            "formats": ["markdown", "extract"],
            "schema": schema
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    
    if !status.is_success() {
        let error_body = test::read_body(resp).await;
        println!("Error response: {}", String::from_utf8_lossy(&error_body));
        panic!("Request failed with status: {}", status);
    }

    let body: serde_json::Value = test::read_body_json(resp).await;
    
    // Example response body
    println!("\nExample response from /scrape with extract:");
    println!("{}", serde_json::to_string_pretty(&body).unwrap());
    
    assert!(body.get("extract").is_some());
}

#[actix_web::test]
async fn test_invalid_url() {
    let app = test::init_service(
        App::new().configure(setup_test_app())
    ).await;

    // Example request body with invalid URL
    println!("Example request with invalid URL:");
    println!("{}", serde_json::to_string_pretty(&json!({
        "url": "not-a-valid-url",
        "formats": ["markdown"]
    })).unwrap());

    let req = test::TestRequest::post()
        .uri("/scrape")
        .set_json(json!({
            "url": "not-a-valid-url",
            "formats": ["markdown"]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    
    // Example error response
    println!("\nExample error response:");
    let error_body = test::read_body(resp).await;
    println!("{}", String::from_utf8_lossy(&error_body));
    
    assert!(status.is_server_error());
}

#[actix_web::test]
async fn test_invalid_format() {
    let app = test::init_service(
        App::new().configure(setup_test_app())
    ).await;

    let req = test::TestRequest::post()
        .uri("/scrape")
        .set_json(json!({
            "url": "https://example.com",
            "formats": ["invalid_format", "markdown"]
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    let status = resp.status();
    
    if !status.is_success() {
        let error_body = test::read_body(resp).await;
        println!("Error response: {}", String::from_utf8_lossy(&error_body));
        panic!("Request failed with status: {}", status);
    }

    let body: serde_json::Value = test::read_body_json(resp).await;
    println!("Response body: {}", serde_json::to_string_pretty(&body).unwrap());
    
    // Invalid format should be filtered out, but markdown should be present
    assert!(body.get("markdown").is_some(), "Response should contain markdown when invalid format is provided");
}
 