# GIF録画

`precc gif`はbashスクリプトからターミナルセッションのアニメーションGIF録画を作成します。これはPro機能です。

## 基本的な使い方

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

最初の引数は実行するコマンドを含むbashスクリプトです。2番目の引数は最大録画時間です。

## スクリプト形式

スクリプトは標準的なbashファイルです：

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## 入力シミュレーション

インタラクティブコマンドの場合、追加引数として入力値を指定します：

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

追加の各引数は、スクリプトが入力を要求するときにstdinの行として渡されます。

## 出力オプション

出力ファイルはデフォルトでスクリプト名に基づいて命名されます（`script.gif`）。GIFはダークターミナルテーマで標準80x24サイズを使用します。

## なぜasciinemaではなくGIFなのか？

組み込みスキル `asciinema-gif` は `asciinema rec` を自動的に `precc gif` に書き換えます。GIFファイルはより移植性が高く、GitHub README、Slack、メールでプレーヤーなしでインライン表示されます。
