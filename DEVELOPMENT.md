# Nebula Canvas: Development Guide

This guide details the setup and contribution workflow for Nebula Canvas.

## 1. Project Architecture

Nebula is a monorepo consisting of:
- `frontend/`: Next.js / React / Tailwind UI.
- `backend/`: Rust / Axum / RusTorch engine.

## 2. Local Setup

### Backend
1. Ensure Rust (stable) is installed.
2. `cd backend`
3. `cargo run` (The database `nebula_canvas.redb` will be created automatically).

### Frontend
1. `cd frontend`
2. `npm install`
3. `npm run dev`

## 3. Key Concepts

### Goal-State Dispatcher
The backend implements a `watch::channel` based dispatcher. When a user types, the WebSocket receiver updates the "Goal State" immediately. A background worker task monitors this state and triggers the `RusTorchEngine` only when it's ready for a new task, ensuring the most recent prompt is always prioritized without blocking the network.

### Pyramid UI Layers
- **L0**: High-level components in `src/app/page.tsx`.
- **L1**: Shared state management via React hooks.
- **L2**: Low-level WebSocket abstraction in `src/hooks/useWebSocket.ts`.

## 4. Coding Standards
- **Rust**: Follow `clippy` and use `anyhow` for error handling. Use `spawn_blocking` for compute.
- **Frontend**: Functional components with Tailwind CSS. Avoid heavy external state libraries unless necessary (currently using local state/props).
