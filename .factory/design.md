# Code Path Lens — visual thesis

## Direction: the impossible field notebook

Code Path Lens is presented as surreal editorial scenery: a midnight surveyor's
field notebook in which source files become ochre strata, function calls become
coral survey pins, and unresolved edges disappear into cobalt doorways. The
imagery explains the product's central act—cutting a small, inspectable route
through a landscape that is too large to hold in working memory. It is not a
generic “AI developer tool” space scene and contains no chat or sparkle motifs.

The static viewer is the practical reverse side of that notebook: dense enough
for evidence, calm enough for review. Decoration stays on the landing page;
analysis output uses the same colors as restrained map notation.

## Palette

The site is intentionally single-mode, painted explicitly as a nocturnal paper
world. A second theme would weaken the authored editorial scene; the exported
viewer includes a high-contrast paper/night toggle for long review sessions.

| Token | Value | Role |
| --- | --- | --- |
| night | `#111827` | page ground, deep repository space |
| ink | `#F7F0DE` | primary text on night |
| vellum | `#E8DDC2` | paper surface |
| print | `#172033` | text on paper |
| ash | `#AAB2C0` | secondary text on night |
| rust | `#EE765E` | entry points, primary action |
| cobalt | `#6FA8FF` | calls and source links |
| moss | `#9BCB89` | types and successful states |
| ochre | `#E5B85C` | data boundaries and warnings |
| danger | `#FF8F91` | errors and invalid licenses |

All text pairings are checked at 4.5:1 or better; hue is always paired with a
label, shape, or line style.

## Type and spacing

Headlines use Georgia (`Georgia, 'Times New Roman', serif`) for the editorial
voice; interface and code use the native monospace stack
(`ui-monospace, SFMono-Regular, Menlo, Consolas, monospace`). Both are local
system fonts, avoiding network font traffic and keeping the entire font budget
at zero. Scale: 12 / 14 / 16 / 20 / 32 / clamp(48–80) px. Body text never drops
below 16 px. The base unit is 4 px, with primary gaps of 8, 16, 24, 40, 64,
and 96 px. Reading measure is 68 characters.

## Interaction grammar

Controls resemble small field labels: square corners softened by 2 px, a clear
2 px ink rule, and a 3 px offset shadow that collapses on press. Graph nodes are
not generic cards: they are numbered evidence slips joined by typed map lines.
Keyboard focus is a 3 px ochre ring with 3 px breathing room. Every control is
at least 44 px high. On a 390 px screen, the scenery is cropped, the proof strip
stacks, and the demo becomes a horizontally scrollable map with a textual edge
list immediately below it.

## Motion

The only narrative motion is a once-only 600 ms reveal: route segments draw
from the entry pin and evidence labels settle by 8 px. UI state changes take
160–220 ms and animate only opacity or transform. Nothing loops. Under
`prefers-reduced-motion: reduce`, drawing and settling are removed and all
states appear immediately.

## Original asset plan and provenance

One generated wide hero plate depicts a tiny coral survey route crossing
impossible dark-paper geological shelves and passing through a cobalt doorway.
It carries no text, logos, code screenshots, people, or watermark, leaving the
left third quiet for the actual HTML headline. It is generated specifically for
this product using `/opt/fleet/lib/gen-image.sh`, then cropped/converted to WebP
and held under 300 KB. Prompt and generator invocation are recorded in the
handoff and adjacent asset metadata. License: original commissioned output for
this product, used under the generator service terms. Interface marks, graph
lines, and icons are hand-authored CSS/SVG in the repository.
