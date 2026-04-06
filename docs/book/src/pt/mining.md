# Mineração

PRECC minera os logs de sessão do Claude Code para aprender padrões de falha-correção. Quando vê o mesmo erro novamente, aplica a correção automaticamente.

## Ingestão de logs de sessão

### Ingerir um único arquivo

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Ingerir todos os logs

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Forçar reingestão

Para reprocessar arquivos já ingeridos:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Como a mineração funciona

1. PRECC lê o arquivo de log JSONL da sessão.
2. Identifica pares de comandos onde o primeiro falhou e o segundo foi uma correção.
3. Extrai o padrão (o que deu errado) e a correção (o que Claude fez diferente).
4. Os padrões são armazenados em `~/.local/share/precc/history.db`.
5. Quando um padrão atinge um limiar de confiança (visto várias vezes), torna-se uma habilidade minerada em `heuristics.db`.

### Exemplo de padrão

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## O daemon precc-learner

O daemon `precc-learner` roda em segundo plano e monitora automaticamente novos logs de sessão:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

O daemon usa notificações do sistema de arquivos (inotify no Linux, FSEvents no macOS) para reagir imediatamente quando uma sessão termina.

## De padrões a habilidades

Padrões minerados se graduam para habilidades quando atendem a estes critérios:

- Vistos pelo menos 3 vezes em sessões diferentes
- Padrão de correção consistente (mesmo tipo de correção a cada vez)
- Nenhum falso positivo detectado

Você pode revisar candidatos a habilidades com:

```bash
$ precc skills advise
```

Veja [Skills](skills.md) para detalhes sobre gerenciamento de habilidades.

## Armazenamento de dados

- **Pares falha-correção**: `~/.local/share/precc/history.db`
- **Habilidades graduadas**: `~/.local/share/precc/heuristics.db`

Ambos são bancos de dados SQLite em modo WAL para acesso concorrente seguro.
