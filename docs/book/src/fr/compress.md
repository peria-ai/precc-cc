# Compression

`precc compress` réduit CLAUDE.md et d'autres fichiers de contexte pour diminuer l'utilisation de tokens lorsque Claude Code les charge. C'est une fonctionnalité Pro.

## Utilisation de base

```bash
$ precc compress .
[precc] Scanning directory: .
[precc] Found 3 context files:
         CLAUDE.md (2,847 tokens -> 1,203 tokens, -57.7%)
         ARCHITECTURE.md (4,112 tokens -> 2,044 tokens, -50.3%)
         ALTERNATIVES.md (3,891 tokens -> 1,967 tokens, -49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
[precc] Files compressed. Use --revert to restore originals.
```

## Exécution à blanc

Aperçu des modifications sans modifier les fichiers :

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Restauration

Les originaux sont sauvegardés automatiquement. Pour les restaurer :

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Ce qui est compressé

Le compresseur applique plusieurs transformations :

- Supprime les espaces et lignes vides redondants
- Raccourcit les formulations verbales tout en préservant le sens
- Condense les tableaux et listes
- Supprime les commentaires et le formatage décoratif
- Préserve tous les blocs de code, chemins et identifiants techniques

La sortie compressée est toujours lisible par un humain -- elle n'est ni minifiée ni obfusquée.

## Cibler des fichiers spécifiques

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
