# Introducere

## Ce este PRECC?

PRECC (Corecție predictivă a erorilor pentru Claude Code) este un instrument Rust care interceptează comenzile bash ale Claude Code prin mecanismul oficial PreToolUse hook. Corectează erorile *înainte să se întâmple*, economisind tokeni și eliminând buclele de reîncercare.

Gratuit pentru utilizatorii comunității.

## Problema

Claude Code irosește tokeni semnificativi pe greșeli ce pot fi prevenite:

- **Erori de director greșit** -- Rularea `cargo build` într-un director părinte fără `Cargo.toml`, apoi reîncercare după citirea erorii.
- **Bucle de reîncercare** -- O comandă eșuată produce ieșire verbosă, Claude o citește, raționează și reîncearcă. Fiecare ciclu consumă sute de tokeni.
- **Ieșire verbosă** -- Comenzi precum `find` sau `ls -R` afișează mii de linii pe care Claude trebuie să le proceseze.

## Cei patru piloni

### Corecție context (cd-prepend)

Detectează când comenzi precum `cargo build` sau `npm test` rulează în directorul greșit și adaugă `cd /correct/path &&` înainte de execuție.

### Depanare GDB

Detectează oportunități de a atașa GDB pentru depanare mai profundă a segfault-urilor și prăbușirilor, furnizând informații de depanare structurate în loc de core dump-uri brute.

### Analiza sesiunilor

Analizează jurnalele sesiunilor Claude Code pentru perechi eroare-corecție. Când aceeași greșeală reapare, PRECC cunoaște deja corecția și o aplică automat.

### Abilități de automatizare

O bibliotecă de abilități integrate și învățate care se potrivesc cu tiparele comenzilor și le rescriu. Abilitățile sunt definite ca fișiere TOML sau rânduri SQLite, făcându-le ușor de inspectat, editat și partajat.

## Cum funcționează (versiunea de 30 de secunde)

1. Claude Code urmează să ruleze o comandă bash.
2. Hook-ul PreToolUse trimite comanda la `precc-hook` ca JSON pe stdin.
3. `precc-hook` rulează comanda prin pipeline (abilități, corecție director, comprimare) în mai puțin de 3 milisecunde.
4. Comanda corectată este returnată ca JSON pe stdout.
5. Claude Code execută comanda corectată în loc.

Claude nu vede niciodată eroarea. Zero tokeni irosiți.

### Comprimare adaptivă

Dacă o comandă eșuează după comprimare, PRECC omite automat comprimarea la reîncercare, astfel încât Claude primește ieșirea completă necomprimată pentru depanare.

## Statistici de utilizare în timp real

Versiunea curentă <span data-stat="current_version">--</span>:

| Metrică | Valoare |
|---|---|
| Invocări hook | <span data-stat="total_invocations">--</span> |
| Tokeni economisiți | <span data-stat="total_tokens_saved">--</span> |
| Raport economii | <span data-stat="saving_pct">--</span>% |
| Rescrieri RTK | <span data-stat="rtk_rewrites">--</span> |
| Corecții CD | <span data-stat="cd_prepends">--</span> |
| Latență hook | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Utilizatori | <span data-stat="unique_users">--</span> |

### Economii pe versiune

<table id="version-breakdown" style="display:none">
<thead><tr><th>Versiune</th><th>Utilizatori</th><th>Invocări hook</th><th>Tokeni economisiți</th><th>Raport economii</th></tr></thead>
<tbody><tr><td colspan="5"><em>Se încarcă...</em></td></tr></tbody>
</table>

<small>Cifrele sunt estimări. Fiecare eșec prevenit evită un ciclu complet de reîncercare: ieșire eroare, raționament model și comandă de reîncercare. Aceste numere se actualizează automat din telemetria anonimizată.</small>

## Linkuri

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Site web: [https://peria.ai](https://peria.ai)
- Documentație: [https://precc.cc](https://precc.cc)
