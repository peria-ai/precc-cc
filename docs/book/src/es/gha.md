# Análisis de GitHub Actions

`precc gha` analiza ejecuciones fallidas de GitHub Actions y sugiere correcciones. Esta es una función Pro.

## Uso

Pase la URL de una ejecución fallida de GitHub Actions:

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

## Qué hace

1. Analiza la URL de ejecución de GitHub Actions para extraer el propietario, repositorio e ID de ejecución.
2. Obtiene los registros de ejecución a través de la API de GitHub (usa `GITHUB_TOKEN` si está configurado, de lo contrario acceso público).
3. Identifica el paso fallido y extrae las líneas de error relevantes.
4. Analiza el error y sugiere una corrección basada en patrones comunes de fallos de CI.

## Patrones de fallo soportados

- Contenedores de servicio faltantes (bases de datos, Redis, etc.)
- SO o arquitectura del ejecutor incorrectos
- Variables de entorno o secretos faltantes
- Fallos de instalación de dependencias
- Tiempos de espera de pruebas
- Errores de permisos
- Fallos de caché que causan compilaciones lentas
