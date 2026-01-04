# rune-data-server Contributing Guide

<!-- TOC -->
* [rune-data-server Contributing Guide](#rune-data-server-contributing-guide)
  * [開発環境](#開発環境)
  * [ローカルでの起動](#ローカルでの起動)
<!-- TOC -->

## 開発環境

rune-data-server の開発に必要な環境は以下の通りです:

- Rust (stable)
- Docker

また rune-data-server をビルド・実行するには以下の環境が必要です．

- MySQL / MariaDB のクライアントライブラリ
  - `mysqlclient-sys` という crate が必要とします．

macOS の場合は Homebrew を使ってインストールすることで解決します．

```shell
brew install mariadb-connector-c
```

Linux の場合は同様の機能を持つパッケージをインストールするか mise を使ってセットアップすることもできます (この場合インストールされるのは MySQL のクライアントライブラリです) ．

```shell
mise install
mise run build # コンパイル
mise run dev   # サーバ起動
```

## ローカルでの起動

- RuneCore がローカルにクローンされている前提です
- RuneCore のデバッグ環境を Docker で構築していない場合はこの方法は使えません．
  - 事前に Paper コンテナと RuneCore を起動してマイグレーションを実行しておきます．

1. `.env.example` をコピーして `.env` を作成します．

    ```shell
    cp .env.example .env
    ```

1. RuneCore のルートディレクトリで以下のコマンドを実行して MariaDB コンテナのみを起動します． ( `./x start` コマンドでは Paper コンテナも起動してしまうため　`start-db` コマンドを使います)

    ```shell
    ./x start-db
    ```

2. MariaDB が立ち上がることを確認します．

    ```shell
    runecore-mariadb  | 2026-01-04 19:56:07 0 [Note] Server socket created on IP: '0.0.0.0', port: '3306'.
    runecore-mariadb  | 2026-01-04 19:56:07 0 [Note] Server socket created on IP: '::', port: '3306'.
    runecore-mariadb  | 2026-01-04 19:56:07 0 [Note] mariadbd: Event Scheduler: Loaded 0 events
    runecore-mariadb  | 2026-01-04 19:56:07 0 [Note] mariadbd: ready for connections.
    ```
   
3. rune-data-server のルートディレクトリで以下のコマンドを実行してサーバを起動します．

    ```shell
    # macOS で mariadb-connector-c をインストールしている場合
    cargo run
    
    # mise のクライアントライブラリを使う場合
    mise run dev
    ```

4. サーバが立ち上がることを確認します．

    ```shell
    {"timestamp":"2026-01-04T20:03:23.786050Z","level":"INFO","fields":{"message":"Database pool created successfully."},"target":"rune_data_server"}
    {"timestamp":"2026-01-04T20:03:23.786841Z","level":"INFO","fields":{"message":"Server listening on 0.0.0.0:8080."},"target":"rune_data_server"}
    ```

5. `http://localhost:8080/health` にアクセスして以下のようなレスポンスが返ってくることを確認します．

    ```json
    {"status":"OK"}
    ```
