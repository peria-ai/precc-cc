# Analysointi

PRECC analysoi Claude Coden istuntolokeja oppiakseen virhe-korjaus-malleja. Kun se näkee saman virheen uudelleen, se soveltaa korjauksen automaattisesti.

## Istuntojen lokien lataaminen

### Lataa yksittäinen tiedosto

```bash
$ precc ingest ~/.claude/logs/session-2026-04-03.jsonl
[precc] Parsing session-2026-04-03.jsonl...
[precc] Found 142 commands, 8 failure-fix pairs
[precc] Stored 8 patterns in history.db
[precc] 2 new skill candidates identified
```

### Lataa kaikki lokit

```bash
$ precc ingest --all
[precc] Scanning ~/.claude/logs/...
[precc] Found 23 session files (14 new, 9 already ingested)
[precc] Parsing 14 new files...
[precc] Found 47 failure-fix pairs across 14 sessions
[precc] Stored 47 patterns in history.db
[precc] 5 new skill candidates identified
```

### Pakotettu uudelleenlataus

Jo ladattujen tiedostojen uudelleenkäsittely:

```bash
$ precc ingest --all --force
[precc] Re-ingesting all 23 session files...
```

## Miten analysointi toimii

1. PRECC lukee istunnon JSONL-lokitiedoston.
2. Se tunnistaa komentoparit, joissa ensimmäinen komento epäonnistui ja toinen oli korjattu uudelleenyritys.
3. Se poimii mallin (mikä meni pieleen) ja korjauksen (mitä Claude teki toisin).
4. Mallit tallennetaan tiedostoon `~/.local/share/precc/history.db`.
5. Kun malli saavuttaa luottamuskynnyksen (nähty useita kertoja), siitä tulee opittu taito tiedostossa `heuristics.db`.

### Esimerkkimalli

```
Failure: pytest tests/test_auth.py
Error:   ModuleNotFoundError: No module named 'myapp'
Fix:     cd /home/user/myapp && pytest tests/test_auth.py
Pattern: pytest outside project root -> prepend cd
```

## precc-learner-daemon

`precc-learner`-daemon toimii taustalla ja seuraa automaattisesti uusia istuntolokeja:

```bash
$ precc-learner &
[precc-learner] Watching ~/.claude/logs/ for new sessions...
[precc-learner] Processing session-2026-04-03-1412.jsonl... 3 new patterns
```

Daemon käyttää tiedostojärjestelmäilmoituksia (inotify Linuxissa, FSEvents macOS:ssä) joten se reagoi välittömästi istunnon päättyessä.

## Malleista taidoiksi

Opitut mallit ylennetään taidoiksi kun ne täyttävät nämä kriteerit:

- Nähty vähintään 3 kertaa eri istunnoissa
- Johdonmukainen korjausmalli (sama korjaustyyppi joka kerta)
- Ei vääriä hälytyksiä havaittu

Voit tarkastella taitoehdokkaita:

```bash
$ precc skills advise
```

Katso [Skills](skills.md) taitojen hallinnan yksityiskohdat.

## Tietojen tallennus

- **Virhe-korjaus-parit**: `~/.local/share/precc/history.db`
- **Ylennetyt taidot**: `~/.local/share/precc/heuristics.db`

Molemmat ovat SQLite-tietokantoja WAL-tilassa turvallista samanaikaista pääsyä varten.
