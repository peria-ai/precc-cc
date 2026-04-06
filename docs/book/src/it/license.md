# Licenza

PRECC offre due livelli: Community (gratuito) e Pro.

## Livello Community (gratuito)

Il livello Community include:

- Tutte le skill integrate (correzione directory errata, traduzione jj, ecc.)
- Hook pipeline con supporto completo Pillar 1 e Pillar 4
- Riepilogo base di `precc savings`
- Mining delle sessioni con `precc ingest`
- Uso locale illimitato

## Livello Pro

Pro sblocca funzionalità aggiuntive:

- **Analisi dettagliata dei risparmi** -- `precc savings --all` con analisi per singolo comando
- **Registrazione GIF** -- `precc gif` per creare GIF animate del terminale
- **Conformità IP geofence** -- Per ambienti regolamentati
- **Report via email** -- `precc mail report` per inviare analitiche
- **Analisi GitHub Actions** -- `precc gha` per il debug di workflow falliti
- **Compressione del contesto** -- `precc compress` per l'ottimizzazione di CLAUDE.md
- **Supporto prioritario**

## Attivazione di una licenza

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Verifica stato licenza

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Attivazione GitHub Sponsors

Se sponsorizzi PRECC tramite GitHub Sponsors, la tua licenza viene attivata automaticamente tramite la tua email GitHub. Nessuna chiave richiesta -- assicurati solo che la tua email sponsor corrisponda:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Impronta digitale del dispositivo

Ogni licenza è legata a un'impronta digitale del dispositivo. Visualizza la tua con:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Se hai bisogno di trasferire la tua licenza su una nuova macchina, prima disattiva:

```bash
precc license deactivate
```

Poi attiva sulla nuova macchina.

## Licenza scaduta?

Quando una licenza Pro scade, PRECC torna al livello Community. Tutte le skill integrate e le funzionalità principali continuano a funzionare. Solo le funzionalità specifiche Pro diventano non disponibili. Vedi le [FAQ](faq.md) per maggiori dettagli.
