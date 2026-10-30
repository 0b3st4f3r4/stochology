//! Competency DAG (`docs/competencies.toml`): the course as an acyclic graph of competencies instead of
//! fixed reading tracks or reader roles, with one verified pathway per negative defect proof (21 pathways)
//! and a cross-cutting practicum thread (Part VII) that runs across all of them.
//!
//! A **pathway** is the prerequisite closure of the competency that carries a negative proof, listed in
//! topological order; reading it end to end prepares the reader to explain why `gates -- --defect <name>`
//! exits with code `1`. The graph is an authorial pedagogical frame over the book (like `docs/SYLLABUS.md`),
//! not part of the protocol; [`CompetencyDag::audit`] keeps it honest against the book and the defect registry.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::defects::{ALL_DEFECTS, CapibaPillar};

/// Teaching depth of a competency: the 30-hour Core track or the 90-hour Full track.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompetencyDepth {
    Core,
    Full,
}

impl CompetencyDepth {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Core => "Core",
            Self::Full => "Full",
        }
    }
}

/// One competency: what the reader can do, where the book teaches it, and how it is evidenced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompetencyNode {
    pub id: String,
    pub title: String,
    /// Chapter tokens: two-digit chapter numbers (`"09"`) or appendix letters (`"B"`).
    pub chapters: Vec<String>,
    pub requires: Vec<String>,
    pub pair: String,
    pub depth: CompetencyDepth,
    pub evidence: String,
    pub defect: Option<String>,
}

/// The cross-cutting practicum thread: competencies without a negative proof that run across every pathway.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PracticumThread {
    pub summary: String,
    pub nodes: Vec<String>,
}

/// One pathway: the closure of the competency carrying `defect`, in topological order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pathway {
    pub defect: String,
    pub terminal: usize,
    pub steps: Vec<usize>,
    pub depth: CompetencyDepth,
}

/// A chapter or appendix of `book-src/SUMMARY.md`, in reading order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookChapter {
    pub token: String,
    pub file: String,
    pub label: String,
    pub part: String,
    pub order: usize,
}

/// Reading-order index of the book, parsed from `book-src/SUMMARY.md`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookIndex {
    pub chapters: Vec<BookChapter>,
}

impl BookIndex {
    /// Parses numbered chapters (`NN-*.md`) and lettered appendices (`appendix-x-*.md`) with their Part.
    pub fn from_summary(summary: &str) -> Self {
        let mut part = String::from("Preface");
        let mut chapters = Vec::new();
        for line in summary.lines() {
            let t = line.trim();
            if let Some(h) = t.strip_prefix("# ") {
                part = h.split('—').next().unwrap_or(h).trim().to_string();
                continue;
            }
            let Some(start) = t.find("](") else { continue };
            let Some(end) = t[start + 2..].find(')') else {
                continue;
            };
            let file = &t[start + 2..start + 2 + end];
            let (token, label) = if file.len() > 3
                && file.as_bytes()[..2].iter().all(u8::is_ascii_digit)
                && file.as_bytes()[2] == b'-'
            {
                let n: usize = file[..2].parse().unwrap_or(0);
                if n == 0 {
                    continue;
                }
                (file[..2].to_string(), format!("Ch {n}"))
            } else if let Some(rest) = file.strip_prefix("appendix-") {
                let letter = rest.split('-').next().unwrap_or_default();
                if letter.len() != 1 {
                    continue;
                }
                let up = letter.to_ascii_uppercase();
                (up.clone(), format!("App. {up}"))
            } else {
                continue;
            };
            chapters.push(BookChapter {
                token,
                file: file.to_string(),
                label,
                part: part.clone(),
                order: chapters.len(),
            });
        }
        Self { chapters }
    }

    /// Reads `book-src/SUMMARY.md` under `root`.
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join("book-src/SUMMARY.md");
        fs::read_to_string(&path)
            .map(|s| Self::from_summary(&s))
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))
    }

    pub fn get(&self, token: &str) -> Option<&BookChapter> {
        self.chapters.iter().find(|c| c.token == token)
    }
}

/// The parsed competency graph and its practicum thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompetencyDag {
    pub nodes: Vec<CompetencyNode>,
    pub practicum: PracticumThread,
}

/// Commands accepted as positive evidence of a competency.
const EVIDENCE_PREFIXES: [&str; 3] = [
    "cargo run --release --bin labs -- --",
    "cargo run --release --bin gates -- --audit-card ",
    "scripts/verify.sh",
];
const LAB_FLAGS: [&str; 8] = [
    "all",
    "layers",
    "uncertainty",
    "lenses",
    "invariants",
    "siga",
    "governance",
    "real-data",
];

fn list(v: &str) -> Vec<String> {
    v.split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn pillar_label(p: CapibaPillar) -> &'static str {
    match p {
        CapibaPillar::Calibrated => "C · Calibrated",
        CapibaPillar::AnytimeValid => "A · Anytime-valid",
        CapibaPillar::PerformativeAware => "P · Performative-aware",
        CapibaPillar::Invariant => "I · Invariant",
        CapibaPillar::Bitemporal => "B · Bitemporal",
        CapibaPillar::Auditing => "A · Auditing",
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

impl CompetencyDag {
    /// Parses the `[node.<id>]` and `[practicum.thread]` sections of `docs/competencies.toml`.
    pub fn parse(text: &str) -> Result<Self, String> {
        let mut sections: Vec<(String, String, BTreeMap<String, String>)> = Vec::new();
        for (i, raw) in text.lines().enumerate() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(h) = line.strip_prefix('[').and_then(|r| r.strip_suffix(']')) {
                let (kind, id) = h.split_once('.').ok_or_else(|| {
                    format!("line {}: section '[{h}]' lacks '<kind>.<id>'", i + 1)
                })?;
                sections.push((kind.to_string(), id.to_string(), BTreeMap::new()));
                continue;
            }
            let (k, v) = line
                .split_once('=')
                .ok_or_else(|| format!("line {}: expected 'key = value'", i + 1))?;
            let section = sections
                .last_mut()
                .ok_or_else(|| format!("line {}: key outside a section", i + 1))?;
            if section
                .2
                .insert(k.trim().to_string(), v.trim().to_string())
                .is_some()
            {
                return Err(format!("line {}: duplicate key '{}'", i + 1, k.trim()));
            }
        }

        let mut nodes = Vec::new();
        let mut practicum = PracticumThread::default();
        for (kind, id, map) in sections {
            let keys: &[&str] = match kind.as_str() {
                "node" => &[
                    "title", "chapters", "requires", "pair", "depth", "evidence", "defect",
                ],
                "practicum" if id == "thread" => &["summary", "nodes"],
                other => return Err(format!("unknown section kind '{other}' for '{id}'")),
            };
            if let Some(extra) = map.keys().find(|k| !keys.contains(&k.as_str())) {
                return Err(format!("[{kind}.{id}] has unknown key '{extra}'"));
            }
            let get = |k: &str| -> Result<String, String> {
                map.get(k)
                    .cloned()
                    .ok_or_else(|| format!("[{kind}.{id}] is missing '{k}'"))
            };
            if kind == "node" {
                let depth = match get("depth")?.as_str() {
                    "core" => CompetencyDepth::Core,
                    "full" => CompetencyDepth::Full,
                    other => return Err(format!("[node.{id}] depth '{other}' (use core or full)")),
                };
                let defect = get("defect")?;
                nodes.push(CompetencyNode {
                    id: id.clone(),
                    title: get("title")?,
                    chapters: list(&get("chapters")?),
                    requires: list(&get("requires")?),
                    pair: get("pair")?,
                    depth,
                    evidence: get("evidence")?,
                    defect: (!defect.is_empty()).then_some(defect),
                });
            } else {
                practicum = PracticumThread {
                    summary: get("summary")?,
                    nodes: list(&get("nodes")?),
                };
            }
        }
        Ok(Self { nodes, practicum })
    }

    /// Reads and parses `docs/competencies.toml` under `root`.
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join("docs/competencies.toml");
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
        Self::parse(&text)
    }

    pub fn index_of(&self, id: &str) -> Option<usize> {
        self.nodes.iter().position(|n| n.id == id)
    }

    fn prereq_indices(&self, i: usize) -> Vec<usize> {
        self.nodes[i]
            .requires
            .iter()
            .filter_map(|r| self.index_of(r))
            .collect()
    }

    /// Latest reading position among the node's chapters.
    fn reading_position(&self, i: usize, book: &BookIndex) -> usize {
        self.nodes[i]
            .chapters
            .iter()
            .filter_map(|t| book.get(t).map(|c| c.order))
            .max()
            .unwrap_or(usize::MAX)
    }

    /// Kahn topological order restricted to `subset`, breaking ties by reading position then file order.
    /// Returns `None` if the subset contains a cycle.
    fn topological(&self, subset: &BTreeSet<usize>, book: &BookIndex) -> Option<Vec<usize>> {
        let mut indegree: BTreeMap<usize, usize> = subset.iter().map(|&i| (i, 0)).collect();
        for &i in subset {
            for p in self.prereq_indices(i) {
                if subset.contains(&p) {
                    *indegree.get_mut(&i)? += 1;
                }
            }
        }
        let mut order = Vec::with_capacity(subset.len());
        let mut ready: BTreeSet<(usize, usize)> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(&i, _)| (self.reading_position(i, book), i))
            .collect();
        while let Some(&(pos, i)) = ready.iter().next() {
            ready.remove(&(pos, i));
            order.push(i);
            for &j in subset {
                if self.prereq_indices(j).contains(&i) {
                    let d = indegree.get_mut(&j)?;
                    *d -= 1;
                    if *d == 0 {
                        ready.insert((self.reading_position(j, book), j));
                    }
                }
            }
        }
        (order.len() == subset.len()).then_some(order)
    }

    /// All competencies `targets` depend on, including the targets themselves.
    pub fn closure(&self, targets: &[usize]) -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut stack: Vec<usize> = targets.to_vec();
        while let Some(i) = stack.pop() {
            if seen.insert(i) {
                stack.extend(self.prereq_indices(i));
            }
        }
        seen
    }

    /// The pathways, one per competency that carries a negative proof, in reading order of their terminal.
    pub fn pathways(&self, book: &BookIndex) -> Vec<Pathway> {
        let all: BTreeSet<usize> = (0..self.nodes.len()).collect();
        let order = self.topological(&all, book).unwrap_or_default();
        order
            .iter()
            .filter_map(|&t| {
                let defect = self.nodes[t].defect.clone()?;
                let steps = self.topological(&self.closure(&[t]), book)?;
                let depth = if steps
                    .iter()
                    .all(|&s| self.nodes[s].depth == CompetencyDepth::Core)
                {
                    CompetencyDepth::Core
                } else {
                    CompetencyDepth::Full
                };
                Some(Pathway {
                    defect,
                    terminal: t,
                    steps,
                    depth,
                })
            })
            .collect()
    }

    /// Competencies of the practicum thread together with their prerequisites.
    pub fn practicum_closure(&self) -> BTreeSet<usize> {
        let targets: Vec<usize> = self
            .practicum
            .nodes
            .iter()
            .filter_map(|p| self.index_of(p))
            .collect();
        self.closure(&targets)
    }

    /// Checks the graph against the book and the defect registry; returns every problem found.
    ///
    /// Verified: unique ids and known prerequisites; acyclicity; chapters exist and every numbered
    /// chapter plus Appendices A and B is taught by some competency; no prerequisite is read later
    /// than its dependent; edges form a transitive reduction; Core competencies require only Core ones;
    /// each of the 21 negative proofs is carried by exactly one competency (so 21 distinct pathways);
    /// evidence commands are runnable; the practicum thread names existing competencies without a
    /// negative proof; and every competency belongs to some pathway or to the practicum closure.
    pub fn audit(&self, book: &BookIndex) -> Vec<String> {
        let mut problems = Vec::new();
        let mut ids = BTreeSet::new();
        for n in &self.nodes {
            if !ids.insert(n.id.as_str()) {
                problems.push(format!("duplicate competency id '{}'", n.id));
            }
            if n.title.trim().is_empty() || n.chapters.is_empty() || n.pair.trim().is_empty() {
                problems.push(format!("'{}' needs a title, chapters, and a pair", n.id));
            }
            for r in &n.requires {
                if self.index_of(r).is_none() {
                    problems.push(format!("'{}' requires unknown competency '{r}'", n.id));
                }
                if r == &n.id {
                    problems.push(format!("'{}' requires itself", n.id));
                }
            }
            for t in &n.chapters {
                if book.get(t).is_none() {
                    problems.push(format!("'{}' cites unknown chapter '{t}'", n.id));
                }
            }
            let ev = n.evidence.as_str();
            let ev_ok = ev == "cargo run --release --bin gates"
                || ev
                    .strip_prefix(EVIDENCE_PREFIXES[0])
                    .is_some_and(|f| LAB_FLAGS.contains(&f))
                || ev
                    .strip_prefix(EVIDENCE_PREFIXES[1])
                    .is_some_and(|p| p.ends_with(".yaml"))
                || ev == EVIDENCE_PREFIXES[2];
            if !ev_ok {
                problems.push(format!("'{}' has unrunnable evidence '{ev}'", n.id));
            }
        }
        if !problems.is_empty() {
            return problems;
        }

        let all: BTreeSet<usize> = (0..self.nodes.len()).collect();
        if self.topological(&all, book).is_none() {
            problems.push("the competency graph has a cycle".into());
            return problems;
        }

        for c in &book.chapters {
            let numbered = c.token.len() == 2;
            let required_appendix = c.token == "A" || c.token == "B";
            if (numbered || required_appendix)
                && !self.nodes.iter().any(|n| n.chapters.contains(&c.token))
            {
                problems.push(format!(
                    "{} ({}) is taught by no competency",
                    c.label, c.file
                ));
            }
        }

        for (i, n) in self.nodes.iter().enumerate() {
            let pos = self.reading_position(i, book);
            for p in self.prereq_indices(i) {
                let pre = &self.nodes[p];
                if self.reading_position(p, book) > pos {
                    problems.push(format!(
                        "'{}' requires '{}', which the book teaches later",
                        n.id, pre.id
                    ));
                }
                if n.depth == CompetencyDepth::Core && pre.depth == CompetencyDepth::Full {
                    problems.push(format!(
                        "Core competency '{}' requires Full competency '{}'",
                        n.id, pre.id
                    ));
                }
                let others: Vec<usize> = self
                    .prereq_indices(i)
                    .into_iter()
                    .filter(|&q| q != p)
                    .collect();
                if self.closure(&others).contains(&p) {
                    problems.push(format!(
                        "edge '{}' -> '{}' is implied by another prerequisite (not a transitive reduction)",
                        pre.id, n.id
                    ));
                }
            }
        }

        let mut carried: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
        for n in &self.nodes {
            if let Some(d) = &n.defect {
                carried.entry(d.as_str()).or_default().push(n.id.as_str());
            }
        }
        for d in &ALL_DEFECTS {
            match carried.get(d.name).map(Vec::len) {
                Some(1) => {}
                Some(k) => problems.push(format!(
                    "negative proof '{}' carried by {k} competencies",
                    d.name
                )),
                None => problems.push(format!(
                    "negative proof '{}' is carried by no competency",
                    d.name
                )),
            }
        }
        for d in carried.keys() {
            if !ALL_DEFECTS.iter().any(|x| x.name == *d) {
                problems.push(format!("unknown negative proof '{d}'"));
            }
        }

        let pathways = self.pathways(book);
        let mut covered: BTreeSet<usize> = pathways
            .iter()
            .flat_map(|p| p.steps.iter().copied())
            .collect();
        for p in &self.practicum.nodes {
            match self.index_of(p) {
                Some(i) if self.nodes[i].defect.is_none() => {}
                Some(_) => {
                    problems.push(format!("practicum lists '{p}', but it carries a pathway"))
                }
                None => problems.push(format!("practicum names unknown competency '{p}'")),
            }
        }
        covered.extend(self.practicum_closure());
        for (i, n) in self.nodes.iter().enumerate() {
            if !covered.contains(&i) {
                problems.push(format!("'{}' belongs to no pathway and no preset", n.id));
            }
        }
        problems
    }

    fn chapter_link(&self, book: &BookIndex, token: &str, links: bool) -> String {
        match book.get(token) {
            Some(c) if links => format!("[{}]({})", c.label, c.file),
            Some(c) => c.label.clone(),
            None => token.to_string(),
        }
    }

    /// Chapters of a pathway in reading order of first appearance.
    fn route(&self, book: &BookIndex, steps: &[usize]) -> Vec<String> {
        let mut seen = BTreeSet::new();
        let mut tokens: Vec<&String> = Vec::new();
        for &s in steps {
            let mut chs: Vec<&String> = self.nodes[s].chapters.iter().collect();
            chs.sort_by_key(|t| book.get(t).map_or(usize::MAX, |c| c.order));
            for t in chs {
                if seen.insert(t.as_str()) {
                    tokens.push(t);
                }
            }
        }
        tokens.into_iter().cloned().collect()
    }

    /// Mermaid flowchart of the graph grouped by Part; Full competencies are dashed and competencies
    /// that carry a negative proof are drawn with a thick border.
    pub fn render_mermaid(&self, book: &BookIndex) -> String {
        let all: BTreeSet<usize> = (0..self.nodes.len()).collect();
        let order = self.topological(&all, book).unwrap_or_default();
        let mut parts: Vec<(String, Vec<usize>)> = Vec::new();
        for &i in &order {
            let part = self.nodes[i]
                .chapters
                .iter()
                .filter_map(|t| book.get(t))
                .max_by_key(|c| c.order)
                .map_or_else(|| "Other".to_string(), |c| c.part.clone());
            match parts.iter_mut().find(|(p, _)| *p == part) {
                Some((_, v)) => v.push(i),
                None => parts.push((part, vec![i])),
            }
        }
        parts.sort_by_key(|(_, v)| {
            v.iter()
                .map(|&i| self.reading_position(i, book))
                .min()
                .unwrap_or(usize::MAX)
        });
        let mut out = String::from("```mermaid\nflowchart TB\n");
        for (k, (part, members)) in parts.iter().enumerate() {
            out.push_str(&format!("  subgraph part{k}[\"{part}\"]\n"));
            for &i in members {
                let n = &self.nodes[i];
                let chs: Vec<String> = n
                    .chapters
                    .iter()
                    .map(|t| self.chapter_link(book, t, false))
                    .collect();
                let proof = n
                    .defect
                    .as_ref()
                    .map_or_else(String::new, |d| format!(" · ⊥ {d}"));
                out.push_str(&format!(
                    "    {}[\"{}<br/><small>{}{}</small>\"]\n",
                    n.id,
                    n.title.replace('"', "'"),
                    chs.join(", "),
                    proof
                ));
            }
            out.push_str("  end\n");
        }
        for &i in &order {
            for p in self.prereq_indices(i) {
                out.push_str(&format!(
                    "  {} --> {}\n",
                    self.nodes[p].id, self.nodes[i].id
                ));
            }
        }
        out.push_str("  classDef full stroke-dasharray: 5 4\n");
        out.push_str("  classDef proof stroke-width: 3px\n");
        let full: Vec<&str> = self
            .nodes
            .iter()
            .filter(|n| n.depth == CompetencyDepth::Full)
            .map(|n| n.id.as_str())
            .collect();
        let proof: Vec<&str> = self
            .nodes
            .iter()
            .filter(|n| n.defect.is_some())
            .map(|n| n.id.as_str())
            .collect();
        out.push_str(&format!("  class {} full\n", full.join(",")));
        out.push_str(&format!("  class {} proof\n", proof.join(",")));
        out.push_str("```\n");
        out
    }

    /// Markdown table of the pathways (one per negative proof) with pillar, depth, size, and route.
    pub fn render_pathway_table(&self, book: &BookIndex, links: bool) -> String {
        let mut out = String::from(
            "| # | Pathway (negative proof) | Pillar | Depth | Competencies | Route (chapters, in order) |\n| :-: | --- | --- | :-: | :-: | --- |\n",
        );
        for (k, p) in self.pathways(book).iter().enumerate() {
            let pillar = ALL_DEFECTS
                .iter()
                .find(|d| d.name == p.defect)
                .map_or("—", |d| pillar_label(d.pillar));
            let route: Vec<String> = self
                .route(book, &p.steps)
                .iter()
                .map(|t| self.chapter_link(book, t, links))
                .collect();
            out.push_str(&format!(
                "| {} | `{}` — {} | {} | {} | {} | {} |\n",
                k + 1,
                p.defect,
                self.nodes[p.terminal].title,
                pillar,
                p.depth.label(),
                p.steps.len(),
                route.join(" → ")
            ));
        }
        out
    }

    /// HTML cards for the preface hero (`.stoch-pathway-card`): one per CAPIBA pillar, linking the
    /// pathways whose negative proof belongs to it, plus one card for the practicum thread.
    pub fn render_pillar_cards(&self, book: &BookIndex) -> String {
        let pathways = self.pathways(book);
        let terminal_file = |i: usize| -> Option<String> {
            self.nodes[i]
                .chapters
                .iter()
                .filter_map(|t| book.get(t))
                .max_by_key(|c| c.order)
                .map(|c| c.file.replace(".md", ".html"))
        };
        let pillars = [
            (CapibaPillar::Calibrated, "calibrated", "C · Calibrated"),
            (CapibaPillar::AnytimeValid, "anytime", "A · Anytime-valid"),
            (
                CapibaPillar::PerformativeAware,
                "performative",
                "P · Performative-aware",
            ),
            (CapibaPillar::Invariant, "invariant", "I · Invariant"),
            (CapibaPillar::Bitemporal, "bitemporal", "B · Bitemporal"),
            (CapibaPillar::Auditing, "auditing", "A · Auditing"),
        ];
        let mut out = String::new();
        for (pillar, slug, label) in pillars {
            let mine: Vec<&Pathway> = pathways
                .iter()
                .filter(|p| {
                    ALL_DEFECTS
                        .iter()
                        .any(|d| d.name == p.defect && d.pillar == pillar)
                })
                .collect();
            let steps: BTreeSet<usize> =
                mine.iter().flat_map(|p| p.steps.iter().copied()).collect();
            out.push_str(&format!(
                "    <div class=\"stoch-pathway-card\" role=\"listitem\" data-pillar=\"{slug}\">\n      <div class=\"stoch-pathway-kicker\">PILLAR {}</div>\n      <p>{} pathways · {} competencies</p>\n      <div class=\"stoch-pathway-links\">\n",
                html_escape(&label.to_uppercase()),
                mine.len(),
                steps.len()
            ));
            for p in mine {
                if let Some(f) = terminal_file(p.terminal) {
                    out.push_str(&format!("        <a href=\"{f}\">⊥ {}</a>\n", p.defect));
                }
            }
            out.push_str("      </div>\n    </div>\n");
        }
        out.push_str(&format!(
            "    <div class=\"stoch-pathway-card\" role=\"listitem\" data-pillar=\"practicum\">\n      <div class=\"stoch-pathway-kicker\">PRACTICUM THREAD</div>\n      <p>{}</p>\n      <div class=\"stoch-pathway-links\">\n",
            html_escape(&self.practicum.summary)
        ));
        for p in &self.practicum.nodes {
            if let Some(i) = self.index_of(p) {
                if let Some(f) = terminal_file(i) {
                    out.push_str(&format!(
                        "        <a href=\"{f}\">{}</a>\n",
                        html_escape(&self.nodes[i].title)
                    ));
                }
            }
        }
        out.push_str("      </div>\n    </div>\n");
        out
    }
}

/// Replaces the text between `<!-- {marker}:begin -->` and `<!-- {marker}:end -->` with `body`.
pub fn replace_marked_block(doc: &str, marker: &str, body: &str) -> Result<String, String> {
    let begin = format!("<!-- {marker}:begin -->");
    let end = format!("<!-- {marker}:end -->");
    let b = doc
        .find(&begin)
        .ok_or_else(|| format!("marker '{begin}' not found"))?;
    let e = doc[b..]
        .find(&end)
        .map(|x| b + x)
        .ok_or_else(|| format!("marker '{end}' not found"))?;
    Ok(format!("{}{begin}\n{body}{}", &doc[..b], &doc[e..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUMMARY: &str = "# Part I — A\n- [One](01-one.md)\n- [Two](02-two.md)\n# Appendices\n- [Appendix A — X](appendix-a-x.md)\n- [Appendix B — Y](appendix-b-y.md)\n";

    fn node(id: &str, ch: &str, req: &str, depth: &str, defect: &str) -> String {
        format!(
            "[node.{id}]\ntitle = {id}\nchapters = {ch}\nrequires = {req}\npair = L1\ndepth = {depth}\nevidence = cargo run --release --bin gates\ndefect = {defect}\n"
        )
    }

    #[test]
    fn closure_order_and_reduction_checks_work_on_a_small_graph() {
        let book = BookIndex::from_summary(SUMMARY);
        assert_eq!(book.chapters.len(), 4);
        assert_eq!(book.get("B").map(|c| c.label.as_str()), Some("App. B"));
        let text = [
            node("a", "01", "", "core", "cromwell"),
            node("b", "02", "a", "core", ""),
            node("c", "A, B", "a, b", "full", ""),
        ]
        .concat();
        let dag = CompetencyDag::parse(&text).expect("parse");
        let problems = dag.audit(&book);
        assert!(problems.iter().any(|p| p.contains("transitive reduction")));
        assert!(
            problems
                .iter()
                .any(|p| p.contains("'phi5' is carried by no"))
        );
        let p = dag.pathways(&book);
        assert_eq!(p.len(), 1);
        assert_eq!(p[0].steps, vec![0]);
        assert_eq!(dag.closure(&[2]).len(), 3);
    }

    #[test]
    fn cycles_forward_edges_and_core_on_full_are_rejected() {
        let book = BookIndex::from_summary(SUMMARY);
        let cyc = [
            node("a", "01", "b", "core", ""),
            node("b", "02", "a", "core", ""),
        ]
        .concat();
        let dag = CompetencyDag::parse(&cyc).expect("parse");
        assert!(dag.audit(&book).iter().any(|p| p.contains("cycle")));
        let fwd = [
            node("a", "02", "", "full", ""),
            node("b", "01", "a", "core", ""),
        ]
        .concat();
        let dag = CompetencyDag::parse(&fwd).expect("parse");
        let problems = dag.audit(&book);
        assert!(problems.iter().any(|p| p.contains("teaches later")));
        assert!(problems.iter().any(|p| p.contains("requires Full")));
    }

    #[test]
    fn markers_are_replaced_in_place() {
        let doc = "x\n<!-- k:begin -->\nold\n<!-- k:end -->\ny";
        assert_eq!(
            replace_marked_block(doc, "k", "new\n").expect("markers"),
            "x\n<!-- k:begin -->\nnew\n<!-- k:end -->\ny"
        );
    }
}
