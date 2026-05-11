# Nebula Canvas: API Specification

Nebula Canvas utilizes a structured WebSocket protocol for real-time communication between the Frontend and Backend.

## 1. Connection Details
- **Default URL**: `ws://127.0.0.1:3001/ws`
- **Format**: JSON (UTF-8)

## 2. Client Messages (Frontend -> Backend)

Messages must include a `type` and optional `data` field.

### `Generate`
Triggers a new image generation.
```json
{
  "type": "Generate",
  "data": {
    "prompt": "A magical forest",
    "seed": 42,
    "steps": 20
  }
}
```

### `GetHistory`
Requests a full dump of the generation history.
```json
{
  "type": "GetHistory"
}
```

## 3. Server Messages (Backend -> Frontend)

### `ImageUpdate`
Sent when a generation completes.
```json
{
  "type": "ImageUpdate",
  "data": {
    "id": "blake3_hash",
    "data_url": "data:image/png;base64,..."
  }
}
```

### `HistoryDump`
Returns an array of all past generations.
```json
{
  "type": "HistoryDump",
  "data": [
    {
      "id": "...",
      "prompt": "...",
      "seed": 42,
      "steps": 20,
      "image": "data:..."
    }
  ]
}
```

### `Status`
Real-time status updates from the engine.
```json
{
  "type": "Status",
  "data": "Generating..."
}
```

### `Error`
Sent when an operation fails.
```json
{
  "type": "Error",
  "data": "ErrorMessage"
}
```
