-- Mining chapter strings
-- Languages: en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_title',
    'Mining',
    'Minería',
    'Mining',
    '挖掘',
    'Exploration',
    'Mineração',
    'マイニング',
    'Khai thác',
    'Mining',
    'Bányászat',
    'التنقيب',
    'استخراج',
    'Madencilik',
    '마이닝',
    'การขุด',
    'တူးဖော်ခြင်း',
    'Олборлолт',
    '',
    'Eksploracja');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_intro',
    'PRECC mines Claude Code session logs to learn failure-fix patterns. When it sees the same mistake again, it applies the fix automatically.',
    'PRECC mina los registros de sesión de Claude Code para aprender patrones de fallo-corrección. Cuando ve el mismo error de nuevo, aplica la corrección automáticamente.',
    'PRECC analysiert Claude Code-Sitzungsprotokolle, um Fehler-Fix-Muster zu lernen. Wenn es denselben Fehler erneut erkennt, wendet es die Lösung automatisch an.',
    'PRECC挖掘Claude Code会话日志以学习失败-修复模式。当它再次看到同样的错误时，会自动应用修复。',
    'PRECC explore les journaux de session Claude Code pour apprendre les schémas échec-correction. Quand il revoit la même erreur, il applique la correction automatiquement.',
    'PRECC minera os logs de sessão do Claude Code para aprender padrões de falha-correção. Quando vê o mesmo erro novamente, aplica a correção automaticamente.',
    'PRECCはClaude Codeのセッションログを解析して失敗-修正パターンを学習します。同じミスを再び見つけると、自動的に修正を適用します。',
    'PRECC khai thác nhật ký phiên Claude Code để học các mẫu lỗi-sửa. Khi gặp lại cùng một lỗi, nó tự động áp dụng cách sửa.',
    'PRECC analyseert Claude Code-sessielogs om fout-fix-patronen te leren. Wanneer het dezelfde fout opnieuw ziet, past het de fix automatisch toe.',
    'A PRECC a Claude Code munkamenet-naplókat elemzi a hiba-javítás minták megtanulásához. Ha újra ugyanazt a hibát látja, automatikusan alkalmazza a javítást.',
    'يقوم PRECC بتنقيب سجلات جلسات Claude Code لتعلم أنماط الخطأ والإصلاح. عندما يرى نفس الخطأ مرة أخرى، يطبق الإصلاح تلقائيًا.',
    'PRECC لاگ‌های جلسات Claude Code را استخراج می‌کند تا الگوهای خطا-اصلاح را بیاموزد. وقتی همان اشتباه را دوباره می‌بیند، اصلاح را به‌طور خودکار اعمال می‌کند.',
    'PRECC, hata-düzeltme kalıplarını öğrenmek için Claude Code oturum günlüklerini analiz eder. Aynı hatayı tekrar gördüğünde düzeltmeyi otomatik olarak uygular.',
    'PRECC는 Claude Code 세션 로그를 마이닝하여 실패-수정 패턴을 학습합니다. 같은 실수를 다시 발견하면 자동으로 수정을 적용합니다.',
    'PRECC ขุดค้นล็อกเซสชัน Claude Code เพื่อเรียนรู้รูปแบบข้อผิดพลาด-แก้ไข เมื่อพบข้อผิดพลาดเดิมอีกครั้ง จะแก้ไขโดยอัตโนมัติ',
    'PRECC သည် Claude Code ဆက်ရှင်မှတ်တမ်းများကို ခွဲခြမ်းစိတ်ဖြာပြီး အမှား-ပြင်ဆင်ပုံစံများကို သင်ယူသည်။',
    'PRECC нь Claude Code-ийн сессийн логуудаас алдаа-засварын загварыг сурдаг. Ижил алдааг дахин олбол засварыг автоматаар хэрэглэнэ.',
    '',
    'PRECC analizuje logi sesji Claude Code, aby uczyć się wzorców awaria-naprawa. Gdy widzi ten sam błąd ponownie, automatycznie stosuje poprawkę.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_ingest_title',
    'Ingesting Session Logs',
    'Ingesta de registros de sesión',
    'Sitzungsprotokolle einlesen',
    '导入会话日志',
    'Ingestion des journaux de session',
    'Ingestão de logs de sessão',
    'セッションログの取り込み',
    'Nhập nhật ký phiên',
    'Sessielogs inlezen',
    'Munkamenet-naplók betöltése',
    'استيعاب سجلات الجلسات',
    'دریافت لاگ‌های جلسه',
    'Oturum günlüklerini alma',
    '세션 로그 수집',
    'การนำเข้าล็อกเซสชัน',
    'ဆက်ရှင်မှတ်တမ်းများ ထည့်သွင်းခြင်း',
    'Сессийн логуудыг оруулах',
    '',
    'Pobieranie logów sesji');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_ingest_single_title',
    'Ingest a Single File',
    'Ingestar un solo archivo',
    'Eine einzelne Datei einlesen',
    '导入单个文件',
    'Ingérer un seul fichier',
    'Ingerir um único arquivo',
    '単一ファイルの取り込み',
    'Nhập một tệp đơn',
    'Eén bestand inlezen',
    'Egyetlen fájl betöltése',
    'استيعاب ملف واحد',
    'دریافت یک فایل',
    'Tek bir dosyayı alma',
    '단일 파일 수집',
    'นำเข้าไฟล์เดียว',
    'ဖိုင်တစ်ခု ထည့်သွင်းခြင်း',
    'Нэг файл оруулах',
    '',
    'Pobranie jednego pliku');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_ingest_all_title',
    'Ingest All Logs',
    'Ingestar todos los registros',
    'Alle Protokolle einlesen',
    '导入所有日志',
    'Ingérer tous les journaux',
    'Ingerir todos os logs',
    'すべてのログの取り込み',
    'Nhập tất cả nhật ký',
    'Alle logs inlezen',
    'Összes napló betöltése',
    'استيعاب جميع السجلات',
    'دریافت همه لاگ‌ها',
    'Tüm günlükleri alma',
    '모든 로그 수집',
    'นำเข้าล็อกทั้งหมด',
    'မှတ်တမ်းအားလုံး ထည့်သွင်းခြင်း',
    'Бүх логуудыг оруулах',
    '',
    'Pobranie wszystkich logów');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_force_title',
    'Force Re-ingest',
    'Forzar reingesta',
    'Erneutes Einlesen erzwingen',
    '强制重新导入',
    'Forcer la réingestion',
    'Forçar reingestão',
    '強制再取り込み',
    'Buộc nhập lại',
    'Opnieuw inlezen forceren',
    'Újbóli betöltés kényszerítése',
    'فرض إعادة الاستيعاب',
    'اجبار دریافت مجدد',
    'Yeniden almayı zorla',
    '강제 재수집',
    'บังคับนำเข้าใหม่',
    'အတင်း ပြန်ထည့်သွင်းခြင်း',
    'Дахин оруулахыг шахах',
    '',
    'Wymuszone ponowne pobranie');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_force_body',
    'To re-process files that were already ingested:',
    'Para reprocesar archivos que ya fueron ingestados:',
    'Um bereits eingelesene Dateien erneut zu verarbeiten:',
    '要重新处理已导入的文件：',
    'Pour retraiter les fichiers déjà ingérés :',
    'Para reprocessar arquivos já ingeridos:',
    'すでに取り込まれたファイルを再処理するには：',
    'Để xử lý lại các tệp đã nhập:',
    'Om reeds ingelezen bestanden opnieuw te verwerken:',
    'A már betöltött fájlok újrafeldolgozásához:',
    'لإعادة معالجة الملفات التي تم استيعابها بالفعل:',
    'برای پردازش مجدد فایل‌هایی که قبلاً دریافت شده‌اند:',
    'Zaten alınmış dosyaları yeniden işlemek için:',
    '이미 수집된 파일을 재처리하려면:',
    'เพื่อประมวลผลไฟล์ที่นำเข้าแล้วอีกครั้ง:',
    'ထည့်သွင်းပြီးသားဖိုင်များကို ပြန်လည်လုပ်ဆောင်ရန်:',
    'Аль хэдийн оруулсан файлуудыг дахин боловсруулахын тулд:',
    '',
    'Aby ponownie przetworzyć pliki, które zostały już pobrane:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_how_title',
    'How Mining Works',
    'Cómo funciona la minería',
    'Wie Mining funktioniert',
    '挖掘的工作原理',
    'Comment fonctionne l''exploration',
    'Como a mineração funciona',
    'マイニングの仕組み',
    'Cách khai thác hoạt động',
    'Hoe mining werkt',
    'Hogyan működik a bányászat',
    'كيف يعمل التنقيب',
    'نحوه کار استخراج',
    'Madencilik nasıl çalışır',
    '마이닝 작동 방식',
    'การขุดทำงานอย่างไร',
    'တူးဖော်ခြင်း အလုပ်လုပ်ပုံ',
    'Олборлолт хэрхэн ажилладаг',
    '',
    'Jak działa eksploracja');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_how_body',
    '1. PRECC reads the session JSONL log file.
2. It identifies command pairs where the first command failed and the second was a corrected retry.
3. It extracts the pattern (what went wrong) and the fix (what Claude did differently).
4. Patterns are stored in `~/.local/share/precc/history.db`.
5. When a pattern reaches a confidence threshold (seen multiple times), it becomes a mined skill in `heuristics.db`.',
    '1. PRECC lee el archivo de registro JSONL de la sesión.
2. Identifica pares de comandos donde el primero falló y el segundo fue un reintento corregido.
3. Extrae el patrón (qué salió mal) y la corrección (qué hizo Claude de manera diferente).
4. Los patrones se almacenan en `~/.local/share/precc/history.db`.
5. Cuando un patrón alcanza un umbral de confianza (visto varias veces), se convierte en una habilidad minada en `heuristics.db`.',
    '1. PRECC liest die JSONL-Sitzungsprotokolldatei.
2. Es identifiziert Befehlspaare, bei denen der erste Befehl fehlschlug und der zweite ein korrigierter Versuch war.
3. Es extrahiert das Muster (was schiefging) und die Lösung (was Claude anders machte).
4. Muster werden in `~/.local/share/precc/history.db` gespeichert.
5. Wenn ein Muster einen Konfidenzschwellenwert erreicht (mehrfach gesehen), wird es zu einem gemeinten Skill in `heuristics.db`.',
    '1. PRECC读取会话JSONL日志文件。
2. 它识别命令对，其中第一个命令失败，第二个是纠正后的重试。
3. 它提取模式（出了什么问题）和修复（Claude做了什么不同的事）。
4. 模式存储在 `~/.local/share/precc/history.db` 中。
5. 当模式达到置信阈值（多次出现）时，它成为 `heuristics.db` 中的挖掘技能。',
    '1. PRECC lit le fichier journal JSONL de la session.
2. Il identifie les paires de commandes où la première a échoué et la seconde était une correction.
3. Il extrait le schéma (ce qui a mal tourné) et la correction (ce que Claude a fait différemment).
4. Les schémas sont stockés dans `~/.local/share/precc/history.db`.
5. Quand un schéma atteint un seuil de confiance (vu plusieurs fois), il devient une compétence minée dans `heuristics.db`.',
    '1. PRECC lê o arquivo de log JSONL da sessão.
2. Identifica pares de comandos onde o primeiro falhou e o segundo foi uma correção.
3. Extrai o padrão (o que deu errado) e a correção (o que Claude fez diferente).
4. Os padrões são armazenados em `~/.local/share/precc/history.db`.
5. Quando um padrão atinge um limiar de confiança (visto várias vezes), torna-se uma habilidade minerada em `heuristics.db`.',
    '1. PRECCはセッションJSONLログファイルを読み取ります。
2. 最初のコマンドが失敗し、2番目が修正されたリトライであるコマンドペアを特定します。
3. パターン（何が問題だったか）と修正（Claudeが何を変えたか）を抽出します。
4. パターンは `~/.local/share/precc/history.db` に保存されます。
5. パターンが信頼度の閾値に達すると（複数回確認）、`heuristics.db` のマイニングスキルになります。',
    '1. PRECC đọc tệp nhật ký JSONL của phiên.
2. Nó xác định các cặp lệnh trong đó lệnh đầu tiên thất bại và lệnh thứ hai là lần thử lại đã sửa.
3. Nó trích xuất mẫu (điều gì sai) và cách sửa (Claude đã làm gì khác).
4. Các mẫu được lưu trong `~/.local/share/precc/history.db`.
5. Khi một mẫu đạt ngưỡng tin cậy (gặp nhiều lần), nó trở thành kỹ năng khai thác trong `heuristics.db`.',
    '1. PRECC leest het JSONL-sessielogbestand.
2. Het identificeert opdrachtparen waarbij de eerste opdracht faalde en de tweede een gecorrigeerde herpoging was.
3. Het extraheert het patroon (wat er misging) en de fix (wat Claude anders deed).
4. Patronen worden opgeslagen in `~/.local/share/precc/history.db`.
5. Wanneer een patroon een betrouwbaarheidsdrempel bereikt, wordt het een ontgonnen vaardigheid in `heuristics.db`.',
    '1. A PRECC beolvassa a munkamenet JSONL naplófájlját.
2. Azonosítja azokat a parancspárokat, ahol az első parancs sikertelen volt és a második egy javított újrapróbálkozás.
3. Kivonja a mintát (mi ment rosszul) és a javítást (mit csinált Claude másképp).
4. A minták a `~/.local/share/precc/history.db` fájlban tárolódnak.
5. Ha egy minta eléri a megbízhatósági küszöböt, bányászott készséggé válik a `heuristics.db`-ben.',
    '1. يقرأ PRECC ملف سجل JSONL للجلسة.
2. يحدد أزواج الأوامر حيث فشل الأمر الأول وكان الثاني إعادة محاولة مصححة.
3. يستخرج النمط (ما الخطأ) والإصلاح (ما فعله Claude بشكل مختلف).
4. تُخزن الأنماط في `~/.local/share/precc/history.db`.
5. عندما يصل نمط إلى عتبة الثقة، يصبح مهارة مُستخرجة في `heuristics.db`.',
    '1. PRECC فایل لاگ JSONL جلسه را می‌خواند.
2. جفت دستوراتی را شناسایی می‌کند که دستور اول شکست خورده و دومی تلاش مجدد اصلاح‌شده بود.
3. الگو (چه اشتباهی رخ داد) و اصلاح (چه کاری متفاوت انجام شد) را استخراج می‌کند.
4. الگوها در `~/.local/share/precc/history.db` ذخیره می‌شوند.
5. وقتی الگویی به آستانه اطمینان برسد، به مهارت استخراج‌شده در `heuristics.db` تبدیل می‌شود.',
    '1. PRECC oturum JSONL günlük dosyasını okur.
2. İlk komutun başarısız olduğu ve ikincisinin düzeltilmiş bir yeniden deneme olduğu komut çiftlerini belirler.
3. Kalıbı (neyin yanlış gittiği) ve düzeltmeyi (Claude''un ne yaptığı) çıkarır.
4. Kalıplar `~/.local/share/precc/history.db` içinde saklanır.
5. Bir kalıp güven eşiğine ulaştığında, `heuristics.db` içinde kazılmış bir beceri olur.',
    '1. PRECC가 세션 JSONL 로그 파일을 읽습니다.
2. 첫 번째 명령이 실패하고 두 번째가 수정된 재시도인 명령 쌍을 식별합니다.
3. 패턴(무엇이 잘못되었는지)과 수정(Claude가 무엇을 다르게 했는지)을 추출합니다.
4. 패턴은 `~/.local/share/precc/history.db`에 저장됩니다.
5. 패턴이 신뢰도 임계값에 도달하면 `heuristics.db`의 마이닝 스킬이 됩니다.',
    '1. PRECC อ่านไฟล์ล็อก JSONL ของเซสชัน
2. ระบุคู่คำสั่งที่คำสั่งแรกล้มเหลวและคำสั่งที่สองเป็นการลองใหม่ที่แก้ไขแล้ว
3. แยกรูปแบบ (อะไรผิดพลาด) และการแก้ไข (Claude ทำอะไรต่างไป)
4. รูปแบบถูกเก็บใน `~/.local/share/precc/history.db`
5. เมื่อรูปแบบถึงเกณฑ์ความเชื่อมั่น จะกลายเป็นทักษะที่ขุดได้ใน `heuristics.db`',
    '1. PRECC သည် ဆက်ရှင် JSONL မှတ်တမ်းဖိုင်ကို ဖတ်ပါသည်။
2. ပထမအမိန့် မအောင်မြင်ပြီး ဒုတိယအမိန့်က ပြင်ဆင်ထားသော ပြန်ကြိုးစားမှုဖြစ်သည့် အမိန့်အတွဲများကို ခွဲခြားသည်။
3. ပုံစံ (ဘာမှားသလဲ) နှင့် ပြင်ဆင်မှု (Claude ဘာကွာသလဲ) ကို ထုတ်ယူသည်။
4. ပုံစံများကို `~/.local/share/precc/history.db` တွင် သိမ်းဆည်းသည်။
5. ပုံစံတစ်ခု ယုံကြည်မှုအဆင့်သို့ ရောက်သောအခါ `heuristics.db` ရှိ ကျွမ်းကျင်မှုဖြစ်လာသည်။',
    '1. PRECC нь сессийн JSONL лог файлыг уншина.
2. Эхний команд амжилтгүй болж, хоёр дахь нь засварласан дахин оролдлого болсон командын хосуудыг тодорхойлно.
3. Загвар (юу буруу болсон) болон засвар (Claude юуг өөрөөр хийсэн)-ыг гаргаж авна.
4. Загварууд `~/.local/share/precc/history.db`-д хадгалагдана.
5. Загвар итгэлийн босгод хүрэхэд `heuristics.db`-д олборлосон ур чадвар болно.',
    '',
    '1. PRECC czyta plik logu sesji JSONL.
2. Identyfikuje pary poleceń, w których pierwsze polecenie zawiodło, a drugie było poprawioną ponowną próbą.
3. Wyodrębnia wzorzec (co poszło nie tak) i poprawkę (co Claude zrobił inaczej).
4. Wzorce są przechowywane w `~/.local/share/precc/history.db`.
5. Gdy wzorzec osiągnie próg pewności, staje się wydobytą umiejętnością w `heuristics.db`.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_example_title',
    'Example Pattern',
    'Ejemplo de patrón',
    'Beispielmuster',
    '示例模式',
    'Exemple de schéma',
    'Exemplo de padrão',
    'パターンの例',
    'Ví dụ mẫu',
    'Voorbeeldpatroon',
    'Példa minta',
    'مثال على نمط',
    'نمونه الگو',
    'Örnek kalıp',
    '패턴 예시',
    'ตัวอย่างรูปแบบ',
    'ပုံစံ ဥပမာ',
    'Загварын жишээ',
    '',
    'Przykład wzorca');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_daemon_title',
    'The precc-learner Daemon',
    'El daemon precc-learner',
    'Der precc-learner-Daemon',
    'precc-learner 守护进程',
    'Le démon precc-learner',
    'O daemon precc-learner',
    'precc-learner デーモン',
    'Daemon precc-learner',
    'De precc-learner-daemon',
    'A precc-learner démon',
    'خدمة precc-learner',
    'دیمن precc-learner',
    'precc-learner arka plan hizmeti',
    'precc-learner 데몬',
    'Daemon precc-learner',
    'precc-learner daemon',
    'precc-learner демон',
    '',
    'Demon precc-learner');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_daemon_intro',
    'The `precc-learner` daemon runs in the background and watches for new session logs automatically:',
    'El daemon `precc-learner` se ejecuta en segundo plano y vigila automáticamente los nuevos registros de sesión:',
    'Der `precc-learner`-Daemon läuft im Hintergrund und überwacht automatisch neue Sitzungsprotokolle:',
    '`precc-learner` 守护进程在后台运行，自动监视新的会话日志：',
    'Le démon `precc-learner` s''exécute en arrière-plan et surveille automatiquement les nouveaux journaux de session :',
    'O daemon `precc-learner` roda em segundo plano e monitora automaticamente novos logs de sessão:',
    '`precc-learner` デーモンはバックグラウンドで実行され、新しいセッションログを自動的に監視します：',
    'Daemon `precc-learner` chạy nền và tự động theo dõi nhật ký phiên mới:',
    'De `precc-learner`-daemon draait op de achtergrond en bewaakt automatisch nieuwe sessielogs:',
    'A `precc-learner` démon a háttérben fut és automatikusan figyeli az új munkamenet-naplókat:',
    'يعمل خادم `precc-learner` في الخلفية ويراقب سجلات الجلسات الجديدة تلقائيًا:',
    'دیمن `precc-learner` در پس‌زمینه اجرا می‌شود و به‌طور خودکار لاگ‌های جلسه جدید را زیر نظر می‌گیرد:',
    '`precc-learner` arka plan hizmeti arka planda çalışır ve yeni oturum günlüklerini otomatik olarak izler:',
    '`precc-learner` 데몬은 백그라운드에서 실행되며 새 세션 로그를 자동으로 감시합니다:',
    'Daemon `precc-learner` ทำงานเบื้องหลังและเฝ้าดูล็อกเซสชันใหม่โดยอัตโนมัติ:',
    '`precc-learner` daemon သည် နောက်ခံတွင် အလုပ်လုပ်ပြီး ဆက်ရှင်မှတ်တမ်းအသစ်များကို အလိုအလျောက် စောင့်ကြည့်သည်:',
    '`precc-learner` демон нь ард дэвсгэрт ажиллаж, шинэ сессийн логуудыг автоматаар хянадаг:',
    '',
    'Demon `precc-learner` działa w tle i automatycznie obserwuje nowe logi sesji:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_daemon_notify',
    'The daemon uses file system notifications (inotify on Linux, FSEvents on macOS) so it reacts immediately when a session ends.',
    'El daemon usa notificaciones del sistema de archivos (inotify en Linux, FSEvents en macOS) por lo que reacciona inmediatamente cuando termina una sesión.',
    'Der Daemon verwendet Dateisystem-Benachrichtigungen (inotify auf Linux, FSEvents auf macOS) und reagiert daher sofort, wenn eine Sitzung endet.',
    '守护进程使用文件系统通知（Linux上的inotify，macOS上的FSEvents），因此在会话结束时立即做出反应。',
    'Le démon utilise les notifications du système de fichiers (inotify sous Linux, FSEvents sous macOS) et réagit donc immédiatement à la fin d''une session.',
    'O daemon usa notificações do sistema de arquivos (inotify no Linux, FSEvents no macOS) para reagir imediatamente quando uma sessão termina.',
    'デーモンはファイルシステム通知（LinuxではinotifyOSではFSEvents）を使用するため、セッション終了時に即座に反応します。',
    'Daemon sử dụng thông báo hệ thống tệp (inotify trên Linux, FSEvents trên macOS) nên phản ứng ngay khi phiên kết thúc.',
    'De daemon gebruikt bestandssysteemmeldingen (inotify op Linux, FSEvents op macOS) en reageert dus direct wanneer een sessie eindigt.',
    'A démon fájlrendszer-értesítéseket használ (inotify Linuxon, FSEvents macOS-en), így azonnal reagál, amikor egy munkamenet véget ér.',
    'يستخدم الخادم إشعارات نظام الملفات (inotify على Linux، FSEvents على macOS) فيتفاعل فورًا عند انتهاء الجلسة.',
    'دیمن از اعلان‌های سیستم فایل (inotify در لینوکس، FSEvents در macOS) استفاده می‌کند و بلافاصله پس از پایان جلسه واکنش نشان می‌دهد.',
    'Arka plan hizmeti dosya sistemi bildirimlerini (Linux''ta inotify, macOS''ta FSEvents) kullanır ve bir oturum sona erdiğinde hemen tepki verir.',
    '데몬은 파일 시스템 알림(Linux의 inotify, macOS의 FSEvents)을 사용하여 세션이 끝나면 즉시 반응합니다.',
    'Daemon ใช้การแจ้งเตือนระบบไฟล์ (inotify บน Linux, FSEvents บน macOS) จึงตอบสนองทันทีเมื่อเซสชันสิ้นสุด',
    'Daemon သည် ဖိုင်စနစ်အကြောင်းကြားချက်များ (Linux တွင် inotify, macOS တွင် FSEvents) ကို အသုံးပြုသောကြောင့် ဆက်ရှင်ပြီးဆုံးသောအခါ ချက်ချင်းတုံ့ပြန်သည်။',
    'Демон нь файлын системийн мэдэгдлүүдийг (Linux дээр inotify, macOS дээр FSEvents) ашигладаг тул сесс дуусахад шууд хариу үйлдэл хийнэ.',
    '',
    'Demon używa powiadomień systemu plików (inotify na Linuksie, FSEvents na macOS), więc reaguje natychmiast po zakończeniu sesji.');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_graduation_title',
    'From Patterns to Skills',
    'De patrones a habilidades',
    'Von Mustern zu Skills',
    '从模式到技能',
    'Des schémas aux compétences',
    'De padrões a habilidades',
    'パターンからスキルへ',
    'Từ mẫu đến kỹ năng',
    'Van patronen naar vaardigheden',
    'Mintáktól a készségekig',
    'من الأنماط إلى المهارات',
    'از الگوها به مهارت‌ها',
    'Kalıplardan becerilere',
    '패턴에서 스킬로',
    'จากรูปแบบสู่ทักษะ',
    'ပုံစံများမှ ကျွမ်းကျင်မှုများသို့',
    'Загвараас ур чадвар руу',
    '',
    'Od wzorców do umiejętności');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_graduation_body',
    'Mined patterns graduate to skills when they meet these criteria:

- Seen at least 3 times across sessions
- Consistent fix pattern (same type of correction each time)
- No false positives detected',
    'Los patrones minados se gradúan a habilidades cuando cumplen estos criterios:

- Vistos al menos 3 veces en diferentes sesiones
- Patrón de corrección consistente (mismo tipo de corrección cada vez)
- Sin falsos positivos detectados',
    'Geminte Muster werden zu Skills, wenn sie diese Kriterien erfüllen:

- Mindestens 3 Mal über Sitzungen hinweg gesehen
- Konsistentes Fix-Muster (gleiche Art der Korrektur jedes Mal)
- Keine Fehlalarme erkannt',
    '挖掘的模式在满足以下条件时升级为技能：

- 跨会话至少出现3次
- 一致的修复模式（每次相同类型的纠正）
- 未检测到误报',
    'Les schémas minés deviennent des compétences lorsqu''ils répondent à ces critères :

- Vus au moins 3 fois sur plusieurs sessions
- Schéma de correction cohérent (même type de correction à chaque fois)
- Aucun faux positif détecté',
    'Padrões minerados se graduam para habilidades quando atendem a estes critérios:

- Vistos pelo menos 3 vezes em sessões diferentes
- Padrão de correção consistente (mesmo tipo de correção a cada vez)
- Nenhum falso positivo detectado',
    'マイニングされたパターンは以下の条件を満たすとスキルに昇格します：

- セッション全体で少なくとも3回確認
- 一貫した修正パターン（毎回同じタイプの修正）
- 誤検出なし',
    'Các mẫu khai thác được nâng cấp thành kỹ năng khi đáp ứng các tiêu chí sau:

- Xuất hiện ít nhất 3 lần qua các phiên
- Mẫu sửa nhất quán (cùng loại sửa mỗi lần)
- Không phát hiện dương tính giả',
    'Ontgonnen patronen worden vaardigheden wanneer ze aan deze criteria voldoen:

- Minstens 3 keer gezien over sessies heen
- Consistent fix-patroon (elke keer hetzelfde type correctie)
- Geen valse positieven gedetecteerd',
    'A bányászott minták készségekké válnak, ha megfelelnek ezeknek a feltételeknek:

- Legalább 3-szor látták különböző munkamenetekben
- Konzisztens javítási minta (minden alkalommal azonos típusú javítás)
- Nem észleltek hamis pozitívot',
    'تتخرج الأنماط المستخرجة إلى مهارات عندما تستوفي هذه المعايير:

- شوهدت 3 مرات على الأقل عبر الجلسات
- نمط إصلاح متسق (نفس نوع التصحيح في كل مرة)
- لم يتم اكتشاف إيجابيات كاذبة',
    'الگوهای استخراج‌شده زمانی به مهارت ارتقا می‌یابند که این معیارها را برآورده کنند:

- حداقل 3 بار در جلسات مختلف دیده شده
- الگوی اصلاح سازگار (همان نوع اصلاح هر بار)
- هیچ مثبت کاذبی شناسایی نشده',
    'Kazılmış kalıplar bu kriterleri karşıladığında becerilere dönüşür:

- Oturumlar boyunca en az 3 kez görülmüş
- Tutarlı düzeltme kalıbı (her seferinde aynı tür düzeltme)
- Yanlış pozitif tespit edilmemiş',
    '마이닝된 패턴은 다음 기준을 충족하면 스킬로 승격됩니다:

- 세션 전체에서 최소 3회 확인
- 일관된 수정 패턴(매번 같은 유형의 수정)
- 오탐지 없음',
    'รูปแบบที่ขุดได้จะเลื่อนขั้นเป็นทักษะเมื่อตรงตามเกณฑ์เหล่านี้:

- พบอย่างน้อย 3 ครั้งข้ามเซสชัน
- รูปแบบการแก้ไขสม่ำเสมอ (การแก้ไขประเภทเดียวกันทุกครั้ง)
- ไม่พบผลบวกปลอม',
    'တူးဖော်ထားသော ပုံစံများသည် ဤစံနှုန်းများနှင့် ကိုက်ညီသောအခါ ကျွမ်းကျင်မှုများသို့ တိုးတက်သည်:

- ဆက်ရှင်များတွင် အနည်းဆုံး 3 ကြိမ် တွေ့မြင်ခဲ့
- တသမတ်တည်း ပြင်ဆင်မှုပုံစံ (တိုင်းတစ်ကြိမ် တူညီသော ပြင်ဆင်မှုအမျိုးအစား)
- အမှားရှာဖွေမှု မတွေ့ရှိ',
    'Олборлосон загварууд дараах шалгуурыг хангасан тохиолдолд ур чадвар болно:

- Сессүүдийн туршид дор хаяж 3 удаа харагдсан
- Тогтмол засварын загвар (болгондоо ижил төрлийн засвар)
- Хуурамч эерэг илрээгүй',
    '',
    'Wydobyte wzorce awansują do umiejętności, gdy spełniają te kryteria:

- Widziane co najmniej 3 razy w różnych sesjach
- Spójny wzorzec naprawy (ten sam typ korekty za każdym razem)
- Brak wykrytych fałszywych trafień');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_graduation_review',
    'You can review skill candidates with:',
    'Puedes revisar los candidatos a habilidades con:',
    'Sie können Skill-Kandidaten überprüfen mit:',
    '您可以通过以下方式查看技能候选：',
    'Vous pouvez examiner les candidats compétences avec :',
    'Você pode revisar candidatos a habilidades com:',
    'スキル候補は以下で確認できます：',
    'Bạn có thể xem xét các ứng viên kỹ năng với:',
    'U kunt vaardigheidskandidaten bekijken met:',
    'A készségjelölteket a következővel tekintheti át:',
    'يمكنك مراجعة مرشحي المهارات باستخدام:',
    'می‌توانید نامزدهای مهارت را بررسی کنید:',
    'Beceri adaylarını şu komutla inceleyebilirsiniz:',
    '스킬 후보를 다음으로 검토할 수 있습니다:',
    'คุณสามารถตรวจสอบผู้สมัครทักษะได้ด้วย:',
    'ကျွမ်းကျင်မှု ကိုယ်စားလှယ်လောင်းများကို ပြန်လည်စစ်ဆေးနိုင်သည်:',
    'Ур чадварын нэр дэвшигчдийг дараахаар шалгаж болно:',
    '',
    'Możesz przejrzeć kandydatów na umiejętności za pomocą:');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_graduation_see_skills',
    'See [Skills](skills.md) for details on managing skills.',
    'Consulte [Skills](skills.md) para detalles sobre la gestión de habilidades.',
    'Siehe [Skills](skills.md) für Details zur Verwaltung von Skills.',
    '有关管理技能的详细信息，请参见 [Skills](skills.md)。',
    'Voir [Skills](skills.md) pour les détails sur la gestion des compétences.',
    'Veja [Skills](skills.md) para detalhes sobre gerenciamento de habilidades.',
    'スキルの管理の詳細については [Skills](skills.md) を参照してください。',
    'Xem [Skills](skills.md) để biết chi tiết về quản lý kỹ năng.',
    'Zie [Skills](skills.md) voor details over het beheren van vaardigheden.',
    'Lásd a [Skills](skills.md) részt a készségek kezelésének részleteiért.',
    'راجع [Skills](skills.md) لتفاصيل إدارة المهارات.',
    'برای جزئیات مدیریت مهارت‌ها به [Skills](skills.md) مراجعه کنید.',
    'Becerileri yönetme ayrıntıları için [Skills](skills.md) bölümüne bakın.',
    '스킬 관리에 대한 자세한 내용은 [Skills](skills.md)를 참조하세요.',
    'ดู [Skills](skills.md) สำหรับรายละเอียดการจัดการทักษะ',
    'ကျွမ်းကျင်မှုများ စီမံခန့်ခွဲခြင်း အသေးစိတ်အတွက် [Skills](skills.md) ကို ကြည့်ပါ။',
    'Ур чадварыг удирдах дэлгэрэнгүйг [Skills](skills.md)-ээс харна уу.',
    '',
    'Szczegóły dotyczące zarządzania umiejętnościami znajdziesz w [Skills](skills.md).');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_storage_title',
    'Data Storage',
    'Almacenamiento de datos',
    'Datenspeicherung',
    '数据存储',
    'Stockage des données',
    'Armazenamento de dados',
    'データストレージ',
    'Lưu trữ dữ liệu',
    'Gegevensopslag',
    'Adattárolás',
    'تخزين البيانات',
    'ذخیره‌سازی داده',
    'Veri depolama',
    '데이터 저장',
    'การจัดเก็บข้อมูล',
    'ဒေတာ သိမ်းဆည်းခြင်း',
    'Өгөгдлийн хадгалалт',
    '',
    'Przechowywanie danych');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_storage_pairs',
    'Failure-fix pairs',
    'Pares de fallo-corrección',
    'Fehler-Fix-Paare',
    '失败-修复对',
    'Paires échec-correction',
    'Pares falha-correção',
    '失敗-修正ペア',
    'Cặp lỗi-sửa',
    'Fout-fix-paren',
    'Hiba-javítás párok',
    'أزواج الخطأ-الإصلاح',
    'جفت‌های خطا-اصلاح',
    'Hata-düzeltme çiftleri',
    '실패-수정 쌍',
    'คู่ข้อผิดพลาด-แก้ไข',
    'အမှား-ပြင်ဆင်မှု အတွဲများ',
    'Алдаа-засварын хосууд',
    '',
    'Pary awaria-naprawa');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_storage_skills',
    'Graduated skills',
    'Habilidades graduadas',
    'Graduierte Skills',
    '升级的技能',
    'Compétences promues',
    'Habilidades graduadas',
    '昇格したスキル',
    'Kỹ năng đã nâng cấp',
    'Gepromoveerde vaardigheden',
    'Előléptetett készségek',
    'المهارات المتخرجة',
    'مهارت‌های ارتقا یافته',
    'Terfi etmiş beceriler',
    '승격된 스킬',
    'ทักษะที่เลื่อนขั้น',
    'တိုးတက်ပြီးသော ကျွမ်းကျင်မှုများ',
    'Дэвшүүлсэн ур чадварууд',
    '',
    'Awansowane umiejętności');

INSERT OR REPLACE INTO strings (key, en, es, de, zh, fr, pt, ja, vi, nl, hu, ar, fa, tr, ko, th, my, mn, bo, pl) VALUES
('min_storage_note',
    'Both are SQLite databases in WAL mode for safe concurrent access.',
    'Ambas son bases de datos SQLite en modo WAL para acceso concurrente seguro.',
    'Beide sind SQLite-Datenbanken im WAL-Modus für sicheren gleichzeitigen Zugriff.',
    '两者都是WAL模式的SQLite数据库，用于安全的并发访问。',
    'Les deux sont des bases de données SQLite en mode WAL pour un accès concurrent sûr.',
    'Ambos são bancos de dados SQLite em modo WAL para acesso concorrente seguro.',
    'どちらもWALモードのSQLiteデータベースで、安全な並行アクセスが可能です。',
    'Cả hai đều là cơ sở dữ liệu SQLite ở chế độ WAL để truy cập đồng thời an toàn.',
    'Beide zijn SQLite-databases in WAL-modus voor veilige gelijktijdige toegang.',
    'Mindkettő SQLite adatbázis WAL módban a biztonságos párhuzamos hozzáféréshez.',
    'كلاهما قاعدتا بيانات SQLite في وضع WAL للوصول المتزامن الآمن.',
    'هر دو پایگاه داده SQLite در حالت WAL برای دسترسی همزمان امن هستند.',
    'Her ikisi de güvenli eşzamanlı erişim için WAL modunda SQLite veritabanlarıdır.',
    '둘 다 안전한 동시 접근을 위해 WAL 모드의 SQLite 데이터베이스입니다.',
    'ทั้งสองเป็นฐานข้อมูล SQLite ในโหมด WAL เพื่อการเข้าถึงพร้อมกันอย่างปลอดภัย',
    'နှစ်ခုစလုံး လုံခြုံသော တပြိုင်နက်ဝင်ရောက်မှုအတွက် WAL မုဒ်ဖြင့် SQLite ဒေတာဘေ့စ်များဖြစ်သည်။',
    'Хоёулаа аюулгүй зэрэгцээ хандалтын төлөө WAL горимтой SQLite мэдээллийн сан юм.',
    '',
    'Obie to bazy danych SQLite w trybie WAL dla bezpiecznego równoczesnego dostępu.');
