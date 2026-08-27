# Handoff — Code Path Lens 0.1.0

## Independent verification status: FAIL

Candidate `85f6581b5525ea8a8f53796a2cac2e58fcd661be` is locally buildable and
the CLI/site checks pass, but **must not be released**. On 2026-08-27 the
required public URL `https://code-path-lens.sociobot.in/` failed TLS hostname
validation. Its Azure certificate has no `code-path-lens.sociobot.in` SAN, and
diagnostic-only `curl -k` returned `404 Site Not Found`. The live deployment
therefore cannot be matched to this candidate or used by a normal visitor.

The other high-severity release issue is that the current source hard-codes
`https://pilot-api.sociobot.in/api/v1` in both the site and paid CLI path.
There is no repository build configuration that switches it to the required
production Sociobot API.

Full exact evidence, commands, accessibility/mobile/offline results, package
consumer exercise, and next steps are in `.factory/verification.md`.

## How to build and verify locally

```sh
npm ci
cargo test
cargo clippy --all-targets --all-features -- -D warnings
npm test
npm run build
cargo package --allow-dirty
```

`npm run build` writes the release binary to `dist/bin/code-path-lens` and the
deployable static site to `dist/site/`. The Rust package was successfully
verified by `cargo package --allow-dirty`; do not publish it from this worker.

## Required release steps

1. Correct TLS/DNS and deploy the exact `dist/site/` artifact so verified HTTPS
   returns Code Path Lens at the public URL.
2. Configure the released site and CLI for `https://api.sociobot.in/api/v1`;
   verify the token-only paid-license flow.
3. Add/verify production cache and security response policy, then repeat live
   URL, header, browser, and Lighthouse checks.
