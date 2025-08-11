# Docker デプロイメント

このプロジェクトはDockerとDocker Composeを使用してデプロイできます。

## 必要な環境

- Docker
- Docker Compose
- Discord Bot Token

## 使用方法

### 1. 環境変数の設定

```bash
# .envファイルを作成（既存の.env.exampleからコピー）
cp .env.example .env

# .envファイルを編集してDISCORD_TOKENを設定
echo "DISCORD_TOKEN=your_discord_bot_token_here" > .env
```

### 2. Docker Composeでの起動

```bash
# バックグラウンドで起動
docker-compose up -d

# ログを確認
docker-compose logs -f

# 停止
docker-compose down
```

### 3. 個別のDockerコマンドでの起動

```bash
# イメージをビルド
docker build -t discord-reaction-bot .

# コンテナを起動
docker run -d --name discord-bot --env-file .env --restart unless-stopped discord-reaction-bot

# ログを確認
docker logs -f discord-bot

# 停止
docker stop discord-bot
docker rm discord-bot
```

## リソース要件

- **最小要件**: RAM 256MB, CPU 1コア
- **推奨要件**: RAM 512MB-1GB, CPU 1-2コア

## トラブルシューティング

### コンテナが起動しない場合

1. DISCORD_TOKENが正しく設定されているか確認
2. Dockerが起動しているか確認
3. ログを確認: `docker-compose logs discord-bot`

### ボットが応答しない場合

1. Discord Developer Portalでボットの権限を確認
2. ボットがサーバーに招待されているか確認
3. ネットワーク接続を確認