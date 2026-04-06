# FAQ

## PRECC è sicuro da usare?

Sì. PRECC utilizza il meccanismo ufficiale PreToolUse hook di Claude Code -- lo stesso punto di estensione che Anthropic ha progettato esattamente per questo scopo. L'hook:

- Funziona interamente offline (nessuna chiamata di rete nel percorso critico)
- Si completa in meno di 5 millisecondi
- È fail-open: se qualcosa va storto, il comando originale viene eseguito senza modifiche
- Modifica solo i comandi, non li esegue mai direttamente
- Memorizza i dati localmente in database SQLite

## PRECC funziona con altri strumenti di coding AI?

PRECC è progettato specificamente per Claude Code. Si basa sul protocollo PreToolUse hook fornito da Claude Code. Non funziona con Cursor, Copilot, Windsurf o altri strumenti di coding AI.

## Quali dati invia la telemetria?

La telemetria è solo opt-in. Quando abilitata, invia:

- Versione PRECC, sistema operativo e architettura
- Conteggi aggregati (comandi intercettati, skill attivate)
- Latenza media dell'hook

Non invia **mai** testo dei comandi, percorsi di file, nomi di progetto o informazioni personali identificabili. Puoi visualizzare l'anteprima del payload esatto con `precc telemetry preview` prima di aderire. Vedi [Telemetry](telemetry.md) per tutti i dettagli.

## Come disinstallo PRECC?

??faq_uninstall_a_intro??

1. Rimuovi la registrazione dell'hook:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Rimuovi il binario:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Rimuovi i dati (opzionale):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## La mia licenza è scaduta. Cosa succede?

PRECC torna al livello Community. Tutte le funzionalità principali continuano a funzionare:

- Le skill integrate restano attive
- La hook pipeline funziona normalmente
- `precc savings` mostra la vista riepilogativa
- `precc ingest` e il mining delle sessioni funzionano

Le funzionalità Pro diventano non disponibili fino al rinnovo:

- `precc savings --all` (analisi dettagliata)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Report via email

## L'hook non sembra essere in esecuzione. Come faccio il debug?

??faq_debug_a_intro??

1. Verifica che l'hook sia registrato:
   ```bash
   precc init
   ```

2. Testa l'hook manualmente:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Verifica che il binario sia nel tuo PATH:
   ```bash
   which precc-hook
   ```

4. Controlla la configurazione dell'hook di Claude Code in `~/.claude/settings.json`.

## PRECC rallenta Claude Code?

No. L'hook si completa in meno di 5 millisecondi (p99). Questo è impercettibile rispetto al tempo che Claude impiega per ragionare e generare risposte.

## Posso usare PRECC in CI/CD?

PRECC è progettato per sessioni interattive di Claude Code. In CI/CD, non c'è un'istanza di Claude Code a cui agganciarsi. Tuttavia, `precc gha` può analizzare le esecuzioni fallite di GitHub Actions da qualsiasi ambiente.

## In cosa differiscono le skill apprese da quelle integrate?

Le skill integrate vengono distribuite con PRECC e coprono i pattern comuni di directory errata. Le skill apprese vengono estratte dai log delle tue sessioni specifiche -- catturano pattern unici del tuo flusso di lavoro. Entrambe sono memorizzate in SQLite e valutate in modo identico dalla hook pipeline.

## Posso condividere le skill con il mio team?

Sì. Esporta qualsiasi skill in TOML con `precc skills export NAME` e condividi il file. I membri del team possono posizionarlo nella loro directory `skills/` o importarlo nel loro database di euristiche.
