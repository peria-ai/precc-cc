-- FAQ chapter strings
-- Languages: en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_title',
    'FAQ',
    'Preguntas frecuentes',
    'FAQ',
    '常见问题',
    'FAQ',
    'Perguntas frequentes',
    'よくある質問',
    'Câu hỏi thường gặp',
    'Veelgestelde vragen',
    'GYIK',
    'الأسئلة الشائعة',
    'سوالات متداول',
    'SSS',
    '자주 묻는 질문',
    'คำถามที่พบบ่อย',
    'မေးလေ့ရှိသော မေးခွန်းများ',
    'Түгээмэл асуултууд',
    '',
    'FAQ');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_safe_q',
    'Is PRECC safe to use?',
    '¿Es seguro usar PRECC?',
    'Ist PRECC sicher?',
    'PRECC安全吗？',
    'PRECC est-il sûr à utiliser ?',
    'O PRECC é seguro?',
    'PRECCは安全ですか？',
    'PRECC có an toàn không?',
    'Is PRECC veilig om te gebruiken?',
    'Biztonságos a PRECC használata?',
    'هل PRECC آمن للاستخدام؟',
    'آیا PRECC برای استفاده امن است؟',
    'PRECC kullanımı güvenli mi?',
    'PRECC는 안전한가요?',
    'PRECC ปลอดภัยไหม?',
    'PRECC ကို သုံးရန် လုံခြုံပါသလား?',
    'PRECC ашиглахад аюулгүй юу?',
    '',
    'Czy PRECC jest bezpieczny?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_safe_a',
    'Yes. PRECC uses the official Claude Code PreToolUse hook mechanism -- the same extension point that Anthropic designed for exactly this purpose. The hook:

- Runs entirely offline (no network calls in the hot path)
- Completes in under 5 milliseconds
- Is fail-open: if anything goes wrong, the original command runs unmodified
- Only modifies commands, never executes them itself
- Stores data locally in SQLite databases',
    'Sí. PRECC usa el mecanismo oficial de hooks PreToolUse de Claude Code -- el mismo punto de extensión que Anthropic diseñó exactamente para este propósito. El hook:

- Se ejecuta completamente sin conexión (sin llamadas de red en la ruta crítica)
- Se completa en menos de 5 milisegundos
- Es fail-open: si algo sale mal, el comando original se ejecuta sin modificar
- Solo modifica comandos, nunca los ejecuta por sí mismo
- Almacena datos localmente en bases de datos SQLite',
    'Ja. PRECC verwendet den offiziellen PreToolUse-Hook-Mechanismus von Claude Code -- denselben Erweiterungspunkt, den Anthropic genau für diesen Zweck entwickelt hat. Der Hook:

- Läuft vollständig offline (keine Netzwerkaufrufe im Hot Path)
- Wird in unter 5 Millisekunden abgeschlossen
- Ist fail-open: bei Problemen wird der ursprüngliche Befehl unverändert ausgeführt
- Ändert nur Befehle, führt sie nie selbst aus
- Speichert Daten lokal in SQLite-Datenbanken',
    '是的。PRECC使用Claude Code官方的PreToolUse钩子机制——Anthropic专门为此目的设计的扩展点。该钩子：

- 完全离线运行（热路径中无网络调用）
- 在5毫秒内完成
- 是fail-open的：如果出现任何问题，原始命令将不受修改地运行
- 只修改命令，从不自己执行它们
- 将数据存储在本地SQLite数据库中',
    'Oui. PRECC utilise le mécanisme officiel de hooks PreToolUse de Claude Code -- le même point d''extension qu''Anthropic a conçu exactement à cet effet. Le hook :

- Fonctionne entièrement hors ligne (pas d''appels réseau dans le chemin critique)
- Se termine en moins de 5 millisecondes
- Est fail-open : en cas de problème, la commande originale s''exécute sans modification
- Ne fait que modifier les commandes, ne les exécute jamais lui-même
- Stocke les données localement dans des bases SQLite',
    'Sim. PRECC usa o mecanismo oficial de hooks PreToolUse do Claude Code -- o mesmo ponto de extensão que a Anthropic projetou exatamente para esse propósito. O hook:

- Roda inteiramente offline (sem chamadas de rede no caminho crítico)
- Completa em menos de 5 milissegundos
- É fail-open: se algo der errado, o comando original roda sem modificação
- Apenas modifica comandos, nunca os executa
- Armazena dados localmente em bancos SQLite',
    'はい。PRECCはClaude Code公式のPreToolUseフックメカニズムを使用しています。Anthropicがまさにこの目的のために設計した拡張ポイントです。フックは：

- 完全にオフラインで動作（ホットパスでのネットワーク呼び出しなし）
- 5ミリ秒未満で完了
- フェイルオープン：問題が発生した場合、元のコマンドがそのまま実行される
- コマンドを変更するだけで、自ら実行することはない
- データはローカルのSQLiteデータベースに保存',
    'Có. PRECC sử dụng cơ chế hook PreToolUse chính thức của Claude Code -- cùng điểm mở rộng mà Anthropic thiết kế cho mục đích này. Hook:

- Chạy hoàn toàn ngoại tuyến (không có lệnh gọi mạng trong đường dẫn nóng)
- Hoàn thành trong dưới 5 mili giây
- Là fail-open: nếu có lỗi, lệnh gốc chạy không thay đổi
- Chỉ sửa đổi lệnh, không bao giờ tự thực thi
- Lưu trữ dữ liệu cục bộ trong cơ sở dữ liệu SQLite',
    'Ja. PRECC gebruikt het officiële Claude Code PreToolUse hook-mechanisme -- hetzelfde uitbreidingspunt dat Anthropic precies hiervoor heeft ontworpen. De hook:

- Draait volledig offline (geen netwerkoproepen in het kritieke pad)
- Voltooit in minder dan 5 milliseconden
- Is fail-open: als er iets misgaat, wordt het oorspronkelijke commando ongewijzigd uitgevoerd
- Wijzigt alleen commando''s, voert ze nooit zelf uit
- Slaat gegevens lokaal op in SQLite-databases',
    'Igen. A PRECC a Claude Code hivatalos PreToolUse hook mechanizmusát használja -- ugyanazt a bővítési pontot, amelyet az Anthropic pontosan erre a célra tervezett. A hook:

- Teljesen offline fut (nincs hálózati hívás a kritikus útvonalon)
- 5 ezredmásodperc alatt befejeződik
- Fail-open: ha bármi rosszul megy, az eredeti parancs módosítatlanul fut
- Csak módosítja a parancsokat, soha nem hajtja végre őket
- Adatokat helyben SQLite adatbázisokban tárol',
    'نعم. يستخدم PRECC آلية hooks PreToolUse الرسمية لـ Claude Code -- نفس نقطة التمديد التي صممتها Anthropic لهذا الغرض بالضبط. الـ hook:

- يعمل بالكامل دون اتصال (لا استدعاءات شبكة في المسار الحرج)
- يكتمل في أقل من 5 مللي ثانية
- هو fail-open: إذا حدث خطأ، يتم تنفيذ الأمر الأصلي دون تعديل
- يعدل الأوامر فقط، ولا ينفذها أبداً بنفسه
- يخزن البيانات محلياً في قواعد بيانات SQLite',
    'بله. PRECC از مکانیزم رسمی hook PreToolUse کد Claude استفاده می‌کند -- همان نقطه توسعه‌ای که Anthropic دقیقاً برای این منظور طراحی کرده است. هوک:

- کاملاً آفلاین اجرا می‌شود (بدون فراخوانی شبکه در مسیر حساس)
- در کمتر از ۵ میلی‌ثانیه تکمیل می‌شود
- fail-open است: اگر مشکلی پیش بیاید، دستور اصلی بدون تغییر اجرا می‌شود
- فقط دستورات را تغییر می‌دهد، هرگز خودش اجرا نمی‌کند
- داده‌ها را به صورت محلی در پایگاه داده SQLite ذخیره می‌کند',
    'Evet. PRECC, Claude Code''un resmi PreToolUse hook mekanizmasını kullanır -- Anthropic''in tam olarak bu amaç için tasarladığı genişletme noktası. Hook:

- Tamamen çevrimdışı çalışır (kritik yolda ağ çağrısı yok)
- 5 milisaniyenin altında tamamlanır
- Fail-open''dır: bir şeyler ters giderse, orijinal komut değiştirilmeden çalışır
- Yalnızca komutları değiştirir, asla kendisi çalıştırmaz
- Verileri yerel olarak SQLite veritabanlarında depolar',
    '네. PRECC는 Claude Code의 공식 PreToolUse 훅 메커니즘을 사용합니다 -- Anthropic이 정확히 이 목적을 위해 설계한 확장 포인트입니다. 훅은:

- 완전히 오프라인으로 실행 (핫 패스에서 네트워크 호출 없음)
- 5밀리초 이내에 완료
- 페일 오픈: 문제가 발생하면 원래 명령이 수정 없이 실행
- 명령만 수정하고 직접 실행하지 않음
- 로컬 SQLite 데이터베이스에 데이터 저장',
    'ใช่ PRECC ใช้กลไก hook PreToolUse อย่างเป็นทางการของ Claude Code -- จุดขยายเดียวกันที่ Anthropic ออกแบบมาเพื่อจุดประสงค์นี้โดยเฉพาะ Hook:

- ทำงานแบบออฟไลน์ทั้งหมด (ไม่มีการเรียกเครือข่ายในเส้นทางหลัก)
- เสร็จสิ้นในเวลาน้อยกว่า 5 มิลลิวินาที
- เป็น fail-open: หากมีข้อผิดพลาด คำสั่งเดิมจะทำงานโดยไม่เปลี่ยนแปลง
- แก้ไขคำสั่งเท่านั้น ไม่เคยเรียกใช้เอง
- จัดเก็บข้อมูลในฐานข้อมูล SQLite ภายในเครื่อง',
    'ဟုတ်ကဲ့။ PRECC သည် Claude Code ၏ တရားဝင် PreToolUse hook ယန္တရားကို အသုံးပြုသည်။ Hook သည်:

- အင်တာနက်မလိုဘဲ အပြည့်အဝ လုပ်ဆောင်သည်
- 5 မီလီစက္ကန့်အတွင်း ပြီးစီးသည်
- Fail-open ဖြစ်သည်: ပြဿနာတစ်ခုခု ဖြစ်ပေါ်ပါက မူလ ကွန်မန်းသည် မပြောင်းလဲဘဲ လုပ်ဆောင်သည်
- ကွန်မန်းများကိုသာ ပြင်ဆင်ပြီး ကိုယ်တိုင် မလုပ်ဆောင်ပါ
- ဒေတာကို SQLite တွင် ဒေသတွင်း သိမ်းဆည်းသည်',
    'Тийм. PRECC нь Claude Code-ын албан ёсны PreToolUse hook механизмыг ашигладаг. Hook нь:

- Бүрэн офлайнаар ажилладаг (халуун замд сүлжээний дуудлага байхгүй)
- 5 миллисекундээс бага хугацаанд дуусдаг
- Fail-open: алдаа гарвал анхны команд өөрчлөгдөхгүй ажилладаг
- Зөвхөн командыг өөрчилдөг, хэзээ ч өөрөө гүйцэтгэдэггүй
- Мэдээллийг SQLite-д дотоод хадгалдаг',
    '',
    'Tak. PRECC używa oficjalnego mechanizmu hooków PreToolUse Claude Code -- tego samego punktu rozszerzenia, który Anthropic zaprojektował do tego celu. Hook:

- Działa całkowicie offline (brak wywołań sieciowych na ścieżce krytycznej)
- Kończy się w mniej niż 5 milisekund
- Jest fail-open: jeśli coś pójdzie nie tak, oryginalne polecenie uruchomi się bez zmian
- Tylko modyfikuje polecenia, nigdy ich sam nie wykonuje
- Przechowuje dane lokalnie w bazach danych SQLite');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_other_tools_q',
    'Does PRECC work with other AI coding tools?',
    '¿PRECC funciona con otras herramientas de codificación IA?',
    'Funktioniert PRECC mit anderen KI-Coding-Tools?',
    'PRECC能与其他AI编码工具一起使用吗？',
    'PRECC fonctionne-t-il avec d''autres outils de codage IA ?',
    'O PRECC funciona com outras ferramentas de codificação IA?',
    'PRECCは他のAIコーディングツールで動作しますか？',
    'PRECC có hoạt động với các công cụ lập trình AI khác không?',
    'Werkt PRECC met andere AI-coderingstools?',
    'Működik a PRECC más AI kódolóeszközökkel?',
    'هل يعمل PRECC مع أدوات البرمجة الذكية الأخرى؟',
    'آیا PRECC با سایر ابزارهای کدنویسی هوش مصنوعی کار می‌کند؟',
    'PRECC diğer yapay zeka kodlama araçlarıyla çalışır mı?',
    'PRECC는 다른 AI 코딩 도구와 호환되나요?',
    'PRECC ทำงานกับเครื่องมือเขียนโค้ด AI อื่นได้ไหม?',
    'PRECC သည် အခြား AI ကုဒ်ရေးသည့် ကိရိယာများနှင့် အလုပ်လုပ်ပါသလား?',
    'PRECC бусад AI кодчилолын хэрэгслүүдтэй ажилладаг уу?',
    '',
    'Czy PRECC działa z innymi narzędziami AI do kodowania?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_other_tools_a',
    'PRECC is designed specifically for Claude Code. It relies on the PreToolUse hook protocol that Claude Code provides. It does not work with Cursor, Copilot, Windsurf, or other AI coding tools.',
    'PRECC está diseñado específicamente para Claude Code. Depende del protocolo de hooks PreToolUse que proporciona Claude Code. No funciona con Cursor, Copilot, Windsurf u otras herramientas de codificación IA.',
    'PRECC ist speziell für Claude Code entwickelt. Es basiert auf dem PreToolUse-Hook-Protokoll, das Claude Code bereitstellt. Es funktioniert nicht mit Cursor, Copilot, Windsurf oder anderen KI-Coding-Tools.',
    'PRECC专为Claude Code设计。它依赖于Claude Code提供的PreToolUse钩子协议。它不适用于Cursor、Copilot、Windsurf或其他AI编码工具。',
    'PRECC est conçu spécifiquement pour Claude Code. Il s''appuie sur le protocole de hooks PreToolUse fourni par Claude Code. Il ne fonctionne pas avec Cursor, Copilot, Windsurf ou d''autres outils de codage IA.',
    'PRECC é projetado especificamente para o Claude Code. Ele depende do protocolo de hooks PreToolUse que o Claude Code fornece. Não funciona com Cursor, Copilot, Windsurf ou outras ferramentas de codificação IA.',
    'PRECCはClaude Code専用に設計されています。Claude Codeが提供するPreToolUseフックプロトコルに依存しています。Cursor、Copilot、Windsurf、その他のAIコーディングツールでは動作しません。',
    'PRECC được thiết kế riêng cho Claude Code. Nó phụ thuộc vào giao thức hook PreToolUse mà Claude Code cung cấp. Nó không hoạt động với Cursor, Copilot, Windsurf hoặc các công cụ lập trình AI khác.',
    'PRECC is specifiek ontworpen voor Claude Code. Het is afhankelijk van het PreToolUse hook-protocol dat Claude Code biedt. Het werkt niet met Cursor, Copilot, Windsurf of andere AI-coderingstools.',
    'A PRECC kifejezetten a Claude Code-hoz készült. A Claude Code által biztosított PreToolUse hook protokollra támaszkodik. Nem működik a Cursor, Copilot, Windsurf vagy más AI kódolóeszközökkel.',
    'تم تصميم PRECC خصيصاً لـ Claude Code. يعتمد على بروتوكول hooks PreToolUse الذي يوفره Claude Code. لا يعمل مع Cursor أو Copilot أو Windsurf أو أدوات البرمجة الذكية الأخرى.',
    'PRECC به طور خاص برای Claude Code طراحی شده است. به پروتکل hook PreToolUse که Claude Code ارائه می‌دهد متکی است. با Cursor، Copilot، Windsurf یا سایر ابزارهای کدنویسی هوش مصنوعی کار نمی‌کند.',
    'PRECC özellikle Claude Code için tasarlanmıştır. Claude Code''un sağladığı PreToolUse hook protokolüne dayanır. Cursor, Copilot, Windsurf veya diğer yapay zeka kodlama araçlarıyla çalışmaz.',
    'PRECC는 Claude Code 전용으로 설계되었습니다. Claude Code가 제공하는 PreToolUse 훅 프로토콜에 의존합니다. Cursor, Copilot, Windsurf 또는 다른 AI 코딩 도구와는 호환되지 않습니다.',
    'PRECC ออกแบบมาสำหรับ Claude Code โดยเฉพาะ มันอาศัยโปรโตคอล hook PreToolUse ที่ Claude Code มีให้ ไม่ทำงานกับ Cursor, Copilot, Windsurf หรือเครื่องมือเขียนโค้ด AI อื่น',
    'PRECC ကို Claude Code အတွက် အထူးသဖြင့် ဒီဇိုင်းထုတ်ထားသည်။ Cursor၊ Copilot၊ Windsurf သို့မဟုတ် အခြား AI ကုဒ်ရေးသည့် ကိရိယာများနှင့် အလုပ်မလုပ်ပါ။',
    'PRECC нь Claude Code-д зориулагдсан. Cursor, Copilot, Windsurf болон бусад AI кодчилолын хэрэгслүүдтэй ажилладаггүй.',
    '',
    'PRECC jest zaprojektowany specjalnie dla Claude Code. Opiera się na protokole hooków PreToolUse, który zapewnia Claude Code. Nie działa z Cursor, Copilot, Windsurf ani innymi narzędziami AI do kodowania.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_telemetry_q',
    'What data does telemetry send?',
    '¿Qué datos envía la telemetría?',
    'Welche Daten sendet die Telemetrie?',
    '遥测发送什么数据？',
    'Quelles données la télémétrie envoie-t-elle ?',
    'Quais dados a telemetria envia?',
    'テレメトリはどのようなデータを送信しますか？',
    'Đo lường từ xa gửi dữ liệu gì?',
    'Welke gegevens stuurt de telemetrie?',
    'Milyen adatokat küld a telemetria?',
    'ما البيانات التي يرسلها القياس عن بعد؟',
    'تله‌متری چه داده‌هایی ارسال می‌کند؟',
    'Telemetri hangi verileri gönderir?',
    '텔레메트리는 어떤 데이터를 전송하나요?',
    'การวัดระยะไกลส่งข้อมูลอะไร?',
    'အဝေးမှတိုင်းတာခြင်းက ဘာဒေတာ ပေးပို့သလဲ?',
    'Телеметр ямар мэдээлэл илгээдэг вэ?',
    '',
    'Jakie dane wysyła telemetria?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_telemetry_a',
    'Telemetry is opt-in only. When enabled, it sends:

- PRECC version, OS, and architecture
- Aggregate counts (commands intercepted, skills activated)
- Average hook latency

It does **not** send command text, file paths, project names, or any personally identifiable information. You can preview the exact payload with `precc telemetry preview` before opting in. See [Telemetry](telemetry.md) for full details.',
    'La telemetría es solo por suscripción. Cuando está habilitada, envía:

- Versión de PRECC, SO y arquitectura
- Conteos agregados (comandos interceptados, habilidades activadas)
- Latencia promedio del hook

**No** envía texto de comandos, rutas de archivos, nombres de proyectos ni información personal identificable. Puede previsualizar la carga exacta con `precc telemetry preview` antes de suscribirse. Vea [Telemetría](telemetry.md) para más detalles.',
    'Telemetrie ist nur Opt-in. Wenn aktiviert, sendet sie:

- PRECC-Version, Betriebssystem und Architektur
- Aggregierte Zähler (abgefangene Befehle, aktivierte Skills)
- Durchschnittliche Hook-Latenz

Sie sendet **keine** Befehlstexte, Dateipfade, Projektnamen oder persönlich identifizierbare Informationen. Sie können die genaue Nutzlast mit `precc telemetry preview` vor der Aktivierung ansehen. Siehe [Telemetrie](telemetry.md) für Details.',
    '遥测仅在选择加入后启用。启用后发送：

- PRECC版本、操作系统和架构
- 汇总计数（拦截的命令、激活的技能）
- 平均钩子延迟

它**不**发送命令文本、文件路径、项目名称或任何个人身份信息。您可以在选择加入前使用 `precc telemetry preview` 预览确切的数据。详见[遥测](telemetry.md)。',
    'La télémétrie est uniquement sur abonnement. Lorsqu''elle est activée, elle envoie :

- Version de PRECC, système d''exploitation et architecture
- Compteurs agrégés (commandes interceptées, compétences activées)
- Latence moyenne du hook

Elle n''envoie **pas** de texte de commande, de chemins de fichiers, de noms de projets ou d''informations personnellement identifiables. Vous pouvez prévisualiser la charge exacte avec `precc telemetry preview` avant de vous abonner. Voir [Télémétrie](telemetry.md) pour les détails.',
    'A telemetria é apenas opt-in. Quando ativada, envia:

- Versão do PRECC, SO e arquitetura
- Contagens agregadas (comandos interceptados, habilidades ativadas)
- Latência média do hook

**Não** envia texto de comandos, caminhos de arquivos, nomes de projetos ou informações pessoais identificáveis. Você pode visualizar a carga exata com `precc telemetry preview` antes de ativar. Veja [Telemetria](telemetry.md) para detalhes.',
    'テレメトリはオプトインのみです。有効にすると送信されるもの：

- PRECCバージョン、OS、アーキテクチャ
- 集計カウント（インターセプトされたコマンド、アクティブ化されたスキル）
- 平均フックレイテンシ

コマンドテキスト、ファイルパス、プロジェクト名、個人を特定できる情報は送信**しません**。オプトイン前に `precc telemetry preview` で正確なペイロードを確認できます。詳細は[テレメトリ](telemetry.md)を参照。',
    'Đo lường từ xa chỉ hoạt động khi bạn đồng ý. Khi được bật, nó gửi:

- Phiên bản PRECC, hệ điều hành và kiến trúc
- Số liệu tổng hợp (lệnh bị chặn, kỹ năng được kích hoạt)
- Độ trễ hook trung bình

Nó **không** gửi văn bản lệnh, đường dẫn tệp, tên dự án hoặc bất kỳ thông tin nhận dạng cá nhân nào. Bạn có thể xem trước dữ liệu chính xác với `precc telemetry preview` trước khi đồng ý. Xem [Đo lường từ xa](telemetry.md) để biết chi tiết.',
    'Telemetrie is alleen opt-in. Indien ingeschakeld, stuurt het:

- PRECC-versie, besturingssysteem en architectuur
- Geaggregeerde tellingen (onderschepte opdrachten, geactiveerde vaardigheden)
- Gemiddelde hook-latentie

Het stuurt **geen** opdrachttekst, bestandspaden, projectnamen of persoonlijk identificeerbare informatie. U kunt de exacte payload bekijken met `precc telemetry preview` voordat u zich aanmeldt. Zie [Telemetrie](telemetry.md) voor details.',
    'A telemetria csak opt-in. Ha engedélyezve van, elküldi:

- PRECC verzió, operációs rendszer és architektúra
- Összesített számok (elfogott parancsok, aktivált képességek)
- Átlagos hook késleltetés

**Nem** küld parancsszöveget, fájlútvonalakat, projektneveket vagy személyazonosításra alkalmas információt. A pontos adatcsomagot megtekintheti a `precc telemetry preview` paranccsal. Lásd [Telemetria](telemetry.md) a részletekért.',
    'القياس عن بعد اختياري فقط. عند تفعيله، يرسل:

- إصدار PRECC ونظام التشغيل والبنية
- أعداد مجمعة (الأوامر المعترضة، المهارات المفعلة)
- متوسط زمن الاستجابة

**لا** يرسل نصوص أوامر أو مسارات ملفات أو أسماء مشاريع أو أي معلومات تعريف شخصية. يمكنك معاينة البيانات بالضبط باستخدام `precc telemetry preview` قبل الاشتراك. انظر [القياس عن بعد](telemetry.md) للتفاصيل.',
    'تله‌متری فقط اختیاری است. وقتی فعال شود، ارسال می‌کند:

- نسخه PRECC، سیستم‌عامل و معماری
- شمارش‌های تجمیعی (دستورات رهگیری‌شده، مهارت‌های فعال‌شده)
- میانگین تأخیر هوک

متن دستور، مسیر فایل، نام پروژه یا اطلاعات شناسایی شخصی ارسال **نمی‌کند**. قبل از فعال‌سازی می‌توانید بار دقیق را با `precc telemetry preview` مشاهده کنید. برای جزئیات [تله‌متری](telemetry.md) را ببینید.',
    'Telemetri yalnızca katılım bazlıdır. Etkinleştirildiğinde şunları gönderir:

- PRECC sürümü, işletim sistemi ve mimari
- Toplu sayımlar (yakalanan komutlar, etkinleştirilen beceriler)
- Ortalama hook gecikmesi

Komut metni, dosya yolları, proje adları veya kişisel tanımlayıcı bilgi göndermez. Katılmadan önce `precc telemetry preview` ile tam yükü önizleyebilirsiniz. Ayrıntılar için [Telemetri](telemetry.md) sayfasına bakın.',
    '텔레메트리는 옵트인 방식입니다. 활성화하면 다음을 전송합니다:

- PRECC 버전, OS 및 아키텍처
- 집계 카운트 (가로챈 명령, 활성화된 스킬)
- 평균 훅 지연 시간

명령 텍스트, 파일 경로, 프로젝트 이름 또는 개인 식별 정보를 전송하지 **않습니다**. 옵트인 전에 `precc telemetry preview`로 정확한 페이로드를 미리 볼 수 있습니다. 자세한 내용은 [텔레메트리](telemetry.md)를 참조하세요.',
    'การวัดระยะไกลเป็นแบบเลือกเข้าร่วมเท่านั้น เมื่อเปิดใช้งานจะส่ง:

- เวอร์ชัน PRECC ระบบปฏิบัติการ และสถาปัตยกรรม
- จำนวนรวม (คำสั่งที่สกัด ทักษะที่เปิดใช้งาน)
- ความหน่วงเฉลี่ยของ hook

**ไม่**ส่งข้อความคำสั่ง เส้นทางไฟล์ ชื่อโปรเจกต์ หรือข้อมูลส่วนบุคคล คุณสามารถดูตัวอย่างข้อมูลที่แน่นอนด้วย `precc telemetry preview` ก่อนเลือกเข้าร่วม ดู[การวัดระยะไกล](telemetry.md)สำหรับรายละเอียด',
    'အဝေးမှတိုင်းတာခြင်းသည် ရွေးချယ်မှသာ ဖြစ်သည်။ ဖွင့်ထားသောအခါ ပေးပို့သည်:

- PRECC ဗားရှင်း၊ OS နှင့် ဗိသုကာ
- စုစုပေါင်း အရေအတွက်များ
- ပျမ်းမျှ hook ကြန့်ကြာချိန်

ကွန်မန်းစာသား၊ ဖိုင်လမ်းကြောင်းများ သို့မဟုတ် ကိုယ်ရေးကိုယ်တာ သတင်းအချက်အလက် ပေးပို့**ခြင်း မရှိပါ**။',
    'Телеметр зөвхөн сонголтоор идэвхждэг. Идэвхжүүлсэн үед илгээдэг:

- PRECC хувилбар, үйлдлийн систем, архитектур
- Нэгтгэсэн тоо (таслан зогсоосон командууд, идэвхжүүлсэн чадварууд)
- Дундаж hook хоцрогдол

Командын текст, файлын зам, төслийн нэр илгээ**дэггүй**.',
    '',
    'Telemetria jest tylko opt-in. Po włączeniu wysyła:

- Wersję PRECC, system operacyjny i architekturę
- Zagregowane liczby (przechwycone polecenia, aktywowane umiejętności)
- Średnie opóźnienie hooka

**Nie** wysyła tekstu poleceń, ścieżek plików, nazw projektów ani żadnych danych osobowych. Przed włączeniem możesz podejrzeć dokładne dane za pomocą `precc telemetry preview`. Szczegóły w [Telemetria](telemetry.md).');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_uninstall_q',
    'How do I uninstall PRECC?',
    '¿Cómo desinstalo PRECC?',
    'Wie deinstalliere ich PRECC?',
    '如何卸载PRECC？',
    'Comment désinstaller PRECC ?',
    'Como desinstalar o PRECC?',
    'PRECCをアンインストールするには？',
    'Làm thế nào để gỡ cài đặt PRECC?',
    'Hoe verwijder ik PRECC?',
    'Hogyan távolítom el a PRECC-et?',
    'كيف أقوم بإلغاء تثبيت PRECC؟',
    'چگونه PRECC را حذف کنم؟',
    'PRECC''yi nasıl kaldırırım?',
    'PRECC를 어떻게 제거하나요?',
    'ฉันจะถอนการติดตั้ง PRECC ได้อย่างไร?',
    'PRECC ကို ဘယ်လို ဖယ်ရှားမလဲ?',
    'PRECC-ийг хэрхэн устгах вэ?',
    '',
    'Jak odinstalować PRECC?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_uninstall_a_intro',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_uninstall_step1',
    'Remove the hook registration:',
    'Eliminar el registro del hook:',
    'Hook-Registrierung entfernen:',
    '移除钩子注册：',
    'Supprimer l''enregistrement du hook :',
    'Remover o registro do hook:',
    'フック登録を削除：',
    'Xóa đăng ký hook:',
    'Hook-registratie verwijderen:',
    'Hook regisztráció eltávolítása:',
    'إزالة تسجيل الـ hook:',
    'حذف ثبت هوک:',
    'Hook kaydını kaldırın:',
    '훅 등록 제거:',
    'ลบการลงทะเบียน hook:',
    'Hook မှတ်ပုံတင်ခြင်း ဖယ်ရှားရန်:',
    'Hook бүртгэлийг устгах:',
    '',
    'Usuń rejestrację hooka:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_uninstall_step2',
    'Remove the binary:',
    'Eliminar el binario:',
    'Binärdatei entfernen:',
    '删除二进制文件：',
    'Supprimer le binaire :',
    'Remover o binário:',
    'バイナリを削除：',
    'Xóa tệp nhị phân:',
    'Binair bestand verwijderen:',
    'Bináris fájl eltávolítása:',
    'إزالة الملف الثنائي:',
    'حذف فایل باینری:',
    'İkili dosyayı kaldırın:',
    '바이너리 제거:',
    'ลบไฟล์ไบนารี:',
    'Binary ဖိုင် ဖယ်ရှားရန်:',
    'Бинар файлыг устгах:',
    '',
    'Usuń plik binarny:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_uninstall_step3',
    'Remove data (optional):',
    'Eliminar datos (opcional):',
    'Daten entfernen (optional):',
    '删除数据（可选）：',
    'Supprimer les données (optionnel) :',
    'Remover dados (opcional):',
    'データを削除（任意）：',
    'Xóa dữ liệu (tùy chọn):',
    'Gegevens verwijderen (optioneel):',
    'Adatok eltávolítása (opcionális):',
    'إزالة البيانات (اختياري):',
    'حذف داده‌ها (اختیاری):',
    'Verileri kaldırın (isteğe bağlı):',
    '데이터 제거 (선택):',
    'ลบข้อมูล (ไม่บังคับ):',
    'ဒေတာ ဖယ်ရှားရန် (ရွေးချယ်နိုင်):',
    'Мэдээллийг устгах (заавал биш):',
    '',
    'Usuń dane (opcjonalne):');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_license_q',
    'My license expired. What happens?',
    'Mi licencia expiró. ¿Qué sucede?',
    'Meine Lizenz ist abgelaufen. Was passiert?',
    '我的许可证过期了。会发生什么？',
    'Ma licence a expiré. Que se passe-t-il ?',
    'Minha licença expirou. O que acontece?',
    'ライセンスが期限切れになりました。どうなりますか？',
    'Giấy phép của tôi đã hết hạn. Điều gì xảy ra?',
    'Mijn licentie is verlopen. Wat gebeurt er?',
    'Lejárt a licencem. Mi történik?',
    'انتهت صلاحية ترخيصي. ماذا يحدث؟',
    'مجوز من منقضی شده. چه اتفاقی می‌افتد؟',
    'Lisansım süresi doldu. Ne olur?',
    '라이선스가 만료되었습니다. 어떻게 되나요?',
    'ใบอนุญาตของฉันหมดอายุ เกิดอะไรขึ้น?',
    'ကျွန်ုပ်၏ လိုင်စင် သက်တမ်းကုန်သွားပါပြီ။ ဘာဖြစ်မလဲ?',
    'Миний лиценз дууссан. Юу болох вэ?',
    '',
    'Moja licencja wygasła. Co się stanie?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_license_a_community',
    'PRECC reverts to the Community tier. All core functionality continues to work:

- Built-in skills remain active
- Hook pipeline runs normally
- `precc savings` shows the summary view
- `precc ingest` and session mining work',
    'PRECC vuelve al nivel Community. Toda la funcionalidad principal sigue funcionando:

- Las habilidades integradas permanecen activas
- El pipeline del hook funciona normalmente
- `precc savings` muestra la vista resumida
- `precc ingest` y la minería de sesiones funcionan',
    'PRECC kehrt zum Community-Tier zurück. Alle Kernfunktionen funktionieren weiterhin:

- Eingebaute Skills bleiben aktiv
- Die Hook-Pipeline läuft normal
- `precc savings` zeigt die Zusammenfassung
- `precc ingest` und Session-Mining funktionieren',
    'PRECC恢复到社区版。所有核心功能继续正常工作：

- 内置技能保持活跃
- 钩子管道正常运行
- `precc savings` 显示摘要视图
- `precc ingest` 和会话挖掘正常工作',
    'PRECC revient au niveau Community. Toutes les fonctionnalités de base continuent de fonctionner :

- Les compétences intégrées restent actives
- Le pipeline du hook fonctionne normalement
- `precc savings` affiche la vue résumée
- `precc ingest` et l''exploration de sessions fonctionnent',
    'PRECC volta ao nível Community. Toda a funcionalidade principal continua funcionando:

- Habilidades integradas permanecem ativas
- O pipeline do hook funciona normalmente
- `precc savings` mostra a visão resumida
- `precc ingest` e mineração de sessões funcionam',
    'PRECCはCommunityティアに戻ります。すべてのコア機能は引き続き動作します：

- 組み込みスキルはアクティブのまま
- フックパイプラインは正常に動作
- `precc savings` はサマリービューを表示
- `precc ingest` とセッションマイニングは動作',
    'PRECC trở về tầng Community. Tất cả chức năng cốt lõi tiếp tục hoạt động:

- Các kỹ năng tích hợp vẫn hoạt động
- Pipeline hook chạy bình thường
- `precc savings` hiển thị chế độ xem tóm tắt
- `precc ingest` và khai thác phiên hoạt động',
    'PRECC keert terug naar het Community-niveau. Alle kernfunctionaliteit blijft werken:

- Ingebouwde vaardigheden blijven actief
- Hook-pipeline draait normaal
- `precc savings` toont de samenvattingsweergave
- `precc ingest` en sessiemining werken',
    'A PRECC visszatér a Community szintre. Minden alapfunkció tovább működik:

- A beépített képességek aktívak maradnak
- A hook pipeline normálisan fut
- A `precc savings` az összefoglaló nézetet mutatja
- A `precc ingest` és a munkamenet bányászat működik',
    'يعود PRECC إلى المستوى المجتمعي. جميع الوظائف الأساسية تستمر في العمل:

- المهارات المدمجة تبقى نشطة
- خط أنابيب الـ hook يعمل بشكل طبيعي
- `precc savings` يعرض العرض الملخص
- `precc ingest` وتعدين الجلسات يعملان',
    'PRECC به سطح Community بازمی‌گردد. تمام عملکردهای اصلی همچنان کار می‌کنند:

- مهارت‌های داخلی فعال می‌مانند
- خط لوله hook به طور عادی اجرا می‌شود
- `precc savings` نمای خلاصه را نشان می‌دهد
- `precc ingest` و استخراج جلسات کار می‌کنند',
    'PRECC, Community katmanına döner. Tüm temel işlevsellik çalışmaya devam eder:

- Yerleşik beceriler aktif kalır
- Hook pipeline normal çalışır
- `precc savings` özet görünümü gösterir
- `precc ingest` ve oturum madenciliği çalışır',
    'PRECC는 Community 티어로 돌아갑니다. 모든 핵심 기능은 계속 작동합니다:

- 기본 스킬은 활성 상태 유지
- 훅 파이프라인은 정상 작동
- `precc savings`는 요약 보기 표시
- `precc ingest`와 세션 마이닝 작동',
    'PRECC กลับไปยังระดับ Community ฟังก์ชันหลักทั้งหมดยังคงทำงาน:

- ทักษะในตัวยังคงใช้งานได้
- Pipeline hook ทำงานปกติ
- `precc savings` แสดงมุมมองสรุป
- `precc ingest` และการขุดเซสชันทำงาน',
    'PRECC သည် Community အဆင့်သို့ ပြန်သွားသည်။ အဓိက လုပ်ဆောင်ချက်များ ဆက်လုပ်ဆောင်သည်:

- Built-in ကျွမ်းကျင်မှုများ အသက်ဝင်နေဆဲ
- Hook pipeline ပုံမှန် လုပ်ဆောင်သည်',
    'PRECC Community түвшинд буцна. Бүх үндсэн функц ажиллсаар байна:

- Суурь чадварууд идэвхтэй хэвээр
- Hook pipeline хэвийн ажилладаг',
    '',
    'PRECC wraca do poziomu Community. Cała podstawowa funkcjonalność nadal działa:

- Wbudowane umiejętności pozostają aktywne
- Pipeline hooka działa normalnie
- `precc savings` pokazuje widok podsumowania
- `precc ingest` i eksploracja sesji działają');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_license_a_pro',
    'Pro features become unavailable until you renew:

- `precc savings --all` (detailed breakdown)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Email reports',
    'Las funciones Pro dejan de estar disponibles hasta que renueve:

- `precc savings --all` (desglose detallado)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Informes por correo electrónico',
    'Pro-Funktionen werden bis zur Verlängerung nicht verfügbar:

- `precc savings --all` (detaillierte Aufschlüsselung)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-Mail-Berichte',
    'Pro功能在续订前不可用：

- `precc savings --all`（详细分类）
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- 电子邮件报告',
    'Les fonctionnalités Pro deviennent indisponibles jusqu''au renouvellement :

- `precc savings --all` (ventilation détaillée)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Rapports par email',
    'Os recursos Pro ficam indisponíveis até a renovação:

- `precc savings --all` (detalhamento completo)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Relatórios por email',
    'Pro機能は更新まで利用できなくなります：

- `precc savings --all`（詳細な内訳）
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- メールレポート',
    'Các tính năng Pro không khả dụng cho đến khi bạn gia hạn:

- `precc savings --all` (phân tích chi tiết)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Báo cáo qua email',
    'Pro-functies worden niet beschikbaar tot verlenging:

- `precc savings --all` (gedetailleerd overzicht)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-mailrapporten',
    'A Pro funkciók a megújításig nem elérhetők:

- `precc savings --all` (részletes bontás)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-mail jelentések',
    'تصبح ميزات Pro غير متوفرة حتى التجديد:

- `precc savings --all` (تفصيل مفصل)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- تقارير البريد الإلكتروني',
    'ویژگی‌های Pro تا تمدید غیرفعال می‌شوند:

- `precc savings --all` (جزئیات کامل)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- گزارش‌های ایمیلی',
    'Pro özellikleri yenileyene kadar kullanılamaz:

- `precc savings --all` (ayrıntılı döküm)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- E-posta raporları',
    'Pro 기능은 갱신할 때까지 사용할 수 없습니다:

- `precc savings --all` (상세 분석)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- 이메일 보고서',
    'ฟีเจอร์ Pro จะไม่สามารถใช้งานได้จนกว่าจะต่ออายุ:

- `precc savings --all` (รายละเอียดการแยกย่อย)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- รายงานทางอีเมล',
    'Pro လုပ်ဆောင်ချက်များ သက်တမ်းတိုးသည်အထိ မရရှိနိုင်ပါ။',
    'Pro функцууд сунгах хүртэл боломжгүй болно.',
    '',
    'Funkcje Pro stają się niedostępne do momentu odnowienia:

- `precc savings --all` (szczegółowy podział)
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- Raporty e-mail');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_debug_q',
    'The hook does not seem to be running. How do I debug?',
    'El hook no parece estar ejecutándose. ¿Cómo depuro?',
    'Der Hook scheint nicht zu laufen. Wie debugge ich?',
    '钩子似乎没有运行。如何调试？',
    'Le hook ne semble pas fonctionner. Comment déboguer ?',
    'O hook não parece estar funcionando. Como depurar?',
    'フックが動作していないようです。どうやってデバッグしますか？',
    'Hook dường như không chạy. Làm thế nào để gỡ lỗi?',
    'De hook lijkt niet te werken. Hoe debug ik?',
    'A hook nem tűnik futónak. Hogyan debugolok?',
    'يبدو أن الـ hook لا يعمل. كيف أقوم بالتصحيح؟',
    'به نظر می‌رسد هوک اجرا نمی‌شود. چگونه اشکال‌زدایی کنم؟',
    'Hook çalışmıyor gibi görünüyor. Nasıl hata ayıklama yapabilirim?',
    '훅이 실행되지 않는 것 같습니다. 어떻게 디버그하나요?',
    'Hook ดูเหมือนไม่ทำงาน ฉันจะแก้ไขจุดบกพร่องอย่างไร?',
    'Hook အလုပ်မလုပ်သလို ဖြစ်နေသည်။ ဘယ်လို debug လုပ်မလဲ?',
    'Hook ажиллахгүй байна. Хэрхэн debug хийх вэ?',
    '',
    'Hook nie wydaje się działać. Jak debugować?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_debug_a_intro',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '',
    '');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_debug_step1',
    'Check that the hook is registered:',
    'Verifique que el hook esté registrado:',
    'Prüfen Sie, ob der Hook registriert ist:',
    '检查钩子是否已注册：',
    'Vérifiez que le hook est enregistré :',
    'Verifique se o hook está registrado:',
    'フックが登録されていることを確認：',
    'Kiểm tra xem hook đã được đăng ký chưa:',
    'Controleer of de hook geregistreerd is:',
    'Ellenőrizze, hogy a hook regisztrálva van-e:',
    'تحقق من تسجيل الـ hook:',
    'بررسی کنید که هوک ثبت شده باشد:',
    'Hook''un kayıtlı olduğunu kontrol edin:',
    '훅이 등록되어 있는지 확인:',
    'ตรวจสอบว่า hook ได้ลงทะเบียนแล้ว:',
    'Hook မှတ်ပုံတင်ထားသလား စစ်ဆေးပါ:',
    'Hook бүртгэгдсэн эсэхийг шалгах:',
    '',
    'Sprawdź, czy hook jest zarejestrowany:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_debug_step2',
    'Test the hook manually:',
    'Pruebe el hook manualmente:',
    'Testen Sie den Hook manuell:',
    '手动测试钩子：',
    'Testez le hook manuellement :',
    'Teste o hook manualmente:',
    'フックを手動でテスト：',
    'Kiểm tra hook thủ công:',
    'Test de hook handmatig:',
    'Tesztelje a hookot manuálisan:',
    'اختبر الـ hook يدوياً:',
    'هوک را به صورت دستی تست کنید:',
    'Hook''u manuel olarak test edin:',
    '훅을 수동으로 테스트:',
    'ทดสอบ hook ด้วยตนเอง:',
    'Hook ကို ကိုယ်တိုင် စမ်းသပ်ပါ:',
    'Hook-г гар аргаар шалгах:',
    '',
    'Przetestuj hook ręcznie:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_debug_step3',
    'Check that the binary is on your PATH:',
    'Verifique que el binario esté en su PATH:',
    'Prüfen Sie, ob die Binärdatei in Ihrem PATH ist:',
    '检查二进制文件是否在PATH中：',
    'Vérifiez que le binaire est dans votre PATH :',
    'Verifique se o binário está no seu PATH:',
    'バイナリがPATHにあることを確認：',
    'Kiểm tra xem tệp nhị phân có trong PATH không:',
    'Controleer of het binaire bestand in uw PATH staat:',
    'Ellenőrizze, hogy a bináris fájl a PATH-ban van-e:',
    'تحقق من أن الملف الثنائي في مسار PATH:',
    'بررسی کنید که فایل باینری در PATH شما باشد:',
    'İkili dosyanın PATH''inizde olduğunu kontrol edin:',
    '바이너리가 PATH에 있는지 확인:',
    'ตรวจสอบว่าไฟล์ไบนารีอยู่ใน PATH:',
    'Binary ဖိုင် PATH ထဲတွင် ရှိမရှိ စစ်ဆေးပါ:',
    'Бинар файл PATH-д байгаа эсэхийг шалгах:',
    '',
    'Sprawdź, czy plik binarny jest w PATH:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_debug_step4',
    'Check Claude Code''s hook configuration in `~/.claude/settings.json`.',
    'Verifique la configuración del hook de Claude Code en `~/.claude/settings.json`.',
    'Prüfen Sie die Hook-Konfiguration von Claude Code in `~/.claude/settings.json`.',
    '检查 `~/.claude/settings.json` 中的Claude Code钩子配置。',
    'Vérifiez la configuration du hook de Claude Code dans `~/.claude/settings.json`.',
    'Verifique a configuração do hook do Claude Code em `~/.claude/settings.json`.',
    '`~/.claude/settings.json` のClaude Codeフック設定を確認。',
    'Kiểm tra cấu hình hook của Claude Code trong `~/.claude/settings.json`.',
    'Controleer de hook-configuratie van Claude Code in `~/.claude/settings.json`.',
    'Ellenőrizze a Claude Code hook konfigurációját a `~/.claude/settings.json` fájlban.',
    'تحقق من تكوين hook في `~/.claude/settings.json`.',
    'پیکربندی hook در `~/.claude/settings.json` را بررسی کنید.',
    '`~/.claude/settings.json` dosyasındaki Claude Code hook yapılandırmasını kontrol edin.',
    '`~/.claude/settings.json`에서 Claude Code 훅 설정을 확인하세요.',
    'ตรวจสอบการกำหนดค่า hook ของ Claude Code ใน `~/.claude/settings.json`',
    '`~/.claude/settings.json` ရှိ Claude Code hook ဖွဲ့စည်းပုံကို စစ်ဆေးပါ။',
    '`~/.claude/settings.json` дахь Claude Code hook тохиргоог шалгана.',
    '',
    'Sprawdź konfigurację hooka Claude Code w `~/.claude/settings.json`.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_slow_q',
    'Does PRECC slow down Claude Code?',
    '¿PRECC ralentiza Claude Code?',
    'Verlangsamt PRECC Claude Code?',
    'PRECC会减慢Claude Code吗？',
    'PRECC ralentit-il Claude Code ?',
    'O PRECC deixa o Claude Code mais lento?',
    'PRECCはClaude Codeを遅くしますか？',
    'PRECC có làm chậm Claude Code không?',
    'Vertraagt PRECC Claude Code?',
    'Lassítja a PRECC a Claude Code-ot?',
    'هل يبطئ PRECC عمل Claude Code؟',
    'آیا PRECC باعث کندی Claude Code می‌شود؟',
    'PRECC, Claude Code''u yavaşlatır mı?',
    'PRECC가 Claude Code를 느리게 하나요?',
    'PRECC ทำให้ Claude Code ช้าลงไหม?',
    'PRECC က Claude Code ကို နှေးစေပါသလား?',
    'PRECC нь Claude Code-ийг удаашруулдаг уу?',
    '',
    'Czy PRECC spowalnia Claude Code?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_slow_a',
    'No. The hook completes in under 5 milliseconds (p99). This is imperceptible compared to the time Claude spends reasoning and generating responses.',
    'No. El hook se completa en menos de 5 milisegundos (p99). Esto es imperceptible comparado con el tiempo que Claude dedica a razonar y generar respuestas.',
    'Nein. Der Hook wird in unter 5 Millisekunden (p99) abgeschlossen. Dies ist im Vergleich zur Zeit, die Claude für Reasoning und Antwortgenerierung benötigt, nicht wahrnehmbar.',
    '不会。钩子在5毫秒内完成（p99）。与Claude推理和生成回复所花费的时间相比，这是不可察觉的。',
    'Non. Le hook se termine en moins de 5 millisecondes (p99). C''est imperceptible par rapport au temps que Claude passe à raisonner et générer des réponses.',
    'Não. O hook completa em menos de 5 milissegundos (p99). Isso é imperceptível comparado ao tempo que o Claude gasta raciocinando e gerando respostas.',
    'いいえ。フックは5ミリ秒未満（p99）で完了します。Claudeが推論と応答生成に費やす時間と比べると知覚できません。',
    'Không. Hook hoàn thành trong dưới 5 mili giây (p99). Điều này không thể cảm nhận được so với thời gian Claude dành cho suy luận và tạo phản hồi.',
    'Nee. De hook wordt in minder dan 5 milliseconden (p99) voltooid. Dit is onmerkbaar vergeleken met de tijd die Claude besteedt aan redeneren en het genereren van antwoorden.',
    'Nem. A hook 5 ezredmásodperc alatt (p99) befejeződik. Ez érzékelhetetlen ahhoz képest, mennyi időt tölt Claude a gondolkodással és a válaszok generálásával.',
    'لا. يكتمل الـ hook في أقل من 5 مللي ثانية (p99). هذا غير محسوس مقارنة بالوقت الذي يقضيه Claude في التفكير وتوليد الاستجابات.',
    'خیر. هوک در کمتر از ۵ میلی‌ثانیه (p99) تکمیل می‌شود. این در مقایسه با زمانی که Claude صرف استدلال و تولید پاسخ می‌کند نامحسوس است.',
    'Hayır. Hook 5 milisaniyenin altında (p99) tamamlanır. Bu, Claude''un akıl yürütme ve yanıt oluşturma süresine kıyasla fark edilemez.',
    '아닙니다. 훅은 5밀리초(p99) 이내에 완료됩니다. 이는 Claude가 추론하고 응답을 생성하는 시간에 비해 감지할 수 없는 수준입니다.',
    'ไม่ Hook เสร็จสิ้นในเวลาน้อยกว่า 5 มิลลิวินาที (p99) ซึ่งแทบไม่รู้สึกเมื่อเทียบกับเวลาที่ Claude ใช้ในการคิดและสร้างคำตอบ',
    'မဟုတ်ပါ။ Hook သည် 5 မီလီစက္ကန့်အတွင်း ပြီးစီးသည်။',
    'Үгүй. Hook 5 миллисекундээс бага хугацаанд дуусдаг (p99).',
    '',
    'Nie. Hook kończy się w mniej niż 5 milisekund (p99). Jest to niezauważalne w porównaniu z czasem, jaki Claude poświęca na rozumowanie i generowanie odpowiedzi.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_cicd_q',
    'Can I use PRECC in CI/CD?',
    '¿Puedo usar PRECC en CI/CD?',
    'Kann ich PRECC in CI/CD verwenden?',
    '我可以在CI/CD中使用PRECC吗？',
    'Puis-je utiliser PRECC en CI/CD ?',
    'Posso usar o PRECC em CI/CD?',
    'CI/CDでPRECCを使用できますか？',
    'Tôi có thể sử dụng PRECC trong CI/CD không?',
    'Kan ik PRECC gebruiken in CI/CD?',
    'Használhatom a PRECC-et CI/CD-ben?',
    'هل يمكنني استخدام PRECC في CI/CD؟',
    'آیا می‌توانم از PRECC در CI/CD استفاده کنم؟',
    'PRECC''yi CI/CD''de kullanabilir miyim?',
    'CI/CD에서 PRECC를 사용할 수 있나요?',
    'ฉันใช้ PRECC ใน CI/CD ได้ไหม?',
    'CI/CD တွင် PRECC ကို သုံးနိုင်ပါသလား?',
    'CI/CD-д PRECC ашиглаж болох уу?',
    '',
    'Czy mogę używać PRECC w CI/CD?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_cicd_a',
    'PRECC is designed for interactive Claude Code sessions. In CI/CD, there is no Claude Code instance to hook into. However, `precc gha` can analyze failed GitHub Actions runs from any environment.',
    'PRECC está diseñado para sesiones interactivas de Claude Code. En CI/CD, no hay una instancia de Claude Code a la que engancharse. Sin embargo, `precc gha` puede analizar ejecuciones fallidas de GitHub Actions desde cualquier entorno.',
    'PRECC ist für interaktive Claude Code-Sitzungen konzipiert. In CI/CD gibt es keine Claude Code-Instanz zum Anhaken. Allerdings kann `precc gha` fehlgeschlagene GitHub Actions-Läufe aus jeder Umgebung analysieren.',
    'PRECC是为交互式Claude Code会话设计的。在CI/CD中，没有Claude Code实例可以挂钩。但是，`precc gha` 可以从任何环境分析失败的GitHub Actions运行。',
    'PRECC est conçu pour les sessions interactives de Claude Code. En CI/CD, il n''y a pas d''instance Claude Code à laquelle se connecter. Cependant, `precc gha` peut analyser les exécutions échouées de GitHub Actions depuis n''importe quel environnement.',
    'PRECC é projetado para sessões interativas do Claude Code. Em CI/CD, não há instância do Claude Code para conectar. No entanto, `precc gha` pode analisar execuções falhas do GitHub Actions de qualquer ambiente.',
    'PRECCはインタラクティブなClaude Codeセッション向けに設計されています。CI/CDでは、フックするClaude Codeインスタンスがありません。ただし、`precc gha` はどの環境からでも失敗したGitHub Actionsの実行を分析できます。',
    'PRECC được thiết kế cho các phiên Claude Code tương tác. Trong CI/CD, không có phiên bản Claude Code nào để hook vào. Tuy nhiên, `precc gha` có thể phân tích các lần chạy GitHub Actions thất bại từ bất kỳ môi trường nào.',
    'PRECC is ontworpen voor interactieve Claude Code-sessies. In CI/CD is er geen Claude Code-instantie om aan te haken. Echter, `precc gha` kan mislukte GitHub Actions-runs vanuit elke omgeving analyseren.',
    'A PRECC interaktív Claude Code munkamenetekhez készült. CI/CD-ben nincs Claude Code példány, amihez csatlakozni lehetne. Azonban a `precc gha` bármilyen környezetből elemezheti a sikertelen GitHub Actions futásokat.',
    'تم تصميم PRECC لجلسات Claude Code التفاعلية. في CI/CD، لا توجد نسخة Claude Code للربط بها. ومع ذلك، يمكن لـ `precc gha` تحليل عمليات GitHub Actions الفاشلة من أي بيئة.',
    'PRECC برای جلسات تعاملی Claude Code طراحی شده است. در CI/CD، نسخه‌ای از Claude Code برای اتصال وجود ندارد. با این حال، `precc gha` می‌تواند اجراهای ناموفق GitHub Actions را از هر محیطی تحلیل کند.',
    'PRECC, etkileşimli Claude Code oturumları için tasarlanmıştır. CI/CD''de bağlanılacak bir Claude Code örneği yoktur. Ancak `precc gha` herhangi bir ortamdan başarısız GitHub Actions çalıştırmalarını analiz edebilir.',
    'PRECC는 대화형 Claude Code 세션을 위해 설계되었습니다. CI/CD에서는 연결할 Claude Code 인스턴스가 없습니다. 그러나 `precc gha`는 모든 환경에서 실패한 GitHub Actions 실행을 분석할 수 있습니다.',
    'PRECC ออกแบบมาสำหรับเซสชัน Claude Code แบบโต้ตอบ ใน CI/CD ไม่มีอินสแตนซ์ Claude Code ให้ hook เข้า อย่างไรก็ตาม `precc gha` สามารถวิเคราะห์การรัน GitHub Actions ที่ล้มเหลวจากสภาพแวดล้อมใดก็ได้',
    'PRECC ကို interactive Claude Code session များအတွက် ဒီဇိုင်းထုတ်ထားသည်။ CI/CD တွင် `precc gha` သည် GitHub Actions ကို ခွဲခြမ်းစိတ်ဖြာနိုင်သည်။',
    'PRECC нь интерактив Claude Code сессүүдэд зориулагдсан. CI/CD-д Claude Code instance байхгүй. Гэхдээ `precc gha` нь GitHub Actions-ийн амжилтгүй гүйлтүүдийг шинжлэх боломжтой.',
    '',
    'PRECC jest zaprojektowany do interaktywnych sesji Claude Code. W CI/CD nie ma instancji Claude Code, do której można się podłączyć. Jednak `precc gha` może analizować nieudane uruchomienia GitHub Actions z dowolnego środowiska.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_mined_q',
    'How do mined skills differ from built-in skills?',
    '¿En qué se diferencian las habilidades minadas de las integradas?',
    'Wie unterscheiden sich geminte Skills von eingebauten Skills?',
    '挖掘的技能与内置技能有何不同？',
    'En quoi les compétences découvertes diffèrent-elles des compétences intégrées ?',
    'Como as habilidades mineradas diferem das integradas?',
    'マイニングされたスキルと組み込みスキルはどう違いますか？',
    'Các kỹ năng khai thác khác gì so với kỹ năng tích hợp?',
    'Hoe verschillen gedolven vaardigheden van ingebouwde vaardigheden?',
    'Miben különböznek a bányászott képességek a beépítettektől?',
    'كيف تختلف المهارات المستخرجة عن المهارات المدمجة؟',
    'مهارت‌های استخراج‌شده چه تفاوتی با مهارت‌های داخلی دارند؟',
    'Keşfedilen beceriler yerleşik becerilerden nasıl farklıdır?',
    '마이닝된 스킬은 기본 스킬과 어떻게 다른가요?',
    'ทักษะที่ขุดมาแตกต่างจากทักษะในตัวอย่างไร?',
    'ရှာဖွေတွေ့ရှိသော ကျွမ်းကျင်မှုများသည် built-in ကျွမ်းကျင်မှုများနှင့် ဘယ်လို ကွာခြားသလဲ?',
    'Олборлосон чадварууд суурь чадваруудаас юугаараа ялгаатай вэ?',
    '',
    'Czym różnią się wydobyte umiejętności od wbudowanych?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_mined_a',
    'Built-in skills ship with PRECC and cover common wrong-directory patterns. Mined skills are learned from your specific session logs -- they capture patterns unique to your workflow. Both are stored in SQLite and evaluated identically by the hook pipeline.',
    'Las habilidades integradas vienen con PRECC y cubren patrones comunes de directorio incorrecto. Las habilidades minadas se aprenden de sus registros de sesión específicos -- capturan patrones únicos de su flujo de trabajo. Ambas se almacenan en SQLite y se evalúan de forma idéntica por el pipeline del hook.',
    'Eingebaute Skills werden mit PRECC ausgeliefert und decken häufige Falsche-Verzeichnis-Muster ab. Geminte Skills werden aus Ihren spezifischen Sitzungsprotokollen gelernt -- sie erfassen Muster, die einzigartig für Ihren Workflow sind. Beide werden in SQLite gespeichert und identisch von der Hook-Pipeline ausgewertet.',
    '内置技能随PRECC提供，涵盖常见的错误目录模式。挖掘的技能从您的特定会话日志中学习——它们捕获您工作流程中独特的模式。两者都存储在SQLite中，并由钩子管道以相同方式评估。',
    'Les compétences intégrées sont livrées avec PRECC et couvrent les erreurs de répertoire courantes. Les compétences découvertes sont apprises de vos journaux de session spécifiques -- elles capturent des modèles uniques à votre flux de travail. Les deux sont stockées dans SQLite et évaluées de manière identique par le pipeline du hook.',
    'Habilidades integradas vêm com PRECC e cobrem padrões comuns de diretório errado. Habilidades mineradas são aprendidas de seus logs de sessão específicos -- capturam padrões únicos do seu fluxo de trabalho. Ambas são armazenadas em SQLite e avaliadas identicamente pelo pipeline do hook.',
    '組み込みスキルはPRECCに同梱され、一般的な間違ったディレクトリパターンをカバーします。マイニングされたスキルはあなたの特定のセッションログから学習されます。両方ともSQLiteに保存され、フックパイプラインで同一に評価されます。',
    'Các kỹ năng tích hợp được cung cấp cùng PRECC và bao gồm các mẫu sai thư mục phổ biến. Các kỹ năng khai thác được học từ nhật ký phiên cụ thể của bạn -- chúng nắm bắt các mẫu riêng biệt cho quy trình làm việc của bạn. Cả hai đều được lưu trữ trong SQLite và được đánh giá giống nhau bởi pipeline hook.',
    'Ingebouwde vaardigheden worden meegeleverd met PRECC en dekken veelvoorkomende verkeerde-map-patronen. Gedolven vaardigheden worden geleerd uit uw specifieke sessielogs -- ze vangen patronen die uniek zijn voor uw werkstroom. Beide worden opgeslagen in SQLite en identiek geëvalueerd door de hook-pipeline.',
    'A beépített képességek a PRECC-el érkeznek és a gyakori rossz könyvtár mintákat fedik le. A bányászott képességek az Ön munkamenet-naplóiból tanultak -- az Ön munkafolyamatára jellemző mintákat rögzítik. Mindkettő SQLite-ban van tárolva és azonos módon értékeli a hook pipeline.',
    'المهارات المدمجة تأتي مع PRECC وتغطي أنماط الدليل الخطأ الشائعة. المهارات المستخرجة يتم تعلمها من سجلات جلساتك الخاصة -- تلتقط أنماطاً فريدة لسير عملك. كلاهما يُخزن في SQLite ويُقيَّم بشكل متطابق بواسطة خط أنابيب الـ hook.',
    'مهارت‌های داخلی با PRECC ارائه می‌شوند و الگوهای رایج پوشه اشتباه را پوشش می‌دهند. مهارت‌های استخراج‌شده از لاگ‌های جلسه خاص شما آموخته می‌شوند. هر دو در SQLite ذخیره و به طور یکسان توسط خط لوله hook ارزیابی می‌شوند.',
    'Yerleşik beceriler PRECC ile birlikte gelir ve yaygın yanlış dizin kalıplarını kapsar. Keşfedilen beceriler, belirli oturum günlüklerinizden öğrenilir -- iş akışınıza özgü kalıpları yakalar. Her ikisi de SQLite''da depolanır ve hook pipeline tarafından aynı şekilde değerlendirilir.',
    '기본 스킬은 PRECC와 함께 제공되며 일반적인 잘못된 디렉터리 패턴을 다룹니다. 마이닝된 스킬은 특정 세션 로그에서 학습됩니다. 둘 다 SQLite에 저장되며 훅 파이프라인에서 동일하게 평가됩니다.',
    'ทักษะในตัวมาพร้อมกับ PRECC และครอบคลุมรูปแบบไดเรกทอรีผิดทั่วไป ทักษะที่ขุดมาเรียนรู้จากบันทึกเซสชันเฉพาะของคุณ ทั้งสองถูกจัดเก็บใน SQLite และถูกประเมินเหมือนกันโดย pipeline hook',
    'Built-in ကျွမ်းကျင်မှုများသည် PRECC နှင့်အတူ ပါဝင်ပြီး ဘုံ wrong-directory ပုံစံများကို ကာမိသည်။ Mined ကျွမ်းကျင်မှုများသည် သင့် session log များမှ သင်ယူသည်။',
    'Суурь чадварууд PRECC-тэй хамт ирдэг. Олборлосон чадварууд таны сессийн логоос суралцдаг. Хоёулаа SQLite-д хадгалагдаж, hook pipeline-аар ижил үнэлэгддэг.',
    '',
    'Wbudowane umiejętności są dostarczane z PRECC i obejmują typowe wzorce złego katalogu. Wydobyte umiejętności są uczone z Twoich konkretnych logów sesji -- przechwytują wzorce unikalne dla Twojego przepływu pracy. Oba typy są przechowywane w SQLite i oceniane identycznie przez pipeline hooka.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_share_q',
    'Can I share skills with my team?',
    '¿Puedo compartir habilidades con mi equipo?',
    'Kann ich Skills mit meinem Team teilen?',
    '我可以与团队共享技能吗？',
    'Puis-je partager des compétences avec mon équipe ?',
    'Posso compartilhar habilidades com minha equipe?',
    'スキルをチームと共有できますか？',
    'Tôi có thể chia sẻ kỹ năng với nhóm không?',
    'Kan ik vaardigheden delen met mijn team?',
    'Megoszthatom a képességeket a csapatommal?',
    'هل يمكنني مشاركة المهارات مع فريقي؟',
    'آیا می‌توانم مهارت‌ها را با تیمم به اشتراک بگذارم؟',
    'Becerileri ekibimle paylaşabilir miyim?',
    '팀과 스킬을 공유할 수 있나요?',
    'ฉันแชร์ทักษะกับทีมได้ไหม?',
    'အဖွဲ့နှင့် ကျွမ်းကျင်မှုများ မျှဝေနိုင်ပါသလား?',
    'Багтайгаа чадваруудаа хуваалцаж болох уу?',
    '',
    'Czy mogę udostępniać umiejętności zespołowi?');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('faq_share_a',
    'Yes. Export any skill to TOML with `precc skills export NAME` and share the file. Team members can place it in their `skills/` directory or import it into their heuristics database.',
    'Sí. Exporte cualquier habilidad a TOML con `precc skills export NAME` y comparta el archivo. Los miembros del equipo pueden colocarlo en su directorio `skills/` o importarlo a su base de datos de heurísticas.',
    'Ja. Exportieren Sie einen Skill mit `precc skills export NAME` als TOML und teilen Sie die Datei. Teammitglieder können sie in ihr `skills/`-Verzeichnis legen oder in ihre Heuristik-Datenbank importieren.',
    '可以。使用 `precc skills export NAME` 将任何技能导出为TOML并共享文件。团队成员可以将其放在 `skills/` 目录中或导入到他们的启发式数据库中。',
    'Oui. Exportez n''importe quelle compétence en TOML avec `precc skills export NAME` et partagez le fichier. Les membres de l''équipe peuvent le placer dans leur répertoire `skills/` ou l''importer dans leur base de données heuristique.',
    'Sim. Exporte qualquer habilidade para TOML com `precc skills export NAME` e compartilhe o arquivo. Membros da equipe podem colocá-lo no diretório `skills/` ou importá-lo para o banco de dados de heurísticas.',
    'はい。`precc skills export NAME` でスキルをTOMLにエクスポートしてファイルを共有できます。チームメンバーは `skills/` ディレクトリに配置するか、ヒューリスティクスデータベースにインポートできます。',
    'Có. Xuất bất kỳ kỹ năng nào sang TOML với `precc skills export NAME` và chia sẻ tệp. Các thành viên nhóm có thể đặt nó trong thư mục `skills/` hoặc nhập vào cơ sở dữ liệu heuristics.',
    'Ja. Exporteer een vaardigheid naar TOML met `precc skills export NAME` en deel het bestand. Teamleden kunnen het in hun `skills/`-map plaatsen of importeren in hun heuristieken-database.',
    'Igen. Exportáljon bármilyen képességet TOML-ba a `precc skills export NAME` paranccsal és ossza meg a fájlt. A csapattagok elhelyezhetik a `skills/` könyvtárukban vagy importálhatják a heurisztika adatbázisukba.',
    'نعم. صدّر أي مهارة إلى TOML باستخدام `precc skills export NAME` وشارك الملف. يمكن لأعضاء الفريق وضعه في مجلد `skills/` أو استيراده إلى قاعدة بياناتهم.',
    'بله. هر مهارتی را با `precc skills export NAME` به TOML صادر کنید و فایل را به اشتراک بگذارید.',
    'Evet. Herhangi bir beceriyi `precc skills export NAME` ile TOML''a aktarın ve dosyayı paylaşın. Ekip üyeleri `skills/` dizinlerine yerleştirebilir veya heuristik veritabanlarına aktarabilir.',
    '네. `precc skills export NAME`으로 스킬을 TOML로 내보내고 파일을 공유할 수 있습니다. 팀원들은 `skills/` 디렉터리에 넣거나 휴리스틱 데이터베이스에 가져올 수 있습니다.',
    'ได้ ส่งออกทักษะเป็น TOML ด้วย `precc skills export NAME` และแชร์ไฟล์ สมาชิกในทีมสามารถวางไว้ในไดเรกทอรี `skills/` หรือนำเข้าสู่ฐานข้อมูลได้',
    'ဟုတ်ကဲ့။ `precc skills export NAME` ဖြင့် TOML သို့ export လုပ်ပြီး ဖိုင်ကို မျှဝေနိုင်သည်။',
    'Тийм. `precc skills export NAME` ашиглан TOML руу экспортлож, файлаа хуваалцаж болно.',
    '',
    'Tak. Wyeksportuj dowolną umiejętność do TOML za pomocą `precc skills export NAME` i udostępnij plik. Członkowie zespołu mogą umieścić go w katalogu `skills/` lub zaimportować do bazy danych heurystyk.');
