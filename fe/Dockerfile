# =============================================================================
# ACC LMS - Frontend Dockerfile (React 19 + Vite)
# =============================================================================
# Multi-stage build for optimized production images
# Stages: base → development → build → production
#
# Usage:
#   docker build -t acc-lms-frontend .
#   docker build --target development -t acc-lms-frontend-dev .
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Base - Node.js LTS with pnpm
# -----------------------------------------------------------------------------
FROM node:22-alpine AS base

# Enable corepack for pnpm
RUN corepack enable && corepack prepare pnpm@latest --activate

WORKDIR /app

# Copy package files for dependency installation
COPY package.json pnpm-lock.yaml* ./

# -----------------------------------------------------------------------------
# Stage 2: Development - Full development environment
# -----------------------------------------------------------------------------
FROM base AS development

# Install all dependencies (including devDependencies)
RUN pnpm install --frozen-lockfile

# Copy source code
COPY . .

# Expose Vite dev server port
EXPOSE 5173

# Environment variables
ENV NODE_ENV=development

# Development command
CMD ["pnpm", "dev", "--host", "0.0.0.0"]

# -----------------------------------------------------------------------------
# Stage 3: Build - Production build
# -----------------------------------------------------------------------------
FROM base AS build

# Install production dependencies only
RUN pnpm install --frozen-lockfile --prod=false

# Copy source code
COPY . .

# Build arguments for environment variables
ARG VITE_API_URL
ARG VITE_APP_ENV=production

# Set environment variables for build
ENV VITE_API_URL=${VITE_API_URL}
ENV VITE_APP_ENV=${VITE_APP_ENV}

# Build the application
RUN pnpm build

# -----------------------------------------------------------------------------
# Stage 4: Production - Nginx serving static files
# -----------------------------------------------------------------------------
FROM nginx:stable-alpine AS production

# Install curl for health checks
RUN apk add --no-cache curl

# Remove default nginx configuration
RUN rm -rf /etc/nginx/conf.d/*

# Copy custom nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built files from build stage
COPY --from=build /app/dist /usr/share/nginx/html

# Add non-root user
RUN adduser -D -g '' appuser && \
    chown -R appuser:appuser /usr/share/nginx/html && \
    chown -R appuser:appuser /var/cache/nginx && \
    chown -R appuser:appuser /var/log/nginx && \
    touch /var/run/nginx.pid && \
    chown -R appuser:appuser /var/run/nginx.pid

# Expose HTTP port
EXPOSE 80

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost/health || exit 1

# Run nginx
CMD ["nginx", "-g", "daemon off;"]
