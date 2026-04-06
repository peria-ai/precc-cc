# Úvod

## Co je PRECC?

PRECC (Prediktivní korekce chyb pro Claude Code) je nástroj v Rustu, který zachytává bash příkazy Claude Code přes oficiální mechanismus PreToolUse hook. Opravuje chyby *dříve, než nastanou*, šetří tokeny a eliminuje smyčky opakování.

Zdarma pro uživatele komunity.

## Problém

Claude Code plýtvá významnými tokeny na předejitelné chyby:

- **Chyby špatného adresáře** -- Spuštění `cargo build` v nadřazeném adresáři bez `Cargo.toml`, poté opakování po přečtení chyby.
- **Smyčky opakování** -- Neúspěšný příkaz vytváří rozvláčný výstup, Claude ho čte, uvažuje a opakuje. Každý cyklus spálí stovky tokenů.
- **Rozvláčný výstup** -- Příkazy jako `find` nebo `ls -R` vypíšou tisíce řádků, které Claude musí zpracovat.

## Čtyři pilíře

### Korekce kontextu (cd-prepend)

Detekuje, když příkazy jako `cargo build` nebo `npm test` běží ve špatném adresáři a předřadí `cd /correct/path &&` před spuštění.

### Ladění GDB

Detekuje příležitosti pro připojení GDB pro hlubší ladění segfaultů a pádů, poskytuje strukturované debug informace místo surových core dump.

### Analýza relací

Analyzuje logy relací Claude Code pro páry chyba-oprava. Když se stejná chyba opakuje, PRECC už zná opravu a aplikuje ji automaticky.

### Automatizační dovednosti

Knihovna vestavěných a naučených dovedností, které porovnávají vzory příkazů a přepisují je. Dovednosti jsou definovány jako TOML soubory nebo SQLite řádky, což je činí snadno kontrolovatelnými, editovatelnými a sdílitelnými.

## Jak to funguje (30sekundová verze)

1. Claude Code se chystá spustit bash příkaz.
2. PreToolUse hook pošle příkaz do `precc-hook` jako JSON na stdin.
3. `precc-hook` spustí příkaz přes pipeline (dovednosti, korekce adresáře, komprese) za méně než 3 milisekundy.
4. Opravený příkaz je vrácen jako JSON na stdout.
5. Claude Code vykoná opravený příkaz.

Claude nikdy nevidí chybu. Nula zbytečných tokenů.

### Adaptivní komprese

Pokud příkaz selže po kompresi, PRECC automaticky přeskočí kompresi při opakování, aby Claude dostal plný nekomprimovaný výstup pro ladění.

## Živé statistiky využití

Aktuální verze <span data-stat="current_version">--</span>:

| Metrika | Hodnota |
|---|---|
| Vyvolání hooku | <span data-stat="total_invocations">--</span> |
| Ušetřené tokeny | <span data-stat="total_tokens_saved">--</span> |
| Poměr úspor | <span data-stat="saving_pct">--</span>% |
| RTK přepisy | <span data-stat="rtk_rewrites">--</span> |
| CD korekce | <span data-stat="cd_prepends">--</span> |
| Latence hooku | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Uživatelé | <span data-stat="unique_users">--</span> |

### Úspory podle verze

<table id="version-breakdown" style="display:none">
<thead><tr><th>Verze</th><th>Uživatelé</th><th>Vyvolání hooku</th><th>Ušetřené tokeny</th><th>Poměr úspor</th></tr></thead>
<tbody><tr><td colspan="5"><em>Načítání...</em></td></tr></tbody>
</table>

<small>Čísla jsou odhady. Každé předejité selhání se vyhne úplnému cyklu opakování: chybový výstup, uvažování modelu a příkaz opakování. Tato čísla se automaticky aktualizují z anonymizované telemetrie.</small>

## Odkazy

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Webové stránky: [https://peria.ai](https://peria.ai)
- Dokumentace: [https://precc.cc](https://precc.cc)
