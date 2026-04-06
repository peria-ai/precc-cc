# 常見問題

## PRECC安全嗎？

是的。PRECC使用Claude Code官方的PreToolUse鉤子機制——Anthropic專門爲此目的設計的擴展點。該鉤子：

- 完全離線運行（熱路徑中無網絡調用）
- 在5毫秒內完成
- 是fail-open的：如果出現任何問題，原始命令將不受修改地運行
- 只修改命令，從不自己執行它們
- 將數據存儲在本地SQLite數據庫中

## PRECC能與其他AI編碼工具一起使用嗎？

PRECC專爲Claude Code設計。它依賴於Claude Code提供的PreToolUse鉤子協議。它不適用於Cursor、Copilot、Windsurf或其他AI編碼工具。

## 遙測發送什麼數據？

遙測僅在選擇加入後啓用。啓用後發送：

- PRECC版本、操作系統和架構
- 彙總計數（攔截的命令、激活的技能）
- 平均鉤子延遲

它**不**發送命令文本、文件路徑、項目名稱或任何個人身份信息。您可以在選擇加入前使用 `precc telemetry preview` 預覽確切的數據。詳見[遙測](telemetry.md)。

## 如何卸載PRECC？

??faq_uninstall_a_intro??

1. 移除鉤子註冊：
   ```bash
   # Delete the hook entry from Claude Code's settings
   # (precc init added it; removing it disables PRECC)
   ```

2. 刪除二進制文件：
   ```bash
   rm ~/.local/bin/precc ~/.local/bin/precc-hook ~/.local/bin/precc-learner
   ```

3. 刪除數據（可選）：
   ```bash
   rm -rf ~/.local/share/precc/
   rm -rf ~/.config/precc/
   ```

## 我的許可證過期了。會發生什麼？

PRECC恢復到社區版。所有核心功能繼續正常工作：

- 內置技能保持活躍
- 鉤子管道正常運行
- `precc savings` 顯示摘要視圖
- `precc ingest` 和會話挖掘正常工作

Pro功能在續訂前不可用：

- `precc savings --all`（詳細分類）
- `precc compress`
- `precc gif`
- `precc gha`
- `precc geofence`
- 電子郵件報告

## 鉤子似乎沒有運行。如何調試？

??faq_debug_a_intro??

1. 檢查鉤子是否已註冊：
   ```bash
   precc init
   ```

2. 手動測試鉤子：
   ```bash
   echo '{"tool_input":{"command":"cargo build"}}' | precc-hook
   ```

3. 檢查二進制文件是否在PATH中：
   ```bash
   which precc-hook
   ```

4. 檢查 `~/.claude/settings.json` 中的Claude Code鉤子配置。

## PRECC會減慢Claude Code嗎？

不會。鉤子在5毫秒內完成（p99）。與Claude推理和生成回覆所花費的時間相比，這是不可察覺的。

## 我可以在CI/CD中使用PRECC嗎？

PRECC是爲交互式Claude Code會話設計的。在CI/CD中，沒有Claude Code實例可以掛鉤。但是，`precc gha` 可以從任何環境分析失敗的GitHub Actions運行。

## 挖掘的技能與內置技能有何不同？

內置技能隨PRECC提供，涵蓋常見的錯誤目錄模式。挖掘的技能從您的特定會話日誌中學習——它們捕獲您工作流程中獨特的模式。兩者都存儲在SQLite中，並由鉤子管道以相同方式評估。

## 我可以與團隊共享技能嗎？

可以。使用 `precc skills export NAME` 將任何技能導出爲TOML並共享文件。團隊成員可以將其放在 `skills/` 目錄中或導入到他們的啓發式數據庫中。
