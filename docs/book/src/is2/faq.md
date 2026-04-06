# Algengar spurningar

## Er PRECC öruggt í notkun?

Já. PRECC notar opinbera Claude Code PreToolUse hook-kerfi -- sama viðbótarstað og Anthropic hannaði einmitt í þessum tilgangi. Hook:

- Keyrir algjörlega ónettengd (engin netköll á skiptu leiðinni)
- Lýkur á undir 5 millisekúndum
- Er fail-open: ef eitthvað fer úrskeiðis keyrir upprunalega skipunin óbreytt
- Breytir aðeins skipunum, keyrir þær aldrei sjálf
- Geymir gögn staðbundið í SQLite-gagnagrunnum

## Virkar PRECC með öðrum AI-forritunartólum?

PRECC er hannað sérstaklega fyrir Claude Code. Það byggir á PreToolUse hook samskiptareglum sem Claude Code veitir. Það virkar ekki með Cursor, Copilot, Windsurf eða öðrum AI-forritunartólum.

## Hvaða gögn sendir fjarmælingin?

Fjarmæling er aðeins á opt-in grundvelli. Þegar virkjuð sendir hún:

- PRECC útgáfu, stýrikerfi og arkitektúr
- Samanlögð teljari (fangaðar skipanir, virkjuð þekking)
- Meðal hook-leynd

Hún sendir **ekki** skipunatexta, skráarslóðir, verkefnaheiti eða persónugreinanlegar upplýsingar. Þú getur forskoðað nákvæmlega hvaða gögn væru send með `precc telemetry preview` áður en þú skráir þig. Sjá [Telemetry](telemetry.md) fyrir frekari upplýsingar.

## Hvernig fjarlægi ég PRECC?

??faq_uninstall_a_intro??

1. Fjarlægðu hook-skráninguna:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Fjarlægðu keyrsluskrána:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Fjarlægðu gögn (valfrjálst):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Leyfið mitt er útrunnið. Hvað gerist?

PRECC snýr aftur í Community-stig. Öll grunnvirkni heldur áfram að virka:

- Innbyggð þekking helst virk
- Hook pipeline keyrir eðlilega
- `precc savings` sýnir yfirlitsyfirlitið
- `precc ingest` og lotugreining virka

Pro-eiginleikar verða ekki aðgengilegir þar til þú endurnýjar:

- `precc savings --all` (ítarleg sundurliðun)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Tölvupóstsskýrslur

## Hook virðist ekki vera í gangi. Hvernig kemst ég að villunni?

??faq_debug_a_intro??

1. Athugaðu hvort hook sé skráð:
   ```bash
   precc init
   ```

2. Prófaðu hook handvirkt:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Athugaðu hvort keyrsluskráin sé í PATH:
   ```bash
   which precc-hook
   ```

4. Athugaðu hook-stillingar Claude Code í `~/.claude/settings.json`.

## Hægir PRECC á Claude Code?

Nei. Hook lýkur á undir 5 millisekúndum (p99). Þetta er ógreinanlegt miðað við þann tíma sem Claude ver í rökhugsun og svaragerð.

## Get ég notað PRECC í CI/CD?

PRECC er hannað fyrir gagnvirkar Claude Code lotur. Í CI/CD er engin Claude Code tilvik til að tengja við. Hins vegar getur `precc gha` greint misheppnaðar GitHub Actions keyrslur úr hvaða umhverfi sem er.

## Hvernig er lærð þekking frábrugðin innbyggðri?

Innbyggð þekking fylgir PRECC og nær yfir algeng röng-möppu mynstur. Lærð þekking er unnin úr þínum sérstöku lotuskrám -- hún fangar mynstur einstök fyrir þitt verkflæði. Bæði er geymt í SQLite og metið eins af hook pipeline.

## Get ég deilt þekkingu með teyminu mínu?

Já. Flyttu út hvaða þekkingu sem er í TOML með `precc skills export NAME` og deildu skránni. Liðsfélagar geta sett hana í sína `skills/` möppu eða flutt inn í eigin þekkingargagnagrunn.
