//! # Wishlist Service
//!
//! Microservice for managing user wishlists.
//!
//! ## Features
//! - Add courses to wishlist
//! - Remove courses from wishlist
//! - View wishlist with course details
//! - Check if course is in wishlist

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use tracing_actix_web::TracingLogger;

mod api;
mod domain;
mod repository;
mod services;

use api::configure_routes;
use repository::WishlistRepository;
use services::WishlistService;

/// Service configuration.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub host: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/acc_lms".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8098".to_string())
                .parse()
                .unwrap_or(8098),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("wishlist_service=debug".parse().unwrap()),
        )
        .init();

    tracing::info!("Starting Wishlist Service");

    let config = Config::from_env();

    // Database pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    tracing::info!("Connected to database");

    // Redis client
    let redis = redis::Client::open(config.redis_url.clone())
        .expect("Failed to create Redis client");

    // Initialize repository and service
    let repository = WishlistRepository::new(pool);
    let service = WishlistService::new(repository, redis);

    let bind_address = format!("{}:{}", config.host, config.port);
    tracing::info!("Binding to {}", bind_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(TracingLogger::default())
            .wrap(middleware::Compress::default())
            .wrap(cors)
            .app_data(web::Data::new(service.clone()))
            .configure(configure_routes)
    })
    .bind(&bind_address)?
    .run()
    .await
}
