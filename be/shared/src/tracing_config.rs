//! # Tracing and Structured Logging Configuration
//!
//! Initializes the logging system with environment-appropriate settings.
//!
//! ## What is Tracing?
//!
//! [Tracing](https://docs.rs/tracing/) is Rust's modern approach to logging.
//! Unlike traditional logging, tracing provides:
//!
//! - **Structured data**: Key-value pairs instead of just strings
//! - **Spans**: Track the execution of functions/operations
//! - **Context propagation**: Trace requests across async tasks
//!
//! ## Output Formats
//!
//! | Environment | Format | Purpose |
//! |-------------|--------|---------|
//! | Development | Pretty | Human-readable, colored output |
//! | Production | JSON | Machine-parseable for log aggregation |
//!
//! ### Development Output Example
//!
//! ```text
//! 2024-01-15T10:30:00.123456Z  INFO auth_service::handlers: User logged in
//!     at src/handlers/auth.rs:42
//!     user_id: "550e8400-e29b-41d4-a716-446655440000"
//!     email: "user@example.com"
//! ```
//!
//! ### Production Output Example (JSON)
//!
//! ```json
//! {
//!   "timestamp": "2024-01-15T10:30:00.123456Z",
//!   "level": "INFO",
//!   "target": "auth_service::handlers",
//!   "message": "User logged in",
//!   "user_id": "550e8400-e29b-41d4-a716-446655440000",
//!   "email": "user@example.com",
//!   "file": "src/handlers/auth.rs",
//!   "line": 42
//! }
//! ```
//!
//! ## Log Levels
//!
//! | Level | When to Use | Default Enabled |
//! |-------|-------------|-----------------|
//! | ERROR | Something failed that shouldn't | Always |
//! | WARN | Recoverable issues, degraded service | Always |
//! | INFO | Significant events (startup, requests) | Always |
//! | DEBUG | Detailed troubleshooting info | Dev only |
//! | TRACE | Very verbose, step-by-step | Never by default |
//!
//! ## Filter Configuration
//!
//! Control verbosity via `RUST_LOG` environment variable:
//!
//! ```bash
//! # All debug
//! RUST_LOG=debug
//!
//! # Info for most, debug for our code
//! RUST_LOG=info,auth_service=debug
//!
//! # Silence noisy crates
//! RUST_LOG=info,hyper=warn,sqlx=warn
//! ```
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use shared::tracing_config::init_tracing;
//!
//! fn main() {
//!     let config = AppConfig::from_env().expect("Config error");
//!     
//!     // Initialize once at startup
//!     init_tracing(&config.service_name, config.is_production());
//!     
//!     // Now use tracing macros anywhere
//!     tracing::info!(user_id = %user.id, "User logged in");
//! }
//! ```
//!
//! ## Best Practices
//!
//! 1. **Use structured fields**: `info!(user_id = %id, "Action")` not `info!("User {} did action", id)`
//! 2. **Use spans for operations**: Track request lifecycle with spans
//! 3. **Don't log sensitive data**: Passwords, tokens, PII
//! 4. **Use appropriate levels**: INFO for business events, DEBUG for troubleshooting
//!
//! ## Related Documentation
//!
//! - [tracing crate](https://docs.rs/tracing/)
//! - [tracing-subscriber crate](https://docs.rs/tracing-subscriber/)
//! - [`_docs/development/development-standards.md`] - Logging guidelines

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

// =============================================================================
// Initialization
// =============================================================================

/// Initializes the tracing/logging system.
///
/// This should be called **once** at the very start of your application,
/// before any other code that might emit logs.
///
/// ## Parameters
///
/// - `service_name`: Name of the service (for identification in logs)
/// - `is_production`: If true, outputs JSON; if false, outputs pretty format
///
/// ## Environment Variables
///
/// - `RUST_LOG`: Controls log filter (e.g., "info,myservice=debug")
///
/// ## Defaults
///
/// If `RUST_LOG` is not set:
/// - Production: `info`
/// - Development: `debug,hyper=info,sqlx=warn`
///
/// ## Example
///
/// ```rust,ignore
/// fn main() {
///     init_tracing("auth-service", false);  // Development
///     // or
///     init_tracing("auth-service", true);   // Production
///     
///     tracing::info!("Application starting");
/// }
/// ```
pub fn init_tracing(service_name: &str, is_production: bool) {
    // Build filter from RUST_LOG env var, or use defaults
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            if is_production {
                // Production: info level, minimal noise
                EnvFilter::new("info")
            } else {
                // Development: debug level, but silence noisy crates
                EnvFilter::new("debug,hyper=info,sqlx=warn")
            }
        });

    if is_production {
        // Production: JSON structured output
        // - Parseable by log aggregation tools (ELK, Datadog, etc.)
        // - Includes all context fields
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .json()                          // JSON format
                    .with_file(true)                 // Include file name
                    .with_line_number(true)          // Include line number
                    .with_thread_ids(true)           // Include thread ID
                    .with_target(true)               // Include module path
                    .with_span_events(FmtSpan::CLOSE) // Log when spans close
                    .flatten_event(true)             // Flatten fields into root
                    .with_current_span(true),        // Include parent span info
            )
            .init();
    } else {
        // Development: Pretty, human-readable output
        // - Colorized for terminal
        // - Indentation for readability
        tracing_subscriber::registry()
            .with(env_filter)
            .with(
                fmt::layer()
                    .pretty()                        // Multi-line, colorized
                    .with_file(true)                 // Include file name
                    .with_line_number(true)          // Include line number
                    .with_target(true)               // Include module path
                    .with_span_events(FmtSpan::CLOSE), // Log when spans close
            )
            .init();
    }

    // Log that tracing is initialized (useful for verifying setup)
    tracing::info!(
        service = service_name,
        production = is_production,
        "Tracing initialized"
    );
}

// =============================================================================
// Helper Macros
// =============================================================================

/// Creates a span for tracking an HTTP request.
///
/// This macro creates an `info_span` with common request fields.
/// Use it to wrap request handlers for tracing.
///
/// ## Example
///
/// ```rust,ignore
/// use shared::request_span;
///
/// async fn handle_request(req: Request) -> Response {
///     let span = request_span!(request_id, "GET", "/api/users");
///     async {
///         // Handle request
///     }
///     .instrument(span)
///     .await
/// }
/// ```
#[macro_export]
macro_rules! request_span {
    ($request_id:expr, $method:expr, $path:expr) => {
        tracing::info_span!(
            "request",
            request_id = %$request_id,
            method = %$method,
            path = %$path,
        )
    };
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Logs an error with context.
///
/// Use this for consistent error logging format across the application.
///
/// ## Example
///
/// ```rust,ignore
/// if let Err(e) = do_something() {
///     log_error(&e, "Failed to process payment");
/// }
/// ```
pub fn log_error<E: std::fmt::Display>(error: &E, context: &str) {
    tracing::error!(
        error = %error,
        context = context,
        "Error occurred"
    );
}

/// Logs a warning with context.
///
/// Use this for recoverable issues that should be monitored.
pub fn log_warning(message: &str, context: &str) {
    tracing::warn!(
        message = message,
        context = context,
        "Warning"
    );
}

