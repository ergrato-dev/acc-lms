#!/bin/bash

# Script para crear estructura Clean Architecture en un microservicio
# Uso: ./create-service-structure.sh <stack> <service-name>

STACK=$1
SERVICE=$2

if [ -z "$STACK" ] || [ -z "$SERVICE" ]; then
    echo "Uso: $0 <stack> <service-name>"
    echo "Ejemplo: $0 fastapi auth-service"
    exit 1
fi

SERVICE_DIR="../be/$STACK/$SERVICE"

# Crear estructura base Clean Architecture
mkdir -p "$SERVICE_DIR"/{docs,src,tests,migrations,config}

# Estructura src/ con Clean Architecture
mkdir -p "$SERVICE_DIR/src"/{domain,application,infrastructure,interfaces}

# Domain layer
mkdir -p "$SERVICE_DIR/src/domain"/{entities,value_objects,repositories,services,events}

# Application layer  
mkdir -p "$SERVICE_DIR/src/application"/{use_cases,ports,dto,events,exceptions}

# Infrastructure layer
mkdir -p "$SERVICE_DIR/src/infrastructure"/{database,cache,messaging,external_apis,repositories}

# Interfaces layer
mkdir -p "$SERVICE_DIR/src/interfaces"/{http,cli,events}
mkdir -p "$SERVICE_DIR/src/interfaces/http"/{routes,controllers,middlewares,dto,validators}

# Tests
mkdir -p "$SERVICE_DIR/tests"/{unit,integration,e2e,fixtures}

# Config
mkdir -p "$SERVICE_DIR/config"

# Docs
mkdir -p "$SERVICE_DIR/docs"

echo "Estructura Clean Architecture creada para $STACK/$SERVICE"
