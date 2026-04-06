-- Compress chapter strings
-- Languages: en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_title',
    'Compress',
    'Comprimir',
    'Komprimieren',
    '压缩',
    'Compression',
    'Comprimir',
    '圧縮',
    'Nén',
    'Comprimeren',
    'Tömörítés',
    'الضغط',
    'فشرده‌سازی',
    'Sıkıştırma',
    '압축',
    'การบีบอัด',
    'ချုံ့ခြင်း',
    'Шахалт',
    '',
    'Kompresja');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_intro',
    '`precc compress` shrinks CLAUDE.md and other context files to reduce token usage when Claude Code loads them. This is a Pro feature.',
    '`precc compress` reduce CLAUDE.md y otros archivos de contexto para disminuir el uso de tokens cuando Claude Code los carga. Esta es una función Pro.',
    '`precc compress` verkleinert CLAUDE.md und andere Kontextdateien, um den Token-Verbrauch zu reduzieren, wenn Claude Code sie lädt. Dies ist eine Pro-Funktion.',
    '`precc compress` 缩小 CLAUDE.md 和其他上下文文件，以减少 Claude Code 加载时的 token 使用量。这是 Pro 功能。',
    '`precc compress` réduit CLAUDE.md et d''autres fichiers de contexte pour diminuer l''utilisation de tokens lorsque Claude Code les charge. C''est une fonctionnalité Pro.',
    '`precc compress` reduz CLAUDE.md e outros arquivos de contexto para diminuir o uso de tokens quando Claude Code os carrega. Este é um recurso Pro.',
    '`precc compress` は CLAUDE.md やその他のコンテキストファイルを圧縮し、Claude Code がそれらを読み込む際のトークン使用量を削減します。これは Pro 機能です。',
    '`precc compress` thu nhỏ CLAUDE.md và các tệp ngữ cảnh khác để giảm sử dụng token khi Claude Code tải chúng. Đây là tính năng Pro.',
    '`precc compress` verkleint CLAUDE.md en andere contextbestanden om tokengebruik te verminderen wanneer Claude Code ze laadt. Dit is een Pro-functie.',
    'A `precc compress` összezsugorítja a CLAUDE.md-t és más kontextusfájlokat, hogy csökkentse a tokenhasználatot, amikor a Claude Code betölti őket. Ez egy Pro funkció.',
    'يقوم `precc compress` بتقليص CLAUDE.md وملفات السياق الأخرى لتقليل استخدام الرموز عندما يحملها Claude Code. هذه ميزة Pro.',
    '`precc compress` فایل CLAUDE.md و سایر فایل‌های زمینه را کوچک می‌کند تا مصرف توکن هنگام بارگذاری توسط Claude Code کاهش یابد. این یک ویژگی Pro است.',
    '`precc compress`, Claude Code yüklediğinde token kullanımını azaltmak için CLAUDE.md ve diğer bağlam dosyalarını küçültür. Bu bir Pro özelliğidir.',
    '`precc compress`는 Claude Code가 로드할 때 토큰 사용량을 줄이기 위해 CLAUDE.md 및 기타 컨텍스트 파일을 축소합니다. Pro 기능입니다.',
    '`precc compress` ย่อ CLAUDE.md และไฟล์บริบทอื่นๆ เพื่อลดการใช้โทเค็นเมื่อ Claude Code โหลดไฟล์เหล่านั้น นี่เป็นฟีเจอร์ Pro',
    '`precc compress` သည် Claude Code တင်သောအခါ token အသုံးပြုမှုကို လျှော့ချရန် CLAUDE.md နှင့် အခြား context ဖိုင်များကို ချုံ့သည်။ ၎င်းသည် Pro အင်္ဂါရပ်ဖြစ်သည်။',
    '`precc compress` нь Claude Code ачаалах үед токен хэрэглээг бууруулахын тулд CLAUDE.md болон бусад контекст файлуудыг шахдаг. Энэ бол Pro функц юм.',
    '',
    '`precc compress` zmniejsza CLAUDE.md i inne pliki kontekstowe, aby ograniczyć zużycie tokenów, gdy Claude Code je ładuje. To funkcja Pro.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_basic_title',
    'Basic Usage',
    'Uso básico',
    'Grundlegende Verwendung',
    '基本用法',
    'Utilisation de base',
    'Uso básico',
    '基本的な使い方',
    'Sử dụng cơ bản',
    'Basisgebruik',
    'Alapvető használat',
    'الاستخدام الأساسي',
    'استفاده پایه',
    'Temel kullanım',
    '기본 사용법',
    'การใช้งานพื้นฐาน',
    'အခြေခံ အသုံးပြုမှု',
    'Үндсэн хэрэглээ',
    '',
    'Podstawowe użycie');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_dryrun_title',
    'Dry Run',
    'Ejecución de prueba',
    'Probelauf',
    '试运行',
    'Exécution à blanc',
    'Execução de teste',
    'ドライラン',
    'Chạy thử',
    'Proefrun',
    'Próbafuttatás',
    'تشغيل تجريبي',
    'اجرای آزمایشی',
    'Deneme çalıştırma',
    '드라이 런',
    'ทดลองรัน',
    'စမ်းသပ် လည်ပတ်ခြင်း',
    'Туршилтын ажиллуулалт',
    '',
    'Przebieg próbny');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_dryrun_body',
    'Preview what would change without modifying files:',
    'Vista previa de lo que cambiaría sin modificar archivos:',
    'Vorschau der Änderungen ohne Dateien zu modifizieren:',
    '预览将要更改的内容而不修改文件：',
    'Aperçu des modifications sans modifier les fichiers :',
    'Visualize o que mudaria sem modificar arquivos:',
    'ファイルを変更せずに変更内容をプレビュー：',
    'Xem trước những gì sẽ thay đổi mà không sửa đổi tệp:',
    'Bekijk wat er zou veranderen zonder bestanden te wijzigen:',
    'A változtatások előnézete fájlok módosítása nélkül:',
    'معاينة ما سيتغير دون تعديل الملفات:',
    'پیش‌نمایش تغییرات بدون اصلاح فایل‌ها:',
    'Dosyaları değiştirmeden nelerin değişeceğini önizleyin:',
    '파일을 수정하지 않고 변경될 내용 미리보기:',
    'ดูตัวอย่างสิ่งที่จะเปลี่ยนแปลงโดยไม่แก้ไขไฟล์:',
    'ဖိုင်များကို မပြောင်းလဲဘဲ ဘာတွေပြောင်းမလဲ ကြိုတင်ကြည့်ရှုခြင်း:',
    'Файлуудыг өөрчлөхгүйгээр юу өөрчлөгдөхийг урьдчилан харах:',
    '',
    'Podgląd zmian bez modyfikowania plików:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_revert_title',
    'Reverting',
    'Reversión',
    'Zurücksetzen',
    '还原',
    'Restauration',
    'Reversão',
    '元に戻す',
    'Hoàn nguyên',
    'Terugzetten',
    'Visszaállítás',
    'الاستعادة',
    'بازگردانی',
    'Geri alma',
    '되돌리기',
    'การย้อนกลับ',
    'ပြန်လည်ပြင်ဆင်ခြင်း',
    'Буцаах',
    '',
    'Przywracanie');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_revert_body',
    'Originals are backed up automatically. To restore them:',
    'Los originales se respaldan automáticamente. Para restaurarlos:',
    'Originale werden automatisch gesichert. Um sie wiederherzustellen:',
    '原始文件会自动备份。要恢复它们：',
    'Les originaux sont sauvegardés automatiquement. Pour les restaurer :',
    'Os originais são salvos automaticamente. Para restaurá-los:',
    '元のファイルは自動的にバックアップされます。復元するには：',
    'Các tệp gốc được sao lưu tự động. Để khôi phục chúng:',
    'Originelen worden automatisch geback-upt. Om ze te herstellen:',
    'Az eredetik automatikusan mentésre kerülnek. A visszaállításhoz:',
    'يتم نسخ الأصول احتياطيًا تلقائيًا. لاستعادتها:',
    'فایل‌های اصلی به‌طور خودکار پشتیبان‌گیری می‌شوند. برای بازیابی آنها:',
    'Orijinaller otomatik olarak yedeklenir. Geri yüklemek için:',
    '원본은 자동으로 백업됩니다. 복원하려면:',
    'ไฟล์ต้นฉบับจะถูกสำรองโดยอัตโนมัติ เพื่อกู้คืน:',
    'မူရင်းများကို အလိုအလျောက် အရန်ကူးထားသည်။ ပြန်ရယူရန်:',
    'Эх файлууд автоматаар нөөцлөгддөг. Сэргээхийн тулд:',
    '',
    'Oryginały są automatycznie kopiowane. Aby je przywrócić:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_what_title',
    'What Gets Compressed',
    'Qué se comprime',
    'Was wird komprimiert',
    '压缩了什么',
    'Ce qui est compressé',
    'O que é comprimido',
    '何が圧縮されるか',
    'Những gì được nén',
    'Wat wordt gecomprimeerd',
    'Mi kerül tömörítésre',
    'ما الذي يتم ضغطه',
    'چه چیزی فشرده می‌شود',
    'Ne sıkıştırılır',
    '무엇이 압축되는가',
    'อะไรถูกบีบอัด',
    'ဘာတွေ ချုံ့မလဲ',
    'Юу шахагдах вэ',
    '',
    'Co jest kompresowane');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_what_body',
    'The compressor applies several transformations:

- Removes redundant whitespace and blank lines
- Shortens verbose phrasing while preserving meaning
- Condenses tables and lists
- Strips comments and decorative formatting
- Preserves all code blocks, paths, and technical identifiers',
    'El compresor aplica varias transformaciones:

- Elimina espacios en blanco redundantes y líneas vacías
- Acorta frases verbosas preservando el significado
- Condensa tablas y listas
- Elimina comentarios y formato decorativo
- Preserva todos los bloques de código, rutas e identificadores técnicos',
    'Der Kompressor wendet mehrere Transformationen an:

- Entfernt überflüssige Leerzeichen und Leerzeilen
- Kürzt ausführliche Formulierungen unter Beibehaltung der Bedeutung
- Verdichtet Tabellen und Listen
- Entfernt Kommentare und dekorative Formatierung
- Behält alle Codeblöcke, Pfade und technische Bezeichner bei',
    '压缩器应用多种转换：

- 删除冗余空白和空行
- 缩短冗长的措辞同时保留含义
- 压缩表格和列表
- 去除注释和装饰性格式
- 保留所有代码块、路径和技术标识符',
    'Le compresseur applique plusieurs transformations :

- Supprime les espaces et lignes vides redondants
- Raccourcit les formulations verbales tout en préservant le sens
- Condense les tableaux et listes
- Supprime les commentaires et le formatage décoratif
- Préserve tous les blocs de code, chemins et identifiants techniques',
    'O compressor aplica várias transformações:

- Remove espaços em branco e linhas vazias redundantes
- Encurta fraseado verboso preservando o significado
- Condensa tabelas e listas
- Remove comentários e formatação decorativa
- Preserva todos os blocos de código, caminhos e identificadores técnicos',
    'コンプレッサーはいくつかの変換を適用します：

- 冗長な空白と空行を削除
- 意味を保ちながら冗長な表現を短縮
- テーブルとリストを圧縮
- コメントと装飾的なフォーマットを除去
- すべてのコードブロック、パス、技術的識別子を保持',
    'Bộ nén áp dụng nhiều phép biến đổi:

- Xóa khoảng trắng và dòng trống thừa
- Rút ngắn cách diễn đạt dài dòng nhưng giữ nguyên ý nghĩa
- Cô đọng bảng và danh sách
- Loại bỏ chú thích và định dạng trang trí
- Giữ nguyên tất cả khối mã, đường dẫn và định danh kỹ thuật',
    'De compressor past meerdere transformaties toe:

- Verwijdert overbodige witruimte en lege regels
- Verkort uitgebreide formuleringen met behoud van betekenis
- Comprimeert tabellen en lijsten
- Verwijdert opmerkingen en decoratieve opmaak
- Behoudt alle codeblokken, paden en technische identifiers',
    'A tömörítő több átalakítást alkalmaz:

- Eltávolítja a felesleges szóközöket és üres sorokat
- Rövidíti a bőbeszédű megfogalmazásokat a jelentés megőrzése mellett
- Tömöríti a táblázatokat és listákat
- Eltávolítja a megjegyzéseket és dekoratív formázást
- Megőrzi az összes kódblokkot, útvonalat és technikai azonosítót',
    'يطبق الضاغط عدة تحويلات:

- يزيل المسافات الفارغة والأسطر الفارغة الزائدة
- يختصر الصياغة المطولة مع الحفاظ على المعنى
- يكثف الجداول والقوائم
- يزيل التعليقات والتنسيق الزخرفي
- يحافظ على جميع كتل الكود والمسارات والمعرفات التقنية',
    'فشرده‌ساز چندین تبدیل اعمال می‌کند:

- فضاهای خالی و خطوط خالی اضافی را حذف می‌کند
- عبارات طولانی را کوتاه می‌کند و معنی را حفظ می‌کند
- جداول و فهرست‌ها را فشرده می‌کند
- نظرات و قالب‌بندی تزئینی را حذف می‌کند
- تمام بلوک‌های کد، مسیرها و شناسه‌های فنی را حفظ می‌کند',
    'Sıkıştırıcı birkaç dönüşüm uygular:

- Gereksiz boşlukları ve boş satırları kaldırır
- Anlamı koruyarak ayrıntılı ifadeleri kısaltır
- Tabloları ve listeleri yoğunlaştırır
- Yorumları ve dekoratif biçimlendirmeyi kaldırır
- Tüm kod bloklarını, yolları ve teknik tanımlayıcıları korur',
    '압축기는 여러 변환을 적용합니다:

- 불필요한 공백과 빈 줄 제거
- 의미를 유지하면서 장황한 표현 단축
- 테이블과 목록 압축
- 주석과 장식적 서식 제거
- 모든 코드 블록, 경로, 기술 식별자 보존',
    'ตัวบีบอัดใช้การแปลงหลายอย่าง:

- ลบช่องว่างและบรรทัดว่างที่ซ้ำซ้อน
- ย่อข้อความที่ยืดยาวโดยรักษาความหมาย
- ย่อตารางและรายการ
- ลบความคิดเห็นและการจัดรูปแบบตกแต่ง
- รักษาบล็อกโค้ด เส้นทาง และตัวระบุทางเทคนิคทั้งหมด',
    'ချုံ့စက်သည် အသွင်ပြောင်းမှုများ အများအပြား ကျင့်သုံးသည်:

- မလိုအပ်သော နေရာလွတ်နှင့် အလွတ်လိုင်းများကို ဖယ်ရှားသည်
- အဓိပ္ပါယ်ကို ထိန်းသိမ်းလျက် ရှည်လျားသော စကားရပ်များကို အတိုချုံ့သည်
- ဇယားများနှင့် စာရင်းများကို ချုံ့သည်
- မှတ်ချက်များနှင့် အလှဆင်ပုံစံချမှုများကို ဖယ်ရှားသည်
- ကုဒ်ဘလောက်၊ လမ်းကြောင်းနှင့် နည်းပညာ ခွဲခြားသတ်မှတ်မှုများအားလုံးကို ထိန်းသိမ်းသည်',
    'Шахагч хэд хэдэн хувиргалт хэрэглэнэ:

- Шаардлагагүй хоосон зай, хоосон мөрүүдийг хасна
- Утгыг хадгалж дэлгэрэнгүй үг хэллэгийг товчилно
- Хүснэгт, жагсаалтыг нягтруулна
- Тайлбар, чимэглэлийн форматыг хасна
- Бүх кодын блок, зам, техникийн тодорхойлогчийг хадгална',
    '',
    'Kompresor stosuje kilka transformacji:

- Usuwa zbędne białe znaki i puste linie
- Skraca rozwlekłe sformułowania zachowując znaczenie
- Kompresuje tabele i listy
- Usuwa komentarze i dekoracyjne formatowanie
- Zachowuje wszystkie bloki kodu, ścieżki i identyfikatory techniczne');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_readable_note',
    'The compressed output is still human-readable -- it is not minified or obfuscated.',
    'La salida comprimida sigue siendo legible para humanos -- no está minificada ni ofuscada.',
    'Die komprimierte Ausgabe ist weiterhin menschenlesbar -- sie ist weder minifiziert noch verschleiert.',
    '压缩后的输出仍然是人类可读的——它不是压缩化或混淆的。',
    'La sortie compressée est toujours lisible par un humain -- elle n''est ni minifiée ni obfusquée.',
    'A saída comprimida ainda é legível por humanos -- não é minificada nem ofuscada.',
    '圧縮された出力はまだ人間が読める形式です——ミニファイや難読化はされていません。',
    'Đầu ra nén vẫn đọc được -- không bị rút gọn hay làm rối.',
    'De gecomprimeerde uitvoer is nog steeds leesbaar -- niet geminificeerd of verhuld.',
    'A tömörített kimenet továbbra is ember által olvasható -- nem minifikált vagy obfuszkált.',
    'المخرجات المضغوطة لا تزال قابلة للقراءة -- ليست مصغرة أو مبهمة.',
    'خروجی فشرده همچنان قابل خواندن است -- فشرده‌سازی یا مبهم‌سازی نشده است.',
    'Sıkıştırılmış çıktı hâlâ okunabilirdir -- küçültülmemiş veya gizlenmemiştir.',
    '압축된 출력은 여전히 사람이 읽을 수 있습니다 -- 축소화되거나 난독화되지 않았습니다.',
    'ผลลัพธ์ที่บีบอัดยังคงอ่านได้โดยมนุษย์ -- ไม่ได้ถูกย่อหรือทำให้สับสน',
    'ချုံ့ထားသော ရလဒ်သည် လူသားဖတ်နိုင်ဆဲဖြစ်သည် -- ချုံ့ခြင်း သို့မဟုတ် ရှုပ်ထွေးစေခြင်း မဟုတ်ပါ။',
    'Шахсан гаралт хүний уншигдахуйц хэвээр -- жижиглэсэн эсвэл бүдгэрүүлээгүй.',
    '',
    'Skompresowany wynik jest nadal czytelny dla człowieka -- nie jest zminifikowany ani zaciemniony.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('cmp_target_title',
    'Targeting Specific Files',
    'Apuntar a archivos específicos',
    'Bestimmte Dateien auswählen',
    '针对特定文件',
    'Cibler des fichiers spécifiques',
    'Direcionar arquivos específicos',
    '特定のファイルを対象にする',
    'Nhắm mục tiêu tệp cụ thể',
    'Specifieke bestanden targeten',
    'Adott fájlok célzása',
    'استهداف ملفات محددة',
    'هدف‌گیری فایل‌های خاص',
    'Belirli dosyaları hedefleme',
    '특정 파일 대상 지정',
    'กำหนดเป้าหมายไฟล์เฉพาะ',
    'တိကျသော ဖိုင်များကို ရွေးချယ်ခြင်း',
    'Тодорхой файлуудыг чиглүүлэх',
    '',
    'Celowanie w konkretne pliki');
