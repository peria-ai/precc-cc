# GYIK

## Biztonságos a PRECC használata?

Igen. A PRECC a Claude Code hivatalos PreToolUse hook mechanizmusát használja -- ugyanazt a bővítési pontot, amelyet az Anthropic pontosan erre a célra tervezett. A hook:

- Teljesen offline fut (nincs hálózati hívás a kritikus útvonalon)
- 5 ezredmásodperc alatt befejeződik
- Fail-open: ha bármi rosszul megy, az eredeti parancs módosítatlanul fut
- Csak módosítja a parancsokat, soha nem hajtja végre őket
- Adatokat helyben SQLite adatbázisokban tárol

## Működik a PRECC más AI kódolóeszközökkel?

A PRECC kifejezetten a Claude Code-hoz készült. A Claude Code által biztosított PreToolUse hook protokollra támaszkodik. Nem működik a Cursor, Copilot, Windsurf vagy más AI kódolóeszközökkel.

## Milyen adatokat küld a telemetria?

A telemetria csak opt-in. Ha engedélyezve van, elküldi:

- PRECC verzió, operációs rendszer és architektúra
- Összesített számok (elfogott parancsok, aktivált képességek)
- Átlagos hook késleltetés

**Nem** küld parancsszöveget, fájlútvonalakat, projektneveket vagy személyazonosításra alkalmas információt. A pontos adatcsomagot megtekintheti a `precc telemetry preview` paranccsal. Lásd [Telemetria](telemetry.md) a részletekért.

## Hogyan távolítom el a PRECC-et?

??faq_uninstall_a_intro??

1. Hook regisztráció eltávolítása:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. Bináris fájl eltávolítása:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Adatok eltávolítása (opcionális):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Lejárt a licencem. Mi történik?

A PRECC visszatér a Community szintre. Minden alapfunkció tovább működik:

- A beépített képességek aktívak maradnak
- A hook pipeline normálisan fut
- A `precc savings` az összefoglaló nézetet mutatja
- A `precc ingest` és a munkamenet bányászat működik

A Pro funkciók a megújításig nem elérhetők:

- `precc savings --all` (részletes bontás)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-mail jelentések

## A hook nem tűnik futónak. Hogyan debugolok?

??faq_debug_a_intro??

1. Ellenőrizze, hogy a hook regisztrálva van-e:
   ```bash
   precc init
   ```

2. Tesztelje a hookot manuálisan:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. Ellenőrizze, hogy a bináris fájl a PATH-ban van-e:
   ```bash
   which precc-hook
   ```

4. Ellenőrizze a Claude Code hook konfigurációját a `~/.claude/settings.json` fájlban.

## Lassítja a PRECC a Claude Code-ot?

Nem. A hook 5 ezredmásodperc alatt (p99) befejeződik. Ez érzékelhetetlen ahhoz képest, mennyi időt tölt Claude a gondolkodással és a válaszok generálásával.

## Használhatom a PRECC-et CI/CD-ben?

A PRECC interaktív Claude Code munkamenetekhez készült. CI/CD-ben nincs Claude Code példány, amihez csatlakozni lehetne. Azonban a `precc gha` bármilyen környezetből elemezheti a sikertelen GitHub Actions futásokat.

## Miben különböznek a bányászott képességek a beépítettektől?

A beépített képességek a PRECC-el érkeznek és a gyakori rossz könyvtár mintákat fedik le. A bányászott képességek az Ön munkamenet-naplóiból tanultak -- az Ön munkafolyamatára jellemző mintákat rögzítik. Mindkettő SQLite-ban van tárolva és azonos módon értékeli a hook pipeline.

## Megoszthatom a képességeket a csapatommal?

Igen. Exportáljon bármilyen képességet TOML-ba a `precc skills export NAME` paranccsal és ossza meg a fájlt. A csapattagok elhelyezhetik a `skills/` könyvtárukban vagy importálhatják a heurisztika adatbázisukba.
