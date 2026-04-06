# GIF錄製

`precc gif` 從bash腳本創建終端會話的動畫GIF錄製。這是Pro功能。

## 基本用法

```bash
$ precc gif script.sh 30s
[precc] Recording script.sh (max 30s)...
[precc] Running: echo "Hello, world!"
[precc] Running: cargo build --release
[precc] Running: cargo test
[precc] Recording complete.
[precc] Output: script.gif (1.2 MB, 24s)
```

第一個參數是包含要運行的命令的bash腳本。第二個參數是最大錄製時長。

## 腳本格式

腳本是標準的bash文件：

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## 輸入模擬

對於交互式命令，提供輸入值作爲額外參數：

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

每個額外參數在腳本提示輸入時作爲一行stdin輸入。

## 輸出選項

輸出文件默認以腳本命名（`script.gif`）。GIF使用深色終端主題，標準80x24尺寸。

## 爲什麼使用GIF而不是asciinema？

內置技能 `asciinema-gif` 自動將 `asciinema rec` 重寫爲 `precc gif`。GIF文件更具可移植性——它們可以在GitHub README、Slack和電子郵件中內聯顯示，無需播放器。
