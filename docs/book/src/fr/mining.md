# Exploration

PRECC explore les journaux de session Claude Code pour apprendre les schémas échec-correction. Quand il revoit la même erreur, il applique la correction automatiquement.

## Ingestion des journaux de session

### Ingérer un seul fichier

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Ingérer tous les journaux

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Forcer la réingestion

Pour retraiter les fichiers déjà ingérés :

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Comment fonctionne l'exploration

1. PRECC lit le fichier journal JSONL de la session.
2. Il identifie les paires de commandes où la première a échoué et la seconde était une correction.
3. Il extrait le schéma (ce qui a mal tourné) et la correction (ce que Claude a fait différemment).
4. Les schémas sont stockés dans `~/.local/share/precc/history.db`.
5. Quand un schéma atteint un seuil de confiance (vu plusieurs fois), il devient une compétence minée dans `heuristics.db`.

### Exemple de schéma

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## Le démon precc-learner

Le démon `precc-learner` s'exécute en arrière-plan et surveille automatiquement les nouveaux journaux de session :

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Le démon utilise les notifications du système de fichiers (inotify sous Linux, FSEvents sous macOS) et réagit donc immédiatement à la fin d'une session.

## Des schémas aux compétences

Les schémas minés deviennent des compétences lorsqu'ils répondent à ces critères :

- Vus au moins 3 fois sur plusieurs sessions
- Schéma de correction cohérent (même type de correction à chaque fois)
- Aucun faux positif détecté

Vous pouvez examiner les candidats compétences avec :

```bash
$ precc skills advise
```

Voir [Skills](skills.md) pour les détails sur la gestion des compétences.

## Stockage des données

- **Paires échec-correction**: `~/.local/share/precc/history.db`
- **Compétences promues**: `~/.local/share/precc/heuristics.db`

Les deux sont des bases de données SQLite en mode WAL pour un accès concurrent sûr.
