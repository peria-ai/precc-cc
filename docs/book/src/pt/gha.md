# Análise do GitHub Actions

`precc gha` analisa execuções falhas do GitHub Actions e sugere correções. Este é um recurso Pro.

## Uso

Passe a URL de uma execução falha do GitHub Actions:

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

## O que faz

1. Analisa a URL de execução do GitHub Actions para extrair o proprietário, repositório e ID de execução.
2. Busca os logs de execução via API do GitHub (usa `GITHUB_TOKEN` se definido, caso contrário acesso público).
3. Identifica o passo que falhou e extrai as linhas de erro relevantes.
4. Analisa o erro e sugere uma correção baseada em padrões comuns de falhas de CI.

## Padrões de falha suportados

- Containers de serviço ausentes (bancos de dados, Redis, etc.)
- SO ou arquitetura do runner incorretos
- Variáveis de ambiente ou secrets ausentes
- Falhas na instalação de dependências
- Timeouts de testes
- Erros de permissão
- Cache misses causando builds lentos
