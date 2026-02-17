---
name: "🔴 Test: Fuzz Testing Swap"
about: Advanced property-based testing
title: "test: Fuzz Testing for Swap Formula"
labels: ["testing", "advanced"]
assignees: []

---

**Context**
The CPMM formula in `swap` is critical.

**Task**
Create a property-based test (using `proptest` or a loop with random inputs) to verify that `k` (invariant) never decreases after a swap (ignoring fees).
