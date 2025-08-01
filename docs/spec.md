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
- ◯: スラッシュコマンド（コマンド名: rmem）
- ◯: メッセージコンテキストメニュー（メニュー名: Reaction Members）
- ✕: ユーザーコンテキストメニュー

#### スラッシュコマンド構文

```txt
/rmem message [exclude_user] [exclude_reaction] [include_message_user] [user_only]
```

#### スラッシュコマンドパラメータ

- message: str（必須）
    - メッセージURL、またはメッセージID（文字列全体が数値に変換できるならメッセージIDと判断する）
- include_user: list \[user | member | role\]（任意）
    - 抽出対象のユーザー、メンバー、またはロール
    - メンション形式で指定する→「@yuki」「<@11111111111>」「@admin」
    - 0件なら全件対象
- exclude_user: list \[user | member | role\]（任意）
    - 除外するユーザー、メンバー、またはロール
    - メンション形式で指定する→「@yuki」「<@11111111111>」「@admin」
    - 0件なら全件対象
- exclude_reaction: list \[str | emoji\]（任意）
    - 除外するリアクション
- mode: combobox（任意、デフォルト: reaction_members）
    - 動作モード
        - reaction_members: リアクションの種類ごとにメンバー一覧を表示する
        - full: リアクションの種類ごとに件数を表示する
        - reaction_count: リアクションの種類ごとに件数とメンバー一覧を表示する
        - members: リアクションの種類に関係なく、リアクションをしたメンバー一覧を表示する（重複は除去する）
        - members_author: リアクションの種類に関係なく、メッセージ作成者と全リアクションメンバーを表示する（重複は除去する）

#### スラッシュコマンド使用例

メッセージID指定

```txt
/rmem message=1234567890
```

↓

```txt

```

メッセージURL指定、メンバーのみ

```txt
/rmem message=https://discord.com/channels/111111/222222/333333 mode=members
```

#### 応答例

下記のメッセージを想定し、オプションやモードごとの例を記載します。

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

- mode=reaction_members、または引数なし

```txt
Information
  📝: <メッセージへのリンク>
  🧔: @user_a 

Reactions:
  👍: @user_a @user_b
  ❤️: @user_c
  😂: @user_c @user_d
```

- mode=full（リアクション、件数、ユーザー表示）、user_exclude=@user_c（user_cを除外）

```txt
Information
  📝: <メッセージへのリンク>
  🧔: @user_a 

Reactions:
  👍: 2 @user_a @user_b
  ❤️: 0
  😂: 1 @user_d
```

- mode=full（リアクション、件数、ユーザー表示）、user_include=@user_c,@user_d（user_c,user_dのみ抽出）

```txt
Information
  📝: <メッセージへのリンク>
  🧔: @user_a 

Reactions:
  👍: 
  ❤️: @user_c
  😂: @user_c @user_d
```

- mode=members、または引数なし

```txt
Information
  📝: <メッセージへのリンク>
  🧔: @user_a 

members:
  @user_a @user_b @user_c @user_d
```

- reaction_exclude=👍

```txt
Information
  📝: <メッセージへのリンク>
  🧔: @user_a 

Reactions:
  ❤️: @user_c
  😂: @user_c @user_d
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

mode=(default)が適用され、その他のオプションは全て空です。
つまりはリアクションとユーザーの一覧が応答結果になります。

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

- 想定する実行環境（ローカル/VPS/クラウド）
- 必要なリソース（メモリ、CPU）