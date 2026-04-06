# Þjöppun

`precc compress` minnkar CLAUDE.md og aðrar samhengsskrár til að draga úr táknanotkuninni þegar Claude Code hleður þeim. Þetta er Pro-eiginleiki.

## Grunnnotkun

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

## Prufukeyrsla

Forskoða hvað myndi breytast án þess að breyta skrám:

```bash
$ precc compress . --dry-run
[precc] Dry run -- no files will be modified.
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
[precc] ARCHITECTURE.md: 4,112 tokens -> 2,044 tokens (-50.3%)
[precc] ALTERNATIVES.md: 3,891 tokens -> 1,967 tokens (-49.5%)
[precc] Total: 10,850 tokens -> 5,214 tokens (-51.9%)
```

## Endurheimt

Upprunaleg afrit eru vistuð sjálfkrafa. Til að endurheimta:

```bash
$ precc compress --revert
[precc] Restored 3 files from backups.
```

## Hvað er þjappað

Þjappari beitir nokkrum umbreytingum:

- Fjarlægir óþarfa bilstafi og tómar línur
- Styttir orðmarga framsetningu en varðveitir merkingu
- Þéttir töflur og lista
- Fjarlægir athugasemdir og skreytilega sniðun
- Varðveitir alla kóðaflokka, slóðir og tæknilega auðkenna

Þjappaða úttakið er enn mannlæsilegt -- það er ekki smættað né þokukennt.

## Ákveðnar skrár

```bash
$ precc compress CLAUDE.md
[precc] CLAUDE.md: 2,847 tokens -> 1,203 tokens (-57.7%)
```
