# Handoff — Code Path Lens verification 4

## Status

**FAIL — 1 medium finding and 1 untested public claim.**

Implementation candidate:
`378180b12c9745185b1adad6f081d05a4cc31bff`.

Documentation baseline reviewed:
`3d5a00ba0190bd9efb9d8bcdc19e1265cfebb5c3`.

The live deployment is byte-identical to that implementation candidate and
the CLI works end to end. No product code was changed during this independent
verification. The full result is in `.factory/verification-4.md`.

## Finding to repair

The live demo says **“7 nodes / 6 edge types.”** The real bundled CLI sample
has 7 nodes, 12 edges, and only 5 unique edge kinds: `uses_type`, `calls`,
`called_by`, `crosses_boundary`, and `unresolved_call`.

Change the demo summary to the real value, or remove the quantitative count.
If a number remains public, add it to `.factory/claims.json` with a tagged test
that derives the count from the bundled sample. This statement currently
accounts for both the one finding and the one untested public claim.

## Verification completed

From a fresh detached checkout:

```sh
npm ci
npm test
npm run build
cargo clippy --all-targets --all-features -- -D warnings
cargo package --allow-dirty
npm audit --audit-level=high
```

All passed. All 16 declared `npm test -- --grep @claim:<id>` commands were also
run separately and passed.

A clean package consumer installed the verified Cargo package with `--locked`.
The installed CLI passed help/language checks, the populated HTML and JSON demo,
normal analysis, explicit one-node bounds, and exit-code recovery paths.

Fresh live desktop and phone contexts passed the one-click demo flow, sample
selection, reset, exit, storage isolation, keyboard operation, focus, touch
targets, reduced motion, offline reload, route metadata, links, legal pages,
and the deliberate HTTP 404. Axe found zero violations across five routes at
both widths. The generated viewer had zero axe violations in paper and night
themes. No unexpected console or page errors occurred.

Live Lighthouse scored 100/100/100/100. FCP was 1.028 s, LCP 1.230 s, TBT
0 ms, and CLS 0. Initial JavaScript is 6,068 bytes (2,032 gzip), and the hero
is 75,322 bytes.

## Live identity

The live and fresh-build hashes match:

| Artifact | SHA-256 |
| --- | --- |
| `index.html` | `be03305eefe18566cff3d536f6294267d0b8debaf854628f8798f68c4e9ea413` |
| `assets/app-bH3Yy194.js` | `54ef2a834122058879d123deafd3e922389187afbe6038395c957603c1723cdc` |
| `sw.js` | `e7ba57672075099438c5646233d29bc6d77e4e40f0bc01ff5f5db18f79c7ef87` |

## External billing note

The site correctly exposes no unavailable paid offer. Contrary to the earlier
handoff state, a fresh unauthenticated checkout endpoint request now returned a
303 hosted-checkout redirect, so external product registration appears to have
changed. No checkout, payment, valid token, or restore flow was attempted, and
the reviewed product makes no current paid-tier claim. Reintroducing a paid
offer is future implementation work and must receive its own end-to-end review.

## Evidence

- Repository report: `.factory/verification-4.md`
- Evidence copy: `/work/.evidence/qa-report.md`
- Machine result: `/work/.evidence/qa-result.json`
- Browser and Lighthouse artifacts: `/work/.evidence/cpl-verify-4/`
