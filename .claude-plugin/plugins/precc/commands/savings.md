---
description: Show PRECC analytics report — hook latency, skill activations, failure patterns, and cost savings
---

Run both of the following commands and display their combined output to the user:

1. `precc savings` — shows PRECC's estimated token savings from command correction and RTK output compression, including per-pillar breakdowns.

2. `precc-ccc-savings.sh` — shows token savings from grep/rg → cocoindex-code semantic search redirections (if any savings have been recorded).

Present both reports together as a unified savings overview. If `precc-ccc-savings.sh` is not installed or reports no data, just show the `precc savings` output alone.
