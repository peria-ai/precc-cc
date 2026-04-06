# 簡介

## 什麼是PRECC？

PRECC (Claude Code 預測性錯誤糾正) 是一個Rust工具，通過官方的PreToolUse鉤子機制攔截Claude Code的bash命令。它在錯誤*發生之前*修復它們，節省token並消除重試循環。

對社區用戶免費。

## 問題

Claude Code在可預防的錯誤上浪費大量token：

- **目錄錯誤** -- 在沒有 `Cargo.toml` 的父目錄中運行 `cargo build`，然後在讀取錯誤後重試。
- **重試循環** -- 失敗的命令產生冗長的輸出，Claude讀取、推理並重試。每個循環消耗數百個token。
- **冗長輸出** -- `find` 或 `ls -R` 等命令輸出數千行，Claude必須處理這些內容。

## 四大支柱

### 上下文修復 (cd-prepend)

檢測到 `cargo build` 或 `npm test` 等命令在錯誤的目錄中運行時，在執行前添加 `cd /正確/路徑 &&`。

### GDB調試

檢測附加GDB進行更深入調試的機會，提供結構化的調試信息而不是原始的核心轉儲。

### 會話挖掘

挖掘Claude Code會話日誌中的失敗-修復對。當同樣的錯誤再次發生時，PRECC已經知道修復方法並自動應用。

### 自動化技能

內置和挖掘技能庫，匹配命令模式並重寫它們。技能定義爲TOML文件或SQLite行，便於檢查、編輯和共享。

## 工作原理（30秒版本）

1. Claude Code即將運行一個bash命令。
2. PreToolUse鉤子將命令作爲JSON通過stdin發送給 `precc-hook`。
3. `precc-hook` 在3毫秒內通過管道（技能、目錄修正、壓縮）處理命令。
4. 修正後的命令作爲JSON通過stdout返回。
5. Claude Code執行修正後的命令。

Claude永遠看不到錯誤。沒有token浪費。

### 自適應壓縮

如果命令在壓縮後失敗，PRECC會自動在重試時跳過壓縮，以便Claude獲得完整的未壓縮輸出來除錯。

## 實時使用統計

當前版本 <span data-stat="current_version">--</span>:

| 指標 | 值 |
|---|---|
| 鉤子調用次數 | <span data-stat="total_invocations">--</span> |
| 節省的token | <span data-stat="total_tokens_saved">--</span> |
| 節省比率 | <span data-stat="saving_pct">--</span>% |
| RTK重寫 | <span data-stat="rtk_rewrites">--</span> |
| CD修正 | <span data-stat="cd_prepends">--</span> |
| 鉤子延遲 | <span data-stat="avg_latency_p50_ms">--</span> ms (p50) |
| 獨立用戶 | <span data-stat="unique_users">--</span> |

### Measured Savings (Ground Truth)

<div id="measured-savings" style="display:none">
<table id="measured-summary">
<thead><tr><th>指標</th><th>值</th></tr></thead>
<tbody>
<tr><td>Original output tokens (without PRECC)</td><td><span data-measured="original_output_tokens">--</span></td></tr>
<tr><td>Actual output tokens (with PRECC)</td><td><span data-measured="actual_output_tokens">--</span></td></tr>
<tr><td>節省的token</td><td><strong><span data-measured="savings_tokens">--</span></strong></td></tr>
<tr><td>節省比率</td><td><strong><span data-measured="savings_pct">--</span>%</strong></td></tr>
<tr><td>Ground-truth measurements</td><td><span data-measured="ground_truth_count">--</span> measurements</td></tr>
</tbody>
</table>
</div>

<div id="rewrite-type-breakdown" style="display:none">

#### By Rewrite Type

<table id="rewrite-type-table">
<thead><tr><th>Type</th><th>Count</th><th>Avg Savings %</th><th>節省的token</th></tr></thead>
<tbody><tr><td colspan="4"><em>載入中...</em></td></tr></tbody>
</table>
</div>

### 各版本節省情況

<table id="version-breakdown" style="display:none">
<thead><tr><th>版本</th><th>獨立用戶</th><th>鉤子調用次數</th><th>節省的token</th><th>節省比率</th></tr></thead>
<tbody><tr><td colspan="5"><em>載入中...</em></td></tr></tbody>
</table>

<small>這些數字會從匿名遙測數據自動更新。</small>

## 鏈接

- GitHub: [https://github.com/peria-ai/precc-cc](https://github.com/peria-ai/precc-cc)
- 網站: [https://peria.ai](https://peria.ai)
- 文檔: [https://precc.cc](https://precc.cc)
