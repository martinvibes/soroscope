---
name: "🟢 Good First Issue: Refactor Error Codes"
about: Improve error handling clarity
title: "refactor: Use 'const' for Error Codes"
labels: ["good first issue", "refactoring"]
assignees: []

---

**Context**
`contracts/liquidity_pool/src/lib.rs` uses an enum with integer values.

**Task**
Refactor the `Error` enum to use explicit `#[repr(u32)]` if not already presenting clearly, or add helper methods to convert errors to readable strings for debugging.
