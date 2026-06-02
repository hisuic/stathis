# Stathis

Stathis は Rust と Serenity で実装した Discord Bot です。現在はシンプルな応答確認コマンドと、ボタンで通知時間を選べるリマインダー機能を備えています。

## 主な機能

- `!ping` に対して `Pong!` と返信
- `!remind` でリマインダーを作成
- リマインダーの通知時間を Discord のボタンから選択
- 指定時間後に、設定したチャンネルで実行者へメンションして通知

## 必要なもの

- Rust
- Cargo
- Discord Bot のトークン
- Bot を招待できる Discord サーバー

## セットアップ

1. リポジトリをクローンします。

```sh
git clone <repository-url>
cd stathis
```

2. Discord Developer Portal で Bot を作成し、トークンを取得します。

3. プロジェクト直下に `.env` を作成し、Bot トークンを設定します。

```env
DISCORD_TOKEN=your_discord_bot_token
```

4. Discord Developer Portal の Bot 設定で `MESSAGE CONTENT INTENT` を有効にします。

5. Bot をサーバーへ招待します。最低限、次の権限が必要です。

- メッセージを読む
- メッセージを送信
- ボタンなどのインタラクションを利用
- メンションを含むメッセージを送信

6. Bot を起動します。

```sh
cargo run
```

起動に成功すると、コンソールに `<Bot名> is connected!` と表示されます。

## 使い方

### 疎通確認

Discord のチャンネルで次のように入力します。

```text
!ping
```

Bot が次のように返信します。

```text
Pong!
```

### リマインダー

Discord のチャンネルで次のように入力します。

```text
!remind レポートを提出する
```

Bot が通知時間を選ぶボタンを表示します。ボタンは 3 分間だけ有効です。

選択できる時間は次の通りです。

- 30 分
- 1 時間
- 2 時間
- 3 時間
- 4 時間
- 5 時間
- 6 時間
- 8 時間
- 10 時間
- 12 時間
- 1 日
- 3 日
- 1 週間
- 2 週間
- 1 か月

時間を選ぶと `Reminder set!` と表示され、指定時間後に次の形式で通知します。

```text
@ユーザー reminder: レポートを提出する
```

## プロジェクト構成

```text
.
├── Cargo.toml
├── README.md
├── scripts
│   └── deploy-latest.sh
└── src
    ├── main.rs
    └── commands
        ├── mod.rs
        ├── ping.rs
        └── remind.rs
```

- `src/main.rs`: Bot の起動、イベントハンドラ、コマンド振り分け
- `src/commands/ping.rs`: `!ping` コマンド
- `src/commands/remind.rs`: `!remind` コマンド
- `scripts/deploy-latest.sh`: Raspberry Pi 上で最新バイナリを取得して配置するスクリプト

## 開発用コマンド

コードの確認:

```sh
cargo check
```

フォーマット:

```sh
cargo fmt
```

リリースビルド:

```sh
cargo build --release
```

## Raspberry Pi へのデプロイ

このリポジトリでは、GitHub Actions で Raspberry Pi 向けのバイナリをビルドします。

`.github/workflows/build-pi.yml` により、`main` ブランチへ push すると `aarch64-unknown-linux-gnu` 向けにリリースビルドされ、GitHub Releases の `latest` に `stathis` バイナリがアップロードされます。

`latest` は常に最新の Raspberry Pi 用ビルドを指すデプロイ用のタグとして扱います。Raspberry Pi では次の URL から最新バイナリを取得できます。

```text
https://github.com/hisuic/stathis/releases/download/latest/stathis
```

Raspberry Pi 上では Rust のビルド環境を用意せず、ビルド済みの `stathis` バイナリだけを配置して実行します。

現在の運用では、Raspberry Pi 上の次の場所にバイナリを配置しています。

```text
/home/murray/builds/stathis
├── stathis
└── .env
```

最新バイナリの取得と差し替えには、次のスクリプトを使います。

```sh
./scripts/deploy-latest.sh
```

このスクリプトは GitHub Releases の `latest` から `stathis` バイナリを取得し、`/home/murray/builds/stathis/stathis` に配置します。配置後、デフォルトで `stathis` service を再起動します。

service の再起動を行わず、バイナリの配置だけ行う場合:

```sh
./scripts/deploy-latest.sh --no-restart
```

配置先や service 名を変える場合は、環境変数で指定できます。

```sh
STATHIS_DEPLOY_DIR=/home/murray/builds/stathis STATHIS_SERVICE_NAME=stathis ./scripts/deploy-latest.sh
```

`DISCORD_TOKEN` は `/home/murray/builds/stathis/.env` に置きます。`stathis.service` では `WorkingDirectory` を `/home/murray/builds/stathis` にしているため、Bot 起動時に `dotenvy` がこの `.env` を読み込みます。

`.env`:

```env
DISCORD_TOKEN=your_discord_bot_token
```

現在の systemd 設定は次のコマンドで確認できます。

```sh
sudo systemctl cat stathis
```

## systemd で常駐させる

Raspberry Pi 上では system-wide な systemd service として起動します。これにより、再起動時の自動起動や、異常終了時の自動再起動ができます。

service ファイルを作成します。

```sh
sudo vim /etc/systemd/system/stathis.service
```

内容:

```ini
[Unit]
Description=Stathis Discord Bot
After=network-online.target
Wants=network-online.target

[Service]
Type=simple
User=murray
Group=murray
WorkingDirectory=/home/murray/builds/stathis
ExecStart=/home/murray/builds/stathis/stathis
Restart=always
RestartSec=5
StandardOutput=inherit
StandardError=inherit

[Install]
WantedBy=multi-user.target
```

service を反映して起動します。

```sh
sudo systemctl daemon-reload
sudo systemctl enable stathis
sudo systemctl start stathis
```

状態確認:

```sh
systemctl status stathis
```

ログ確認:

```sh
journalctl -u stathis -f
```

バイナリを更新したときは、配置済みの `/home/murray/builds/stathis/stathis` を新しいものに差し替えてから service を再起動します。

```sh
sudo systemctl restart stathis
```

## 注意点

- リマインダーは現在の Bot プロセス内で待機しています。Bot を再起動すると、設定済みのリマインダーは失われます。
- `!remind` の後ろに空白だけを入れた場合でも、現在の実装では空のリマインダーを設定できます。
- コマンドのプレフィックスは現在 `!` 固定です。
- Bot がメッセージ本文を読むため、Discord Developer Portal で `MESSAGE CONTENT INTENT` を有効にする必要があります。

## 今後の改善候補

- リマインダーの永続化
- リマインダーのキャンセル機能
- 通知時刻を自然言語や日時指定で入力する機能
- スラッシュコマンド対応
- エラー処理とログ出力の改善
