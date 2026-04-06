# Instalacja

## Szybka instalacja (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash
```

To pobiera najnowszy plik binarny wydania dla Twojej platformy, weryfikuje sumę kontrolną SHA256 i umieszcza go w `~/.local/bin/`.

Po instalacji zainicjalizuj PRECC:

```bash
precc init
```

`precc init` rejestruje hook PreToolUse w Claude Code, tworzy katalogi danych i inicjalizuje bazę umiejętności.

## Opcje instalacji

### Weryfikacja SHA256

Domyślnie instalator weryfikuje sumę kontrolną pliku binarnego względem opublikowanej sumy SHA256. Aby pominąć weryfikację (niezalecane):

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --no-verify
```

### Niestandardowy prefiks instalacji

Zainstaluj w niestandardowej lokalizacji:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --prefix /opt/precc
```

### Narzędzia towarzyszące (--extras)

PRECC jest dostarczany z opcjonalnymi narzędziami towarzyszącymi. Zainstaluj je z `--extras`:

```bash
curl -fsSL https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.sh | bash -s -- --extras
```

To instaluje:

| Narzędzie | Cel |
|------|---------|
| **RTK** | Zestaw narzędzi do przepisywania poleceń |
| **lean-ctx** | Kompresja kontekstu dla plików CLAUDE.md i prompt |
| **nushell** | Strukturalny shell do zaawansowanych potoków |
| **cocoindex-code** | Indeksowanie kodu dla szybszego rozwiązywania kontekstu |

## Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/peria-ai/precc-cc/main/scripts/install.ps1 | iex
```

Następnie zainicjalizuj:

```powershell
precc init
```

## Instalacja ręczna

1. Pobierz binarny plik wydania dla swojej platformy z [GitHub Releases](https://github.com/peria-ai/precc-cc/releases).
2. Zweryfikuj sumę kontrolną SHA256 względem pliku `.sha256` w wydaniu.
3. Umieść plik binarny w katalogu w swoim `PATH` (np. `~/.local/bin/`).
4. Uruchom `precc init`.

## Aktualizacja

```bash
precc update
```

Wymuś aktualizację do konkretnej wersji:

```bash
precc update --force --version 0.3.0
```

Włącz automatyczne aktualizacje:

```bash
precc update --auto
```

## Weryfikacja instalacji

```bash
$ precc --version
precc 0.3.0

$ precc savings
Session savings: 0 tokens (no commands intercepted yet)
```

Jeśli `precc` nie został znaleziony, upewnij się, że `~/.local/bin` jest w Twoim `PATH`.
