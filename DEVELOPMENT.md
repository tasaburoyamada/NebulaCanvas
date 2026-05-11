# Nebula Canvas: 開発ガイド

このガイドでは、Nebula Canvas のセットアップと開発ワークフローの詳細について説明します。

## 1. プロジェクト構成

Nebula は以下の構成からなるモノレポです。
- `frontend/`: Next.js / React / Tailwind UI
- `backend/`: Rust / Axum / RusTorch エンジン

## 2. ローカルセットアップ

### バックエンド (Backend)
1. Rust (stable) がインストールされていることを確認してください。
2. `cd backend`
3. `Settings.toml` を作成して設定を調整します（オプション）。
4. `cargo run` を実行します（データベースは `Settings.toml` の指定に基づき自動作成されます）。

### フロントエンド (Frontend)
1. `cd frontend`
2. `.env.local` を作成し、`NEXT_PUBLIC_WS_URL` を設定します。
3. `npm install`
4. `npm run dev`

## 3. 主要なコンセプト

### ゴール状態ディスパッチャ (Goal-State Dispatcher)
バックエンドは `watch::channel` ベースのディスパッチャを実装しています。ユーザーが入力を行うと、WebSocket 受信ループは即座に「最新の目標状態（Goal State）」を更新します。バックグラウンドのワーカータスクはこの状態を監視し、エンジンが準備でき次第、常に「最新のプロンプト」を優先して処理します。これにより、ネットワークをブロックすることなく、意図した通りの画像を生成できます。

### ピラミッド UI (Pyramid UI)
- **L0 (表層)**: `src/app/page.tsx` に配置された高レベルコンポーネント。
- **L1 (拡張)**: React hook を通じた共有状態管理。
- **L2 (深層)**: `src/hooks/useWebSocket.ts` による低レベルな WebSocket 抽象化。

## 4. コーディング規約
- **Rust**: `clippy` の推奨に従い、エラーハンドリングには `anyhow` を使用してください。計算コストの高い処理は `spawn_blocking` で実行します。
- **Frontend**: Tailwind CSS を使用した関数型コンポーネントを推奨します。現在はローカルステートで管理されていますが、必要に応じて Zustand 等の導入を検討してください。
