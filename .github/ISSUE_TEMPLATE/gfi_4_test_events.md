---
name: "🟢 Good First Issue: Verify Event Emission"
about: Verify events in Token contract
title: "test: Verify Event Emission in Token Contract"
labels: ["good first issue", "testing"]
assignees: []

---

**Context**
`contracts/token` emits events on transfer.

**Task**
Add a test case in `contracts/token/src/test.rs` that specifically asserts that the correct topics and data are published during a `transfer`.
