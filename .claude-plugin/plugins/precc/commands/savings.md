---
description: Show PRECC unified savings report — command correction, output compression, semantic search, and context compression
---

Run all of the following commands and display their combined output to the user as a unified savings overview:

1. `precc savings` — token savings from command correction and RTK output compression, including per-pillar breakdowns.

2. `precc-ccc-savings.sh` — token savings from grep/rg → cocoindex-code semantic search redirections.

3. `precc-ts-savings.sh` — token savings from context file compression (CLAUDE.md, memory files) via token-saver patterns.

If any command is not installed or reports no data, skip it and show the rest. Present a brief total at the end summarizing savings across all sources.
