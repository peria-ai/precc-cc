# Analyse GitHub Actions

`precc gha` analyse les exécutions GitHub Actions échouées et suggère des corrections. C'est une fonctionnalité Pro.

## Utilisation

Passez l'URL d'une exécution GitHub Actions échouée :

```bash
$ precc gha https://github.com/myorg/myrepo/actions/runs/12345678
[precc] Fetching run 12345678...
[precc] Run: CI / build (ubuntu-latest)
[precc] Status: failure
[precc] Failed step: Run cargo test

[precc] Log analysis:
  Error: test result: FAILED. 2 passed; 1 failed
  Failed test: tests::integration::test_database_connection
  Cause: thread 'tests::integration::test_database_connection' panicked at
         'called Result::unwrap() on an Err value: Connection refused'

[precc] Suggested fix:
  The test requires a database connection but the CI environment does not
  start a database service. Add a services block to your workflow:

    services:
      postgres:
        image: postgres:15
        ports:
          - 5432:5432
        env:
          POSTGRES_PASSWORD: test
```

## Ce qu'il fait

1. Analyse l'URL d'exécution GitHub Actions pour extraire le propriétaire, le dépôt et l'ID d'exécution.
2. Récupère les journaux d'exécution via l'API GitHub (utilise `GITHUB_TOKEN` si défini, sinon accès public).
3. Identifie l'étape échouée et extrait les lignes d'erreur pertinentes.
4. Analyse l'erreur et suggère une correction basée sur les modèles courants d'échec CI.

## Modèles d'échec pris en charge

- Conteneurs de services manquants (bases de données, Redis, etc.)
- Système d'exploitation ou architecture du runner incorrects
- Variables d'environnement ou secrets manquants
- Échecs d'installation de dépendances
- Délais d'attente des tests
- Erreurs de permissions
- Manques de cache causant des builds lents
