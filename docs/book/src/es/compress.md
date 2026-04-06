# Comprimir

`precc compress` reduce CLAUDE.md y otros archivos de contexto para disminuir el uso de tokens cuando Claude Code los carga. Esta es una función Pro.

## Uso básico

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

## Ejecución de prueba

Vista previa de lo que cambiaría sin modificar archivos:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Reversión

Los originales se respaldan automáticamente. Para restaurarlos:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Qué se comprime

El compresor aplica varias transformaciones:

- Elimina espacios en blanco redundantes y líneas vacías
- Acorta frases verbosas preservando el significado
- Condensa tablas y listas
- Elimina comentarios y formato decorativo
- Preserva todos los bloques de código, rutas e identificadores técnicos

La salida comprimida sigue siendo legible para humanos -- no está minificada ni ofuscada.

## Apuntar a archivos específicos

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
