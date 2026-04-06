# 遙測

PRECC支持可選的匿名遙測以幫助改進工具。除非您明確同意，否則不會收集任何數據。

## 選擇加入

```bash
$ precc telemetry consent
[precc] Telemetry enabled. Thank you for helping improve PRECC.
[precc] You can revoke consent at any time with: precc telemetry revoke
```

## 選擇退出

```bash
$ precc telemetry revoke
[precc] Telemetry disabled. No further data will be sent.
```

## 檢查狀態

```bash
$ precc telemetry status
Telemetry: disabled
Last sent: never
```

## 預覽將發送的數據

在選擇加入之前，您可以查看將收集的確切數據：

```bash
$ precc telemetry preview
Telemetry payload (this session):
{
  "version": "0.3.0",
  "os": "linux",
  "arch": "x86_64",
  "skills_activated": 12,
  "commands_intercepted": 87,
  "pillars_used": [1, 4],
  "avg_hook_latency_ms": 2.3,
  "session_count": 1
}
```

## 收集的數據

- PRECC版本、操作系統和架構
- 彙總計數：攔截的命令、激活的技能、使用的支柱
- 平均鉤子延遲
- 會話數

## 不收集的數據

- 不收集命令文本或參數
- 不收集文件路徑或目錄名
- 不收集項目名稱或倉庫URL
- 不收集個人身份信息（PII）
- 不收集IP地址（服務器不記錄它們）

## 環境變量覆蓋

無需運行命令即可禁用遙測（適用於CI或共享環境）：

```bash
export PRECC_NO_TELEMETRY=1
```

這優先於同意設置。

## 數據目的地

遙測數據通過HTTPS發送到 `https://telemetry.peria.ai/v1/precc`。數據僅用於瞭解使用模式和確定開發優先級。
