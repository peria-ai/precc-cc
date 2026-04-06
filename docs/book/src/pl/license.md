# Licencja

PRECC oferuje dwa poziomy: Community (darmowy) i Pro.

## Poziom Community (darmowy)

Poziom Community obejmuje:

- Wszystkie wbudowane umiejętności (korekta katalogu, tłumaczenie jj itp.)
- Pipeline hooków z pełnym wsparciem Pillar 1 i Pillar 4
- Podstawowe podsumowanie `precc savings`
- Eksploracja sesji za pomocą `precc ingest`
- Nieograniczone lokalne użycie

## Poziom Pro

Pro odblokowuje dodatkowe funkcje:

- **Szczegółowy rozkład oszczędności** -- `precc savings --all` z analizą per polecenie
- **Nagrywanie GIF** -- `precc gif` do tworzenia animowanych GIF-ów terminala
- **Zgodność z geofence IP** -- Dla środowisk regulowanych
- **Raporty e-mail** -- `precc mail report` do wysyłania analiz
- **Analiza GitHub Actions** -- `precc gha` do debugowania nieudanych workflow
- **Kompresja kontekstu** -- `precc compress` do optymalizacji CLAUDE.md
- **Priorytetowe wsparcie**

## Aktywacja licencji

```bash
$ precc license activate XXXX-XXXX-XXXX-XXXX --email you@example.com
[precc] License activated for you@example.com
[precc] Plan: Pro
[precc] Expires: 2027-04-03
```

## Sprawdzanie statusu licencji

```bash
$ precc license status
License: Pro
Email:   you@example.com
Expires: 2027-04-03
Status:  Active
```

## Aktywacja GitHub Sponsors

Jeśli sponsorujesz PRECC przez GitHub Sponsors, Twoja licencja jest aktywowana automatycznie przez e-mail GitHub. Klucz nie jest wymagany -- upewnij się tylko, że e-mail sponsora pasuje:

```bash
$ precc license status
License: Pro (GitHub Sponsors)
Email:   you@example.com
Status:  Active (auto-renewed)
```

## Odcisk urządzenia

Każda licencja jest powiązana z odciskiem urządzenia. Sprawdź swój za pomocą:

```bash
$ precc license fingerprint
Fingerprint: a1b2c3d4e5f6...
```

Jeśli musisz przenieść licencję na nową maszynę, najpierw dezaktywuj:

```bash
precc license deactivate
```

Następnie aktywuj na nowej maszynie.

## Licencja wygasła?

Gdy licencja Pro wygaśnie, PRECC wraca do poziomu Community. Wszystkie wbudowane umiejętności i podstawowe funkcje nadal działają. Tylko funkcje specyficzne dla Pro stają się niedostępne. Zobacz [FAQ](faq.md), aby uzyskać więcej szczegółów.
