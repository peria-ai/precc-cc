# Pipeline do Hook

O binário `precc-hook` é o núcleo do PRECC. Ele fica entre o Claude Code e o shell, processando cada comando bash em menos de 5 milissegundos.

## Como o Claude Code invoca o Hook

O Claude Code suporta hooks PreToolUse -- programas externos que podem inspecionar e modificar entradas de ferramentas antes da execução. Quando o Claude está prestes a executar um comando bash, ele envia JSON para `precc-hook` via stdin e lê a resposta de stdout.

## Estágios do Pipeline

```
Claude Code
    |
    v
+---------------------------+
| 1. Parse JSON stdin       |  Read the command from Claude Code
+---------------------------+
    |
    v
+---------------------------+
| 2. Skill matching         |  Query heuristics.db for matching skills (Pillar 4)
+---------------------------+
    |
    v
+---------------------------+
| 3. Directory correction   |  Resolve correct working directory (Pillar 1)
+---------------------------+
    |
    v
+---------------------------+
| 4. GDB check              |  Detect debug opportunities (Pillar 2)
+---------------------------+
    |
    v
+---------------------------+
| 5. RTK rewriting          |  Apply command rewrites for token savings
+---------------------------+
    |
    v
+---------------------------+
| 6. Emit JSON stdout       |  Return modified command to Claude Code
+---------------------------+
    |
    v
  Shell executes corrected command
```

## Exemplo: Entrada e saída JSON

### Entrada (do Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

O PRECC detecta que o diretório atual não tem `Cargo.toml`, mas `./myapp/Cargo.toml` existe.

### Saída (para o Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Se nenhuma modificação for necessária, `updatedInput.command` estará vazio e o Claude Code usará o comando original.

## Detalhes dos estágios

### Estágio 1: Analisar JSON

Lê o objeto JSON completo do stdin. Extrai `tool_input.command`. Se a análise falhar, o hook sai imediatamente e o Claude Code usa o comando original (design fail-open).

### Estágio 2: Correspondência de habilidades

Consulta o banco de dados heurístico SQLite para habilidades cujo padrão de gatilho corresponda ao comando. As habilidades são verificadas em ordem de prioridade. Tanto habilidades TOML integradas quanto mineradas são avaliadas.

### Estágio 3: Correção de diretório

Para comandos de build (`cargo`, `go`, `make`, `npm`, `python`, etc.), verifica se o arquivo de projeto esperado existe no diretório atual. Se não, escaneia diretórios próximos para a correspondência mais próxima e adiciona `cd <dir> &&` no início.

A varredura de diretório usa um índice de sistema de arquivos em cache com TTL de 5 segundos para manter a velocidade.

### Estágio 4: Verificação GDB

Se o comando provavelmente produzirá um crash (ex., executar um binário de depuração), o PRECC pode sugerir ou injetar wrappers GDB para capturar saída de depuração estruturada em vez de logs de crash brutos.

### Estágio 5: Reescrita RTK

Aplica regras RTK (Rewrite Toolkit) que encurtam comandos verbosos, suprimem saída ruidosa ou reestruturam comandos para eficiência de tokens.

### Estágio 6: Emitir JSON

Serializa o comando modificado de volta para JSON e escreve no stdout. Se nenhuma alteração foi feita, a saída sinaliza ao Claude Code para usar o comando original.

## Desempenho

Todo o pipeline é concluído em menos de 5 milissegundos (p99). Otimizações principais:

- SQLite em modo WAL para leituras concorrentes sem bloqueio
- Padrões regex pré-compilados para correspondência de habilidades
- Varreduras de sistema de arquivos em cache (TTL de 5 segundos)
- Nenhuma chamada de rede no caminho quente
- Fail-open: qualquer erro retorna ao comando original

## Testando o Hook manualmente

Você pode invocar o hook diretamente:

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
