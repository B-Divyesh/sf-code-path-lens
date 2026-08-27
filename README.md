# Code Path Lens

Code Path Lens makes a small, deterministic review slice around one symbol. It
finds callers, callees, referenced types, data boundaries, source excerpts, and
unresolved calls without uploading a repository or inventing runtime behavior.
It is for developers reviewing an unfamiliar Rust, TypeScript/JavaScript,
Python, or Go path who want bounded evidence instead of another index.

> Static analysis is an approximation. Dynamic dispatch, reflection, generated
> code, and runtime configuration can change the actual path. The lens labels
> unresolved evidence and never claims runtime completeness.

## Install

Download a release binary, or build from source with Rust 1.85+:

```sh
cargo install --path .
```

## Usage

Create a self-contained HTML review slice (free: depth ≤ 2, nodes ≤ 40):

```sh
code-path-lens trace handle_order --root ./shop --output order-lens.html
```

Export stable JSON for scripts, or Graphviz DOT:

```sh
code-path-lens trace handle_order --root ./shop --json > order-lens.json
code-path-lens trace handle_order --root ./shop --format dot --output order-lens.dot
```

Inspect supported adapters and explicit fallback behavior:

```sh
code-path-lens languages
```

Useful options:

```text
--depth <0..8>          Call steps in each direction (default 2)
--max-nodes <1..250>   Hard graph budget (default 40)
--exclude <GLOB>       Repeatable extra exclusion
--include-generated    Include files marked as generated
--link-template <TEXT> Source URL with {path}, {abs}, and {line} tokens
--license <TOKEN>      Verify a Pro token without storing source or token
```

The Pro unlock is a one-time $29 purchase that raises the caps to depth 8 and
250 nodes. It changes limits only: HTML, JSON, DOT, source evidence, keyboard
support, and accessibility are free. Verification sends only the license token
to Sociobot; repository contents remain local. You can also set
`CODE_PATH_LENS_LICENSE` to avoid shell history.

Exit codes are `0` success, `1` analysis/I/O failure, `2` invalid arguments,
`3` symbol not found or ambiguous, and `4` a paid limit needs a valid license.
The CLI never prompts, making it safe in CI.

## Viewer keyboard map

- `Tab`: reach filters, nodes, source links, and theme control.
- Arrow keys: move between graph nodes.
- `Enter`: select a node and reveal its evidence excerpt.
- `/`: focus the node filter; `Escape`: clear it.

## Develop and verify

```sh
npm install
npm test
npm run build            # release CLI + site -> dist/
npm run build:site       # landing/docs site -> dist/site/
cargo package --allow-dirty
```

`npm run dev` serves the site. The package contains no telemetry, runtime CDN,
or cloud indexer. `.gitignore` and common generated/vendor directories are
respected by default.

## Deploy

The factory deploys `dist/site/` to
<https://code-path-lens.sociobot.in>. Do not deploy infrastructure from this
repository.

## License

MIT © 2026 Sociobot (Param Factory). See [LICENSE](LICENSE).
