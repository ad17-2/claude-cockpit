import type { EntityType } from "$lib/commands/entities";

export function getTemplate(entityType: EntityType): string {
  switch (entityType) {
    case "agents":
      return `---
name:
description:
model: sonnet
tools:
  - Read
  - Grep
  - Glob
---

You are a specialized agent.

## Role


## Capabilities

`;
    case "rules":
      return `---
name:
description:
globs:
  - "**/*"
---

## Requirements

`;
    case "commands":
      return `---
name:
description:
---

#!/bin/bash
set -euo pipefail

`;
    case "skills":
      return `---
name:
description:
---

## Instructions

`;
    case "hooks":
      return `---
name:
description:
event: PostToolUse
---

#!/bin/bash
set -euo pipefail

`;
  }
}
