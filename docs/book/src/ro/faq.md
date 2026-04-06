# Întrebări frecvente

## Este PRECC sigur de utilizat?

Da. PRECC folosește mecanismul oficial PreToolUse hook al Claude Code -- același punct de extensie pe care Anthropic l-a proiectat exact pentru acest scop. Hook-ul:

- Rulează complet offline (fără apeluri de rețea pe calea critică)
- Se finalizează în mai puțin de 5 milisecunde
- Este fail-open: dacă ceva merge greșit, comanda originală rulează nemodificată
- Modifică doar comenzi, nu le execută niciodată singur
- Stochează datele local în baze de date SQLite

## Funcționează PRECC cu alte instrumente de codare AI?

PRECC este proiectat specific pentru Claude Code. Se bazează pe protocolul PreToolUse hook pe care Claude Code îl furnizează. Nu funcționează cu Cursor, Copilot, Windsurf sau alte instrumente de codare AI.

## Ce date trimite telemetria?

Telemetria este doar opt-in. Când este activată, trimite:

- Versiunea PRECC, SO și arhitectura
- Numărători agregate (comenzi interceptate, abilități activate)
- Latența medie a hook-ului

Nu trimite **niciodată** text de comenzi, căi de fișiere, nume de proiecte sau informații personale identificabile. Puteți previzualiza exact datele cu `precc telemetry preview` înainte de aderare. Vedeți [Telemetry](telemetry.md) pentru detalii complete.

## Cum dezinstalez PRECC?

??faq_uninstall_a_intro??

1. Eliminați înregistrarea hook-ului:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Eliminați binarul:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Eliminați datele (opțional):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Licența mea a expirat. Ce se întâmplă?

PRECC revine la nivelul Community. Toată funcționalitatea de bază continuă să funcționeze:

- Abilitățile integrate rămân active
- Hook pipeline rulează normal
- `precc savings` arată vizualizarea sumar
- `precc ingest` și analiza sesiunilor funcționează

Funcțiile Pro devin indisponibile până la reînnoire:

- `precc savings --all` (detaliere completă)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Rapoarte prin e-mail

## Hook-ul nu pare să ruleze. Cum fac depanarea?

??faq_debug_a_intro??

1. Verificați că hook-ul este înregistrat:
   ```bash
   precc init
   ```

2. Testați hook-ul manual:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Verificați că binarul este în PATH:
   ```bash
   which precc-hook
   ```

4. Verificați configurația hook-ului Claude Code în `~/.claude/settings.json`.

## PRECC încetinește Claude Code?

Nu. Hook-ul se finalizează în mai puțin de 5 milisecunde (p99). Aceasta este imperceptibil comparativ cu timpul pe care Claude îl petrece raționând și generând răspunsuri.

## Pot folosi PRECC în CI/CD?

PRECC este proiectat pentru sesiuni interactive Claude Code. În CI/CD, nu există o instanță Claude Code la care să se conecteze. Cu toate acestea, `precc gha` poate analiza rulările eșuate de GitHub Actions din orice mediu.

## Cum diferă abilitățile învățate de cele integrate?

Abilitățile integrate vin cu PRECC și acoperă tiparele comune de director greșit. Abilitățile învățate sunt extrase din jurnalele sesiunilor dvs. specifice -- captează tipare unice fluxului dvs. de lucru. Ambele sunt stocate în SQLite și evaluate identic de hook pipeline.

## Pot partaja abilități cu echipa mea?

Da. Exportați orice abilitate în TOML cu `precc skills export NAME` și partajați fișierul. Membrii echipei pot să-l plaseze în directorul lor `skills/` sau să-l importe în baza lor de euristici.
