---
name: "🟡 Feature: Pausable Functionality"
about: Add emergency stop mechanism
title: "feat: Add Pausable Functionality"
labels: ["enhancement", "security"]
assignees: []

---

**Context**
Smart contracts often need an emergency stop.

**Task**
1. Add a `paused` boolean state to `LiquidityPool`. 
2. Add `set_paused(bool)` (admin only).
3. Ensure `deposit` and `swap` revert when paused.
