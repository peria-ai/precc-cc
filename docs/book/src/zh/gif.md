# GIF录制

`precc gif` 从bash脚本创建终端会话的动画GIF录制。这是Pro功能。

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

第一个参数是包含要运行的命令的bash脚本。第二个参数是最大录制时长。

## 脚本格式

脚本是标准的bash文件：

```bash
#!/bin/bash
echo "Building project..."
cargo build --release
echo "Running tests..."
cargo test
echo "Done!"
```

## 输入模拟

对于交互式命令，提供输入值作为额外参数：

```bash
$ precc gif interactive-demo.sh 60s "yes" "my-project" "3"
```

每个额外参数在脚本提示输入时作为一行stdin输入。

## 输出选项

输出文件默认以脚本命名（`script.gif`）。GIF使用深色终端主题，标准80x24尺寸。

## 为什么使用GIF而不是asciinema？

内置技能 `asciinema-gif` 自动将 `asciinema rec` 重写为 `precc gif`。GIF文件更具可移植性——它们可以在GitHub README、Slack和电子邮件中内联显示，无需播放器。
