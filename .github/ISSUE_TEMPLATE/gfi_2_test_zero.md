---
name: "🟢 Good First Issue: Add Zero-Amount Deposit Test"
about: Improve test coverage for edge cases
title: "test: Add Zero-Amount Deposit Test"
labels: ["good first issue", "testing"]
assignees: []

---

**Context**
`contracts/liquidity_pool/src/test.rs` doesn't explicitly test depositing 0 tokens.

**Task**
Write a test case that attempts to call `deposit` with 0 amounts and assert the expected error or behavior.

**Resources**
- [Soroban Testing Guide](https://soroban.stellar.org/docs/fundamentals-and-concepts/testing)
