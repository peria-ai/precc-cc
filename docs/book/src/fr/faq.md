# FAQ

## PRECC est-il sûr à utiliser ?

Oui. PRECC utilise le mécanisme officiel de hooks PreToolUse de Claude Code -- le même point d'extension qu'Anthropic a conçu exactement à cet effet. Le hook :

- Fonctionne entièrement hors ligne (pas d'appels réseau dans le chemin critique)
- Se termine en moins de 5 millisecondes
- Est fail-open : en cas de problème, la commande originale s'exécute sans modification
- Ne fait que modifier les commandes, ne les exécute jamais lui-même
- Stocke les données localement dans des bases SQLite

## PRECC fonctionne-t-il avec d'autres outils de codage IA ?

PRECC est conçu spécifiquement pour Claude Code. Il s'appuie sur le protocole de hooks PreToolUse fourni par Claude Code. Il ne fonctionne pas avec Cursor, Copilot, Windsurf ou d'autres outils de codage IA.

## Quelles données la télémétrie envoie-t-elle ?

La télémétrie est uniquement sur abonnement. Lorsqu'elle est activée, elle envoie :

- Version de PRECC, système d'exploitation et architecture
- Compteurs agrégés (commandes interceptées, compétences activées)
- Latence moyenne du hook

Elle n'envoie **pas** de texte de commande, de chemins de fichiers, de noms de projets ou d'informations personnellement identifiables. Vous pouvez prévisualiser la charge exacte avec `precc telemetry preview` avant de vous abonner. Voir [Télémétrie](telemetry.md) pour les détails.

## Comment désinstaller PRECC ?

??faq_uninstall_a_intro??

1. Supprimer l'enregistrement du hook :
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Supprimer le binaire :
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Supprimer les données (optionnel) :
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Ma licence a expiré. Que se passe-t-il ?

PRECC revient au niveau Community. Toutes les fonctionnalités de base continuent de fonctionner :

- Les compétences intégrées restent actives
- Le pipeline du hook fonctionne normalement
- `precc savings` affiche la vue résumée
- `precc ingest` et l'exploration de sessions fonctionnent

Les fonctionnalités Pro deviennent indisponibles jusqu'au renouvellement :

- `precc savings --all` (ventilation détaillée)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Rapports par email

## Le hook ne semble pas fonctionner. Comment déboguer ?

??faq_debug_a_intro??

1. Vérifiez que le hook est enregistré :
   ```bash
   precc init
   ```

2. Testez le hook manuellement :
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Vérifiez que le binaire est dans votre PATH :
   ```bash
   which precc-hook
   ```

4. Vérifiez la configuration du hook de Claude Code dans `~/.claude/settings.json`.

## PRECC ralentit-il Claude Code ?

Non. Le hook se termine en moins de 5 millisecondes (p99). C'est imperceptible par rapport au temps que Claude passe à raisonner et générer des réponses.

## Puis-je utiliser PRECC en CI/CD ?

PRECC est conçu pour les sessions interactives de Claude Code. En CI/CD, il n'y a pas d'instance Claude Code à laquelle se connecter. Cependant, `precc gha` peut analyser les exécutions échouées de GitHub Actions depuis n'importe quel environnement.

## En quoi les compétences découvertes diffèrent-elles des compétences intégrées ?

Les compétences intégrées sont livrées avec PRECC et couvrent les erreurs de répertoire courantes. Les compétences découvertes sont apprises de vos journaux de session spécifiques -- elles capturent des modèles uniques à votre flux de travail. Les deux sont stockées dans SQLite et évaluées de manière identique par le pipeline du hook.

## Puis-je partager des compétences avec mon équipe ?

Oui. Exportez n'importe quelle compétence en TOML avec `precc skills export NAME` et partagez le fichier. Les membres de l'équipe peuvent le placer dans leur répertoire `skills/` ou l'importer dans leur base de données heuristique.
