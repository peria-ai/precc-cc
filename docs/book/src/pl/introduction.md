# Wprowadzenie

## Czym jest PRECC?

PRECC (Predykcyjna korekcja błędów dla Claude Code) to narzędzie Rust, które przechwytuje polecenia bash Claude Code za pomocą oficjalnego mechanizmu hooków PreToolUse. Naprawia błędy *zanim się pojawią*, oszczędzając tokeny i eliminując pętle ponawiania.

Bezpłatnie dla użytkowników społeczności.

## Problem

Claude Code marnuje znaczną liczbę tokenów na możliwe do uniknięcia błędy:

- **Błędy katalogu** -- Uruchomienie `cargo build` w katalogu nadrzędnym bez `Cargo.toml`, a następnie ponowna próba po przeczytaniu błędu.
- **Pętle ponawiania** -- Nieudane polecenie generuje szczegółowe wyjście, Claude je czyta, analizuje i ponawia próbę.
- **Szczegółowe wyjście** -- Polecenia takie jak `find` lub `ls -R` generują tysiące linii, które Claude musi przetworzyć.

## Cztery filary

### Korekta kontekstu (cd-prepend)

Wykrywa, gdy polecenia takie jak `cargo build` lub `npm test` są uruchamiane w złym katalogu i dodaje `cd /correct/path &&` przed wykonaniem.

### Debugowanie GDB

Wykrywa możliwości podpięcia GDB do głębszego debugowania segfaultów i awarii, dostarczając ustrukturyzowane informacje debugowania.

### Eksploracja sesji

Przeszukuje logi sesji Claude Code w poszukiwaniu par awaria-naprawa. Gdy ten sam błąd się powtarza, PRECC już zna poprawkę i stosuje ją automatycznie.

### Umiejętności automatyzacji

Biblioteka wbudowanych i wydobytych umiejętności dopasowujących wzorce poleceń i je przepisujących. Umiejętności definiowane są jako pliki TOML lub wiersze SQLite.

## Jak to działa (wersja 30-sekundowa)

1. Claude Code zamierza uruchomić polecenie bash.
2. Hook PreToolUse wysyła polecenie do `precc-hook` jako JSON na stdin.
3. `precc-hook` przetwarza polecenie przez potok (umiejętności, korekta katalogu, kompresja) w mniej niż 3 milisekundy.
4. Poprawione polecenie jest zwracane jako JSON na stdout.
5. Claude Code wykonuje poprawione polecenie.

Claude nigdy nie widzi błędu. Żadne tokeny nie są marnowane.

### Kompresja adaptacyjna

Jeśli polecenie nie powiedzie się po kompresji, PRECC automatycznie pomija kompresję przy ponownej próbie, aby Claude otrzymał pełne nieskompresowane wyjście do debugowania.

## Statystyki użycia na żywo

Aktualna wersja <span data-stat="current_version">--</span>:

| Metryka | Wartość |
|---|---|
| Wywołania hooka | <span data-stat="total_invocations">--</span> |
| Zaoszczędzone tokeny | <span data-stat="total_tokens_saved">--</span> |
| Współczynnik oszczędności | <span data-stat="saving_pct">--</span>% |
| Przepisania RTK | <span data-stat="rtk_rewrites">--</span> |
| Korekty CD | <span data-stat="cd_prepends">--</span> |
| Opóźnienie hooka | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| Użytkownicy | <span data-stat="unique_users">--</span> |

### Measured Savings (Ground Truth)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>Metryka</th><th>Wartość</th></tr></thead>
<tbody>
<tr><td>Original output tokens (without PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Actual output tokens (with PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>Zaoszczędzone tokeny</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>Współczynnik oszczędności</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Ground-truth measurements</td><td><span data-measured="ground_truth_count">--</span> measurements</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### By Rewrite Type

<table id="rewrite-type-table">
<thead><tr><th>Type</th><th>Count</th><th>Avg Savings %</th><th>Zaoszczędzone tokeny</th></tr></thead>
<tbody><tr><td colspan="4"><em>Ładowanie...</em></td></tr></tbody>
</table>
</div>

### Oszczędności wg wersji

<table id="version-breakdown" style="display:none">
<thead><tr><th>Wersja</th><th>Użytkownicy</th><th>Wywołania hooka</th><th>Zaoszczędzone tokeny</th><th>Współczynnik oszczędności</th></tr></thead>
<tbody><tr><td colspan="5"><em>Ładowanie...</em></td></tr></tbody>
</table>

<small>Te liczby aktualizują się automatycznie z zanonimizowanej telemetrii.</small>

## Linki

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- Strona: [https://peria.ai](https://peria.ai)
- Dokumentacja: [https://precc.cc](https://precc.cc)
