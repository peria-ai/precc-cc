# Comprimir

`precc compress` reduz CLAUDE.md e outros arquivos de contexto para diminuir o uso de tokens quando Claude Code os carrega. Este é um recurso Pro.

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

## Execução de teste

Visualize o que mudaria sem modificar arquivos:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Reversão

Os originais são salvos automaticamente. Para restaurá-los:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## O que é comprimido

O compressor aplica várias transformações:

- Remove espaços em branco e linhas vazias redundantes
- Encurta fraseado verboso preservando o significado
- Condensa tabelas e listas
- Remove comentários e formatação decorativa
- Preserva todos os blocos de código, caminhos e identificadores técnicos

A saída comprimida ainda é legível por humanos -- não é minificada nem ofuscada.

## Direcionar arquivos específicos

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
