# プロジェクト概要

## 概要

Discordのリアクション情報を抽出するDiscord Bot。
コマンドが入力されたらリアクションの種類とリアクションを人をまとめ、ユーザーにメッセージとして通知する。

多言語対応を行う。現状は日本語と英語を対象とする。

## コマンド概要

### Reaction Members: リアクションした人の情報を表示

メッセージを指定し、そのメッセージのリアクションの情報を収集してユーザーにメッセージとして投稿する。
投稿するメッセージはユーザー部分はコードブロックで囲まれた状態とする。
コピペがしやすいように、かつ実際にメンションされないようにという意図がある。
このコマンド結果のメッセージは使用者のみに通知する。

#### 制約

- コマンドを実行するユーザーとBotがメッセージ、リアクションに対して読み込み権限を持つこと

#### 提供形式

- ✕: メッセージイベント
- ◯: スラッシュコマンド（コマンド名: reaction_members）
- ◯: メッセージコンテキストメニュー（メニュー名: "Get reaction members", "Get reaction-grouping members"）
- ✕: ユーザーコンテキストメニュー

#### スラッシュコマンド構文

```txt
/reaction_members message [is_author_include] [is_show_count] [is_reaction_grouping]
```

#### スラッシュコマンドパラメータ

- message: Message（必須）
    - メッセージURL、またはメッセージID
- is_author_include: bool（任意、デフォルト: false）
    - メッセージ送信者を結果に含めるかどうか
- is_show_count: bool（任意、デフォルト: false）
    - リアクションの件数表示を含めるかどうか
- is_reaction_grouping: bool（任意、デフォルト: false）
    - True: リアクションごとにユーザーを集計します
    - False: 全てのリアクションを合算してユーザーを集計します

#### スラッシュコマンド使用例

メッセージID指定（基本）

```txt
/reaction_members message:1234567890
```

メッセージURL指定、件数表示あり

```txt
/reaction_members message:https://discord.com/channels/111111/222222/333333 is_show_count:True
```

リアクション別にグループ化して表示

```txt
/reaction_members message:1234567890 is_reaction_grouping:True
```

メッセージ作成者も含めて表示

```txt
/reaction_members message:1234567890 is_author_include:True
```

#### 応答例

下記のメッセージを想定し、パラメータ設定ごとの例を記載します。

```
メッセージURL:
  https://discord.com/channels/{guild_id}/{channel_id}/{message_id}
メッセージ送信者：
  @user_a
リアクション:
  👍: @user_a @user_b
  ❤️: @user_c
  😂: @user_c @user_d
```

- 基本（パラメータなし）

```txt
Information
  📝: <メッセージへのリンク>

Users:
  @user_a @user_b @user_c @user_d
```

- is_reaction_grouping=True（リアクション別にグループ化）

```txt
Information
  📝: <メッセージへのリンク>

Reactions:
  👍: @user_a @user_b
  ❤️: @user_c
  😂: @user_c @user_d
```

- is_show_count=True（件数表示あり）

```txt
Information
  📝: <メッセージへのリンク>

Users (4):
  @user_a @user_b @user_c @user_d
```

- is_author_include=True（メッセージ作成者を含む）

```txt
Information
  📝: <メッセージへのリンク>
  🧔: @user_a 

Users:
  @user_a @user_b @user_c @user_d
```

- is_reaction_grouping=True かつ is_show_count=True

```txt
Information
  📝: <メッセージへのリンク>

Reactions:
  👍 (2): @user_a @user_b
  ❤️ (1): @user_c
  😂 (2): @user_c @user_d
```

#### エラーケース

- メッセージに対してリアクションがついていない場合

```
Information
  📝: <メッセージへのリンク>
  🧔: <@user_a> 

Reactions:
  No one reacted.
```

- メッセージが存在しない、アクセスできない、削除されている場合

```
  📝: <パラメーターのmessage>

⚠️ The message cannot be read.
- The message does not exist.
- You do not have permission to read the message.
- The message has been deleted.
```

- リアクションが多い場合

```aiignore
Due to the large number of reactions, it takes time to compile the results.
```

※このメッセージはインタラクションの3秒以内に返せる見込みがない時に表示し、集計後に正常メッセージを別途送信する。

#### メッセージコンテキストメニューから呼び出した場合

2つのコンテキストメニューオプションがあります：
1. **"Get reaction members"**: デフォルト設定が適用されます (is_reaction_grouping=false, is_author_include=false, is_show_count=false)
2. **"Get reaction-grouping members"**: リアクションごとのグループ化が有効になります (is_reaction_grouping=true, is_author_include=false, is_show_count=false)

## Botインストール対象

- ◯: ユーザー
- ◯: サーバー（guild）

## セットアップ

### 前提条件

- Rust 1.88.0以上
- Discord Developer Portalでのアプリケーション作成

### インストール手順

1. リポジトリをクローン
2. `.env`ファイルの設定
3. `cargo run`で実行

### デプロイ

#### 想定する実行環境

**ローカル環境**
- 開発・テスト用途
- 個人サーバーでの小規模運用
- メモリ: 最小256MB、推奨512MB
- CPU: 1コア以上

**VPS/クラウド環境**
- 中規模サーバーでの運用
- 24時間稼働が必要な場合
- メモリ: 最小512MB、推奨1GB
- CPU: 1-2コア
- ストレージ: 1GB以上の空き容量

**Docker環境**

```dockerfile
FROM rust:1.88-slim as builder
WORKDIR /app
COPY .. .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/discord_reaction_info_selenity /usr/local/bin/
CMD ["discord_reaction_info_selenity"]
```

**Docker Compose**
```yaml
version: '3.8'
services:
  discord-bot:
    build: .
    environment:
      - DISCORD_TOKEN=${DISCORD_TOKEN}
    restart: unless-stopped
```

#### 必要なリソース

**最小要件**
- RAM: 256MB
- CPU: 1コア
- ディスク: 500MB
- ネットワーク: 安定したインターネット接続

**推奨要件**
- RAM: 512MB-1GB
- CPU: 1-2コア
- ディスク: 1GB
- ネットワーク: 低レイテンシのインターネット接続

#### デプロイ手順

1. **本番環境での環境変数設定**
   ```bash
   export DISCORD_TOKEN="your_token_here"
   export RUST_LOG="info"
   ```

2. **systemdサービス設定（Linux）**
   ```ini
   [Unit]
   Description=Discord Reaction Info Bot
   After=network.target

   [Service]
   Type=simple
   User=discord-bot
   WorkingDirectory=/opt/discord-bot
   ExecStart=/opt/discord-bot/discord_reaction_info_selenity
   Environment=DISCORD_TOKEN=your_token_here
   Environment=RUST_LOG=info
   Restart=always
   RestartSec=10

   [Install]
   WantedBy=multi-user.target
   ```

3. **サービス起動**
   ```bash
   sudo systemctl enable discord-bot
   sudo systemctl start discord-bot
   ```

## トラブルシューティング

### よくある問題と解決方法

#### Bot が起動しない

**症状**: `cargo run` を実行してもBotが起動しない

**原因と対策**:
1. **トークンが無効**
   - `.env` ファイルの `DISCORD_TOKEN` を確認
   - Discord Developer Portal でトークンを再生成

2. **権限不足**
   - Bot に必要な権限が付与されているか確認
   - サーバー管理者権限でBotを再招待

3. **ネットワーク接続エラー**
   - インターネット接続を確認
   - ファイアウォール設定を確認

#### コマンドが応答しない

**症状**: `/rmem` コマンドを実行しても応答がない

**原因と対策**:
1. **スラッシュコマンド未登録**
   - Bot再起動後、コマンド登録に数分かかる場合がある
   - Discord で `/` を入力してコマンド一覧を確認

2. **権限不足**
   - メッセージ読み取り権限を確認
   - チャンネルアクセス権限を確認

3. **メッセージ指定エラー**
   - メッセージURLまたはIDが正しいか確認
   - 削除されたメッセージでないか確認

#### パフォーマンス問題

**症状**: コマンド実行が遅い、タイムアウトする

**原因と対策**:
1. **大量のリアクション**
   - 100個を超えるリアクションがある場合、処理時間が長くなる
   - より小さな範囲に分割して実行することを検討

2. **大量のユーザー**
   - 1000人を超えるユーザーがリアクションしている場合
   - より小さな範囲に分割して実行することを検討

3. **Discord API レート制限**
   - 短時間で大量のコマンドを実行した場合
   - 数分待ってから再実行

#### メモリ不足エラー

**症状**: Bot がクラッシュ、メモリ関連エラー

**原因と対策**:
1. **リソース不足**
   - サーバーのメモリを増やす
   - 他のプロセスを停止してリソース確保

2. **メモリリーク**
   - Bot を定期的に再起動
   - ログで異常な動作がないか確認

### ログ確認方法

**ログレベル設定**
```bash
export RUST_LOG=debug  # 詳細ログ
export RUST_LOG=info   # 通常ログ
export RUST_LOG=error  # エラーのみ
```

**ログ出力確認**
```bash
# 標準出力で確認
cargo run

# ファイルに出力
cargo run > bot.log 2>&1

# systemd でのログ確認
journalctl -u discord-bot -f
```

### サポート情報

問題が解決しない場合:

1. **GitHub Issues**: バグ報告や機能要求
2. **Discord サポートサーバー**: リアルタイムサポート（準備中）
3. **ドキュメント**: `docs/` フォルダ内の詳細情報

**問題報告時に含める情報**:
- 使用環境（OS、Rustバージョン）
- エラーメッセージ（ログ）
- 実行したコマンドとパラメータ
- 期待していた動作と実際の動作