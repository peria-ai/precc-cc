# Instalação

## Instalação rápida (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

Isso baixa o binário da versão mais recente para sua plataforma, verifica a soma de verificação SHA256 e o coloca em `~/.local/bin/`.

Após a instalação, inicialize o PRECC:

```bash
precc init
```

`precc init` registra o hook PreToolUse no Claude Code, cria os diretórios de dados e inicializa o banco de dados de habilidades.

## Opções de instalação

### Verificação SHA256

Por padrão, o instalador verifica a soma de verificação do binário contra a soma SHA256 publicada. Para pular a verificação (não recomendado):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Prefixo de instalação personalizado

Instalar em um local personalizado:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Ferramentas complementares (--extras)

O PRECC inclui ferramentas complementares opcionais. Instale-as com `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

Isso instala:

| Ferramenta | Finalidade |
|------|---------|
| **RTK** | Kit de reescrita de comandos |
| **lean-ctx** | Compressão de contexto para CLAUDE.md e arquivos de prompt |
| **nushell** | Shell estruturado para pipelines avançados |
| **cocoindex-code** | Indexação de código para resolução de contexto mais rápida |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Em seguida, inicialize:

```powershell
precc init
```

## Instalação manual

1. Baixe o binário de lançamento para sua plataforma em [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Verifique a soma de verificação SHA256 com o arquivo `.sha256` da versão.
3. Coloque o binário em um diretório no seu `PATH` (ex.: `~/.local/bin/`).
4. Execute `precc init`.

## Atualização

```bash
precc update
```

Forçar atualização para uma versão específica:

```bash
precc update --force --version 0.3.0
```

Ativar atualizações automáticas:

```bash
precc update --auto
```

## Verificando a instalação

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Se `precc` não for encontrado, certifique-se de que `~/.local/bin` está no seu `PATH`.
