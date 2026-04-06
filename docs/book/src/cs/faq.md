# Časté otázky

## Je PRECC bezpečný k použití?

Ano. PRECC používá oficiální mechanismus PreToolUse hook Claude Code -- stejný rozšiřovací bod, který Anthropic navrhl přesně pro tento účel. Hook:

- Běží zcela offline (žádná síťová volání na kritické cestě)
- Dokončí se za méně než 5 milisekund
- Je fail-open: pokud se něco pokazí, původní příkaz se spustí beze změny
- Pouze modifikuje příkazy, nikdy je sám nevykonává
- Ukládá data lokálně v databázích SQLite

## Funguje PRECC s jinými AI kódovacími nástroji?

PRECC je navržen specificky pro Claude Code. Spoléhá na protokol PreToolUse hook, který Claude Code poskytuje. Nefunguje s Cursor, Copilot, Windsurf ani jinými AI kódovacími nástroji.

## Jaká data telemetrie odesílá?

Telemetrie je pouze opt-in. Když je povolena, odesílá:

- Verzi PRECC, OS a architekturu
- Agregované počty (zachycené příkazy, aktivované dovednosti)
- Průměrnou latenci hooku

**Neodesílá** text příkazů, cesty k souborům, názvy projektů ani žádné osobní identifikační údaje. Přesná data si můžete prohlédnout pomocí `precc telemetry preview` před přihlášením. Viz [Telemetry](telemetry.md) pro kompletní podrobnosti.

## Jak odinstaluji PRECC?

??faq_uninstall_a_intro??

1. Odstraňte registraci hooku:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Odstraňte binární soubor:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Odstraňte data (volitelné):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Moje licence vypršela. Co se stane?

PRECC se vrátí na úroveň Community. Veškerá základní funkčnost pokračuje:

- Vestavěné dovednosti zůstávají aktivní
- Hook pipeline běží normálně
- `precc savings` zobrazuje souhrnný přehled
- `precc ingest` a analýza relací fungují

Funkce Pro se stanou nedostupnými do obnovení:

- `precc savings --all` (podrobný rozpis)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-mailové zprávy

## Hook se nezdá být aktivní. Jak ladím?

??faq_debug_a_intro??

1. Ověřte, že hook je zaregistrovaný:
   ```bash
   precc init
   ```

2. Otestujte hook ručně:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Ověřte, že binární soubor je ve vašem PATH:
   ```bash
   which precc-hook
   ```

4. Zkontrolujte konfiguraci hooku Claude Code v `~/.claude/settings.json`.

## Zpomaluje PRECC Claude Code?

Ne. Hook se dokončí za méně než 5 milisekund (p99). To je nepostřehnutelné ve srovnání s časem, který Claude stráví uvažováním a generováním odpovědí.

## Mohu použít PRECC v CI/CD?

PRECC je navržen pro interaktivní relace Claude Code. V CI/CD není žádná instance Claude Code, ke které by se připojil. Nicméně `precc gha` může analyzovat neúspěšné běhy GitHub Actions z jakéhokoli prostředí.

## Jak se liší naučené dovednosti od vestavěných?

Vestavěné dovednosti jsou dodávány s PRECC a pokrývají běžné vzory špatného adresáře. Naučené dovednosti jsou extrahovány z logů vašich konkrétních relací -- zachycují vzory unikátní pro váš pracovní postup. Obě jsou uloženy v SQLite a vyhodnocovány identicky hook pipeline.

## Mohu sdílet dovednosti s týmem?

Ano. Exportujte jakoukoli dovednost do TOML pomocí `precc skills export NAME` a sdílejte soubor. Členové týmu ho mohou umístit do svého adresáře `skills/` nebo importovat do své databáze heuristik.
