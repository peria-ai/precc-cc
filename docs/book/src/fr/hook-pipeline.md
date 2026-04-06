# Pipeline du Hook

Le binaire `precc-hook` est le cœur de PRECC. Il se place entre Claude Code et le shell, traitant chaque commande bash en moins de 5 millisecondes.

## Comment Claude Code invoque le Hook

Claude Code prend en charge les hooks PreToolUse -- des programmes externes qui peuvent inspecter et modifier les entrées d'outils avant l'exécution. Quand Claude est sur le point d'exécuter une commande bash, il envoie du JSON à `precc-hook` sur stdin et lit la réponse depuis stdout.

## Étapes du Pipeline

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

## Exemple : Entrée et sortie JSON

### Entrée (depuis Claude Code)

```json
{
  "tool_input": {
    "command": "cargo build"
  }
}
```

PRECC détecte que le répertoire actuel n'a pas de `Cargo.toml`, mais `./myapp/Cargo.toml` existe.

### Sortie (vers Claude Code)

```json
{
  "hookSpecificOutput": {
    "updatedInput": {
      "command": "cd /home/user/projects/myapp && cargo build"
    }
  }
}
```

Si aucune modification n'est nécessaire, `updatedInput.command` est vide et Claude Code utilise la commande originale.

## Détails des étapes

### Étape 1 : Analyser le JSON

Lit l'objet JSON complet depuis stdin. Extrait `tool_input.command`. Si l'analyse échoue, le hook se termine immédiatement et Claude Code utilise la commande originale (conception fail-open).

### Étape 2 : Correspondance des compétences

Interroge la base de données heuristique SQLite pour les compétences dont le modèle de déclenchement correspond à la commande. Les compétences sont vérifiées par ordre de priorité. Les compétences TOML intégrées et les compétences extraites sont évaluées.

### Étape 3 : Correction de répertoire

Pour les commandes de build (`cargo`, `go`, `make`, `npm`, `python`, etc.), vérifie si le fichier projet attendu existe dans le répertoire actuel. Sinon, analyse les répertoires voisins pour trouver la correspondance la plus proche et ajoute `cd <dir> &&` en préfixe.

L'analyse de répertoire utilise un index de système de fichiers en cache avec un TTL de 5 secondes pour rester rapide.

### Étape 4 : Vérification GDB

Si la commande est susceptible de produire un crash (ex. exécution d'un binaire de débogage), PRECC peut suggérer ou injecter des wrappers GDB pour capturer une sortie de débogage structurée au lieu de logs de crash bruts.

### Étape 5 : Réécriture RTK

Applique les règles RTK (Rewrite Toolkit) qui raccourcissent les commandes verbeuses, suppriment les sorties bruyantes ou restructurent les commandes pour l'efficacité des tokens.

### Étape 6 : Émettre le JSON

Sérialise la commande modifiée en JSON et l'écrit sur stdout. Si aucun changement n'a été effectué, la sortie signale à Claude Code d'utiliser la commande originale.

## Performance

L'ensemble du pipeline s'exécute en moins de 5 millisecondes (p99). Optimisations clés :

- SQLite en mode WAL pour des lectures concurrentes sans verrou
- Modèles regex précompilés pour la correspondance des compétences
- Scans du système de fichiers en cache (TTL de 5 secondes)
- Aucun appel réseau dans le chemin critique
- Fail-open : toute erreur retombe sur la commande originale

## Tester le Hook manuellement

Vous pouvez invoquer le hook directement :

```bash
$ echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
{"hookSpecificOutput":{"updatedInput":{"command":"cd /home/user/myapp && cargo build"}}}
```
