---
name: "🟡 Feature: Add 'burn'"
about: Add explicit burn function
title: "feat: Add 'burn' to Liquidity Pool"
labels: ["enhancement", "defi"]
assignees: []

---

**Context**
Users can withdraw, but a dedicated `burn` function that mirrors the token interface is missing.

**Task**
Implement `burn(from, amount)` which calls `withdraw` internally or shares logic.
