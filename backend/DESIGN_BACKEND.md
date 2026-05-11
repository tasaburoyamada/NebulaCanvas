# Nebula Canvas: Production Backend Engine Design (RusTorch Edition)

## 1. Objective
Transform the mock Rust backend into a robust engine powered by **RusTorch**. The architecture will be optimized for the specific performance characteristics of RusTorch and the workflow of IME (Japanese input) users, where "intentional updates" are more frequent than "character-by-character" streaming.

## 2. Architecture Overview
A modular design that treats RusTorch as the first-class compute provider.

### 2.1. Core Components
- **API Layer (Axum):** Handles WebSocket upgrades and structured JSON communication.
- **Task Dispatcher (Simplified):** A "Latest-Prompt-Wins" executor. Instead of aggressive cancellation during IME composition, it ensures that only the most recent *fully dispatched* prompt is being processed, preventing a backlog of intermediate states.
- **RusTorch Bridge:** A dedicated module that wraps RusTorch's tensor operations and model inference logic.
- **Local Persistence (`redb`):** Shared with other projects in the ecosystem to track generated metadata and prompt history.

## 3. IME-Aware Considerations
- **Debounce & Commit:** The frontend will be tuned to distinguish between IME composition and "conversion/commit" events. The backend Task Dispatcher will treat each incoming message as a discrete "desired state" rather than a stream of bytes to be cancelled.
- **Throughput over Latency:** Since conversion takes time, we prioritize finishing the *current* meaningful generation with high quality rather than interrupting it for partial romaji strings.

## 4. Implementation Steps

### Phase 1: Modularization
- Split code into `api`, `engine (rustorch_impl)`, and `state`.
- Define an interface that accepts `PromptRequest` and returns `ImageResponse`.

### Phase 2: RusTorch Integration
- Link the `backend` crate with the `rustorch` crate in the workspace.
- Implement the model loading and inference loop using RusTorch primitives.

### Phase 3: Metadata & Persistence
- Use `redb` to store generation history, allowing the "Timeline Parallel" feature to persist across sessions.

## 5. Proposed File Structure (Backend)
```text
backend/
├── src/
│   ├── main.rs          # Entry point and server setup
│   ├── api/             # WebSocket handlers and JSON routes
│   │   ├── mod.rs
│   │   └── websocket.rs
│   ├── engine/          # Inference logic and ML framework bridge
│   │   ├── mod.rs
│   │   ├── trait.rs     # Interface definition
│   │   └── stable_diff.rs # Implementation
│   ├── state/           # Session and history management
│   │   └── mod.rs
│   └── util/            # Image encoding and helper functions
│       └── mod.rs
└── Cargo.toml
```
