# 開発に関して

## 開発環境

### 必要なツール

**基本環境**
- 言語: Rust 1.88.0以上
- フレームワーク: Poise (SerenityベースのDiscord APIクライアント)
- パッケージマネージャー: Cargo

**推奨開発ツール**
- エディタ: VS Code, IntelliJ IDEA, vim/neovim
- VS Code拡張: rust-analyzer, CodeLLDB
- Git: バージョン管理
- Docker: コンテナ環境での動作確認

### 開発環境セットアップ

#### 1. Rustのインストール

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
1. https://rustup.rs/ から rustup-init.exe をダウンロード
2. インストーラーを実行
3. PowerShellまたはコマンドプロンプトを再起動

**バージョン確認:**
```bash
rustc --version
cargo --version
```

#### 2. プロジェクトのクローンと設定

```bash
# リポジトリをクローン
git clone https://github.com/varubogu/discord_reaction_info_selenity.git
cd discord_reaction_info_selenity

# 依存関係をインストール
cargo build

# テストを実行してセットアップを確認
cargo test
```

#### 3. Discord Bot設定

**Discord Developer Portalでの設定:**
1. https://discord.com/developers/applications にアクセス
2. 「New Application」をクリック
3. アプリケーション名を入力
4. 「Bot」タブに移動
5. 「Add Bot」をクリック
6. トークンをコピー（後で使用）

**環境変数の設定:**
```bash
# .envファイルを作成
cp .env.example .env

# .envファイルを編集
echo "DISCORD_TOKEN=your_bot_token_here" > .env
echo "RUST_LOG=debug" >> .env  # 開発時は詳細ログを有効
```

#### 4. 開発用サーバーでのBot招待

**OAuth2 URL生成:**
1. Discord Developer Portal の OAuth2 タブ
2. 「URL Generator」を選択
3. Scopes: `bot`, `applications.commands`
4. Bot Permissions:
   - Read Message History
   - Send Messages
   - Use Slash Commands
   - Read Message Reactions
5. 生成されたURLでテストサーバーに招待

### 開発ワークフロー

#### ローカル開発

```bash
# 開発モードで実行（ホットリロードなし）
cargo run

# リリースモードでビルド
cargo build --release

# 特定のテストを実行
cargo test test_name

# ドキュメント生成
cargo doc --open
```

#### Dockerを使用した開発

Dockerを使用した開発・デプロイについては、[Dockerデプロイメントガイド](docker-deploy.md)を参照してください。

#### コードフォーマットとリント

```bash
# コードフォーマット
cargo fmt

# リント実行
cargo clippy

# 厳密なリント
cargo clippy -- -D warnings
```

#### デバッグ方法

**ログ出力レベル設定:**
```bash
# 詳細デバッグログ
export RUST_LOG=debug

# 特定モジュールのみ
export RUST_LOG=discord_reaction_info_selenity=debug

# 複数レベル指定
export RUST_LOG=debug,selenity=info
```

**デバッガー使用:**
- VS Code: CodeLLDB拡張を使用
- コマンドライン: `rust-gdb target/debug/discord_reaction_info_selenity`

#### パフォーマンス分析

```bash
# プロファイリング付きビルド
cargo build --profile release-with-debug

# ベンチマーク実行
cargo bench

# メモリ使用量分析
valgrind --tool=massif target/release/discord_reaction_info_selenity
```

## 技術仕様

### 使用ライブラリ

- tokio: 非同期ランタイム
- poise: Discord bot フレームワーク (Serenityベース)
- anyhow: エラーハンドリング
- tracing: ログ出力
- dotenv: 環境変数管理

### 必要なDiscord Bot権限

- メッセージ履歴の読み取り
- メッセージの送信
- リアクションの読み取り
- スラッシュコマンドの使用

### 環境変数

- `DISCORD_TOKEN`: BotのDiscordトークン

## 制限事項

- Discord APIのレート制限に従う
- 大量のリアクションがある場合の処理時間
- メッセージの保持期間による制約
- 同時実行数の制限

## パフォーマンス考慮

- 1メッセージあたりの最大リアクション数: 100個
- 1リアクションあたりの最大ユーザー数: 1000人
- 1メッセージあたりの最大ユーザー数: 10000人
- コマンド実行タイムアウト: 15秒
- 同時実行制限: ユーザーあたり1件

## 注意点

- プライベートチャンネルでの動作制限
- 削除されたユーザーの扱い
- 大きなサーバーでのパフォーマンス考慮

## コーディング規約

- 1行あたりの文字数: 半角80文字以内
- 1関数あたりの行数: 50行以下
- docstringがつけられるものは必ずつける。（構造体、トレイト、関数など）
- 1ファイルあたりの行数: 200行以下（テストモジュール部分を除く）
- ネスト: 関数の中に入ってから3段階まで
- DRY原則、SOLID原則を遵守する
- 単体テストは同ファイルのtestモジュールで行う
- 結合テストは`tests/integration/**.rs`で行う
- 総合テストは`tests/system/**.rs`で行う
- その他、Rustの一般的なコーディング規約に遵守する

## プロジェクト構成

```
(root)
├── .env          # 環境変数（ローカル用）
├── Cargo.toml    # cargoクレート定義
├── Cargo.lock    # cargoインストール済みクレート定義
├── docs/         # ドキュメント
├── src/          # ソースコード
│   ├── main.rs   # エントリポイント～イベントループまでのロジック
│   ├── init.rs   # bot起動までの初期化処理を定義
│   ├── events/                 # イベント全般 ※受け口のみを定義し、実際のロジックはsrc/services/配下に配置する
│   │   ├── mod.rs              # モジュール宣言
│   │   ├── on_message.rs       # メッセージが送信された時のリスナー
│   │   ├── on_reaction_add.rs  # メッセージにリアクションされた時のリスナー
│   │   ├── xxxxx.rs            # イベントに応じたリスナー、イベント名をそのままファイル名にする
│   │   ├── interactions/       # インタラクション全般
│   │   │   ├── command_interactions/     # コマンド系インタラクション
│   │   │   │   ├── mod.rs                # モジュール宣言
│   │   │   │   ├── slash/                # スラッシュコマンドの定義
│   │   │   │   │   ├── mod.rs            # モジュール宣言
│   │   │   │   │   ├── reaction_members.rs     # リアクションしたユーザー情報を収集して表示するスラッシュコマンド
│   │   │   │   │   ├── xxxx_slash.rs     # スラッシュコマンドは「_slash.rs」で終わること
│   │   │   │   ├── contextmenu/                          # コンテキストメニュー系インタラクション
│   │   │   │   │   ├── mod.rs                            # モジュール宣言
│   │   │   │   │   ├── reaction_users_context_menu.rs    # リアクションしたユーザー情報を収集して表示するコンテキストメニュー
│   │   │   │   │   ├── reaction_grouping_users_context_menu.rs # リアクションごとにグループ化したユーザー情報のコンテキストメニュー
│   │   │   │   │   ├── xxxx_context_menu.rs              # コンテキストメニューは「_context_menu.rs」で終わること
│   │   │   ├── components/             # コンポーネントのインタラクション
│   │   │   │   ├── xxxx_component.rs   # コンポーネントは「_component.rs」で終わること
│   │   │   ├── modal/                  # モーダルをユーザーが送信した時のインタラクション
│   │   │   │   ├── xxxx_modal.rs       # モーダルは「_modal.rs」で終わること
│   ├── services/               # ビジネスロジックを配置、規模が大きい場合はフォルダ分けも検討する
│   │   ├── mod.rs              # モジュール宣言
│   │   ├── reaction_users.rs   # リアクションしたユーザー情報を収集し、結果を返す
│   │   ├── xxx.rs              # ビジネスロジック
│   ├── utils/        # 汎用的な処理をまとめるフォルダ
│   │   ├── mod.rs    # モジュール宣言
│   │   ├── xxx.rs    # 汎用的な処理
├── tests/            # テストコード
│   ├── integration/  # 結合テスト
│   ├── system/       # 総合テスト
```
