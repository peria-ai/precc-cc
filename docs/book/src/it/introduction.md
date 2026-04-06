# Introduzione

## Cos'è PRECC?

PRECC (Correzione predittiva degli errori per Claude Code) è uno strumento Rust che intercetta i comandi bash di Claude Code tramite il meccanismo ufficiale PreToolUse hook. Corregge gli errori *prima che accadano*, risparmiando token ed eliminando i cicli di retry.

Gratuito per sempre per gli utenti community.

## Il problema

Claude Code spreca una quantità significativa di token per errori prevenibili:

- **Errori di directory errata** -- Esecuzione di `cargo build` in una directory padre che non ha `Cargo.toml`, poi nuovo tentativo dopo aver letto l'errore.
- **Cicli di retry** -- Un comando fallito produce output verbose, Claude lo legge, ragiona e ritenta. Ogni ciclo brucia centinaia di token.
- **Output verbose** -- Comandi come `find` o `ls -R` producono migliaia di righe che Claude deve elaborare.

## I quattro pilastri

### Correzione contesto (cd-prepend)

Rileva quando comandi come `cargo build` o `npm test` vengono eseguiti nella directory errata e prepone `cd /correct/path &&` prima dell'esecuzione.

### Debug GDB

Rileva opportunità per collegare GDB per un debug più approfondito di segfault e crash, fornendo informazioni di debug strutturate invece di core dump grezzi.

### Mining delle sessioni

Analizza i log delle sessioni di Claude Code alla ricerca di coppie errore-correzione. Quando lo stesso errore si ripresenta, PRECC conosce già la correzione e la applica automaticamente.

### Skill di automazione

Una libreria di skill integrate e apprese che corrispondono a pattern di comandi e li riscrivono. Le skill sono definite come file TOML o righe SQLite, rendendole facili da ispezionare, modificare e condividere.

## Come funziona (versione da 30 secondi)

1. Claude Code sta per eseguire un comando bash.
2. Il PreToolUse hook invia il comando a `precc-hook` come JSON su stdin.
3. `precc-hook` esegue il comando attraverso la pipeline (skill, correzione directory, compressione) in meno di 3 millisecondi.
4. Il comando corretto viene restituito come JSON su stdout.
5. Claude Code esegue il comando corretto invece dell'originale.

Claude non vede mai l'errore. Zero token sprecati.

### Compressione adattiva

Se un comando fallisce dopo la compressione, PRECC salta automaticamente la compressione al tentativo successivo, così Claude riceve l'output completo non compresso per il debug.

## Statistiche d'uso in tempo reale

Versione attuale <span data-stat="current_version">--</span>:

| Metrica | Valore |
|---|---|
| Invocazioni hook | <span data-stat="total_invocations">--</span> |
| Token risparmiati | <span data-stat="total_tokens_saved">--</span> |
| Rapporto di risparmio | <span data-stat="saving_pct">--</span>% |
| Riscritture RTK | <span data-stat="rtk_rewrites">--</span> |
| Correzioni CD | <span data-stat="cd_prepends">--</span> |
| Latenza hook | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Utenti | <span data-stat="unique_users">--</span> |

### Risparmi per versione

<table id="version-breakdown" style="display:none">
<thead><tr><th>Versione</th><th>Utenti</th><th>Invocazioni hook</th><th>Token risparmiati</th><th>Rapporto di risparmio</th></tr></thead>
<tbody><tr><td colspan="5"><em>Caricamento...</em></td></tr></tbody>
</table>

<small>Le cifre sono stime. Ogni fallimento prevenuto evita un ciclo completo di retry: output dell'errore, ragionamento del modello e comando di retry. Questi numeri si aggiornano automaticamente dalla telemetria anonimizzata.</small>

## Link

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Sito web: [https://peria.ai](https://peria.ai)
- Documentazione: [https://precc.cc](https://precc.cc)
