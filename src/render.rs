use anyhow::{Context, Result};

use crate::model::{EdgeKind, LensGraph};

pub fn render_json(graph: &LensGraph) -> Result<String> {
    serde_json::to_string_pretty(graph).context("could not serialize graph as JSON")
}

pub fn render_dot(graph: &LensGraph) -> String {
    let mut output = String::from(
        "digraph code_path_lens {\n  rankdir=LR;\n  node [shape=box, fontname=\"monospace\"];\n",
    );
    for node in &graph.nodes {
        output.push_str(&format!(
            "  \"{}\" [label=\"{}\\n{:?}\"];\n",
            dot_escape(&node.id),
            dot_escape(&node.label),
            node.kind
        ));
    }
    for edge in &graph.edges {
        let style = match edge.kind {
            EdgeKind::UnresolvedCall => "dashed",
            EdgeKind::UsesType => "dotted",
            _ => "solid",
        };
        output.push_str(&format!(
            "  \"{}\" -> \"{}\" [label=\"{:?}\", style={}];\n",
            dot_escape(&edge.from),
            dot_escape(&edge.to),
            edge.kind,
            style
        ));
    }
    output.push_str("}\n");
    output
}

pub fn render_html(graph: &LensGraph) -> Result<String> {
    let data = render_json(graph)?.replace("</", "<\\/");
    let title = html_escape(&format!("{} — Code Path Lens", graph.symbol));
    Ok(format!(
        r##"<!doctype html>
<html lang="en" data-theme="paper">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="light dark">
<title>{title}</title>
<style>
:root{{--paper:#e8ddc2;--ink:#172033;--muted:#5b6370;--rule:#384359;--call:#1859a9;--entry:#a83220;--type:#35622d;--boundary:#7a5100;--unresolved:#8c2c3a;--focus:#995f00;--panel:#f7f0de}}
[data-theme="night"]{{--paper:#111827;--ink:#f7f0de;--muted:#c1c8d4;--rule:#71809b;--call:#80b5ff;--entry:#ff8f79;--type:#a9dc96;--boundary:#f2c565;--unresolved:#ff9aa7;--focus:#f2c565;--panel:#1b263a}}
*{{box-sizing:border-box}} body{{margin:0;background:var(--paper);color:var(--ink);font:16px/1.55 ui-monospace,SFMono-Regular,Menlo,Consolas,monospace}} a{{color:var(--call)}} a:focus-visible,button:focus-visible,input:focus-visible{{outline:3px solid var(--focus);outline-offset:3px}} .skip{{position:absolute;left:12px;top:-80px;background:var(--ink);color:var(--paper);padding:12px;z-index:5}}.skip:focus{{top:12px}} header,main,footer{{width:min(1180px,calc(100% - 32px));margin:auto}}header{{padding:40px 0 24px;border-bottom:2px solid var(--rule)}}.eyebrow{{margin:0 0 8px;color:var(--entry);font-size:13px;text-transform:uppercase;letter-spacing:.12em}}h1{{font:700 clamp(32px,6vw,64px)/.98 Georgia,serif;margin:0;overflow-wrap:anywhere}}.notice{{max-width:72ch;color:var(--muted)}}.summary{{display:flex;gap:24px;flex-wrap:wrap;margin-top:20px}}.summary strong{{display:block;color:var(--ink);font-size:20px}}.summary span{{color:var(--muted);font-size:13px}}main{{padding:28px 0 64px}}.toolbar{{display:flex;align-items:end;gap:16px;flex-wrap:wrap;margin-bottom:24px}}label{{display:grid;gap:6px;font-size:13px;color:var(--muted)}}input{{min-height:44px;width:min(360px,80vw);border:2px solid var(--rule);background:var(--panel);color:var(--ink);padding:8px 12px;font:inherit}}button{{min-height:44px;border:2px solid var(--rule);border-radius:2px;background:var(--panel);color:var(--ink);padding:8px 14px;font:700 14px inherit;box-shadow:3px 3px 0 var(--rule);cursor:pointer}}button:active{{transform:translate(2px,2px);box-shadow:1px 1px 0 var(--rule)}}.layout{{display:grid;grid-template-columns:minmax(0,1.25fr) minmax(300px,.75fr);gap:24px}}.map{{min-height:420px;border:2px solid var(--rule);padding:24px;background:var(--panel);overflow:auto}}.nodes{{position:relative;display:grid;grid-template-columns:repeat(3,minmax(150px,1fr));gap:48px 28px;min-width:640px}}.node{{position:relative;text-align:left;min-height:88px;background:var(--paper);z-index:1}}.node small{{display:block;color:var(--muted);font-weight:400;margin-top:4px}}.node[data-kind="entry"]{{border-color:var(--entry);color:var(--entry)}}.node[data-kind="type"]{{border-style:dotted;border-color:var(--type)}}.node[data-kind="data_boundary"]{{border-color:var(--boundary)}}.node[data-kind="unresolved"]{{border-style:dashed;border-color:var(--unresolved)}}.node[aria-pressed="true"]{{box-shadow:5px 5px 0 var(--call)}}.key{{display:flex;gap:16px;flex-wrap:wrap;margin:0 0 20px;padding:0;list-style:none;color:var(--muted);font-size:13px}}.key span{{display:inline-block;width:14px;height:10px;border:2px solid currentColor;margin-right:5px}}.evidence{{border-left:2px solid var(--rule);padding-left:20px;min-width:0}}.evidence h2{{font:700 28px/1.1 Georgia,serif;margin:0 0 8px}}.meta,.reason{{color:var(--muted)}}pre{{white-space:pre;overflow:auto;background:var(--ink);color:var(--paper);padding:16px;font:13px/1.6 inherit;tab-size:2}}.edges{{margin-top:32px;border-top:2px solid var(--rule);padding-top:20px}}.edges li{{margin:10px 0}}.warnings{{margin:32px 0;padding:16px 20px;border:2px solid var(--boundary);background:color-mix(in srgb,var(--boundary) 8%,transparent)}}footer{{border-top:2px solid var(--rule);padding:24px 0 40px;color:var(--muted);font-size:13px}}[hidden]{{display:none!important}}@media(max-width:760px){{header{{padding-top:28px}}.layout{{grid-template-columns:1fr}}.map{{min-height:320px;padding:16px}}.evidence{{border-left:0;border-top:2px solid var(--rule);padding:20px 0 0}}}}@media(prefers-reduced-motion:no-preference){{.node{{animation:settle .42s both;animation-delay:calc(var(--i)*25ms)}}@keyframes settle{{from{{opacity:0;transform:translateY(8px)}}}}}}@media(prefers-reduced-motion:reduce){{*,*::before,*::after{{scroll-behavior:auto!important;animation-duration:.01ms!important;animation-iteration-count:1!important;transition-duration:.01ms!important}}}}
</style>
</head>
<body>
<a class="skip" href="#main">Skip to evidence</a>
<header>
  <p class="eyebrow">Code Path Lens / static review slice</p>
  <h1>{}</h1>
  <p class="notice">{}</p>
  <div class="summary" aria-label="Scan summary"><div><strong>{}</strong><span>nodes</span></div><div><strong>{}</strong><span>edges</span></div><div><strong>{}</strong><span>parsed files</span></div><div><strong>{}</strong><span>call depth</span></div></div>
</header>
<main id="main">
  <div class="toolbar"><label for="filter">Filter evidence<input id="filter" type="search" autocomplete="off" placeholder="Function, type, boundary…"></label><button id="theme" type="button" aria-pressed="false">Use night paper</button></div>
  <ul class="key" aria-label="Node key"><li><span style="color:var(--entry)"></span>Entry</li><li><span style="color:var(--call)"></span>Function</li><li><span style="color:var(--type)"></span>Type</li><li><span style="color:var(--boundary)"></span>Boundary</li><li><span style="color:var(--unresolved)"></span>Unresolved</li></ul>
  <div class="layout"><section class="map" aria-label="Bounded code path"><div id="nodes" class="nodes"></div><p id="empty" hidden>No nodes match this filter. Clear the filter to restore the path.</p></section><aside class="evidence" aria-live="polite"><h2 id="ev-title">Select evidence</h2><p id="ev-meta" class="meta">Use Tab or arrow keys to move through nodes, then Enter to inspect the source excerpt.</p><p id="ev-reason" class="reason"></p><a id="ev-link" hidden>Open source</a><pre id="ev-source" hidden></pre></aside></div>
  <section class="edges"><h2>Evidence ledger</h2><ol id="edge-list"></ol></section>
  <section class="warnings"><h2>Limits and warnings</h2><ul id="warning-list"></ul></section>
</main>
<footer>Generated locally by Code Path Lens 0.1.0. No source was uploaded. This is bounded static evidence, not a runtime trace.</footer>
<script id="lens-data" type="application/json">{data}</script>
<script>
const graph=JSON.parse(document.querySelector('#lens-data').textContent);const nodes=document.querySelector('#nodes'),filter=document.querySelector('#filter'),empty=document.querySelector('#empty');let buttons=[];
const kindLabel=s=>s.replaceAll('_',' ');function select(node,button){{buttons.forEach(b=>b.setAttribute('aria-pressed',b===button?'true':'false'));document.querySelector('#ev-title').textContent=node.label;document.querySelector('#ev-meta').textContent=[node.language,node.location&&`${{node.location.path}}:${{node.location.line}}`,kindLabel(node.kind)].filter(Boolean).join(' · ');document.querySelector('#ev-reason').textContent=node.evidence;const link=document.querySelector('#ev-link'),source=document.querySelector('#ev-source');if(node.location){{link.hidden=false;link.href=node.location.link;link.textContent=`Open ${{node.location.path}} at line ${{node.location.line}}`;}}else link.hidden=true;if(node.excerpt){{source.hidden=false;source.textContent=node.excerpt;}}else source.hidden=true;}}
graph.nodes.forEach((node,i)=>{{const b=document.createElement('button');b.type='button';b.className='node';b.dataset.kind=node.kind;b.style.setProperty('--i',i);b.setAttribute('aria-pressed','false');b.innerHTML=`<strong></strong><small></small>`;b.querySelector('strong').textContent=node.label;b.querySelector('small').textContent=kindLabel(node.kind)+(node.location?` · ${{node.location.path}}:${{node.location.line}}`:'');b.addEventListener('click',()=>select(node,b));b.addEventListener('keydown',e=>{{if(!['ArrowRight','ArrowDown','ArrowLeft','ArrowUp'].includes(e.key))return;e.preventDefault();const shown=buttons.filter(x=>!x.hidden),at=shown.indexOf(b),step=['ArrowRight','ArrowDown'].includes(e.key)?1:-1;shown[(at+step+shown.length)%shown.length]?.focus();}});nodes.append(b);buttons.push(b);}});if(graph.nodes[0])select(graph.nodes[0],buttons[0]);
graph.edges.forEach(edge=>{{const from=graph.nodes.find(n=>n.id===edge.from)?.label||edge.from,to=graph.nodes.find(n=>n.id===edge.to)?.label||edge.to,li=document.createElement('li');li.textContent=`${{from}} — ${{kindLabel(edge.kind)}} → ${{to}}. ${{edge.evidence}}`;document.querySelector('#edge-list').append(li);}});graph.warnings.forEach(w=>{{const li=document.createElement('li');li.textContent=w;document.querySelector('#warning-list').append(li);}});
function applyFilter(){{const q=filter.value.trim().toLowerCase();let shown=0;graph.nodes.forEach((node,i)=>{{const visible=!q||`${{node.label}} ${{node.kind}} ${{node.evidence}} ${{node.location?.path||''}}`.toLowerCase().includes(q);buttons[i].hidden=!visible;if(visible)shown++;}});empty.hidden=shown>0;}}filter.addEventListener('input',applyFilter);document.addEventListener('keydown',e=>{{if(e.key==='/'&&document.activeElement!==filter){{e.preventDefault();filter.focus();}}if(e.key==='Escape'&&document.activeElement===filter){{filter.value='';applyFilter();filter.blur();}}}});
document.querySelector('#theme').addEventListener('click',e=>{{const night=document.documentElement.dataset.theme!=='night';document.documentElement.dataset.theme=night?'night':'paper';e.currentTarget.setAttribute('aria-pressed',String(night));e.currentTarget.textContent=night?'Use paper theme':'Use night paper';}});
</script>
</body></html>"##,
        html_escape(&graph.symbol),
        html_escape(&graph.approximation_notice),
        graph.nodes.len(),
        graph.edges.len(),
        graph.scanned.parsed_files,
        graph.limits.depth,
    ))
}

fn dot_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

fn html_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use crate::model::{LensGraph, Limits, ScanSummary};

    use super::*;

    #[test]
    fn html_has_accessibility_basics_and_safe_title() {
        let graph = LensGraph {
            schema_version: 1,
            symbol: "<entry>".into(),
            root: "/tmp".into(),
            approximation_notice: "Static only".into(),
            limits: Limits {
                depth: 2,
                max_nodes: 40,
                truncated: false,
            },
            scanned: ScanSummary::default(),
            nodes: vec![],
            edges: vec![],
            warnings: vec![],
        };
        let html = render_html(&graph).unwrap();
        assert!(html.contains("<html lang=\"en\""));
        assert_eq!(html.matches("<h1>").count(), 1);
        assert!(html.contains("<main id=\"main\">"));
        assert!(html.contains("&lt;entry&gt;"));
    }
}
