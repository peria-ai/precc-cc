---
description: Show token savings from grep→ccc semantic search redirections
---

Run `precc-ccc-savings.sh` to display the cocoindex-code savings report. This shows how many grep/rg commands were intercepted and redirected through AST-aware semantic search, and the resulting token savings.

If no savings are recorded yet, explain that savings are logged automatically when PRECC's hook intercepts grep/rg commands and redirects them through `ccc search` in projects with a cocoindex-code index (`.cocoindex_code/` directory).

To start collecting savings, the user should:
1. Run `ccc init && ccc index` in their project
2. Use Claude Code normally — grep/rg commands will be automatically redirected
