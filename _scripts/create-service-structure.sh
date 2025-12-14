#!/bin/bash

# Script para crear estructura Clean Architecture en un microservicio Rust
# Uso: ./create-service-structure.sh <service-name> [framework]
# framework: actix (default) | axum

SERVICE=$1
FRAMEWORK=${2:-actix}

if [ -z "$SERVICE" ]; then
    echo "Uso: $0 <service-name> [framework]"
    echo "Ejemplo: $0 auth-service actix"
    echo "Ejemplo: $0 courses-service axum"
    exit 1
fi

SERVICE_DIR="../be/$SERVICE"

# Crear estructura base Clean Architecture para Rust
mkdir -p "$SERVICE_DIR"/{docs,src,tests,migrations,config}

# Estructura src/ con Clean Architecture
mkdir -p "$SERVICE_DIR/src"/{domain,application,infrastructure,interfaces}

# Domain layer
mkdir -p "$SERVICE_DIR/src/domain"/{entities,value_objects,repositories,services,events}

# Application layer  
mkdir -p "$SERVICE_DIR/src/application"/{use_cases,ports,dto,events,errors}

# Infrastructure layer
mkdir -p "$SERVICE_DIR/src/infrastructure"/{database,cache,messaging,external_apis,repositories}

# Interfaces layer
mkdir -p "$SERVICE_DIR/src/interfaces"/{http,cli,events}
mkdir -p "$SERVICE_DIR/src/interfaces/http"/{routes,handlers,middlewares,dto,extractors}

# Tests
mkdir -p "$SERVICE_DIR/tests"/{unit,integration,e2e,fixtures}

# Config
mkdir -p "$SERVICE_DIR/config"

# Docs
mkdir -p "$SERVICE_DIR/docs"

# Crear archivos base de Rust
cat > "$SERVICE_DIR/Cargo.toml" << EOF
[package]
name = "$SERVICE"
version = "0.1.0"
edition = "2021"

[dependencies]
# Framework: $FRAMEWORK
$(if [ "$FRAMEWORK" = "axum" ]; then
echo 'axum = { version = "0.7", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }'
else
echo 'actix-web = "4"
actix-rt = "2"
actix-cors = "0.7"'
fi)

# Database
sqlx = { version = "0.7", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Utils
uuid = { version = "1", features = ["v7", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
thiserror = "1"
anyhow = "1"

# Observability
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Config
config = "0.14"
dotenvy = "0.15"

[dev-dependencies]
tokio-test = "0.4"
EOF

cat > "$SERVICE_DIR/src/main.rs" << 'EOF'
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting service...");

    // TODO: Initialize app
    
    Ok(())
}
EOF

echo "✅ Servicio Rust '$SERVICE' creado con framework '$FRAMEWORK'"
echo "📁 Ubicación: $SERVICE_DIR"
echo ""
echo "Próximos pasos:"
echo "  cd $SERVICE_DIR"
echo "  cargo build"
echo "  cargo run"

echo "Estructura Clean Architecture creada para $STACK/$SERVICE"
