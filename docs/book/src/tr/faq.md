# SSS

## PRECC kullanımı güvenli mi?

Evet. PRECC, Claude Code'un resmi PreToolUse hook mekanizmasını kullanır -- Anthropic'in tam olarak bu amaç için tasarladığı genişletme noktası. Hook:

- Tamamen çevrimdışı çalışır (kritik yolda ağ çağrısı yok)
- 5 milisaniyenin altında tamamlanır
- Fail-open'dır: bir şeyler ters giderse, orijinal komut değiştirilmeden çalışır
- Yalnızca komutları değiştirir, asla kendisi çalıştırmaz
- Verileri yerel olarak SQLite veritabanlarında depolar

## PRECC diğer yapay zeka kodlama araçlarıyla çalışır mı?

PRECC özellikle Claude Code için tasarlanmıştır. Claude Code'un sağladığı PreToolUse hook protokolüne dayanır. Cursor, Copilot, Windsurf veya diğer yapay zeka kodlama araçlarıyla çalışmaz.

## Telemetri hangi verileri gönderir?

Telemetri yalnızca katılım bazlıdır. Etkinleştirildiğinde şunları gönderir:

- PRECC sürümü, işletim sistemi ve mimari
- Toplu sayımlar (yakalanan komutlar, etkinleştirilen beceriler)
- Ortalama hook gecikmesi

Komut metni, dosya yolları, proje adları veya kişisel tanımlayıcı bilgi göndermez. Katılmadan önce `precc telemetry preview` ile tam yükü önizleyebilirsiniz. Ayrıntılar için [Telemetri](telemetry.md) sayfasına bakın.

## PRECC'yi nasıl kaldırırım?

??faq_uninstall_a_intro??

1. Hook kaydını kaldırın:
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. İkili dosyayı kaldırın:
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. Verileri kaldırın (isteğe bağlı):
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## Lisansım süresi doldu. Ne olur?

PRECC, Community katmanına döner. Tüm temel işlevsellik çalışmaya devam eder:

- Yerleşik beceriler aktif kalır
- Hook pipeline normal çalışır
- `precc savings` özet görünümü gösterir
- `precc ingest` ve oturum madenciliği çalışır

Pro özellikleri yenileyene kadar kullanılamaz:

- `precc savings --all` (ayrıntılı döküm)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-posta raporları

## Hook çalışmıyor gibi görünüyor. Nasıl hata ayıklama yapabilirim?

??faq_debug_a_intro??

1. Hook'un kayıtlı olduğunu kontrol edin:
   ```bash
   precc init
   ```

2. Hook'u manuel olarak test edin:
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. İkili dosyanın PATH'inizde olduğunu kontrol edin:
   ```bash
   which precc-hook
   ```

4. `~/.claude/settings.json` dosyasındaki Claude Code hook yapılandırmasını kontrol edin.

## PRECC, Claude Code'u yavaşlatır mı?

Hayır. Hook 5 milisaniyenin altında (p99) tamamlanır. Bu, Claude'un akıl yürütme ve yanıt oluşturma süresine kıyasla fark edilemez.

## PRECC'yi CI/CD'de kullanabilir miyim?

PRECC, etkileşimli Claude Code oturumları için tasarlanmıştır. CI/CD'de bağlanılacak bir Claude Code örneği yoktur. Ancak `precc gha` herhangi bir ortamdan başarısız GitHub Actions çalıştırmalarını analiz edebilir.

## Keşfedilen beceriler yerleşik becerilerden nasıl farklıdır?

Yerleşik beceriler PRECC ile birlikte gelir ve yaygın yanlış dizin kalıplarını kapsar. Keşfedilen beceriler, belirli oturum günlüklerinizden öğrenilir -- iş akışınıza özgü kalıpları yakalar. Her ikisi de SQLite'da depolanır ve hook pipeline tarafından aynı şekilde değerlendirilir.

## Becerileri ekibimle paylaşabilir miyim?

Evet. Herhangi bir beceriyi `precc skills export NAME` ile TOML'a aktarın ve dosyayı paylaşın. Ekip üyeleri `skills/` dizinlerine yerleştirebilir veya heuristik veritabanlarına aktarabilir.
