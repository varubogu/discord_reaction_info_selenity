# Discord リアクション情報Bot (Poise)

Discordメッセージのリアクション情報を抽出・表示するDiscord Botです。RustとPoiseフレームワーク（Serenityをベースに構築）で構築され、スラッシュコマンドとコンテキストメニューを通じてメッセージのリアクション分析を便利に行えます。

[English](README.md) | 日本語

## 機能

- **リアクション分析**: 任意のDiscordメッセージから詳細なリアクション情報を抽出
- **複数のアクセス方法**: スラッシュコマンドとコンテキストメニューで利用可能
- **柔軟なフィルタリング**: 特定のユーザー、ロール、リアクションタイプの包含・除外
- **複数の表示モード**: リアクション別、ユーザーリスト、サマリー件数表示
- **多言語対応**: 日本語と英語をサポート
- **プライバシー重視**: 結果はコマンド実行者にのみ表示

## コマンド

### `/reaction_members` - リアクションメンバーコマンド

指定されたメッセージのリアクションを分析して情報を表示します。

**使用方法:**
```
/reaction_members message:<メッセージURLまたはID> [is_author_include:true/false] [is_show_count:true/false] [is_reaction_grouping:true/false]
```

**パラメータ:**
- `message` (必須): メッセージURLまたはメッセージID
- `is_author_include` (任意): メッセージ作成者を結果に含める (デフォルト: false)
- `is_show_count` (任意): リアクション件数を結果に表示 (デフォルト: false)
- `is_reaction_grouping` (任意): リアクションタイプ別にユーザーをグループ化 (デフォルト: false)
  - `true`: リアクションタイプごとにユーザーを表示
  - `false`: すべてのリアクションユーザーを統合して表示（重複除去）

### コンテキストメニュー

任意のメッセージを右クリックして以下の2つのオプションから選択できます：
- **"Get reaction members"**: デフォルト設定でクイックリアクション分析（統合リアクション）
- **"Get reaction-grouping members"**: リアクションタイプ別にグループ化したクイックリアクション分析

## インストール

### 前提条件

- Rust 1.88.0以上
- Discord Developer Portalで作成されたDiscordアプリケーション
- Discord Botトークン

### セットアップ手順

1. **リポジトリをクローン:**
   ```bash
   git clone https://github.com/varubogu/discord_reaction_info_selenity.git
   cd discord_reaction_info_selenity
   ```

2. **環境設定ファイルを作成:**
   ```bash
   cp .env.example .env
   ```
   
3. **Discordトークンを設定:**
   `.env`ファイルを編集してDiscord botトークンを追加:
   ```
   DISCORD_TOKEN=あなたのbotトークン
   ```

4. **ビルドと実行:**
   ```bash
   cargo build --release
   cargo run
   ```

### Dockerデプロイメント

サーバーでのDocker デプロイについては、[Docker デプロイメントガイド](docs/ja/docker-deploy.md)でDockerとDocker Composeを使用した詳細な手順を参照してください。

## Discord Bot セットアップ

### 必要な権限

Botには以下のDiscord権限が必要です:
- メッセージ履歴の読み取り
- メッセージの送信
- スラッシュコマンドの使用
- メッセージリアクションの読み取り

### サーバーへの追加

1. Discord Developer Portalにアクセス
2. あなたのアプリケーションを選択
3. OAuth2 > URL Generatorへ移動
4. 「bot」と「applications.commands」スコープを選択
5. 上記の必要な権限を選択
6. 生成されたURLを使用してBotをサーバーに招待

## 使用例

### 基本的な使用方法
```
/reaction_members message:1234567890123456789
```

### リアクション件数を表示
```
/reaction_members message:https://discord.com/channels/111/222/333 is_show_count:true
```

### リアクションタイプ別にグループ化
```
/reaction_members message:1234567890123456789 is_reaction_grouping:true
```

### メッセージ作成者も含めて表示
```
/reaction_members message:1234567890123456789 is_author_include:true
```

## パフォーマンス制限

- メッセージあたり最大100リアクション
- リアクションあたり最大1000ユーザー
- コマンド実行タイムアウト: 15秒
- Discord APIレート制限の遵守

## 開発

開発に関する詳細情報、コーディング規約、プロジェクト構造については[docs/develop.md](docs/ja/develop.md)を参照してください。

## ドキュメント

- [プロジェクト仕様](docs/ja/spec.md) - 詳細なコマンド仕様と例
- [開発ガイド](docs/ja/develop.md) - 開発環境とコーディング規約

## ライセンス

このプロジェクトはMITライセンスの下で公開されています - 詳細は[LICENSE](LICENSE)ファイルを参照してください。

## サポート

問題が発生したり質問がある場合:
1. `docs/`フォルダ内のドキュメントを確認
2. 上記の例を参照
3. GitHubでissueを作成

## 貢献

貢献を歓迎します！コーディング規約とプロジェクト構造ガイドラインについては、`docs/develop.md`の開発ガイドを参照してください。


