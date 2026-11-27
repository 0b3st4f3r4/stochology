#!/usr/bin/env python3
"""Custody-tier, anti-DoS bloat, and Git provenance oracle for Stochology."""
from __future__ import annotations
import argparse
from dataclasses import asdict, dataclass
import difflib
import html
import json
import os
from pathlib import Path
import posixpath
import re
import subprocess
import sys
import unicodedata
import urllib.error
import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET
_SCRIPT_DIR = str(Path(__file__).resolve().parent)
if _SCRIPT_DIR not in sys.path:
  sys.path.insert(0, _SCRIPT_DIR)
from _custody_identity import (  # noqa: E402
  AI_MODEL_COMBO_RE, AMBIGUOUS_SINGLE_AI_RE, CAMEL_AI_BRAND_RE, CAMEL_BOT_RE,
  CANONICAL_SPECIALIST_NAMES, COAUTHOR_TRAILER_RE, CREDENTIAL_SUFFIX_RE,
  EMAIL_TOKEN_RE, IDENTITY_QUOTE_RE, LEONTIEF_VERDICT_RE, NON_HUMAN_SIGNOFF_RE,
  NOREPLY_EMAIL_RE, NUMBERED_BOT_SUFFIX_RE, PLACEHOLDER_RE,
  SPECIALIST_HEADER_RE, TARGET_FILES_HEADER_RE, UPPER_AI_RE,
  _AI_SEG_DIGIT_RE, _AI_WORD_DIGIT_RE, _canonical_display_name,
  _expand_simple_braces, _extract_coauthors, _extract_email_identities,
  _is_non_human_reviewer, _norm_identity, _parse_reviewers,
  _reviewer_equiv_key, _reviewer_identity_keys, _split_address_list,
  _split_target_tokens, _strip_delimited_groups, _strip_invisible_and_fold,
  _target_header_covers_path as _id_target_header_covers_path,
  _unwrap_md_links,
  _validate_blind_review_artifact as _id_validate_blind_review_artifact,
  _CM_UNESC_RE,
)
_rc, _rm, _rs, _ru, _rfa, _rsp = re.compile, re.match, re.search, re.sub, re.findall, re.split
_rik, _eei, _ni, _nhr, _rek = _reviewer_identity_keys, _extract_email_identities, _norm_identity, _is_non_human_reviewer, _reviewer_equiv_key
_ne = lambda s: [ln.strip() for ln in _lf_lines(s) if ln.strip()]
_is_rev_md = lambda x: x.startswith("reviews/") and x.endswith(".md")
_ID_TOK_RE = _rc(r"[\s@\-_./:|+]+")
_GEN_LOC, _SLD = frozenset("noreply no-reply admin info dev root support contact team mail security bot ci".split()), frozenset("co ac gov edu org com net".split())
def _ep(em):
  if "@" not in em or em.endswith("@users.noreply.github.com"): return set(), set(), set()
  loc, dom = em.lower().split("@", 1)
  dp = [p for p in dom.split(".") if p]
  db = dp[:-2] if (len(dp) >= 3 and dp[-2] in _SLD) else (dp[:-1] if len(dp) >= 2 else dp)
  lt = {t for t in _ID_TOK_RE.split(loc.split("+", 1)[0]) if len(t) >= 2 and t not in _GEN_LOC}
  dt = {t for p in db for t in _ID_TOK_RE.split(p) if len(t) >= 2 and t not in _GEN_LOC}
  return lt, dt, lt | dt
def _mk_actor_matcher(blk, bk):
  b1, bw_a, b_s, e_s = set(), set(), [], []
  def _st(s):
    s0 = _ru(r"<[^>]+>", "", s)
    if not _rs(r"<[^>]*@[^>]*>", s): s0 = EMAIL_TOKEN_RE.sub("", s0)
    if not (bc := _canonical_display_name(s0.strip()).lower()): return "", [], []
    a = [t for w in bc.split() for t in _ID_TOK_RE.split(w) if t]
    return bc, a, [t for t in a if len(t) >= 3]
  for b in list(blk):
    if (ek := _rek(b)): bk.add(ek)
    bk.update(_rik(b))
    bc, a, subs = _st(b)
    if bc:
      bk.add(bc)
      if len(ws := bc.split()) >= 3: bk.add(f"{ws[0]} {ws[-1]}")
      if len(a) == 1 and subs: b1.add(subs[0])
      elif subs: bw_a.update(subs)
      if len(ns := {t for t in a if len(t) >= 2 and t not in _GEN_LOC}) >= 2: b_s.append(ns)
    for em in _eei(b):
      lt, dt, et = _ep(em)
      if len(et) >= 2 and lt: e_s.append((lt, dt, et))
      if "@" in em and not em.endswith("@users.noreply.github.com"):
        lp = [t for t in _rsp(r"[._-]+", em.split("@", 1)[0].split("+", 1)[0]) if t]
        if len(lp) == 1 and len(lp[0]) >= 3 and lp[0] not in _GEN_LOC: b1.add(lp[0])
  def _hit(tok):
    if not tok: return False
    tn, ek, ems = _ni(tok), _rek(tok), _eei(tok)
    _, a, subs = _st(tn) if tn else ("", [], [])
    a2 = {t for t in a if len(t) >= 2 and t not in _GEN_LOC}
    eh = any(len(et & bw_a) >= 2 or any((lt & ns) and len(et & ns) >= 2 for ns in b_s) or any((lt & d1) and (l1 & dt) and len(et & e1) >= 2 for l1, d1, e1 in e_s) for em in ems for lt, dt, et in (_ep(em),) if et)
    nh = len(a2) >= 2 and (any((l1 & a2) and len(e1 & a2) >= 2 for l1, _, e1 in e_s) or any(ns <= a2 for ns in b_s))
    return bool((tn and (tn in blk or tn in bk)) or (ems & blk) or (ek and ek in bk) or (_rik(tok) & bk) or (b1 & set(subs)) or (len(a) == 1 and subs and subs[0] in bw_a) or eh or nh)
  return _hit
ROOT = Path(os.environ.get("STOCHOLOGY_REPO_ROOT") or Path(__file__).resolve().parents[2]).resolve()
EMPTY_TREE_SHA = "4b825dc642cb6eb9a060e54bf8d69288fbee4904"
TIER1_PATTERNS = [
  _rc(p) for p in (
    r"^AGENTS\.md$", r"^CONTRIBUTING\.md$", r"^\.gitmessage$", r"^book\.toml$", r"^mathjax\.js$", r"^CITATION\.cff$",
    r"^theme/", r"^book-src/SUMMARY\.md$", r"^book-src/(?:00|20|27|29|40|41|4[2-9]|50|56|68|7[6-9]|8[0-9]|9[0-9])-.*\.md$",
    r"^book-src/(?:index|part-(?:iv|vi|vii)-(?:introduction|conclusion)|stratum-(?:vi|vii)-opening)\.md$",
    r"^book-src/figures/.*\.svg$", r"^book-src/appendix-[abcd]-.*\.md$", r"^crates/", r"^adapters/", r"^consumers/",
    r"^\.cargo/", r"^rust-toolchain(?:\.toml)?$", r"(?:^|/)Cargo\.(?:toml|lock)$", r"^\.github/",
    r"^\.agents/rules/", r"^\.agents/scripts/", r"^\.agents/skills/", r"^\.pre-commit-config\.yaml$",
    r"(?:^|/)\.(?:gitignore|gitattributes)$",
  )
]
TOOLING_PATTERNS = [
  _rc(r"^reviews/.*\.md$"),
]
_CITATION_YEAR_RE = (
  r"(?:\*?c\.?\*?\s*)?(?:"
  r"\[[12]\d{3}[a-z]?\]"
  r"|[12]\d{3}(?:\s*[–\-/]\s*(?:[12]\d{3}|\d{2}))?(?:\s*\[[12]\d{3}[a-z]?\])?[a-z]?"
  r"|\d{1,4}(?:\s*[–\-/]\s*\d{1,4})?\s*(?:BCE|CE)"
  r")"
)
_CITE_LEAD_RE = r"(?:(?:see|cf\.?|e\.g\.|compare|following|citing|after),?\s+)*"
_CITE_PARTICLE_RE = r"(?:da|de|del|della|der|des|di|du|la|le|van|von|ten|ter|bin|ibn|al-|el-)"
_CITE_UPPER_RE = "[A-ZÀ-ÖØ-Þ" + "".join(f"\\u{c:04x}" for r in (range(0x100,0x131,2),range(0x134,0x137,2),range(0x139,0x148,2),range(0x14a,0x179,2),range(0x179,0x17e,2)) for c in r) + "]"
_CITE_AUTHOR_RE = rf"(?:{_CITE_PARTICLE_RE}\s+)*{_CITE_UPPER_RE}[A-Za-zÀ-ÖØ-öø-ÿ\u0100-\u024f\-']*"
TIER1_DIFF_CONTENT_PATTERNS = [_rc(p) for p in (
  r"\$\$", r"\\begin\{", r"(?i)(?:\\|&#(?:0*92|x0*5c);|&bsol;)(?:[\(\[]|&#(?:0*(?:40|91)|x0*(?:28|5b));|&(?:lpar|lbrack|lsqb);)",
  r"(?<!\\)(?:\\\\)*\$[^$\n]+\$", r"(?i)(?:&#(?:0*36|x0*24);|&dollar;)",
  r"\\(?:tau|delta|alpha|beta|gamma|epsilon|varepsilon|zeta|eta|theta|iota|kappa|lambda|mu|nu|xi|pi|rho|sigma|phi|varphi|chi|psi|omega|Gamma|Delta|Theta|Lambda|Xi|Pi|Sigma|Phi|Psi|Omega|infty|partial|nabla|forall|exists|notin|subset|subseteq|supset|supseteq|cup|cap|setminus|emptyset|varnothing|le|leq|ge|geq|ne|neq|approx|equiv|sim|simeq|propto|pm|mp|times|cdot|circ|oplus|otimes|to|mapsto|implies|iff|Rightarrow|Leftarrow|Leftrightarrow|mathbb|mathcal|mathrm|mathbf|mathit|mathsf|mathtt|mathfrak|operatorname|text|frac|dfrac|tfrac|sqrt|lceil|rceil|lfloor|rfloor|sum|prod|int|iint|oint|lim|sup|inf|min|max|arg|log|ln|exp|sin|cos|tan|det|tr|Pr|binom)\b",
  r"\b(?:Theorems?|Propositions?|Corollar(?:y|ies)|Lemmas?|Axioms?|Definitions?|Principles?|(?i:Collapse\s+Propositions?|Reflexive\s+Brakes?|Falsifiers?|Regimes?\s+(?:I|II|III)(?:-[AB])?|Guard\s+Sites?))\b",
  r"(?i)\b(?:Nuclear Triad|co-diagnostic|non-substitutable|never an optimization objective|strictly diagnostic|not a control law|Waiver-Reopening|Sunset Separation|Counter-?Inductive Reopen|Leontief|lexicographic|algedonic|status-blind|non-self-certifiab\w*|binding constitutional|binding liability|premise rigging)\b",
  r"\bT_?\{?[1-7]\}?\b|`T[1-7]`|\bT[1-7](?:Framing|Severity|Archive|Formalization|Temporal|Performative|Liability)\b",
  r"\b(?:[ST](?:0|[89]|[1-9]\d+)|L(?:00|3[6-9]|[4-9]\d|\d{3,}))\b",
  r"^>\s*Layer\s+\d{2}\b", r"^#{1,6}\s+(?:By the end of this chapter you can|Bridge)\b",
  r"(?i)The minimum is not a weighted sum|the exact composition is additive|not one reading among three|designation recorded rather than derived",
  r"\\Omega(?:\\?_)?\{?\\mathrm\{(?:spec|unspec|harm)\}|U(?:\\?_)?\{?\\mathrm\{escape\}",
  rf"\({_CITE_LEAD_RE}{_CITE_AUTHOR_RE}(?:[^()\n]{{0,80}}?)\s+{_CITATION_YEAR_RE}(?:\s*[,;]\s*[^)\n]+)?\)|\bet al\.?,?\s*\(?{_CITATION_YEAR_RE}|\b{_CITE_AUTHOR_RE}(?:\s+(?:&|and|v\.?)\s+{_CITE_AUTHOR_RE})?\s+(?:\({_CITATION_YEAR_RE}(?:\s*[,;]\s*[^)\n]+)?\)|\[[12]\d{{3}}[a-z]?\])",
)]
SPEC_SHRINKING_PATTERNS = [_rc(p) for p in (
  r"#\[\s*(?:ignore|test|allow)\b", r"\b(?:assert(?:_eq|_ne)?|debug_assert(?:_eq|_ne)?|prop_assert(?:_eq|_ne)?|panic|unreachable|unimplemented)!",
  r"\bDECLARED_(?:SKIPS|OPEN)\b", r"\binexpressible\b",
  r"\b(?:pytest\.mark\.skip|skip_unless)\b|\b(?:it|test|describe)\.(?:skip|only|todo)\b|\bt\.skip\s*\(|\bskip\s*:\s*true\b",
  r"\b(?:process\.(?:exit|exitCode)|MUTATIONS)\b|\bclean\s*=\s*true\b|\bassert\.(?:fail|equal|strictEqual|deepStrictEqual|throws|rejects|ok|match)\b|@ts-(?:ignore|expect-error|nocheck)|eslint-disable",
  r"\b(?:EM_DASH_EXEMPT_FILES|QUOTE_MASK_ALLOWED_FILES|ATTESTED_LITERAL_EMDASH_QUOTES)\b",
  r"\b(?:PROSE_BLOAT|PER_FILE_BLOAT|GROSS_PROSE_ADDED|PER_FILE_GROSS_ADDED|GROSS_WORD_DIFF|PER_FILE_GROSS_WORD_DIFF|TOTAL_PROSE_LINE_CHURN|GOVERNANCE_(?:ADDED|CHURN|WORD_DIFF|ADDED_BYTES)|PER_FILE_GOVERNANCE_(?:ADDED|WORD_DIFF|ADDED_BYTES)|CODE_(?:ADDED|CHURN|WORD_DIFF|ADDED_BYTES)|PER_FILE_CODE_(?:ADDED|WORD_DIFF|ADDED_BYTES)|REVIEW_(?:TOTAL_ADDED|WORD_DIFF|ADDED_BYTES)|PER_FILE_REVIEW_(?:ADDED|WORD_DIFF|ADDED_BYTES))(?:_LINE|_WORD|_BYTES)?_THRESHOLD\b",
  r'"ADVISORY"|--strict-(?:trailers|bloat|online)|--verify-online\b|--github-(?:commit-)?pr-api\b|--(?:strict|offline|locked|staged)\b|fail-on-severity',
  r"\b(?:VALID_ORACLE_TOKENS|CANONICAL_SPECIALIST_NAMES|TARGET_FILES_HEADER_RE|NON_HUMAN_SIGNOFF_RE|CAMEL_BOT_RE|UPPER_AI_RE|CAMEL_AI_BRAND_RE|NUMBERED_BOT_SUFFIX_RE|AMBIGUOUS_SINGLE_AI_RE|AI_MODEL_COMBO_RE|PLACEHOLDER_RE|IDENTITY_QUOTE_RE|EMAIL_TOKEN_RE|CREDENTIAL_SUFFIX_RE|SUPPRESSION_COMMENT_RE|TIER1_PATTERNS|TIER1_DIFF_CONTENT_PATTERNS|SPEC_SHRINKING_PATTERNS)\b",
  r"\b(?:CONFUSABLE_LATIN_EXT_CODEPOINTS|CONFUSABLE_GREEK_CHARS|INVISIBLE_CHARS|COLLISION_FAMILIES|EXPECTED_SINKS)\b",
  r"\\Omega(?:\\?_)?\{?\\mathrm\{(?:spec|unspec)\}|U(?:\\?_)?\{?\\mathrm\{escape\}",
  r"\bAPPARATUS\b|\b(?:structure|surface|build_pdf|wasm_gates|repro_aot|security_audit)\.py\b|\bcargo\s+(?:\+[^\s]+\s+)*(?:miri\s+)?test\b|\bnode\b[^\n]*\.m?js\b",
  r"\b(?:def\s+)?(?:check_(?:apparatus|spine|links|figures|b1|ledger|citations|contents|concordance|guard_sites|math|notation|attribution|falsifiers|concordance_polarity|collapse_refutation|guard_footing|year_citation|ledger_coherence|ratio_limit|graphs|symbolic_math|style_and_citations|custody_provenance|crossrefs)|validate_commit_trailers|scan_text_file|_audit_commits|_audit_commit_msg_file|_commit_max_tier|_commit_tier_info|_is_non_human_reviewer|_should_ignore_extra_url|_extract_prose_from_svg_line|_fetch_github_pr_identities|_fetch_github_pr_bundle|_fetch_commit_pr_identities|_verify_merge_commit_against_pr|_build_push_pr_resolver|_check_online_url|_has_path_spec_shrinking|_enclosing_context_lines|_reviewer_equiv_key|_reviewer_identity_keys|_canonical_display_name|_find_author_reconstructing_tokens|_has_ambiguous_signoff_pair|_split_address_list|_read_git_blob_text|_validate_blind_review_artifact|_target_header_covers_path)\b|\bdef\s+(?:analyze|main)\s*\(|\b(?:analyze|main)\(",
  r"(?i)continue-on-error\s*:\s*true|\bfail-fast\s*:\s*false|\bif\s*:\s*(?:\$\{\{\s*)?(?:false|!\s*true|0\b|\d+\s*(?:==|!=)\s*\d+|[^\n]*(?:never|schedule|workflow_dispatch))|--trailer-cutoff-commit\b|--no-verify\b",
)]
_T1, _T2, _TL = "tier1-invariant-core", "tier2-reversible-periphery", "tooling-only"
VALID_VINTAGES = {"human-original", "human-led-ai-assisted", "ai-audit-only", "deterministic-script"}
VALID_CUSTODY_TIERS = {_T1, _T2, _TL}
VALID_ORACLE_TOKENS = {
  "layer0-gauntlet", "structure", "graphs", "notation", "sympy", "style",
  "custody", "crossrefs", "cargo-test", "mdbook", "pdf", "external-sources",
  "blind-review", "na",
}
REQUIRED_TRAILERS = ("Authorship-Vintage", "Custody-Tier", "Oracle-Verified", "Human-T1-Signoff")
SUPPRESSION_COMMENT_RE = _rc(
  r"@ts-(?:ignore|expect-error|nocheck)|eslint-disable|(?:istanbul|c8|v8)\s+ignore|\bnoqa\b|type:\s*ignore|pragma:\s*no\s*cover",
  re.I,
)
SYNTHETIC_PR_MERGE_RE = _rc(r"^Merge\s+[0-9a-f]{7,40}\s+into\s+[0-9a-f]{7,40}$", re.I)
MERGED_PR_SUBJECT_RE = _rc(r"^Merge\s+pull\s+request\s+#\d+\b", re.I)
PROSE_BLOAT_LINE_THRESHOLD=250
PROSE_BLOAT_WORD_THRESHOLD=2000
PER_FILE_BLOAT_LINE_THRESHOLD=80
PER_FILE_BLOAT_WORD_THRESHOLD=1000
GROSS_PROSE_ADDED_LINE_THRESHOLD=250
TOTAL_PROSE_LINE_CHURN_THRESHOLD=400
PER_FILE_GROSS_ADDED_LINE_THRESHOLD=100
GROSS_WORD_DIFF_THRESHOLD=5000
PER_FILE_GROSS_WORD_DIFF_THRESHOLD=1500
GOVERNANCE_ADDED_LINE_THRESHOLD=450
GOVERNANCE_CHURN_LINE_THRESHOLD=650
PER_FILE_GOVERNANCE_ADDED_LINE_THRESHOLD=250
GOVERNANCE_WORD_DIFF_THRESHOLD=4500
PER_FILE_GOVERNANCE_WORD_DIFF_THRESHOLD=2000
GOVERNANCE_ADDED_BYTES_THRESHOLD=45000
PER_FILE_GOVERNANCE_ADDED_BYTES_THRESHOLD=20000
CODE_ADDED_LINE_THRESHOLD=5200
CODE_CHURN_LINE_THRESHOLD=5800
PER_FILE_CODE_ADDED_LINE_THRESHOLD=3200
CODE_WORD_DIFF_THRESHOLD=25000
PER_FILE_CODE_WORD_DIFF_THRESHOLD=14000
CODE_ADDED_BYTES_THRESHOLD=260000
PER_FILE_CODE_ADDED_BYTES_THRESHOLD=150000
REVIEW_TOTAL_ADDED_LINE_THRESHOLD=3000
PER_FILE_REVIEW_ADDED_LINE_THRESHOLD=450
REVIEW_WORD_DIFF_THRESHOLD=25000
PER_FILE_REVIEW_WORD_DIFF_THRESHOLD=4000
REVIEW_ADDED_BYTES_THRESHOLD=220000
PER_FILE_REVIEW_ADDED_BYTES_THRESHOLD=35000
PER_FILE_SVG_ADDED_BYTES_THRESHOLD=40000
SVG_TOTAL_ADDED_BYTES_THRESHOLD=80000
PER_FILE_SVG_RAW_ADDED_LINES_THRESHOLD=400
_SVG_CTL_RE=_rc(r"[\x00-\x1f]+")
_CSS_COMMENT_RE=_rc(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/")
_CSS_HEX_ESC_RE=_rc(r"\\([0-9a-fA-F]{1,6})(?:\r\n|[ \t\r\n\f])?")
_CSS_LIT_ESC_RE=_rc(r"\\([^\r\n\f0-9a-fA-F])")
_XML_NS=r"(?:[A-Za-z_][\w.-]*+:)?"
_SVG_OPEN_RE=_rc(rf"(?i)<{_XML_NS}svg(?=$|[ \t\r\n\f/>])")
_HTML_CMT_END_RE=_rc(r"--!?>")
_SVG_BAN_TAGS=frozenset("script iframe foreignobject object embed textarea xmp noframes noembed noscript plaintext b big blockquote body br center code dd div dl dt em font h1 h2 h3 h4 h5 h6 head hr i img li listing math menu meta nobr ol p pre ruby s small span strong strike sub sup table tt u ul var".split())
_SVG_DATA_URI=r"data\s*:\s*(?:text/html|image/svg\+xml|(?:text|application)/(?:xhtml\+)?xml|(?:text|application)/(?:x-)?javascript)"
_TAG_OPEN_RE=_rc(r"<(/)?([A-Za-z_][^ \t\r\n\f/><]*+)(?=$|[ \t\r\n\f/>])")
_SVG_ACTIVE_PATTERNS=tuple(_rc(p) for p in (
  rf"(?i)<\s*+{_XML_NS}script\b",r"(?i)(?<![A-Za-z0-9_-])on[a-z]{2,}\s*=",r"(?is)j\s*a\s*v\s*a\s*s\s*c\s*r\s*i\s*p\s*t\s*:",
  rf"(?is){_SVG_DATA_URI}",
  rf"(?i)<\s*+{_XML_NS}(?:iframe|foreignobject|object|embed|textarea|xmp|noframes|noembed|noscript|plaintext)\b|<\?(?!xml(?:\s+[^?>]*)?\?>)|<!-{{2,3}}>|--!>|@import\b",
  rf"(?is)url\s*\(\s*['\"]?\s*(?:javascript:|(?:https?:[/\\]|[/\\]{{2}})|{_SVG_DATA_URI})",
))
_TD_CLOSE_RE={t: _rc(rf"(?i)</{_XML_NS}{t}[ \t\r\n\f]*+>") for t in ("title","desc","style")}
_ANCESTOR_CACHE={}
def _html_unescape_n(s,n=3):
  dec=s
  for _ in range(n):
    if (nxt := html.unescape(dec))==dec: break
    dec=nxt
  return dec
def _normalize_css_svg(s):
  no_cmt=re.sub(r"\\\r?\n","",_CSS_COMMENT_RE.sub("",s))
  dec_hex=_CSS_HEX_ESC_RE.sub(lambda m: chr(int(m.group(1),16)) if 0<int(m.group(1),16)<=0x10FFFF else "",no_cmt)
  return _SVG_CTL_RE.sub("",_CSS_LIT_ESC_RE.sub(r"\1",dec_hex))
def _scan_html_tag(s,i):
  if not (m := _TAG_OPEN_RE.match(s,i)): return None
  ic,rn,j,n,at=bool(m.group(1)),m.group(2),m.end(),len(s),[]
  def _ws(k):
    while k<n and s[k]<=" ": k+=1
    return k
  while (j := _ws(j))<n:
    if s[j]==">": return (rn,ic,False,at,j+1,True)
    if s[j:j+2]=="/>": return (rn,ic,True,at,j+2,True)
    if s[j]=="/":
      j+=1
      continue
    a0=j
    while j<n and s[j]>" " and s[j] not in "=/>": j+=1
    an=s[a0:j]
    j=(j+1) if (not an and s[j]=="=") else _ws(j)
    if j<n and s[j]=="=":
      j=_ws(j+1)
      if j<n and s[j] in ('"',"'"):
        k=s.find(s[j],j+1)
        av,j=(s[j+1:],n) if k== -1 else (s[j+1:k],k+1)
      else:
        v0=j
        while j<n and s[j]>" " and s[j]!=">": j+=1
        av=s[v0:j]
      at.append((an,av))
    elif an: at.append((an,""))
  return (rn,ic,False,at,n,False)
def _skip_non_tag(s,k,n):
  if s.startswith("<!-->",k): return k+5
  if s.startswith("<!--->",k): return k+6
  if s.startswith("<!--",k): return m.end() if (m := _HTML_CMT_END_RE.search(s,k+4)) else n
  if s[k:k+9].upper()=="<![CDATA[": return (e+3) if (e := s.find("]]>",k+9))!= -1 else n
  if s.startswith(("<?","<!"),k) or (s.startswith("</",k) and not _rm(r"[A-Za-z_]",s[k+2:k+3])): return (e+1) if (e := s.find(">",k+2))!= -1 else n
  return -1
def _iter_html_tags(s):
  i,n=0,len(s)
  while (k := s.find("<",i))!= -1:
    if (sk := _skip_non_tag(s,k,n))!= -1: i=sk
    elif (t := _scan_html_tag(s,k)):
      yield (k,*t)
      i=t[4]
    else: i=k+1
def _unclosed_td(s,lt,ep): return lt in _TD_CLOSE_RE and ((kn := s.find("<",ep))== -1 or not _TD_CLOSE_RE[lt].match(s,kn))
def _has_unsafe_svg_payload(rt):
  if not rt: return False
  for _,rn,ic,sc,at,ep,_ in _iter_html_tags(rt):
    lt=rn.rsplit(":",1)[-1].lower()
    if lt in _SVG_BAN_TAGS or (not ic and not sc and _unclosed_td(rt,lt,ep)): return True
    for an,av in at:
      al=an.rsplit(":",1)[-1].lower()
      if re.fullmatch(r"on[a-z]{2,}",al): return True
      vc=_SVG_CTL_RE.sub("",_html_unescape_n(av)).strip()
      if _rs(rf"(?is)javascript\s*:|{_SVG_DATA_URI}",vc) or (lt in ("use","image","feimage") and al=="href" and _rm(r"(?i)^(?:https?:|[/\\]{2})",vc)) or (lt in ("animate","set") and ((al=="attributename" and _rm(rf"(?i)^{_XML_NS}(?:href|on[a-z]+)$",vc)) or (al in ("to","values","from") and _rs(r"(?i)(?:javascript:|data:)",vc)))): return True
  dec=_html_unescape_n(rt)
  c1=_normalize_css_svg(dec)
  return any(p.search(v) for v in (rt,dec,_SVG_CTL_RE.sub(" ",dec),c1,_normalize_css_svg(c1)) for p in _SVG_ACTIVE_PATTERNS)
def _md_has_unsafe_inline_svg(md):
  if not md: return False
  pos,n=0,len(md)
  while (m0 := _SVG_OPEN_RE.search(md,pos)):
    st=m0.start()
    t0=_scan_html_tag(md,st)
    if not t0 or not t0[5]: return _has_unsafe_svg_payload(md[st:])
    if t0[2]:
      if _has_unsafe_svg_payload(md[st:t0[4]]): return True
      pos=t0[4]
      continue
    dp,i=1,t0[4]
    while dp>0 and i<n and (k := md.find("<",i))!= -1:
      if (sk := _skip_non_tag(md,k,n))!= -1:
        i=sk
        continue
      if not (t := _scan_html_tag(md,k)):
        i=k+1
        continue
      if not t[5]: return _has_unsafe_svg_payload(md[st:])
      lt,i=t[0].rsplit(":",1)[-1].lower(),t[4]
      if lt in _SVG_BAN_TAGS or (not t[1] and not t[2] and _unclosed_td(md,lt,i)): return True
      if lt=="svg": dp+= -1 if t[1] else (0 if t[2] else 1)
    if dp>0: i=n
    if _has_unsafe_svg_payload(md[st:i]) or dp>0: return True
    pos=i
  return False
def _is_git_ancestor(anc_sha,desc_sha):
  a,d=(anc_sha or "").strip(),(desc_sha or "").strip()
  if not a or not d or a==d: return False
  key=(str(ROOT),a,d)
  if key not in _ANCESTOR_CACHE:
    res=subprocess.run(["git","merge-base","--is-ancestor",a,d],cwd=ROOT,capture_output=True,text=True,check=False)
    _ANCESTOR_CACHE[key]=res.returncode==0
  return _ANCESTOR_CACHE[key]
PROTECTED_SURFACES=("AGENTS.md","book-src/SUMMARY.md","crates/",".github/checks/")
GOVERNANCE_MANIFEST_SURFACES=(
  ".agents/rules/",".agents/skills/",".github/workflows/",".pre-commit-config.yaml",".gitmessage",
  ".gitignore",":(glob)**/.gitignore",".gitattributes",":(glob)**/.gitattributes",
  "book.toml","CITATION.cff","mathjax.js","theme/",".cargo/","rust-toolchain","rust-toolchain.toml",
  "Cargo.toml","Cargo.lock","crates/host-workspace/Cargo.toml","crates/host-workspace/Cargo.lock",
)
@dataclass
class CommitAudit:
  sha: str
  subject: str
  author: str
  inferred_tier: str
  spec_shrinking: bool
  trailers: dict[str,str]
  issues: list[str]
  status: str
def _is_prose_file(rel_path):
  return (rel_path.startswith("book-src/") and (rel_path.endswith(".md") or (rel_path.startswith("book-src/figures/") and rel_path.endswith(".svg")))) or rel_path in ("README.md","CONTRIBUTING.md")
def _is_code_bloat_file(rel_path):
  return rel_path.startswith((".agents/scripts/",".github/checks/","crates/","adapters/","consumers/","theme/")) or rel_path=="mathjax.js" or rel_path.endswith(("Cargo.toml","Cargo.lock"))
def _is_review_bloat_file(rel_path):
  return rel_path.startswith("reviews/")
def _iter_line_windows(lines):
  windows=list(lines)
  for width in (2,3):
    for idx in range(len(lines)-width+1):
      if (joined := " ".join(s.strip() for s in lines[idx : idx+width] if s.strip())): windows.append(joined)
  if len(lines)>3 and (full := " ".join(s.strip() for s in lines if s.strip())): windows.append(full)
  return windows
_TIER1_HTML_TAG_RE=_rc(r"</?[A-Za-z](?:[^>\n\"']++|\"(?:\\.|[^\"\\\n])*+\"|'(?:\\.|[^'\\\n])*+')*+>")
_PROSE_NUM_RE=_rc(
  r"(?<![A-Za-z0-9_.])(?:(?<![+−-])[+−-])?(?!(?:\d+(?:st|nd|rd|th))(?![A-Za-z0-9_]))(?:\d+(?:[.,]\d+)?|\.\d+)",
  re.I,
)
def _strip_ed_nums(s):
  s=re.sub(r"\[([^\[\]\n]*)\]\([^)\n]*\)",r"\1",s)
  s=re.sub(r"https?://\S+|\b\d{4}-\d{2}-\d{2}(?:[T ]\d{2}:\d{2}(?::\d{2})?(?:Z|[+-]\d{2}:?\d{2})?)?\b|(?<![A-Za-z0-9_./-])[A-Za-z0-9_./-]*[A-Za-z][A-Za-z0-9_./-]*\.(?:md|py|rs|toml|yml|yaml|json|svg|js|cff|sh|lean)(?::[1-9]\d*(?:[–\-][1-9]\d*)?)?\b","",s)
  s=re.sub(r"\b(?:UTF|ISO|RFC|IEEE|SHA)-\d+(?:-\d+)*\b","",s,flags=re.I)
  s=re.sub(r"^\s*(?:#{1,6}\s+\d{1,3}(?:\.\d{1,3})*[.)]|#{1,6}\s+\d{1,3}\.\d{1,3}\.\d{1,3}(?:\.\d{1,3})*|(?:[-*+]\s+)?\d{1,3}[.)])\s+(?=[A-Za-z*`\[])","",s)
  u_base=r"$|\s*(?:,(?!\d)|\.(?!\d)|[;:!?)\]}\u2014/]|[\u2013\-](?!\s*\d))|\s+\d{1,2}[.)]\s+(?=[A-Za-z*\[])|\s+(?i:and|or|in|of|to|for|with|from|by|on|at|as|also|that|which|where|when|shows?|presents?|lists?|records?|maps?|defines?|proves?|states?|argues?|gives?|carries?|holds?|checks?|covers?|isolates?|subjects?|open|below|above|through|via|vs\.?|versus)\b"
  u_ok,u_ft= rf"(?={u_base}|\s+(?i:is|are|was|were|be)\b)",rf"(?={u_base})"
  prng=r"(?:\s*\(\s*(?:\d{2}|[1-9])\s*[–\-]\s*(?:\d{2}|[1-9])\s*\))?"
  for pat in (
    rf"\b(?:Parts?|Strata|Stratum|Regimes?|Prologue)\s+(?:(?:I{{1,3}}|IV|VI{{0,2}})(?:-[AB])?|[ST][1-7]\b|00){prng}{u_ok}",
    rf"(?<![0-9.])\b00\b{prng}{u_ok}|\[\^\d+\]|\b(?:[ST][1-7]\b{prng}{u_ok}|L(?:0[1-9]|[12]\d|3[0-5])\b)",
    rf"\b(?:Appendices?|Appendix)\s+[A-G](?:\.[1-9]\d?)?{u_ok}",
    rf"\b(?:Layers?|Orders?)\s+L(?:0[1-9]|[12]\d|3[0-5]){u_ok}",
    rf"\b(?:Brakes?\s+[1-4]|Propositions?\s+[1-3]|Falsifiers?\s+F?[1-5]){u_ok}",
    rf"\b(?i:Sections?)\s+(?:[A-G]|\d{{1,2}})(?:\.[1-9]\d?){{0,2}}{u_ok}",
    rf"\b(?i:Chapter|Ch\.)\s+(?:[A-G]\.[1-9]\d?|(?:\d{{2}}|[1-9])(?:[–\-](?:\d{{2}}|[1-9]))?){u_ok}",
    rf"\b(?i:Chapters|Chs\.)\s+(?:[A-G]\.[1-9]\d?[–\-][A-G]\.[1-9]\d?|(?:\d{{2}}|[1-9])[–\-](?:\d{{2}}|[1-9])){u_ok}",
    rf"\b(?i:Figure|Fig\.|Table|Tab\.)\s+(?:[A-G]\.[1-9]\d?|(?:\d{{2}}|[1-9])){u_ft}",
    rf"\b(?i:Figures|Figs\.|Tables|Tabs\.)\s+(?:[A-G]\.[1-9]\d?[–\-][A-G]\.[1-9]\d?|(?:\d{{2}}|[1-9])[–\-](?:\d{{2}}|[1-9])){u_ft}",
  ): s=re.sub(pat,"",s)
  return s
def _has_tier1_content(lines):
  _cln= lambda s: _ru(r"[`*~]+|(?<![A-Za-z0-9\\}])_|_(?![A-Za-z0-9{])","",_CM_UNESC_RE.sub(r"\1",s))
  def _sh(s):
    pos=0
    while (ci_ := s.find("<!--",pos))!= -1:
      if (len(s[:ci_])-len(s[:ci_].rstrip("\\"))) % 2==1:
        pos=ci_+4
        continue
      if (cj_ := s.find("-->",ci_+4))== -1: break
      s=s[:ci_]+s[cj_+3:]
    return _TIER1_HTML_TAG_RE.sub(lambda m: m.group(0) if (len(s[:m.start()])-len(s[:m.start()].rstrip("\\"))) % 2==1 else "",s)
  _uw= lambda s: _ru(r"\[([^\[\]\n]+)\]\[[^\[\]\n]*\]",r"\1",_unwrap_md_links(s,use_quoted=True))
  def _cc(ln):
    lit=_strip_invisible_and_fold(ln)
    tl,ul=_sh(lit),_sh(_uw(lit))
    td=_strip_invisible_and_fold(_html_unescape_n(tl,2)) if "&" in tl else tl
    ud=_strip_invisible_and_fold(_html_unescape_n(ul,2)) if "&" in ul else ul
    return (_cln(td),_cln(_uw(td)),_cln(ud))
  for ln in _iter_line_windows(lines):
    c=_cc(ln)
    if any(p.search(x) for x in ((ln,_html_unescape_n(ln,2),*c) if "&" in ln else (ln,*c)) for p in TIER1_DIFF_CONTENT_PATTERNS): return True
  if any(_PROSE_NUM_RE.search(_strip_ed_nums(x)) for ln in lines for x in _cc(ln)): return True
  return len(lines)>1 and any(_PROSE_NUM_RE.search(_ru(r"-\n\s*","-","\n".join(_strip_ed_nums(_cc(ln)[k]) for ln in lines))) for k in range(3))
def _has_spec_shrinking(lines): return any(p.search(ln) for ln in _iter_line_windows(lines) for p in SPEC_SHRINKING_PATTERNS)
def classify_file(rel_path,changed_lines=None):
  if any(p.search(rel_path) for p in TIER1_PATTERNS): return _T1
  if any(p.search(rel_path) for p in TOOLING_PATTERNS): return _TL
  if _is_prose_file(rel_path): return _T1 if (changed_lines and (_has_tier1_content(changed_lines) or _has_spec_shrinking(changed_lines))) else _T2
  return _T1
_CTRL_NL_RE=_rc(r"[\r\x0b\x0c\x1c-\x1e\x85\u2028\u2029]")
def _norm_rel(p):
  return posixpath.normpath(p.strip().replace("\\","/"))
def _pr_str(obj,*keys):
  for k in keys:
    obj=(obj or {}).get(k) if isinstance(obj,dict) else None
  return str(obj or "").strip()
def _lf_lines(s):
  return s[:-1].split("\n") if s.endswith("\n") else (s.split("\n") if s else [])
def _read_raw_text(p):
  try: return p.read_bytes().decode("utf-8",errors="replace") if p.is_file() else ""
  except OSError: return ""
def _run_git(args,*,check=True):
  proc=subprocess.run(["git","-c","core.quotePath=false",*args],cwd=ROOT,capture_output=True,text=False,check=False)
  if proc.returncode!=0:
    if check:
      detail=(proc.stderr or proc.stdout).decode("utf-8",errors="replace").strip() or f"exit {proc.returncode}"
      raise RuntimeError(f"git {' '.join(args)} failed: {detail}")
    return ""
  return proc.stdout.decode("utf-8",errors="replace")
def _git_diff(*args,check=True):
  return _run_git(["diff","--no-renames",*args],check=check)
def _git_dt(*args,check=True):
  return _run_git(["diff-tree","-m","--first-parent","--no-renames","--no-commit-id",*args],check=check)
def _rev_verify(ref,suf="commit",*,check=True):
  return _run_git(["rev-parse","--verify",f"{ref}^{{{suf}}}"],check=check).strip()
def _rev_stat(target=None,*,staged=False):
  return ["diff",*(("--cached",) if staged else ()),"--no-renames","--name-status",*(() if staged else (target,)),"--","reviews/"]
def _dt_rev(sha,*extra):
  return ["diff-tree",*extra,"-m","--first-parent","--no-renames","--no-commit-id","--name-status","-r",sha,"--","reviews/"]
def _split_ctrl_lines(lines):
  return lines+[s for ln in lines if _CTRL_NL_RE.search(ln) for s in _CTRL_NL_RE.split(ln) if s]
SVG_PROSE_LOCAL_TAGS={"text","title","desc","foreignobject"}
def _extract_svg_text_lines(svg_content):
  if not svg_content or not svg_content.strip(): return []
  try: root=ET.fromstring(svg_content)
  except ET.ParseError: return _ne(svg_content)
  lines=[]
  def _walk(el):
    if (isinstance(el.tag,str) and el.tag.rsplit("}",1)[-1].lower() in SVG_PROSE_LOCAL_TAGS):
      lines.extend(n for r in html.unescape(" ".join(el.itertext())).splitlines() if (n := re.sub(r"\s+"," ",r).strip()))
      return
    for ch in el: _walk(ch)
  _walk(root)
  return lines
def _iter_diff_files(raw):
  cf,po,ih=None,None,False
  for ln in _lf_lines(raw):
    if ln.startswith("diff --git "): cf,po,ih=None,None,False
    elif not ih and ln.startswith("--- a/"): po=ln[6:].strip()
    elif not ih and (ln.startswith("+++ b/") or (ln.startswith("+++ /dev/null") and po is not None)):
      cf=ln[6:].strip() if ln.startswith("+++ b/") else po
      yield cf,None
    elif cf is not None and ln.startswith("@@ "):
      ih=True
      yield cf,ln
    elif cf is not None and ih: yield cf,ln
def _read_git_blob_text(rel_path,git_ref=None,*,cached_index=False):
  if git_ref: raw=_git_diff("-a","-U999999",EMPTY_TREE_SHA,git_ref,"--",rel_path,check=False)
  elif cached_index: raw=_git_diff("-a","--cached","-U999999",EMPTY_TREE_SHA,"--",rel_path,check=False)
  else: return _read_raw_text(ROOT / rel_path)
  if not raw: return ""
  out,no_nl=[],False
  for _,ln in _iter_diff_files(raw):
    if ln is None or ln.startswith("@@ "): continue
    if ln.startswith("\\"): no_nl=True
    elif ln.startswith(("+"," ")):
      out.append(ln[1:])
      no_nl=False
  return ("\n".join(out)+("" if no_nl else "\n")) if out else ""
_rgt,_hus,_nr,_ll,_rt,_rg,_ipf,_iga=_read_git_blob_text,_has_unsafe_svg_payload,_norm_rel,_lf_lines,_read_raw_text,_run_git,_is_prose_file,_is_git_ancestor
def _extract_diff_changed_lines(git_diff_args,out_linenos=None,out_added_bytes=None):
  bf,nw,nl,ol,ob={},{},0,out_linenos,out_added_bytes
  for cf,ln in _iter_diff_files(_rg(git_diff_args)):
    if ln is None:
      bf.setdefault(cf,[])
      nw.setdefault(cf,0)
      if ol is not None: ol.setdefault(cf,set())
      if ob is not None: ob.setdefault(cf,0)
      nl=0
    elif ln.startswith("@@ "):
      if (hm := _rs(r"\+(\d+)(?:,(\d+))?",ln)):
        nl=int(hm.group(1))
        if (int(hm.group(2)) if hm.group(2) is not None else 1)==0 and ol is not None: ol[cf].add(max(1,nl))
    elif ln.startswith(" "):
      if ol is not None and nl>0: nl+=1
    elif ln.startswith(("+","-")):
      rb=ln[1:]
      bd=rb[:-1] if rb.endswith("\r") else rb
      bf[cf].append(bd)
      if _CTRL_NL_RE.search(bd): bf[cf].extend(s for s in _CTRL_NL_RE.split(bd) if s)
      wc=len(bd.split())
      if ln.startswith("+"):
        nw[cf]+=wc
        if ob is not None: ob[cf]+=len(rb.encode("utf-8"))+1
        if ol is not None and nl>0:
          ol[cf].add(nl)
          nl+=1
      else:
        nw[cf]-=wc
        if ol is not None: ol[cf].add(max(1,nl))
  return bf,nw
def _has_path_spec_shrinking(p,rl,cl):
  if _is_rev_md(p): return False
  if _has_spec_shrinking(cl): return True
  if not rl: return False
  bn=posixpath.basename(p)
  if not (p in ("AGENTS.md","CONTRIBUTING.md",".gitmessage",".pre-commit-config.yaml","book.toml","book-src/SUMMARY.md",".github/CODEOWNERS","rust-toolchain","rust-toolchain.toml","mathjax.js","CITATION.cff") or bn in (".gitignore",".gitattributes") or p.endswith(("Cargo.toml","Cargo.lock")) or p.startswith((".cargo/",".github/checks/",".github/workflows/",".agents/scripts/",".agents/rules/",".agents/skills/","crates/","adapters/","consumers/","theme/","book-src/figures/","reviews/")) or (p.startswith("book-src/") and not _ipf(p))): return False
  if p.endswith(".md"): return any(ln.strip() for ln in rl)
  if p.endswith((".ts",".mjs",".js",".rs")): return any(ln.strip() and (SUPPRESSION_COMMENT_RE.search(ln) or not _rm(r"^\s*(?://.*|/\*(?:(?!\*/).)*\*/\s*)$",ln)) for ln in rl)
  hok=p.endswith((".py",".yml",".yaml",".toml",".sh",".cff")) or bn in (".gitignore",".gitattributes",".gitmessage","CODEOWNERS") or p=="rust-toolchain"
  return any(ln.strip() and (SUPPRESSION_COMMENT_RE.search(ln) or not (hok and ln.lstrip().startswith("#"))) for ln in rl)
def _enclosing_context_lines(rel_path,linenos,*,git_ref=None,cached_index=False):
  if not linenos: return []
  imp=_ipf(rel_path) and rel_path.endswith(".md")
  if not (imp or rel_path.endswith((".py",".rs",".yml",".yaml",".ts",".mjs",".js",".kv",".toml"))): return []
  text=_rgt(rel_path,git_ref=git_ref,cached_index=cached_index)
  if not text or not (fl := _ll(text)): return []
  nl,ex,seen=len(fl),[],set()
  tlns,iof=[min(max(1,r),nl) for r in sorted(linenos)],lambda s: len(s)-len(s.lstrip())
  _mi= lambda ln: iof(fl[ln-1]) if fl[ln-1].strip() else 10**6
  def emit(s):
    if s and s not in seen:
      seen.add(s)
      ex.append(s)
  if imp:
    ind,dop,iop,csp,cop=[False]*(nl+1),False,False,{},0
    for idx,ln in enumerate(fl,1):
      cnt,sc,pin=len(_rfa(r"(?<!\\)\$\$",ln)),len(_rfa(r"(?<!\\)(?:\\\\)*\$(?!\$)",_ru(r"(?<!\\)(?:\\\\)*+\$\$","",ln))),iop
      if not ln.strip() or _rm(r"^#{1,6}\s+",ln): iop=False
      elif sc%2: iop= not iop
      if dop or cnt or pin or iop: ind[idx]=True
      if cnt%2: dop= not dop
      cp=0
      while cp<len(ln):
        if cop:
          if (cj := ln.find("-->",cp))== -1: break
          for k in range(cop,idx+1): csp[k]=(cop,idx)
          cop,cp=0,cj+3
        elif (ci := ln.find("<!--",cp))== -1: break
        else: cop,cp=idx,ci+4
    if cop:
      for k in range(cop,nl+1): csp[k]=(cop,nl)
    pkw,_pb=r"Collapse Proposition|Theorem|Proposition|Corollary|Lemma|Axiom|Definition|Falsifier|Reflexive Brake",r"^(?:#{1,6}|(?:[-*+]|\d{1,3}[.)]))\s+"
    bpr=_rc(rf"^\s*(?:\d+\.\s+)?\*\*(?:{pkw}|Nuclear Triad|\d+\.)")
    for ln in tlns:
      if ind[ln]: emit("$$")
      if ln in csp: emit(" ".join(s.strip() for s in fl[csp[ln][0]-1:csp[ln][1]] if s.strip()))
      for hi in range(ln,0,-1):
        if _rm(r"^#{1,6}\s+",fl[hi-1]):
          emit(fl[hi-1].strip())
          break
      ps=pe=ln
      while ps>1 and fl[ps-2].strip() and not _rm(_pb,fl[ps-2]) and not _rm(_pb,fl[ps-1]): ps-=1
      while pe<nl and fl[pe].strip() and not _rm(_pb,fl[pe]): pe+=1
      emit(" ".join(s.strip() for s in fl[ps-1:pe] if s.strip()))
      for pi in range(ln-1,0,-1):
        pl=fl[pi-1]
        if _rm(r"^#{1,6}\s+",pl): break
        if bpr.match(pl) or (pi>=max(1,ln-8) and (_rm(r"^\s*(?:\d+\.|[-*])\s+",pl) or _rs(rf"\b(?:{pkw})\b",pl))):
          emit(pl.strip())
          break
    return ex
  if rel_path.endswith(".py"):
    phr=_rc(r"^\s*(?:async\s+def|def|class)\s+[A-Za-z_]\w*|^[A-Z][A-Z0-9_]*\s*(?::[^=]+)?=")
    for ln in tlns:
      mi=_mi(ln)
      for pi in range(ln,0,-1):
        if not (s := (pl := fl[pi-1]).strip()) or s.startswith("#"): continue
        if pi==ln or (i0 := iof(pl))<=mi:
          if phr.match(pl):
            emit(s)
            if not (i0 := iof(pl)): break
            mi=i0-1
          elif not iof(pl) and not pl.lstrip().startswith((")","]","}","@")): break
    return ex
  if rel_path.endswith(".kv"):
    for ln in tlns:
      for pi in range(ln,max(0,ln-400),-1):
        if not (s := fl[pi-1].strip()) or s.startswith("#"): continue
        emit(s)
        if s.startswith("[") and s.endswith("]"): break
    return ex
  if rel_path.endswith((".rs",".ts",".mjs",".js")):
    ir=rel_path.endswith(".rs")
    hrx=_rc(r"^\s*(?:#\[|pub(?:\([^)]*\))?\s+|(?:async\s+|unsafe\s+)?fn\s+|struct\s+|enum\s+|impl\b|trait\s+|mod\s+|const\s+|static\s+|macro_rules!\s+)" if ir else r"^\s*(?:export\s+)?(?:async\s+)?(?:function\s+[A-Za-z_$][\w$]*|(?:const|let|var)\s+[A-Za-z_$][\w$]*\s*=|class\s+[A-Za-z_$][\w$]*|(?:test|it|describe)(?:\.(?:skip|only|todo))?\s*\()")
    _sup= lambda lo,hi: [emit(t) for k in range(lo,hi+1) if SUPPRESSION_COMMENT_RE.search(t := fl[k-1].strip())]
    if not ir: _sup(1,min(30,nl))
    for ln in tlns:
      _sup(max(1,ln-3),min(nl,ln))
      mi=_mi(ln)
      for pi in range(ln,max(0,ln-250),-1):
        if SUPPRESSION_COMMENT_RE.search(s := (pl := fl[pi-1]).strip()): emit(s)
        if not s or s.startswith("//"): continue
        if (pi==ln or (i0 := iof(pl))<=mi) and hrx.match(pl):
          emit(s)
          mi=i0=iof(pl)
          if ir and not s.startswith("#["):
            for ai in range(pi-1,max(0,pi-20),-1):
              if not (al := fl[ai-1].strip()) or al.startswith("//"): continue
              if al.startswith("#["): emit(al)
              else: break
          if not i0 and (not ir or not s.startswith("#[")): break
    return ex
  ykr=_rc(r"^\s*(?:-\s+)?[A-Za-z0-9_.\-]+\s*:")
  for ln in tlns:
    mi,sind=_mi(ln),None
    for pi in range(ln,0,-1):
      if not (s := (pl := fl[pi-1]).strip()) or s.startswith("#"): continue
      i0=iof(pl)
      if (pi==ln or i0<mi or (sind is None and i0==mi and s.startswith("- "))) and (ykr.match(pl) or s.startswith("- ")):
        emit(s)
        if sind is None and s.startswith("- "): sind=i0
        mi=i0
        if not i0: break
    if sind is not None:
      for si in range(ln,0,-1):
        if not (st := (sl := fl[si-1]).strip()) or st.startswith("#"): continue
        if (i0 := iof(sl))<sind: break
        if _rm(r"^\s*(?:-\s*)?(?:if|continue-on-error|run|uses|entry|stages|files|exclude|paths)\s*:",sl): emit(st)
        if i0==sind and st.startswith("- "): break
  return ex
def _extract_word_diff_added(target,*,staged=False):
  aw={}
  for cf,ln in _iter_diff_files(_git_diff("-a","--cached","--word-diff=porcelain") if staged else _git_diff("-a","--word-diff=porcelain",target)):
    if ln is None: aw.setdefault(cf,0)
    elif not ln.startswith("@@ ") and ln.startswith("+"): aw[cf]+=len(ln[1:].split())
  return aw
def _untracked_files():
  return [p for ln in _ll(_rg(["status","--porcelain","-uall"])) if ln.startswith("?? ") and (ROOT / (p := ln[3:].strip().strip('"'))).is_file()]
def _resolve_base_ref(diff_ref,head_ref):
  _rev_verify(diff_ref)
  _rev_verify(head_ref)
  return diff_ref if (diff_ref=="HEAD" and head_ref=="HEAD") else (_rg(["merge-base",diff_ref,head_ref],check=False).strip() or diff_ref)
def _diff_target_args(diff_ref,head_ref,*,working_tree=False):
  if diff_ref:
    b=_resolve_base_ref(diff_ref,head_ref)
    return (b if (working_tree and head_ref=="HEAD") else f"{b}..{head_ref}"),b,False
  if _git_diff("--name-only","HEAD").strip() or _untracked_files(): return "HEAD",None,False
  return ("HEAD~1..HEAD" if _rev_verify("HEAD~1",check=False) else "HEAD"),None,True
def _svg_versions_from_diff(target,rel_path):
  ol,nl=[],[]
  for _,ln in _iter_diff_files(_git_diff("-a","-U999999",target,"--",rel_path,check=False)):
    if ln is None or ln.startswith(("@@ ","\\")): continue
    if ln.startswith("-"): ol.append(ln[1:])
    elif ln.startswith("+"): nl.append(ln[1:])
    else:
      c=ln[1:] if ln.startswith(" ") else ln
      ol.append(c)
      nl.append(c)
  return "\n".join(ol),"\n".join(nl)
def _working_stats(diff_ref,head_ref="HEAD",*,staged=False,working_tree=False,out_raw_changed=None,out_added_bytes=None,out_raw_svg_added=None,out_svg_unsafe=None):
  orc,oab,orsa,osu=out_raw_changed,out_added_bytes,out_raw_svg_added,out_svg_unsafe
  tgt,rbase,fb=("HEAD","HEAD",False) if staged else _diff_target_args(diff_ref,head_ref,working_tree=working_tree)
  df,tail=["diff","-a",*(("--cached",) if staged else ()),"--no-renames"],(() if staged else (tgt,))
  utr=[] if staged else (_untracked_files() if (head_ref=="HEAD" and not fb and (not diff_ref or working_tree)) else [])
  rnum,lmap=_rg([*df,"--numstat",*tail]),{}
  cmap,wmap=_extract_diff_changed_lines([*df,"--unified=0",*tail],out_linenos=lmap,out_added_bytes=oab)
  if orc is not None: orc.update((k,list(v)) for k,v in cmap.items())
  cref=None if (staged or (working_tree and head_ref=="HEAD") or (not diff_ref and not fb)) else head_ref
  for fp,lns in lmap.items():
    if (ctx := _enclosing_context_lines(fp,lns,git_ref=cref,cached_index=staged)): cmap[fp]=cmap.get(fp,[])+ctx
  wdmap,rows=_extract_word_diff_added(tgt,staged=staged),[]
  s_old,s_new=("HEAD","__CACHED__") if staged else (tgt.split("..",1) if ".." in tgt else (tgt,None))
  _ntxt= lambda p: _rgt(p,cached_index=True) if s_new=="__CACHED__" else (_rt(ROOT / p) if s_new is None else _rgt(p,git_ref=s_new))
  _mu= lambda p: osu is not None and p not in osu and osu.append(p)
  _ops= lambda a,b: difflib.SequenceMatcher(None,a,b).get_opcodes()
  for ln in _ll(rnum):
    if len(ps := ln.split("\t",2))!=3: continue
    as_,ds_,p=ps
    an,dn=int(as_) if as_.isdigit() else 0,int(ds_) if ds_.isdigit() else 0
    if p.endswith(".svg"):
      if orsa is not None: orsa[p]=an
      osv,nsv=_rgt(p,git_ref=s_old),_ntxt(p)
      nn=s_new not in (None,"__CACHED__") and not nsv
      if not osv or nn:
        do_,dn_=_svg_versions_from_diff(tgt,p)
        osv,nsv=osv or do_,(dn_ if nn else nsv)
      if _hus(nsv): _mu(p)
      ol,nl=_extract_svg_text_lines(osv),_extract_svg_text_lines(nsv)
      ops=_ops(ol,nl)
      an,dn=sum(j2-j1 for t,_,_,j1,j2 in ops if t in ("replace","insert")),sum(i2-i1 for t,i1,i2,_,_ in ops if t in ("replace","delete"))
      ot,nt=[t for x in ol for t in x.split()],[t for x in nl for t in x.split()]
      wmap[p],wdmap[p]=len(nt)-len(ot),sum(j2-j1 for t,_,_,j1,j2 in _ops(ot,nt) if t in ("replace","insert"))
    elif p.endswith(".md"):
      nmd=_ntxt(p)
      if not as_.isdigit():
        ml=_ll(nmd)
        an,cmap[p],wmap[p]=len(ml),_split_ctrl_lines(ml),(wc := len(nmd.split()))
        wdmap[p]=wc
        if oab is not None: oab[p]=len(nmd.encode("utf-8"))
      if _md_has_unsafe_inline_svg(nmd): _mu(p)
    elif not as_.isdigit():
      an=len(cmap.get(p,[]))
      if oab is not None: oab.setdefault(p,0)
    rows.append((an,dn,p))
  for up in utr:
    ut=_rt(ROOT / up)
    ul=_ll(ut)
    if up.endswith(".svg"):
      if orsa is not None: orsa[up]=len(ul)
      if _hus(ut): _mu(up)
      vis=_extract_svg_text_lines(ut)
      wc,lc=sum(len(x.split()) for x in vis),len(vis)
    else:
      if up.endswith(".md") and _md_has_unsafe_inline_svg(ut): _mu(up)
      wc,lc=len(ut.split()),len(ul)
    rows.append((lc,0,up))
    cmap[up]=_split_ctrl_lines(ul)
    if orc is not None: orc[up]=list(ul)
    if oab is not None: oab[up]=len(ut.encode("utf-8"))
    wmap[up]=wdmap[up]=wc
  return rows,cmap,wmap,wdmap,rbase
def _matches_surface_prefix(p,prefixes):
  return any((p==(s := pr[10:]) or p.endswith(f"/{s}")) if pr.startswith(":(glob)**/") else (p.startswith(pr) if pr.endswith("/") else (p==pr or p.startswith(f"{pr}/"))) for pr in prefixes)
def _diff_paths(diff_ref,prefixes,head_ref="HEAD",*,staged=False,working_tree=False):
  if staged: return _ne(_git_diff("--cached","--name-only","--",*prefixes))
  tgt,_,fb=_diff_target_args(diff_ref,head_ref,working_tree=working_tree)
  ps=_ne(_git_diff("--name-only",tgt,"--",*prefixes))
  if head_ref=="HEAD" and not fb and (not diff_ref or working_tree):
    seen=set(ps)
    ps.extend(up for up in _untracked_files() if up not in seen and _matches_surface_prefix(up,prefixes))
  return ps
def _classify_paths_tier(paths,cmap,lmap,*,git_ref=None,cached_index=False):
  hi,sh,t1p=_TL,False,[]
  for fp in paths:
    ls=cmap.get(fp,[])
    comb=ls+_enclosing_context_lines(fp,lmap.get(fp,set()),git_ref=git_ref,cached_index=cached_index)
    psh=_has_path_spec_shrinking(fp,ls,comb)
    sh=sh or psh
    t=_T1 if psh else classify_file(fp,comb)
    if t==_T1:
      hi=_T1
      if not _is_rev_md(fp): t1p.append(fp)
    elif t==_T2 and hi==_TL: hi=_T2
  return (_T1 if sh else hi),sh,t1p
def _commit_tier_info(sha):
  lm={}
  cm,_=_extract_diff_changed_lines(["show","-a","-m","--first-parent","--no-renames","--unified=0","--format=",sha],out_linenos=lm)
  return _classify_paths_tier(_ne(_git_dt("--name-only","-r",sha)),cm,lm,git_ref=sha)
def _commit_max_tier(sha): return _commit_tier_info(sha)[:2]
def _gh_get_json(url,tok,opener=None,*,allow_404=False):
  req=urllib.request.Request(url,headers={"Authorization": f"Bearer {tok}","Accept": "application/vnd.github+json","User-Agent": "Stochology-Custody-CI"})
  try:
    with (opener or urllib.request.urlopen)(req,timeout=10) as r: return json.loads(r.read().decode("utf-8"))
  except urllib.error.HTTPError as exc:
    if allow_404 and exc.code==404: return None
    raise
def _gh_get_paginated(path,tok,opener=None):
  out,pg,sep=[],1,("&" if "?" in path else "?")
  while pg<=20:
    d=_gh_get_json(f"https://api.github.com{path}{sep}per_page=100&page={pg}",tok,opener)
    if not isinstance(d,list) or not d: break
    out.extend(d)
    if len(d)<100: break
    pg+=1
  return out
def _add_co_actors(co,pr_au,pr_ak):
  def _add(v):
    if v:
      pr_au.add(v)
      pr_ak.update(_rik(v))
  for ak in ("author","committer"):
    ga=co.get(ak) or {}
    if (nm := (ga.get("name") or "").strip()): _add(_ni(nm))
    for v in _eei((ga.get("email") or "").strip()): _add(v)
  for ci_t in _extract_coauthors(co.get("message") or ""): _add(ci_t)
def _fetch_github_pr_bundle(repo,pr_num,pr_author,pr_head_sha,token,*,opener=None):
  if not pr_head_sha.strip(): raise RuntimeError("PR_HEAD_SHA required for --github-pr-api")
  _ro= lambda cs: bool(cf := (_ne(_git_dt("--root","--name-only","-r",cs,check=False)) if (cs and _commit_exists_locally(cs)) else [])) and all(_is_rev_md(p) for p in cf)
  pr_au={_ni(pr_author)} if pr_author.strip() else set()
  pr_ak={k for au in pr_au for k in _rik(au)}
  commits=_gh_get_paginated(f"/repos/{repo}/pulls/{pr_num}/commits",token,opener)
  logs,rev_c=set(),[]
  for c in commits:
    if (cs := _pr_str(c,"sha")) and _ro(cs):
      rev_c.append((cs,c))
      continue
    for ak in ("author","committer"):
      if (lg := _pr_str(c,ak,"login")):
        pr_au.add(nl := _ni(lg))
        pr_ak|=_rik(nl)
        if lg.lower()!="web-flow": logs.add(lg)
    _add_co_actors(c.get("commit") or {},pr_au,pr_ak)
  ukc,unc,umc,uec={},{},{},{}
  def _fuk(lr,*,am=False):
    if lr in ukc: return ukc[lr]
    try: uo=_gh_get_json(f"https://api.github.com/users/{urllib.parse.quote(lr,safe='')}",token,opener,allow_404=am)
    except Exception as exc: raise RuntimeError(f"failed /users/{lr}: {exc}") from exc
    if uo is None:
      umc[lr],ukc[lr],unc[lr],uec[lr]=True,set(),"",""
      return set()
    umc[lr],unc[lr],uec[lr]=False,(uo.get("name") or "").strip(),(uo.get("email") or "").strip()
    ukc[lr]={k for f in ("login","name","email") if (fv := (uo.get(f) or "").strip()) for k in _rik(fv)}
    return ukc[lr]
  def _add_u(lr,*,am=True):
    pr_ak.update(_fuk(lr,am=am))
    for uv in (unc.get(lr),uec.get(lr)):
      if uv: pr_au.add(_ni(uv))
  if (pa := pr_author.strip()) and pa.lower()!="web-flow": _add_u(pa,am=pa.lower().endswith("[bot]"))
  for ll in sorted(logs): _add_u(ll)
  pr_hit=_mk_actor_matcher(pr_au,pr_ak)
  def _gh_ok(lg,lt):
    if not lg or lt!="User" or lg.lower().endswith("[bot]"): return False,set(),""
    lk=_fuk(lg,am=True)
    if umc.get(lg): return False,set(),""
    ln,le=unc.get(lg,""),uec.get(lg,"")
    nh=(_nhr(ln) or _nhr(f"{ln} <{lg}@users.noreply.github.com>")) if ln else _nhr(lg)
    ak=_rik(lg)|lk
    return (False,set(),ln) if (nh or pr_hit(lg) or (ln and pr_hit(ln)) or (le and pr_hit(le)) or (ak&pr_ak)) else (True,ak,ln)
  rev_act={}
  for cs,c in rev_c:
    co=c.get("commit") or {}
    gal,gat,gcl,gct=_pr_str(c,"author","login"),(_pr_str(c,"author","type") or "User"),_pr_str(c,"committer","login"),(_pr_str(c,"committer","type") or "User")
    oka,ak,an=_gh_ok(gal,gat)
    akeys,raws,cn=set(ak),([an or gal] if oka else []),""
    valid=oka and bool(gcl) and (co.get("verification") or {}).get("verified") is True
    if valid and gcl.lower()!="web-flow":
      okc,ck,cn=_gh_ok(gcl,gct)
      valid=okc and gal.lower()==gcl.lower()
      if valid:
        akeys|=ck
        raws.append(cn or gcl)
    vhn={l.lower(): nm for l,nm in ((gal,an),(gcl,cn)) if l}
    for akn in ("author","committer"):
      ga=co.get(akn) or {}
      gn,ge=(ga.get("name") or "").strip(),(ga.get("email") or "").strip()
      if akn=="committer" and gcl.lower()=="web-flow" and gn=="GitHub" and ge.lower()=="noreply@github.com": continue
      astr= f"{gn} <{ge}>" if (gn and ge) else (gn or ge)
      if valid and ((astr and _nhr(astr,verified_human_names=vhn)) or pr_hit(gn) or pr_hit(ge)): valid=False
    for ci_t in _extract_coauthors(co.get("message") or ""):
      if valid and (_nhr(ci_t) or pr_hit(ci_t)): valid=False
    if valid and akeys: rev_act[cs]=(akeys,tuple(raws),_ni(gal))
    else:
      rev_act[cs]=(set(),("__UNVERIFIED_REVIEW_COMMIT__",),"")
      for lg in (gal,gcl):
        if lg and lg.lower()!="web-flow":
          pr_au.add(_ni(lg))
          pr_ak|=_rik(lg)
          _add_u(lg)
      _add_co_actors(co,pr_au,pr_ak)
      pr_hit=_mk_actor_matcher(pr_au,pr_ak)
  latest={}
  for rev in _gh_get_paginated(f"/repos/{repo}/pulls/{pr_num}/reviews",token,opener):
    u=rev.get("user") or {}
    lr=(u.get("login") or "").strip()
    if (lgn := _ni(lr)) and _gh_ok(lr,(u.get("type") or "User").strip())[0] and (st := (rev.get("state") or "").upper()) in ("APPROVED","CHANGES_REQUESTED","DISMISSED"):
      latest[lgn]=(st,(rev.get("commit_id") or "").strip(),lr,(rev.get("author_association") or "").strip().upper())
  app_rl={u: rl for u,(st,cid,rl,asc) in latest.items() if st=="APPROVED" and cid==pr_head_sha and asc in ("OWNER","MEMBER","COLLABORATOR")}
  return set(app_rl),pr_au,{u for u,(st,_c,_r,_a) in latest.items() if st=="CHANGES_REQUESTED"},commits,rev_act,{u: unc.get(rl,"").strip() for u,rl in app_rl.items()}
def _fetch_github_pr_identities(repo,pr_num,pr_author,pr_head_sha,token,*,opener=None):
  return _fetch_github_pr_bundle(repo,pr_num,pr_author,pr_head_sha,token,opener=opener)[:3]
def _commit_exists_locally(sha):
  return bool(sha.strip()) and subprocess.run(["git","cat-file","-e",f"{sha.strip()}^{{commit}}"],cwd=ROOT,capture_output=True,text=False,check=False).returncode==0
def _commit_parents(sha): return _rg(["rev-list","--parents","-n","1",sha],check=False).strip().split()[1:]
def _verify_merge_commit_against_pr(sha,phd):
  ph=phd.strip()
  if not ph or not _commit_exists_locally(sha) or not _commit_exists_locally(ph): return False
  ps=_commit_parents(sha)
  if len(ps)>=2: return len(ps)==2 and ps[1]==ph and _is_clean_synthetic_merge_commit(sha,f"{ps[0]} {ps[1]}")
  return (True if ph==sha else _is_clean_synthetic_merge_commit(sha,f"{ps[0]} {ph}")) if len(ps)==1 else ph==sha
def _query_commit_pulls(repo,sha,tok,*,opener=None): return _gh_get_paginated(f"/repos/{repo}/commits/{sha}/pulls",tok,opener)
def _record_pr_meta(out_ra,out_vn,ra,vn,sha_map=None):
  if out_ra is not None:
    for k,v in ra.items():
      tk=sha_map.get(k,k) if sha_map else k
      if tk not in out_ra or not v[0]: out_ra[tk]=v
  if out_vn is not None: out_vn.update(vn)
def _fetch_commit_pr_identities(repo,head_sha,token,*,opener=None,target_branch=None,out_review_actors=None,out_approver_names=None):
  if not head_sha.strip(): raise RuntimeError("HEAD SHA is required when --github-commit-pr-api is set")
  pg,tb=_pr_str,(target_branch or "").strip()
  mg=[p for p in _query_commit_pulls(repo,head_sha,token,opener=opener) if p.get("merged_at") and (not tb or pg(p,"base","ref")==tb)]
  if not mg: return set(),set(),set()
  bm=[p for p in mg if pg(p,"merge_commit_sha")==head_sha]
  if bm:
    if any(not _verify_merge_commit_against_pr(head_sha,pg(p,"head","sha")) for p in bm): return set(),set(),set()
    cands=bm
  else: cands=[p for p in mg if pg(p,"head","sha")==head_sha and _commit_exists_locally(head_sha)]
  if not cands: return set(),set(),set()
  app,au,blk=None,set(),set()
  for pr in cands:
    if not (pn := pg(pr,"number")): continue
    a,u,bl,_cm,ra,vn=_fetch_github_pr_bundle(repo,pn,pg(pr,"user","login"),pg(pr,"head","sha") or head_sha,token,opener=opener)
    app=set(a) if app is None else (app&a)
    au|=u
    blk|=bl
    _record_pr_meta(out_review_actors,out_approver_names,ra,vn)
  return (app or set())-au,au,blk
def _build_push_pr_resolver(repo,base_ref,head_ref,token,target_branch,*,opener=None,out_review_actors=None,out_approver_names=None):
  r_ord,cinfo,pg,_cel,_csm=[],{},_pr_str,_commit_exists_locally,_is_clean_synthetic_merge_commit
  for rec in _rg(["log",f"{base_ref}..{head_ref}","--format=%H%x1f%P%x1f%ae%x1f%aI%x1f%B%x1e"],check=False).split("\x1e"):
    ps=rec.strip("\n").split("\x1f",4)
    if len(ps)<5 or not (s := ps[0].strip()): continue
    r_ord.append(s)
    cinfo[s]={"parents": ps[1].split(),"email": ps[2].strip().lower(),"date": ps[3].strip(),"msg": ps[4].strip()}
  r_set,mapped,cmg,errs,pc,bc,tb=set(r_ord),{},set(),{},{},{},(target_branch or "").strip()
  def _mg(s):
    if s not in pc: pc[s]=_query_commit_pulls(repo,s,token,opener=opener)
    return [p for p in pc[s] if p.get("merged_at") and (not tb or pg(p,"base","ref")==tb)]
  def _bd(pr):
    if not (pn := pg(pr,"number")): return None
    pau,phd=pg(pr,"user","login"),pg(pr,"head","sha")
    if (ck := f"{pn}:{phd}") not in bc: bc[ck]=_fetch_github_pr_bundle(repo,pn,pau,phd,token,opener=opener)
    return bc[ck]
  _rec= lambda ra,vn,sm=None: _record_pr_meta(out_review_actors,out_approver_names,ra,vn,sm)
  _err= lambda s,m: errs.setdefault(s,[]).append(m)
  def _bind(cs,app,au,blk):
    ca=set(app)-set(au)
    if cs not in mapped: mapped[cs]=(ca,set(au),set(blk))
    else:
      pa,pau,pb=mapped[cs]
      cau=pau|set(au)
      mapped[cs]=((pa&ca)-cau,cau,pb|set(blk))
  for s in r_ord:
    ps=cinfo[s]["parents"]
    if len(ps)<2: continue
    if len(ps)!=2:
      _err(s,f"octopus {s[:8]} ({len(ps)}p)")
      continue
    p1,p2=ps
    pulls=[p for p in _mg(s) if pg(p,"merge_commit_sha")==s] or [p for p in _mg(p2) if pg(p,"merge_commit_sha")==s]
    if not pulls: continue
    acl,hbl=True,False
    for pr in pulls:
      phd,pn=pg(pr,"head","sha"),pg(pr,"number")
      if p2!=phd or not _csm(s,f"{p1} {p2}"):
        acl=False
        _err(s,f"merge {s[:8]} != ({p1[:8]},{phd[:8]}) PR #{pn}")
        continue
      if not (bd := _bd(pr)):
        acl=False
        continue
      app,au,blk,prc,ra,vn=bd
      pshas=[cs for c in prc if (cs := pg(c,"sha"))]
      if phd not in pshas and p2 not in pshas:
        acl=False
        _err(s,f"parent {p2[:8]} missing in PR #{pn}")
        continue
      if blk: hbl=True
      _rec(ra,vn)
      _bind(s,app,au,blk)
      for cs in pshas:
        if cs in r_set: _bind(cs,app,au,blk)
    if acl and not hbl: cmg.add(s)
  for s in r_ord:
    if s in mapped or len(cinfo[s]["parents"])!=1: continue
    for pr in [p for p in _mg(s) if pg(p,"merge_commit_sha")==s or pg(p,"head","sha")==s]:
      phd,pn=pg(pr,"head","sha"),pg(pr,"number")
      if not (bd := _bd(pr)): continue
      app,au,blk,prc,ra,vn=bd
      if s==phd:
        _rec(ra,vn)
        for ac in prc:
          if (cs := pg(ac,"sha")) in r_set: _bind(cs,app,au,blk)
        _bind(s,app,au,blk)
        continue
      cb=False
      if len(prc)>=2:
        crev,cur=[],s
        for _ in range(len(prc)):
          if not cur or cur not in r_set:
            crev=[]
            break
          crev.append(cur)
          ps=cinfo[cur]["parents"]
          cur=ps[0] if len(ps)==1 else None
        if len(crev)==len(prc):
          ch=list(reversed(crev))
          if all(cinfo[rs]["email"]==pg(ac,"commit","author","email").lower() and cinfo[rs]["msg"]==pg(ac,"commit","message") for rs,ac in zip(ch,prc)):
            bps=cinfo[ch[0]]["parents"]
            if len(bps)==1 and phd and _cel(phd) and (s==phd or _csm(s,f"{bps[0]} {phd}")):
              _rec(ra,vn,{pg(ac,"sha"): rs for rs,ac in zip(ch,prc)})
              for rs in ch: _bind(rs,app,au,blk)
              cb=True
            else:
              _err(s,f"rebased PR #{pn} {s[:8]} != {phd[:8]}")
              continue
      if cb: continue
      if not _verify_merge_commit_against_pr(s,phd):
        _err(s,f"squashed {s[:8]} != PR #{pn} {phd[:8]}")
        continue
      _rec(ra,vn,{pg(prc[0],"sha"): s} if len(prc)==1 and pg(prc[0],"sha") else None)
      _bind(s,app,au,blk)
      if len(prc)>=2 and phd and _cel(phd):
        pb=_resolve_base_ref(cinfo[s]["parents"][0],phd)
        pal=_audit_commits(max(len(prc),10),pb,phd,resolved_base=pb,verified_approvers_set=app-au,pr_authors_set=au,blocking_reviewers_set=blk,verified_approver_names=vn,review_commit_actors=ra)
        msq=_reviews_history_mutations(_rev_stat(f"{pb}..{phd}"))
        sqe=([f"mutated reviews/ PR #{pn}: {msq}"] if msq else [])+[f"PR #{pn} {pa.sha}: {iss}" for pa in pal for iss in pa.issues]+([f"PR #{pn} CHANGES_REQUESTED: {sorted(blk)}"] if blk else [])
        if pal and not sqe: cmg.add(s)
        else:
          cmg.discard(s)
          errs.setdefault(s,[]).extend(sqe or [f"PR #{pn} empty"])
  return (lambda s: mapped.get(s,(set(),set(),set()))),cmg,errs
def _target_header_covers_path(target_line,req_path):
  return _id_target_header_covers_path(target_line,req_path,ROOT)
def _validate_blind_review_artifact(tpath,text,*,required_paths=None,out_domains=None):
  return _id_validate_blind_review_artifact(tpath,text,ROOT,required_paths=required_paths,out_domains=out_domains)
def _has_bad_trailer_uni(val):
  return any((unicodedata.category(ch) in ("Cc","Cf","Cs","Zl","Zp") and ch!="\t") or ch in "\u115f\u1160\u2800\u3164\uffa0" for ch in val)
def _pairs_hit(xs,ys,keys):
  return any({f"{a} {b}",f"{b} {a}",f"{a}{b}",f"{b}{a}"}&keys for a in xs for b in ys)

def validate_commit_trailers(trailers,inferred_tier,*,author_name="",committer_name="",spec_shrinking=False,verified_approvers=None,pr_authors=None,verified_approvers_set=None,pr_authors_set=None,blocking_reviewers_set=None,review_artifacts_available=None,review_artifacts_added=None,review_artifact_reader=None,commit_tier1_paths=None,review_artifact_add_authors=None,commit_index=None,commit_sha="",same_commit_review_paths=None,allow_pending_blind_review=False,verified_approver_names=None,review_commit_actors=None,is_post_patch_fn=None,review_commit_trailers=None):
  vas,van,rav,rad,rar,raa,rca_m,rct_m,apb=verified_approvers_set,verified_approver_names,review_artifacts_available,review_artifacts_added,review_artifact_reader,review_artifact_add_authors,review_commit_actors,review_commit_trailers,allow_pending_blind_review
  issues,_tr=[],lambda k: trailers[k].split("#",1)[0].strip()
  err,_nh=issues.append,lambda a: _nhr(a,verified_human_names=van)
  if (dup := trailers.get("__DUPLICATE_TRAILERS__")): err(f"duplicate custody trailer(s): {dup}")
  for k in REQUIRED_TRAILERS:
    if k not in trailers or not trailers[k].strip(): err(f"missing {k}")
  if issues: return issues
  for k,v in trailers.items():
    if k not in REQUIRED_TRAILERS: continue
    if _has_bad_trailer_uni(v): err(f"disallowed char in {k}: {v!r}")
    if not (cl := v.split("#",1)[0].strip()) or PLACEHOLDER_RE.search(cl): err(f"empty/placeholder {k}: {v!r}")
  vin,dt=_tr("Authorship-Vintage"),_tr("Custody-Tier")
  if vin not in VALID_VINTAGES: err(f"invalid Authorship-Vintage {vin!r}")
  if dt not in VALID_CUSTODY_TIERS: err(f"invalid Custody-Tier {dt!r}")
  elif dt!=inferred_tier: err(f"Custody-Tier {dt!r} != {inferred_tier!r}")
  if vin=="ai-audit-only" and (inferred_tier!=_TL or dt!=_TL): err(f"ai-audit-only requires {_TL}")
  t1=inferred_tier==_T1 or dt==_T1 or spec_shrinking
  blk,bk=set(pr_authors_set or ()),set()
  for tok in (author_name,committer_name,*(_split_address_list(pr_authors) if pr_authors else ())):
    if tok:
      if (tn := _ni(tok)): blk.add(tn)
      blk.update(_eei(tok))
      bk.update(_rik(tok))
  _mb=_mk_actor_matcher(blk,bk)
  nav={_nr(p) for p in rav} if rav is not None else None
  nad={_nr(p) for p in rad} if rad is not None else None
  rots=[t.strip() for t in _rsp(r"[,;]+",_tr("Oracle-Verified")) if t.strip()]
  if not rots: err("empty Oracle-Verified trailer")
  else:
    porc,br_p,ov_s,dst=[],"blind-review","in Oracle-Verified",{"req": set(),"cov": set()}
    for rt in rots:
      if ":" in rt:
        tnm,tpr=rt.split(":",1)
        tnm,tps=tnm.strip().lower(),tpr.strip().replace("\\","/")
        if tnm!=br_p:
          err(f"invalid suffix {ov_s}: {rt!r}")
          continue
        tp=_nr(tps) if tps else ""
        if not tp or tp in (".","..") or tps.startswith("/") or tp.startswith(("/",".")) or ".." in tp.split("/") or not _is_selectable_review_artifact(tp):
          err(f"invalid {br_p} {ov_s}: {rt!r}")
          continue
        if same_commit_review_paths and tp in same_commit_review_paths:
          err(f"self-bundled {br_p} {tp!r}")
          continue
        if apb:
          if nav is not None and tp in nav: err(f"stale pre-existing {br_p} {tp!r}")
          else: porc.append((tnm,tp))
          continue
        if (tp not in nav) if nav is not None else (rar is None and not (ROOT / tp).is_file()):
          err(f"missing {br_p} {tp!r} {ov_s}")
          continue
        if nad is not None and tp not in nad:
          err(f"stale {br_p} {tp!r} {ov_s}")
          continue
        if raa is not None:
          pfn,vlg,vks=is_post_patch_fn or _is_git_ancestor,{_ni(va) for va in (vas or ()) if va},{k for va in (*(vas or ()),*(van or {}).values()) for k in _rik(va)}
          vok=False
          for _,rs,rk,rr in raa.get(tp,[]):
            if rs==commit_sha or (commit_sha and not pfn(commit_sha,rs)) or (rk&bk): continue
            cwf=bool(vas is not None and rca_m and rs in rca_m and rca_m[rs][0] and "__UNVERIFIED_REVIEW_COMMIT__" not in rca_m[rs][1])
            if any(a and not (si==1 and cwf and a.strip().lower()=="github <noreply@github.com>") and (_nh(a) or _mb(a)) for si,a in enumerate(rr)): continue
            if rct_m is not None and rs in rct_m:
              ci2,ct2=rct_m[rs]
              cv2={k: ct2.get(k,"").split("#",1)[0].strip().lower() for k in REQUIRED_TRAILERS}
              if "__DUPLICATE_TRAILERS__" in ct2 or ci2!=_TL or cv2["Custody-Tier"]!=_TL or cv2["Authorship-Vintage"]!="human-original" or cv2["Oracle-Verified"]!="na" or cv2["Human-T1-Signoff"]!="na": continue
            if vas is not None:
              if not rca_m or rs not in rca_m: continue
              rca=rca_m[rs]
              gk,gr,aok=rca[0],rca[1],((rca[2] in vlg and not _mb(rca[2])) if len(rca)>2 else bool(rca[0]&vks))
              if not gk or "__UNVERIFIED_REVIEW_COMMIT__" in gr or (gk&bk) or any(_nh(a) or _mb(a) for a in gr if a) or not aok: continue
            vok=True
            break
          if not vok:
            err(f"{br_p} {tp!r} needs distinct post-patch human reviewer")
            continue
        atx=rar(tp) if rar is not None else (_rt(ROOT / tp) if (ROOT / tp).is_file() else None)
        if atx is None: err(f"unreadable {br_p} {tp!r}")
        elif (ae := _validate_blind_review_artifact(tp,atx,required_paths=commit_tier1_paths,out_domains=dst)): err(f"invalid {br_p} {tp!r}: {ae}")
        else: porc.append((tnm,tp))
      else:
        tnm=rt.lower()
        if tnm==br_p: err(f"bare 'blind-review' {ov_s}; use 'blind-review:reviews/<report>.md'")
        elif tnm not in VALID_ORACLE_TOKENS: err(f"unknown Oracle-Verified: {rt!r}")
        else: porc.append((tnm,None))
    onm={n for n,_ in porc}
    if "na" in onm and (inferred_tier!=_TL or dt!=_TL): err(f"Oracle-Verified cannot be 'na' unless {_TL}")
    if inferred_tier==_TL and dt==_TL and onm!={"na"}: err(f"Oracle-Verified must be 'na' on {_TL}")
    if t1 and porc:
      if "layer0-gauntlet" not in onm and not ({"structure","notation","sympy","custody"}<=onm): err(f"{_T1} needs 'layer0-gauntlet' or 4 checks")
      if not any(n==br_p and p is not None for n,p in porc): err(f"tier1 needs 'blind-review:reviews/<report>.md' {ov_s}")
      elif commit_tier1_paths and not apb and dst["req"]-dst["cov"]: err(f"tier1 missing domain coverage: {sorted(dst['req']-dst['cov'])[:4]}")
  if blocking_reviewers_set: err(f"CHANGES_REQUESTED blocks merge: {sorted(blocking_reviewers_set)}")
  if t1:
    _ud= lambda s: (_ru(r"[()\[\]{}«»]","",s),_ru(r"[()\[\]{}«»]+"," ",s))
    def _mcl(toks):
      cls=[]
      for t in toks:
        tk=set()
        for cd in (t,*_ud(t)):
          tk|=_rik(cd)
          if (ek := _rek(cd)): tk.add(ek)
        if tk:
          mg,cls=tk,[cl for cl in cls if not (tk&cl and (tk.update(cl) or True))]
          cls.append(mg)
      return cls
    def _mcm(left,right):
      mr={}
      def dfs(u,seen):
        for v,rc in enumerate(right):
          if v not in seen and (left[u]&rc):
            seen.add(v)
            if v not in mr or dfs(mr[v],seen):
              mr[v]=u
              return True
        return False
      return sum(1 for u in range(len(left)) if dfs(u,set()))
    def _wb(tok):
      out=[]
      for pc in _ru(r"[()\[\]{}«»,;<>\"'`]+"," ",EMAIL_TOKEN_RE.sub(" ",_strip_invisible_and_fold(tok))).lower().split():
        ps=[w for w in _rfa(r"[^\W_]+",pc) if w]
        out.extend(ps)
        if len(ps)>=2 and (jn := "".join(ps)) not in out: out.append(jn)
      return list(dict.fromkeys([*out,*(f"{out[k]}{out[k+1]}" for k in range(len(out)-1))]))
    def _frc(toks):
      if len(toks)<2: return set()
      bad,nms,wbs=set(),[_ni(t.strip()) for t in toks],[_wb(t) for t in toks]
      for i in range(len(toks)):
        if wbs[i]:
          for j in range(i+1,len(toks)):
            if wbs[j] and (_pairs_hit([nms[i]],[nms[j]],bk) or _pairs_hit(wbs[i][:24],wbs[j][:24],bk)): bad.update({i,j})
      return bad
    sr=trailers["Human-T1-Signoff"]
    revs=_parse_reviewers(sr)
    if (amb := _frc(revs)): err(f"self-signoff in Human-T1-Signoff: split tokens match author in {sr!r}")
    vl=[]
    for idx,rev in enumerate(revs):
      if idx in amb: continue
      rc,pgs=_strip_delimited_groups(rev)
      if _nh(rev):
        err(f"non-human Human-T1-Signoff: {rev!r}")
        continue
      rn,rems=_ni(rc),_eei(rev)
      bws=[w for w in rn.split() if w]
      pb=any(_mb(c) for c in _ud(rev)) or any(_mb(pg) or _pairs_hit(bws,[pw for pw in _rsp(r"[\s,;]+",_ni(pg)) if pw],bk) for pg in pgs)
      sns=[sn for sn in (_ni(t) for t in _ru(r"<[^>]+>","",rc).replace('"',"").replace("'","").replace("`","").split(",")) if sn] if "," in rc else []
      cb=any(_mb(sn) for sn in sns) or (len(sns)>=2 and any(_pairs_hit([sns[i]],[sns[j]],bk) or _pairs_hit(sns[i].split()[:6],sns[j].split()[:6],bk) for i in range(len(sns)) for j in range(i+1,len(sns))))
      if pb or cb or _mb(rev) or _mb(rc):
        err(f"self-signoff in Human-T1-Signoff: {rev!r} matches author")
        continue
      hc=EMAIL_TOKEN_RE.sub("",_ru(r"<[^>]+>","",rc)).strip()
      if verified_approvers is None and vas is None and not (re.fullmatch(r"@(?:[A-Za-z0-9]|[A-Za-z0-9][A-Za-z0-9-]{0,37}[A-Za-z0-9])",hc) or len(rn.split())>=2): err(f"ambiguous Human-T1-Signoff {rev!r}; use '@handle' or full name")
      elif rn or rems: vl.append(rev)
    rcls,req_r=_mcl(vl),2 if spec_shrinking else 1
    if len(rcls)<req_r and not any("signoff" in i.lower() for i in issues): err("spec-shrinking needs >= 2 distinct non-author Human-T1-Signoff" if spec_shrinking else "tier1 needs named non-author Human-T1-Signoff")
    if verified_approvers is not None or vas is not None:
      vtoks=[t for t in (list(vas) if vas is not None else _split_address_list(verified_approvers or "")) if t and not _nh(t) and not _mb(t)]
      vcls=_mcl(vtoks)
      if len(vcls)<req_r or _mcm(vcls,rcls)<req_r: err(f"PR approvals {sorted(vtoks)} < {req_r} distinct Human-T1-Signoff")
  elif _tr("Human-T1-Signoff").lower()!="na": err("Human-T1-Signoff must be 'na' on non-tier1 commits")
  return issues
def _parse_trailers(text):
  stanzas,cur=[],[]
  for ln in _ll(text.replace("\r\n","\n")):
    if ln.lstrip().startswith("#"): continue
    if not ln.strip():
      if cur:
        stanzas.append(cur)
        cur=[]
    else: cur.append(ln)
  if cur: stanzas.append(cur)
  if not stanzas: return {}
  tr,dups={},[]
  for line in stanzas[-1]:
    for k in REQUIRED_TRAILERS:
      if (m := _rm(rf"^{re.escape(k)}:\s*(.+)$",line.strip())):
        if k in tr: dups.append(k)
        tr[k]=m.group(1).strip()
  if dups: tr["__DUPLICATE_TRAILERS__"]=", ".join(sorted(set(dups)))
  return tr
MUTABLE_REVIEW_POINTERS={"reviews/00-deterministic-report.md","reviews/triage-latest.md"}
def _reviews_history_mutations(status_args):
  bad=[]
  for line in _ll(_rg(status_args,check=False)):
    parts=line.split("\t")
    if len(parts)<2 or not (st := parts[0].strip()): continue
    code=st[0].upper()
    for path in parts[1:]:
      if (pn := _nr(path)).startswith("reviews/") and pn not in MUTABLE_REVIEW_POINTERS and code!="A" and pn not in bad: bad.append(pn)
  return bad
def _is_clean_synthetic_merge_commit(full_sha,parents):
  if len(ps := parents.split())!=2: return False
  mt=subprocess.run(["git","merge-tree","--write-tree",ps[0],ps[1]],cwd=ROOT,capture_output=True,text=True,check=False)
  return bool(mt.returncode==0 and (out := mt.stdout.strip()) and (exp := out.splitlines()[0].strip()) and exp==_rev_verify(full_sha,"tree",check=False))
def _is_selectable_review_artifact(rel_path):
  return _is_rev_md(rel_path) and rel_path not in MUTABLE_REVIEW_POINTERS and not posixpath.basename(rel_path).startswith(("00-deterministic-report","triage"))
def _collect_review_artifacts(status_args,*,tree_ref=None,include_disk=False):
  added={p for ln in _ll(_rg(status_args,check=False) if status_args else "") if len(ps := ln.split("\t"))>=2 and ps[0].strip()[:1].upper()=="A" and _is_selectable_review_artifact(p := _nr(ps[-1]))}
  avail=set(added)
  if tree_ref: avail.update(p for ln in _ll(_rg(["ls-tree","-r","--name-only",tree_ref,"--","reviews/"],check=False)) if _is_selectable_review_artifact(p := _nr(ln)))
  if include_disk and (rev_dir := ROOT / "reviews").is_dir(): avail.update(p for fp in rev_dir.rglob("*.md") if fp.is_file() and _is_selectable_review_artifact(p := _nr(fp.relative_to(ROOT).as_posix())))
  return avail,added
def _commit_actor_identity_info(author_name,author_email,committer_name,committer_email,body):
  keys,raws=set(),[]
  for idx,(n,e) in enumerate(((author_name,author_email),(committer_name,committer_email))):
    ns,es=(n or "").strip(),(e or "").strip()
    raws.append(f"{ns} <{es}>" if (ns and es) else (ns or es))
    if idx==1 and ns.lower()=="github" and es.lower()=="noreply@github.com": continue
    for part in (ns,es):
      if part: keys|=_rik(part)
  for co in _extract_coauthors(body):
    raws.append(co)
    keys|=_rik(co)
  return keys,tuple(raws)
def _audit_commits(limit,diff_ref,head_ref="HEAD",resolved_base=None,skip_synthetic_pr_merge=False,verified_approvers=None,pr_authors=None,verified_approvers_set=None,pr_authors_set=None,blocking_reviewers_set=None,trailer_cutoff_commit=None,commit_pr_resolver=None,clean_verified_merges=None,merge_blocking_errors=None,verified_approver_names=None,review_commit_actors=None,allow_pending_on_non_main=False):
  fmt="%h%x1f%H%x1f%P%x1f%an%x1f%ae%x1f%cn%x1f%ce%x1f%s%x1f%b%x1e"
  r_head=_rev_verify(head_ref) if skip_synthetic_pr_merge else ""
  cur_br=_rg(["symbolic-ref","--short","HEAD"],check=False).strip() if (allow_pending_on_non_main and verified_approvers_set is None and commit_pr_resolver is None) else ""
  p_br_ok=bool(cur_br and cur_br not in ("main","master","HEAD"))
  if trailer_cutoff_commit: _rev_verify(trailer_cutoff_commit)
  r_adds,r_trails,pc_adds,sha_idx={},{},{},{}
  if diff_ref:
    rrng= f"{resolved_base or _resolve_base_ref(diff_ref,head_ref)}..{head_ref}"
    if not (rshas := _ne(_rg(["rev-list","--topo-order","--reverse",rrng]))): return []
    args,(r_avail,r_added)=["log",rrng,f"--format={fmt}"],_collect_review_artifacts(_rev_stat(rrng),tree_ref=head_ref)
  else:
    rshas,args=list(reversed(_ne(_rg(["rev-list","--topo-order",f"-n{limit}",head_ref],check=False)))),["log",f"-n{limit}",head_ref,f"--format={fmt}"]
    r_avail,r_added=_collect_review_artifacts([],tree_ref=head_ref)[0],set()
  for idx,csha in enumerate(rshas):
    sha_idx[csha]=idx
    if not (c_add := _collect_review_artifacts(_dt_rev(csha,"--root"))[1]): continue
    pc_adds[csha]=set(c_add)
    if not diff_ref and r_added is not None: r_added|=c_add
    mp=_rg(["log","-n1","--format=%an%x1f%ae%x1f%cn%x1f%ce%x1f%b",csha],check=False).split("\x1f",4)
    c2_b=mp[4] if len(mp)>4 else ""
    rk,rr=_commit_actor_identity_info(*mp[:4],c2_b) if len(mp)>=4 else (set(),())
    r_trails[csha]=(_commit_tier_info(csha)[0],_parse_trailers(c2_b))
    for rp in c_add: r_adds.setdefault(rp,[]).append((idx,csha,rk,rr))
  audits=[]
  for rec in _rg(args).split("\x1e"):
    if not (rec := rec.strip("\n")) or len(ps := rec.split("\x1f",8))<8: continue
    ssha,fsha,par,anm,aem,cnm,cem,subj=ps[:8]
    body=ps[8] if len(ps)>8 else ""
    mut_r,merrs=_reviews_history_mutations(_dt_rev(fsha)),list((merge_blocking_errors or {}).get(fsha,[]))
    if skip_synthetic_pr_merge and fsha==r_head and SYNTHETIC_PR_MERGE_RE.match(subj.strip()) and _is_clean_synthetic_merge_commit(fsha,par) and not mut_r and not merrs: continue
    if (clean_verified_merges and fsha in clean_verified_merges and not mut_r and not merrs) or (trailer_cutoff_commit and _iga(fsha,trailer_cutoff_commit)): continue
    cv,cb,cblk=verified_approvers_set,blocking_reviewers_set,set(pr_authors_set or ())
    if commit_pr_resolver is not None:
      cv,rau,cb=commit_pr_resolver(fsha)
      cblk|=rau
    tr,(inf_t,shrink,t1ps)=_parse_trailers(body),_commit_tier_info(fsha)
    cblk|=_eei(aem)|_eei(cem)|_extract_coauthors(body)
    _rdr= lambda rp,_cs=fsha: ((_rgt(rp,git_ref=head_ref) or _rgt(rp,git_ref=_cs)) if diff_ref else (_rgt(rp,git_ref=_cs) or _rgt(rp,git_ref=head_ref) or _rgt(rp))) or None
    c_idx=sha_idx.get(fsha,-1)
    cr_add={p for p in r_added if any(_iga(fsha,rs) for _,rs,_,_ in r_adds.get(p,[]))} if (r_added is not None and fsha) else r_added
    dbr=[t.split(":",1)[1].strip() for t in _rsp(r"[,;]+",tr.get("Oracle-Verified","").split("#",1)[0]) if t.strip().lower().startswith("blind-review:")]
    issues=validate_commit_trailers(
      tr,inf_t,author_name=anm,committer_name=cnm,spec_shrinking=shrink,verified_approvers=verified_approvers,pr_authors=pr_authors,
      verified_approvers_set=cv,pr_authors_set=cblk,blocking_reviewers_set=cb,review_artifacts_available=r_avail,review_artifacts_added=cr_add,
      review_artifact_reader=_rdr,commit_tier1_paths=t1ps,review_artifact_add_authors=r_adds,commit_index=c_idx if c_idx>=0 else None,
      commit_sha=fsha,same_commit_review_paths=pc_adds.get(fsha,set()),verified_approver_names=verified_approver_names,
      review_commit_actors=review_commit_actors,review_commit_trailers=r_trails,
      allow_pending_blind_review=bool(p_br_ok and dbr and all(_nr(p) not in (cr_add or set()) for p in dbr)),
    )
    if merrs: issues.extend(merrs)
    if mut_r: issues.append(f"mutated/deleted reviews/: {mut_r}")
    audits.append(CommitAudit(sha=ssha,subject=subj,author=anm,inferred_tier=inf_t,spec_shrinking=shrink,trailers=tr,issues=issues,status="PASS" if not issues else "INVALID-OR-MISSING"))
  return audits
def _audit_commit_msg_file(msg_path):
  text,srm,sp,lm=_rt(msg_path),_reviews_history_mutations(_rev_stat(staged=True)),_ne(_git_diff("--cached","--name-only")),{}
  cm,_=_extract_diff_changed_lines(["diff","-a","--cached","--no-renames","--unified=0"],out_linenos=lm)
  highest,sh,t1p=_classify_paths_tier(sp,cm,lm,cached_index=True)
  if (not sp or "amend" in os.environ.get("GIT_REFLOG_ACTION","").lower()) and _commit_exists_locally("HEAD"):
    ht,hs,h1=_commit_tier_info("HEAD")
    rk={_TL: 0,_T2: 1,_T1: 2}
    if rk[ht]>rk[highest]: highest=ht
    sh=sh or hs
    t1p.extend(tp for tp in h1 if tp not in t1p)
  if sh: highest=_T1
  _ge= lambda k,fb="": (os.environ.get(k) or fb).strip()
  cn0,ce0=_rg(["config","user.name"],check=False).strip(),_rg(["config","user.email"],check=False).strip()
  an,cn,ae,ce=_ge("GIT_AUTHOR_NAME",cn0),_ge("GIT_COMMITTER_NAME",cn0 or _ge("GIT_AUTHOR_NAME",cn0)),_ge("GIT_AUTHOR_EMAIL",ce0),_ge("GIT_COMMITTER_EMAIL",ce0)
  issues=validate_commit_trailers(
    _parse_trailers(text),highest,author_name=an,committer_name=cn,spec_shrinking=sh,
    pr_authors_set=_extract_coauthors(text)|_eei(ae)|_eei(ce),
    review_artifacts_available=_collect_review_artifacts([],tree_ref="HEAD" if _commit_exists_locally("HEAD") else None)[0],
    review_artifacts_added=None,commit_tier1_paths=t1p,
    same_commit_review_paths={_nr(p) for p in sp if _is_rev_md(p)} if highest==_T1 else set(),
    allow_pending_blind_review=True,
  )
  if srm: issues.append(f"mutated staged reviews/: {srm}")
  if issues:
    sys.stderr.write(f"Commit provenance FAILED ({highest}):\n" + "".join(f"  - {i}\n" for i in issues))
    return 1
  return 0
def analyze(commits=5,diff_ref=None,head_ref="HEAD",strict_trailers=False,strict_bloat=False,staged=False,working_tree=False,**audit_kw):
  rlm,abm,rsm,su={},{},{},[]
  numstats,clm,nwm,wdm,rbase=_working_stats(diff_ref,head_ref,staged=staged,working_tree=working_tree,out_raw_changed=rlm,out_added_bytes=abm,out_raw_svg_added=rsm,out_svg_unsafe=su)
  tc,cf,pbf,pnb={_T1: 0,_T2: 0,_TL: 0},[],[],[]
  st=dict.fromkeys("book_added book_deleted total_prose_churn net_prose_delta net_added_words positive_prose_lines positive_prose_words gross_word_diff_added svg_added_bytes svg_unsafe_files governance_added governance_deleted governance_churn governance_word_diff_added governance_added_bytes code_added code_deleted code_churn code_word_diff_added code_added_bytes review_added review_word_diff_added review_added_bytes".split(),0)
  st["svg_unsafe_files"]=su
  rhm=_reviews_history_mutations(_rev_stat(staged=True) if staged else _rev_stat(_diff_target_args(diff_ref,head_ref,working_tree=working_tree)[0]))
  nb={"code": (PER_FILE_CODE_ADDED_LINE_THRESHOLD,PER_FILE_CODE_WORD_DIFF_THRESHOLD,PER_FILE_CODE_ADDED_BYTES_THRESHOLD),"review": (PER_FILE_REVIEW_ADDED_LINE_THRESHOLD,PER_FILE_REVIEW_WORD_DIFF_THRESHOLD,PER_FILE_REVIEW_ADDED_BYTES_THRESHOLD),"governance": (PER_FILE_GOVERNANCE_ADDED_LINE_THRESHOLD,PER_FILE_GOVERNANCE_WORD_DIFF_THRESHOLD,PER_FILE_GOVERNANCE_ADDED_BYTES_THRESHOLD)}
  for an,dn,path in numstats:
    cl,rl=clm.get(path,[]),rlm.get(path,[])
    pss=_has_path_spec_shrinking(path,rl,cl)
    tier=_T1 if pss else classify_file(path,cl)
    tc[tier]+=1
    nl,nw,wda,ab=an-dn,nwm.get(path,0),wdm.get(path,0),abm.get(path,0)
    cf.append({"path": path,"tier": tier,"spec_shrinking": pss,"added": an,"deleted": dn,"net": nl,"net_words": nw,"word_diff_added": wda,"added_bytes": ab})
    if _ipf(path):
      for k,v in (("book_added",an),("book_deleted",dn),("net_added_words",nw),("gross_word_diff_added",wda),("positive_prose_lines",max(0,nl)),("positive_prose_words",max(0,nw))): st[k]+=v
      if path.endswith(".svg"):
        st["svg_added_bytes"]+=ab
        if ab>PER_FILE_SVG_ADDED_BYTES_THRESHOLD or rsm.get(path,0)>PER_FILE_SVG_RAW_ADDED_LINES_THRESHOLD: pbf.append(path)
      if (nl>PER_FILE_BLOAT_LINE_THRESHOLD or nw>PER_FILE_BLOAT_WORD_THRESHOLD or an>PER_FILE_GROSS_ADDED_LINE_THRESHOLD or wda>PER_FILE_GROSS_WORD_DIFF_THRESHOLD) and path not in pbf: pbf.append(path)
    else:
      pfx="code" if _is_code_bloat_file(path) else ("review" if _is_review_bloat_file(path) else "governance")
      st[f"{pfx}_added"]+=an
      if pfx!="review": st[f"{pfx}_deleted"]+=dn
      st[f"{pfx}_word_diff_added"]+=wda
      st[f"{pfx}_added_bytes"]+=ab
      if any(v>t for v,t in zip((an,wda,ab),nb[pfx])): pnb.append(path)
  st["net_prose_delta"],st["total_prose_churn"]=st["book_added"]-st["book_deleted"],st["book_added"]+st["book_deleted"]
  st["governance_churn"],st["code_churn"]=st["governance_added"]+st["governance_deleted"],st["code_added"]+st["code_deleted"]
  bloat_alert=bool(pbf or pnb) or any(st[k]>t for k,t in (
    ("positive_prose_lines",PROSE_BLOAT_LINE_THRESHOLD),("positive_prose_words",PROSE_BLOAT_WORD_THRESHOLD),("book_added",GROSS_PROSE_ADDED_LINE_THRESHOLD),
    ("total_prose_churn",TOTAL_PROSE_LINE_CHURN_THRESHOLD),("gross_word_diff_added",GROSS_WORD_DIFF_THRESHOLD),("svg_added_bytes",SVG_TOTAL_ADDED_BYTES_THRESHOLD),
    ("governance_added",GOVERNANCE_ADDED_LINE_THRESHOLD),("governance_churn",GOVERNANCE_CHURN_LINE_THRESHOLD),("governance_word_diff_added",GOVERNANCE_WORD_DIFF_THRESHOLD),
    ("governance_added_bytes",GOVERNANCE_ADDED_BYTES_THRESHOLD),("code_added",CODE_ADDED_LINE_THRESHOLD),("code_churn",CODE_CHURN_LINE_THRESHOLD),
    ("code_word_diff_added",CODE_WORD_DIFF_THRESHOLD),("code_added_bytes",CODE_ADDED_BYTES_THRESHOLD),("review_added",REVIEW_TOTAL_ADDED_LINE_THRESHOLD),
    ("review_word_diff_added",REVIEW_WORD_DIFF_THRESHOLD),("review_added_bytes",REVIEW_ADDED_BYTES_THRESHOLD),
  ))
  pt,gt=_diff_paths(diff_ref,PROTECTED_SURFACES,head_ref,staged=staged,working_tree=working_tree),_diff_paths(diff_ref,GOVERNANCE_MANIFEST_SURFACES,head_ref,staged=staged,working_tree=working_tree)
  ca=[] if staged else _audit_commits(commits,diff_ref,head_ref,resolved_base=rbase,**audit_kw)
  bc,ere=len(rhm)+len(su),False
  if strict_trailers:
    if diff_ref and not staged and not ca and cf and not audit_kw.get("clean_verified_merges"): ere,bc=True,bc+1
    bc+=sum(1 for a in ca if a.issues)
  if strict_bloat and bloat_alert: bc+=1
  return {"summary": {"files_in_diff": len(cf),"tier_counts": tc,**st,"bloat_alert": bloat_alert,"per_file_bloat_files": pbf,"per_file_non_prose_bloat_files": pnb,"protected_touched": pt,"governance_touched": gt,"reviews_history_mutated": rhm,"strict_trailers": strict_trailers,"strict_bloat": strict_bloat,"staged": staged,"empty_range_error": ere,"blocking_count": bc,"verdict": "PASS" if bc==0 else "FAIL"},"files": cf,"commits": [asdict(c) for c in ca]}
def format_markdown(report):
  s,tc=report["summary"],report["summary"]["tier_counts"]
  ls=[
    "## Custody Tier & Git Vintage Provenance (`.agents/scripts/check_custody_provenance.py`)","",
    f"- **Status:** `{s['verdict']}` | **Tiers:** `tier1={tc[_T1]}`, `tier2={tc[_T2]}`, `tooling={tc[_TL]}` | **Prose:** `+{s['book_added']}/-{s['book_deleted']}` (churn `{s['total_prose_churn']}L`, net `{s['net_prose_delta']:+d}L/{s['net_added_words']:+d}w`, pos `+{s['positive_prose_lines']}L/+{s['positive_prose_words']}w`, wd `+{s['gross_word_diff_added']}w`) | **Non-prose:** gov `+{s['governance_added']}` (`{s['governance_churn']}L`, wd `+{s['governance_word_diff_added']}w`, `{s['governance_added_bytes']}B`), code `+{s['code_added']}` (`{s['code_churn']}L`, wd `+{s['code_word_diff_added']}w`, `{s['code_added_bytes']}B`), rev `+{s['review_added']}L` (wd `+{s['review_word_diff_added']}w`, `{s['review_added_bytes']}B`)",
    f"- **Bloat Alert:** `{s['bloat_alert']}` | **Protected:** `{len(s['protected_touched'])}` | **Gov manifests:** `{len(s['governance_touched'])}`","",
  ]
  add=ls.append
  if s["empty_range_error"]: add("- **Trailer Error:** empty audit range.\n")
  for k,lbl in (("reviews_history_mutated","Append-Only Error"),("svg_unsafe_files","SVG Error"),("per_file_bloat_files","Prose bloat"),("per_file_non_prose_bloat_files","Non-prose bloat")):
    if s[k]: add(f"- **{lbl}:** `{'`, `'.join(s[k])}`\n")
  if report["files"]:
    ls.extend(["### Modified Files by Custody Tier","","| Path | Custody Tier | + / - / Net (Words) |","| :--- | :--- | :---: |"])
    for f in report["files"][:45]:
      ws= f" ({f['net_words']:+d}w, wd+{f['word_diff_added']}w)" if _ipf(f["path"]) else f" (wd+{f['word_diff_added']}w, {f['added_bytes']}B)"
      add(f"| `{f['path']}` | `{f['tier']}` | `+{f['added']} / -{f['deleted']} ({f['net']:+d}){ws}` |")
    if len(report["files"])>45: add(f"| `...` | *(truncated)* | `+{len(report['files'])-45} more (--json)` |")
    add("")
  if report["commits"]:
    ls.extend(["### Recent Commit Trailer Audit","","| Commit | Tier | Subject | Status | Issues |","| :--- | :--- | :--- | :---: | :--- |"])
    for c in report["commits"]: add(f"| `{c['sha']}` | `{c['inferred_tier']}` | {c['subject'][:52]} | `{c['status']}` | {'; '.join(c['issues']) if c['issues'] else '—'} |")
  return "\n".join(ls)+"\n"
def main(argv=None):
  p=argparse.ArgumentParser(description="Audit custody tiers, bloat, and provenance trailers.")
  p.add_argument("--commits",type=int,default=5)
  p.add_argument("--head-ref",type=str,default="HEAD")
  for fd,kw in ((("--diff-ref","--verified-approvers","--pr-authors","--target-branch","--trailer-cutoff-commit"),dict(type=str,default=None)),(("--staged","--working-tree","--github-pr-api","--github-commit-pr-api","--skip-synthetic-pr-merge","--allow-pending-on-non-main","--markdown","--json","--strict-trailers","--strict-bloat"),dict(action="store_true")),(("--out","--commit-msg-file"),dict(type=Path))):
    for fl in fd: p.add_argument(fl,**kw)
  args=p.parse_args(argv)
  try:
    if args.commit_msg_file: return _audit_commit_msg_file(args.commit_msg_file)
    v_set=pa_set=b_set=c_res=c_merges=m_errs=r_act=v_names=None
    if args.github_pr_api:
      repo,pr_num,pr_au,phd,tok=(os.environ.get(k,"") for k in "GITHUB_REPOSITORY PR_NUM PR_AUTHOR PR_HEAD_SHA GH_TOKEN".split())
      if not (repo and pr_num and phd and tok): raise RuntimeError("--github-pr-api missing env vars")
      v_set,pa_set,b_set,_cm,r_act,v_names=_fetch_github_pr_bundle(repo,pr_num,pr_au,phd,tok)
    elif args.github_commit_pr_api:
      repo,tok,tb=os.environ.get("GITHUB_REPOSITORY",""),os.environ.get("GH_TOKEN",""),(args.target_branch or os.environ.get("GITHUB_REF_NAME") or "").strip()
      if not (repo and tok and tb): raise RuntimeError("--github-commit-pr-api missing env/branch")
      _rev_verify(args.head_ref)
      r_act,v_names={},{}
      if args.diff_ref: c_res,c_merges,m_errs=_build_push_pr_resolver(repo,_resolve_base_ref(args.diff_ref,args.head_ref),args.head_ref,tok,tb,out_review_actors=r_act,out_approver_names=v_names)
      else:
        pc={}
        c_res= lambda sha: pc.setdefault(sha,_fetch_commit_pr_identities(repo,sha,tok,target_branch=tb,out_review_actors=r_act,out_approver_names=v_names))
    report=analyze(**{k: getattr(args,k) for k in "commits diff_ref head_ref strict_trailers strict_bloat staged working_tree skip_synthetic_pr_merge allow_pending_on_non_main verified_approvers pr_authors trailer_cutoff_commit".split()},verified_approvers_set=v_set,pr_authors_set=pa_set,blocking_reviewers_set=b_set,commit_pr_resolver=c_res,clean_verified_merges=c_merges,merge_blocking_errors=m_errs,verified_approver_names=v_names,review_commit_actors=r_act)
  except (RuntimeError,OSError,urllib.error.URLError) as exc:
    sys.stderr.write(f"custody provenance check ERROR: {exc}\n")
    return 1
  text=(json.dumps(report,indent=2)+"\n") if args.json else format_markdown(report)
  if args.out:
    args.out.parent.mkdir(parents=True,exist_ok=True)
    args.out.write_text(text,encoding="utf-8")
  sys.stdout.write(text)
  return 0 if report["summary"]["blocking_count"]==0 else 1
if __name__=="__main__":
  raise SystemExit(main())
