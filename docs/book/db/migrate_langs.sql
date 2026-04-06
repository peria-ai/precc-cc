-- Add new language columns: vi, ja, fr, pt
-- Drop pl column by recreating the table

CREATE TABLE IF NOT EXISTS strings_new (
    key   TEXT PRIMARY KEY,
    en    TEXT NOT NULL,
    es    TEXT NOT NULL DEFAULT '',
    de    TEXT NOT NULL DEFAULT '',
    zh    TEXT NOT NULL DEFAULT '',
    fr    TEXT NOT NULL DEFAULT '',
    pt    TEXT NOT NULL DEFAULT '',
    ja    TEXT NOT NULL DEFAULT '',
    vi    TEXT NOT NULL DEFAULT ''
);

INSERT OR IGNORE INTO strings_new (key, en, es, de, zh)
    SELECT key, en, es, de, zh FROM strings;

DROP TABLE strings;
ALTER TABLE strings_new RENAME TO strings;

-- Add translations for new languages (fr, pt, ja, vi)
-- We'll update the key strings that have translations

UPDATE strings SET
    fr = 'PRECC', pt = 'PRECC', ja = 'PRECC', vi = 'PRECC'
    WHERE key = 'product_name';

UPDATE strings SET
    fr = 'Correction prédictive des erreurs pour Claude Code',
    pt = 'Correção preditiva de erros para o Claude Code',
    ja = 'Claude Codeの予測的エラー修正',
    vi = 'Sửa lỗi dự đoán cho Claude Code'
    WHERE key = 'product_tagline';

UPDATE strings SET
    fr = 'latence moyenne inférieure à 3ms',
    pt = 'latência média inferior a 3ms',
    ja = '平均レイテンシ3ms未満',
    vi = 'độ trễ trung bình dưới 3ms'
    WHERE key = 'hook_latency_target';

UPDATE strings SET
    fr = 'Si le hook rencontre une erreur, il sort avec le code 0 instantanément — la commande originale s''exécute sans modification. Claude Code n''est jamais bloqué.',
    pt = 'Se o hook encontrar qualquer erro, ele sai com código 0 instantaneamente — o comando original é executado sem alterações. Claude Code nunca é bloqueado.',
    ja = 'フックがエラーに遭遇した場合、即座にコード0で終了します。元のコマンドはそのまま実行されます。Claude Codeがブロックされることはありません。',
    vi = 'Nếu hook gặp bất kỳ lỗi nào, nó thoát với mã 0 ngay lập tức — lệnh gốc chạy không thay đổi. Claude Code không bao giờ bị chặn.'
    WHERE key = 'fail_open_guarantee';

UPDATE strings SET
    fr = 'Détecte quand des commandes comme `cargo build` ou `npm test` sont exécutées dans le mauvais répertoire et ajoute `cd /bon/chemin &&` avant l''exécution.',
    pt = 'Detecta quando comandos como `cargo build` ou `npm test` são executados no diretório errado e adiciona `cd /caminho/correto &&` antes da execução.',
    ja = '`cargo build` や `npm test` などのコマンドが間違ったディレクトリで実行された場合を検出し、実行前に `cd /正しい/パス &&` を追加します。',
    vi = 'Phát hiện khi các lệnh như `cargo build` hoặc `npm test` chạy trong thư mục sai và thêm `cd /đường/dẫn/đúng &&` trước khi thực thi.'
    WHERE key = 'pillar1_desc';

UPDATE strings SET
    fr = 'Correction de contexte (cd-prepend)',
    pt = 'Correção de contexto (cd-prepend)',
    ja = 'コンテキスト修正 (cd-prepend)',
    vi = 'Sửa ngữ cảnh (cd-prepend)'
    WHERE key = 'pillar1_name';

UPDATE strings SET
    fr = 'Enveloppe les commandes dans [RTK](https://github.com/rtk-ai/rtk) pour comprimer la sortie CLI verbose de 60 à 90%. Au lieu de lire des centaines de lignes, Claude voit un résumé compact.',
    pt = 'Envolve comandos no [RTK](https://github.com/rtk-ai/rtk) para comprimir a saída CLI verbosa em 60–90%. Em vez de ler centenas de linhas, Claude vê um resumo compacto.',
    ja = '[RTK](https://github.com/rtk-ai/rtk)でコマンドをラップし、冗長なCLI出力を60〜90%圧縮します。Claudeは数百行の代わりにコンパクトなサマリーを見ます。',
    vi = 'Bọc các lệnh trong [RTK](https://github.com/rtk-ai/rtk) để nén đầu ra CLI dài dòng 60–90%. Thay vì đọc hàng trăm dòng, Claude thấy bản tóm tắt gọn.'
    WHERE key = 'rtk_desc';

UPDATE strings SET
    fr = 'Si une commande échoue après compression, PRECC ignore automatiquement la compression lors de la prochaine tentative pour que Claude obtienne la sortie complète non compressée pour le débogage.',
    pt = 'Se um comando falhar após compressão, PRECC automaticamente pula a compressão na próxima tentativa para que Claude obtenha a saída completa não comprimida para debug.',
    ja = 'コマンドが圧縮後に失敗した場合、PRECCは次のリトライで圧縮を自動的にスキップし、Claudeがデバッグ用の完全な非圧縮出力を取得できるようにします。',
    vi = 'Nếu một lệnh thất bại sau khi nén, PRECC tự động bỏ qua nén ở lần thử tiếp theo để Claude nhận được đầu ra đầy đủ không nén để gỡ lỗi.'
    WHERE key = 'adaptive_expand_desc';

UPDATE strings SET
    fr = 'Les chiffres sont des estimations. Chaque échec évité épargne un cycle complet de nouvelle tentative : sortie d''erreur, raisonnement du modèle et commande de nouvelle tentative.',
    pt = 'Os números são estimativas. Cada falha prevenida evita um ciclo completo de retry: saída de erro, raciocínio do modelo e comando de retry.',
    ja = '数値は推定値です。各予防された失敗は、エラー出力、モデルの推論、リトライコマンドの完全なリトライサイクルを回避します。',
    vi = 'Các con số là ước tính. Mỗi lỗi được ngăn chặn tránh được một chu trình thử lại hoàn chỉnh: đầu ra lỗi, suy luận của mô hình và lệnh thử lại.'
    WHERE key = 'token_model_note';

-- Introduction chapter i18n strings
UPDATE strings SET
    fr = 'Introduction', pt = 'Introdução', ja = 'はじめに', vi = 'Giới thiệu'
    WHERE key = 'intro_title';
UPDATE strings SET
    fr = 'Qu''est-ce que PRECC ?', pt = 'O que é PRECC?', ja = 'PRECCとは？', vi = 'PRECC là gì?'
    WHERE key = 'intro_what_is';
UPDATE strings SET
    fr = 'est un outil Rust qui intercepte les commandes bash de Claude Code via le mécanisme officiel de hooks PreToolUse. Il corrige les erreurs *avant qu''elles ne se produisent*, économisant des tokens et éliminant les boucles de nouvelle tentative.',
    pt = 'é uma ferramenta Rust que intercepta comandos bash do Claude Code via o mecanismo oficial de hooks PreToolUse. Corrige erros *antes que aconteçam*, economizando tokens e eliminando ciclos de retry.',
    ja = 'は、公式のPreToolUseフックメカニズムを通じてClaude Codeのbashコマンドを傍受するRustツールです。エラーを*発生する前に*修正し、トークンを節約してリトライループを排除します。',
    vi = 'là một công cụ Rust chặn các lệnh bash của Claude Code thông qua cơ chế hook PreToolUse chính thức. Nó sửa lỗi *trước khi chúng xảy ra*, tiết kiệm token và loại bỏ vòng lặp thử lại.'
    WHERE key = 'intro_what_is_body';
UPDATE strings SET
    fr = 'Le problème', pt = 'O problema', ja = '問題', vi = 'Vấn đề'
    WHERE key = 'intro_problem_title';
UPDATE strings SET
    fr = 'Claude Code gaspille des tokens significatifs sur des erreurs évitables :

- **Erreurs de répertoire** -- Exécuter `cargo build` dans un répertoire parent sans `Cargo.toml`, puis réessayer après avoir lu l''erreur.
- **Boucles de nouvelle tentative** -- Une commande échouée produit une sortie verbose, Claude la lit, raisonne et réessaie. Chaque cycle brûle des centaines de tokens.
- **Sortie verbose** -- Des commandes comme `find` ou `ls -R` produisent des milliers de lignes que Claude doit traiter.',
    pt = 'Claude Code desperdiça tokens significativos em erros evitáveis:

- **Erros de diretório** -- Executar `cargo build` em um diretório pai sem `Cargo.toml`, depois tentar novamente após ler o erro.
- **Ciclos de retry** -- Um comando falhado produz saída verbosa, Claude lê, raciocina e tenta novamente. Cada ciclo queima centenas de tokens.
- **Saída verbosa** -- Comandos como `find` ou `ls -R` produzem milhares de linhas que Claude precisa processar.',
    ja = 'Claude Codeは防げるミスで大量のトークンを浪費します：

- **ディレクトリエラー** -- `Cargo.toml`のない親ディレクトリで`cargo build`を実行し、エラーを読んでからリトライ。
- **リトライループ** -- 失敗したコマンドが冗長な出力を生成し、Claudeがそれを読み、推論し、リトライ。各サイクルで数百トークンを消費。
- **冗長な出力** -- `find`や`ls -R`のようなコマンドが数千行を出力し、Claudeが処理する必要がある。',
    vi = 'Claude Code lãng phí token đáng kể vào các lỗi có thể phòng tránh:

- **Lỗi thư mục** -- Chạy `cargo build` trong thư mục cha không có `Cargo.toml`, rồi thử lại sau khi đọc lỗi.
- **Vòng lặp thử lại** -- Lệnh thất bại tạo ra đầu ra dài dòng, Claude đọc, suy luận và thử lại. Mỗi chu trình đốt hàng trăm token.
- **Đầu ra dài dòng** -- Các lệnh như `find` hoặc `ls -R` xuất hàng nghìn dòng mà Claude phải xử lý.'
    WHERE key = 'intro_problem_body';
UPDATE strings SET
    fr = 'Les quatre piliers', pt = 'Os quatro pilares', ja = '4つの柱', vi = 'Bốn trụ cột'
    WHERE key = 'intro_pillars_title';
UPDATE strings SET
    fr = 'Débogage GDB', pt = 'Depuração GDB', ja = 'GDBデバッグ', vi = 'Gỡ lỗi GDB'
    WHERE key = 'pillar2_name';
UPDATE strings SET
    fr = 'Détecte les opportunités d''attacher GDB pour un débogage approfondi des segfaults et crashes, fournissant des informations de débogage structurées.',
    pt = 'Detecta oportunidades de anexar GDB para depuração profunda de segfaults e crashes, fornecendo informações de depuração estruturadas.',
    ja = 'セグフォルトやクラッシュの詳細なデバッグのためにGDBをアタッチする機会を検出し、構造化されたデバッグ情報を提供します。',
    vi = 'Phát hiện cơ hội gắn GDB để gỡ lỗi sâu hơn các segfault và crash, cung cấp thông tin gỡ lỗi có cấu trúc.'
    WHERE key = 'pillar2_desc';
UPDATE strings SET
    fr = 'Exploration de sessions', pt = 'Mineração de sessões', ja = 'セッションマイニング', vi = 'Khai thác phiên'
    WHERE key = 'pillar3_name';
UPDATE strings SET
    fr = 'Analyse les journaux de session Claude Code pour trouver des paires échec-correction. Quand la même erreur se reproduit, PRECC connaît déjà la correction et l''applique automatiquement.',
    pt = 'Analisa os logs de sessão do Claude Code em busca de pares falha-correção. Quando o mesmo erro recorre, PRECC já conhece a correção e a aplica automaticamente.',
    ja = 'Claude Codeのセッションログを分析して失敗-修正ペアを見つけます。同じミスが再発すると、PRECCはすでに修正方法を知っていて自動的に適用します。',
    vi = 'Khai thác nhật ký phiên Claude Code để tìm các cặp lỗi-sửa. Khi cùng một lỗi tái diễn, PRECC đã biết cách sửa và áp dụng tự động.'
    WHERE key = 'pillar3_desc';
UPDATE strings SET
    fr = 'Compétences d''automatisation', pt = 'Habilidades de automação', ja = '自動化スキル', vi = 'Kỹ năng tự động hóa'
    WHERE key = 'pillar4_name';
UPDATE strings SET
    fr = 'Une bibliothèque de compétences intégrées et découvertes qui correspondent aux modèles de commandes et les réécrivent. Définies comme fichiers TOML ou lignes SQLite.',
    pt = 'Uma biblioteca de habilidades integradas e mineradas que correspondem a padrões de comandos e os reescrevem. Definidas como arquivos TOML ou linhas SQLite.',
    ja = 'コマンドパターンにマッチして書き換える、組み込みおよびマイニングされたスキルのライブラリ。TOMLファイルまたはSQLite行として定義されます。',
    vi = 'Thư viện các kỹ năng tích hợp và khai thác khớp với mẫu lệnh và viết lại chúng. Được định nghĩa dưới dạng tệp TOML hoặc hàng SQLite.'
    WHERE key = 'pillar4_desc';
UPDATE strings SET
    fr = 'Comment ça marche (version 30 secondes)', pt = 'Como funciona (versão de 30 segundos)', ja = '仕組み（30秒バージョン）', vi = 'Cách hoạt động (phiên bản 30 giây)'
    WHERE key = 'intro_how_it_works_title';
UPDATE strings SET
    fr = '1. Claude Code est sur le point d''exécuter une commande bash.
2. Le hook PreToolUse envoie la commande à `precc-hook` en JSON sur stdin.
3. `precc-hook` traite la commande à travers le pipeline (compétences, correction de répertoire, compression) en moins de 3 millisecondes.
4. La commande corrigée est retournée en JSON sur stdout.
5. Claude Code exécute la commande corrigée.

Claude ne voit jamais l''erreur. Aucun token gaspillé.',
    pt = '1. Claude Code está prestes a executar um comando bash.
2. O hook PreToolUse envia o comando para `precc-hook` como JSON no stdin.
3. `precc-hook` processa o comando através do pipeline (habilidades, correção de diretório, compressão) em menos de 3 milissegundos.
4. O comando corrigido é retornado como JSON no stdout.
5. Claude Code executa o comando corrigido.

Claude nunca vê o erro. Nenhum token desperdiçado.',
    ja = '1. Claude Codeがbashコマンドを実行しようとしています。
2. PreToolUseフックがコマンドをJSON形式でstdinを通じて`precc-hook`に送信します。
3. `precc-hook`がパイプライン（スキル、ディレクトリ修正、圧縮）を通じて3ミリ秒未満でコマンドを処理します。
4. 修正されたコマンドがJSON形式でstdoutに返されます。
5. Claude Codeが修正されたコマンドを実行します。

Claudeはエラーを見ることがありません。トークンの無駄はありません。',
    vi = '1. Claude Code chuẩn bị chạy một lệnh bash.
2. Hook PreToolUse gửi lệnh tới `precc-hook` dưới dạng JSON qua stdin.
3. `precc-hook` xử lý lệnh qua pipeline (kỹ năng, sửa thư mục, nén) trong dưới 3 mili giây.
4. Lệnh đã sửa được trả về dưới dạng JSON qua stdout.
5. Claude Code thực thi lệnh đã sửa.

Claude không bao giờ thấy lỗi. Không có token nào bị lãng phí.'
    WHERE key = 'intro_how_it_works_body';
UPDATE strings SET
    fr = '### Compression adaptative', pt = '### Compressão adaptativa', ja = '### 適応圧縮', vi = '### Nén thích ứng'
    WHERE key = 'intro_adaptive_title';
UPDATE strings SET
    fr = 'Statistiques d''utilisation en direct', pt = 'Estatísticas de uso ao vivo', ja = 'リアルタイム使用統計', vi = 'Thống kê sử dụng trực tiếp'
    WHERE key = 'intro_live_stats_title';
UPDATE strings SET
    fr = 'Métrique', pt = 'Métrica', ja = '指標', vi = 'Chỉ số'
    WHERE key = 'stat_label';
UPDATE strings SET
    fr = 'Valeur', pt = 'Valor', ja = '値', vi = 'Giá trị'
    WHERE key = 'stat_value';
UPDATE strings SET
    fr = 'Invocations du hook', pt = 'Invocações do hook', ja = 'フック呼び出し', vi = 'Số lần gọi hook'
    WHERE key = 'stat_invocations';
UPDATE strings SET
    fr = 'Tokens économisés', pt = 'Tokens economizados', ja = '節約されたトークン', vi = 'Token đã tiết kiệm'
    WHERE key = 'stat_tokens_saved';
UPDATE strings SET
    fr = 'Ratio d''économie', pt = 'Taxa de economia', ja = '節約率', vi = 'Tỷ lệ tiết kiệm'
    WHERE key = 'stat_saving_pct';
UPDATE strings SET
    fr = 'Réécritures RTK', pt = 'Reescritas RTK', ja = 'RTK書き換え', vi = 'Viết lại RTK'
    WHERE key = 'stat_rtk_rewrites';
UPDATE strings SET
    fr = 'Corrections CD', pt = 'Correções CD', ja = 'CD修正', vi = 'Sửa CD'
    WHERE key = 'stat_cd_prepends';
UPDATE strings SET
    fr = 'Latence du hook', pt = 'Latência do hook', ja = 'フックレイテンシ', vi = 'Độ trễ hook'
    WHERE key = 'stat_latency';
UPDATE strings SET
    fr = 'Ces chiffres se mettent à jour automatiquement à partir de la télémétrie anonymisée.',
    pt = 'Esses números são atualizados automaticamente a partir de telemetria anonimizada.',
    ja = 'これらの数値は匿名化されたテレメトリから自動的に更新されます。',
    vi = 'Các con số này tự động cập nhật từ dữ liệu đo lường ẩn danh.'
    WHERE key = 'stats_live_note';
UPDATE strings SET
    fr = 'Liens', pt = 'Links', ja = 'リンク', vi = 'Liên kết'
    WHERE key = 'intro_links_title';
UPDATE strings SET
    fr = 'Site web', pt = 'Website', ja = 'ウェブサイト', vi = 'Trang web'
    WHERE key = 'website_label';
UPDATE strings SET
    fr = 'Documentation', pt = 'Documentação', ja = 'ドキュメント', vi = 'Tài liệu'
    WHERE key = 'docs_label';

-- Update free_forever: "Free for community users" instead of "Open source. Free forever."
UPDATE strings SET
    en = 'Free for community users.',
    es = 'Gratuito para usuarios de la comunidad.',
    de = 'Kostenlos für Community-Nutzer.',
    zh = '对社区用户免费。',
    fr = 'Gratuit pour les utilisateurs communautaires.',
    pt = 'Gratuito para usuários da comunidade.',
    ja = 'コミュニティユーザーは無料。',
    vi = 'Miễn phí cho người dùng cộng đồng.'
    WHERE key = 'free_forever';
