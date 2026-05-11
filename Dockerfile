# --- Build Stage for Backend ---
FROM rust:1.80-slim as backend-builder
WORKDIR /usr/src/app

# Copy RusTorch (dependency) and Nebula Backend
COPY RusTorch /usr/src/RusTorch
COPY nebula-canvas/backend /usr/src/app/nebula-backend

WORKDIR /usr/src/app/nebula-backend
RUN cargo build --release

# --- Build Stage for Frontend ---
FROM node:18-alpine as frontend-builder
WORKDIR /usr/src/app

COPY nebula-canvas/frontend /usr/src/app/nebula-frontend
WORKDIR /usr/src/app/nebula-frontend
RUN npm install
RUN npm run build

# --- Runtime Stage ---
FROM debian:bookworm-slim
WORKDIR /app

# Install necessary runtime libs (e.g., OpenSSL)
RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /usr/src/app/nebula-backend/target/release/backend /app/nebula-backend
COPY --from=backend-builder /usr/src/app/nebula-backend/Settings.toml /app/Settings.toml

# Copy frontend build
# Note: In a production Docker setup, usually the frontend is served via a CDN or Nginx,
# but for simplicity, we'll keep the frontend build available or use Next.js standalone.
COPY --from=frontend-builder /usr/src/app/nebula-frontend/.next /app/frontend/.next
COPY --from=frontend-builder /usr/src/app/nebula-frontend/public /app/frontend/public
COPY --from=frontend-builder /usr/src/app/nebula-frontend/package.json /app/frontend/package.json
COPY --from=frontend-builder /usr/src/app/nebula-frontend/node_modules /app/frontend/node_modules

# Startup script
COPY nebula-canvas/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 3000 3001
ENTRYPOINT ["/app/entrypoint.sh"]
