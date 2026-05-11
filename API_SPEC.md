# Nebula Canvas: API 仕様書

Nebula Canvas は、フロントエンドとバックエンド間のリアルタイム通信に構造化された WebSocket プロトコルを使用します。

## 1. 接続詳細
- **デフォルト URL**: `ws://127.0.0.1:3001/ws`
- **形式**: JSON (UTF-8)

## 2. クライアント・メッセージ (Frontend -> Backend)

全てのメッセージは `type` フィールドを含み、必要に応じて `data` フィールドを持ちます。

### `Generate`
新しい画像の生成を開始します。
```json
{
  "type": "Generate",
  "data": {
    "prompt": "魔法の森",
    "seed": 42,
    "steps": 20
  }
}
```

### `GetHistory`
生成履歴の全ダンプを要求します。
```json
{
  "type": "GetHistory"
}
```

## 3. サーバー・メッセージ (Backend -> Frontend)

### `ImageUpdate`
生成が完了した際に送信されます。
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
過去の全ての生成結果を配列で返します。
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
エンジンのリアルタイムな状態を通知します。
```json
{
  "type": "Status",
  "data": "Generating..."
}
```

### `Error`
操作が失敗した際に送信されます。
```json
{
  "type": "Error",
  "data": "エラーメッセージの内容"
}
```
