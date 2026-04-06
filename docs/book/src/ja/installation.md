# インストール

## クイックインストール (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

これはお使いのプラットフォーム用の最新リリースバイナリをダウンロードし、SHA256チェックサムを検証して `~/.local/bin/` に配置します。

インストール後、PRECCを初期化します：

```bash
precc init
```

`precc init` はPreToolUseフックをClaude Codeに登録し、データディレクトリを作成し、スキルデータベースを初期化します。

## インストールオプション

### SHA256検証

デフォルトでは、インストーラーは公開されたSHA256サムに対してバイナリチェックサムを検証します。検証をスキップするには（非推奨）：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### カスタムインストールプレフィックス

カスタムの場所にインストール：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### コンパニオンツール (--extras)

PRECCにはオプションのコンパニオンツールが付属しています。`--extras` でインストールします：

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

これにより以下がインストールされます：

| ツール | 用途 |
|------|---------|
| **RTK** | コマンド書き換えツールキット |
| **lean-ctx** | CLAUDE.mdおよびプロンプトファイルのコンテキスト圧縮 |
| **nushell** | 高度なパイプライン用の構造化シェル |
| **cocoindex-code** | より高速なコンテキスト解決のためのコードインデックス |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

次に初期化します：

```powershell
precc init
```

## 手動インストール

1. お使いのプラットフォーム用のリリースバイナリを [GitHub Releases](https://github.com/peria-ai/precc-cc/releases) からダウンロードします。
2. リリースの `.sha256` ファイルに対してSHA256チェックサムを検証します。
3. バイナリを `PATH` 上のディレクトリ（例：`~/.local/bin/`）に配置します。
4. `precc init` を実行します。

## アップデート

```bash
precc update
```

特定のバージョンへ強制アップデート：

```bash
precc update --force --version 0.3.0
```

自動アップデートを有効にする：

```bash
precc update --auto
```

## インストールの確認

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

`precc` が見つからない場合は、`~/.local/bin` が `PATH` に含まれていることを確認してください。
