#!/usr/bin/env python3
"""Style & citation oracle."""
from __future__ import annotations
import argparse,json,os,re,subprocess,sys,unicodedata as _ud,urllib.error as _ue,urllib.parse as _up,urllib.request as _ur,xml.etree.ElementTree as ET
from dataclasses import asdict,dataclass
from pathlib import Path
ROOT=Path(os.environ.get("STOCHOLOGY_REPO_ROOT") or Path(__file__).resolve().parents[2]).resolve()
_BS,_FG=ROOT/"book-src",ROOT/"book-src/figures"
_AC,_AG,_CT,_RM=_BS/"appendix-c-sources.md",ROOT/"AGENTS.md",ROOT/"CONTRIBUTING.md",ROOT/"README.md"
_EM,_rc,_rm,_rs,_ru,_rf,_I,_B,_A,_U8="\u2014",re.compile,re.match,re.search,re.sub,re.findall,re.I,"BLOCKING","ADVISORY","utf-8"
_fs=frozenset
_CX={"SUMMARY.md","appendix-c-sources.md"}
_EX=_CX|{"AGENTS.md"}
_CQ='"the cults of communicability and impartiality\u2014again, with or without accuracy"'
_AQ={"45-world-to-archive-big-data-paradox.md":('"As the relevant evidence at our disposal increases, the magnitude of the probability of the argument may either decrease or increase, according as the new knowledge strengthens the unfavourable or the favourable evidence; but *something* seems to have increased in either case,\u2014we have a more substantial basis upon which to rest our conclusion."',),"50-methodological-families-reflexivity.md":(_CQ,),"appendix-d-claim-ledger.md":(_CQ,"*ISO 16:1975: Acoustics \u2014 Standard tuning frequency (Standard musical pitch)*")}
_AM={c:("BOM / ZERO WIDTH NO-BREAK SPACE" if c=="\ufeff" else _ud.name(c,"FORMAT")) for c in "\u00ad\u034f\u115f\u1160\u17b4\u17b5\u180b\u180c\u180d\u180e\u180f\u200b\u200c\u200d\u200e\u200f\u202a\u202b\u202c\u202d\u202e\u2060\u2061\u2062\u2063\u2064\u2066\u2067\u2068\u2069\u3164\ufeff\uffa0"}
_AN={c:_ud.name(c,"SPACE") for c in "\u0085\u00a0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u2800\u3000"}
_BA=_fs((0x02BE,0x02BF))
_A1=_fs(ord(c) for c in "ıƖɑɡɢɩɪɴʀʋʏʙʜʟᴀᴄᴅᴇᴊᴋᴍᴏᴘᴛᴜᴠᴡᴢꜰꜱꞮꞯ")
_A3=_fs("ΑΒΕΖΗΙΚΜΝΟΡΤΥΧακνορυχιϲϹϳͿ")
_AU=_fs("ΑΒΕΖΗΙΚΜΝΟΡΤΥΧϲϹϳͿο")
_A2=[(k,_rc((r"(?:^|\.\s+)" if k.endswith(",") else r"\b")+k.replace("underscores","underscores?").replace(" ",r"\s+")+("" if k.endswith(",") else r"\b"),_I)) for k in ("delve into","tapestry","testament to","cornerstone of","multifaceted","paramount","pivotal role","it is worth noting","it is important to note","underscores the importance","moreover,","furthermore,")]
_AX=_rc(r"\[(?:Prologue|Part\s+[IVX]+|Stratum\s+[IVX]+|Chapter\s+\d{2}|Appendix\s+[A-G])\s+\u2014[^\u2014\]]*\]\([^)\s]+?\.md(?:#[^)\s]+)?\)")
_AD,_A9=_rc(r"<(/?)([A-Za-z][A-Za-z0-9-]*)(?=[\s/>]|$)"),_rc(r"[A-Za-z_:][A-Za-z0-9_.:-]*")
_AE,_AS=_fs(("foreignobject","desc","title")),_fs(("text","tspan"))
_AF=_fs(("svg",))|_AE|_AS
_AK=_fs("b big blockquote body br center code dd div dl dt em embed font h1 h2 h3 h4 h5 h6 head hr i img li listing menu meta nobr ol p pre ruby s small span strong strike sub sup table tt u ul var".split())
_AV=_AF|_AK|_fs("a article aside audio button canvas details figure figcaption form g header footer iframe input label line link object path picture polygon rect section source summary video".split())
_H6="address|article|aside|base(?:font)?|blockquote|body|caption|center|col(?:group)?|dd|details|dialog|dir|div|dl|dt|fieldset|fig(?:caption|ure)|footer|form|frameset|frame|h[1-6]|head(?:er)?|hr|html|iframe|legend|li(?:nk)?|main|menu(?:item)?|nav|noframes|ol|opt(?:group|ion)|p(?:aram)?|search|section|summary|table|tbody|td|tfoot|th(?:ead)?|title|tr(?:ack)?|ul"
_H0="pre|script|style|textarea"
_H1=rf"<(?:{_H0})(?:[\s>]|$)"
_AY=((_rc(rf"^ {{0,3}}{_H1}",_I),_rc(rf"</(?:{_H0})>",_I)),(_rc(r"^ {0,3}<\?"),_rc(r"\?>")),(_rc(r"^ {0,3}<![A-Za-z]"),_rc(r">")),(_rc(r"^ {0,3}<!\[CDATA\["),_rc(r"\]\]>")),(_rc(rf"^ {{0,3}}</?(?:{_H6})(?:[\s/>]|$)",_I),""))
_AW,_A4=_rc(r"^([-*+]|\d{1,9}[.)])([ \t]*|$)"),_rc(r"^ {0,3}> ?")
_A5=_rc(rf"^ {{0,3}}(?:#{{1,6}}(?:[ \t]|$)|>|(?:[-*+]|1[.)])[ \t]+\S|(?:-[ \t]*){{3,}}$|(?:\*[ \t]*){{3,}}$|(?:_[ \t]*){{3,}}$|(?:-+|=+)[ \t]*$|<!--|<\?|<!\[CDATA\[|<![A-Za-z]|{_H1}|</?(?:{_H6})(?:[\s/>]|$))",_I)
_T2,_S7=_rc(r"^ {0,3}<!--"),_rc(r"^ {0,3}<svg(?=[\s/>])",_I)
_AT=_rc(r"[βγδεζηθλμπστωφψΔΘΛΞΠΣΦΨΩ][A-Z0-9_]+")
_AP=_rc(r"https?://(?:[^\s<>\"'`)\],()]|\([^\s<>\"'`)\],()]*\))+",_I)
_A8=_rc(r"""\b(?:alt|aria-label|title)\s*=\s*(?:"([^"]*)"|'([^']*)')""",_I)
_AR=_rc(r"\|\s*"+_EM+r"\s*(?=\|)")
_AH,_AZ=_rc(r"^ {0,3}#{1,6}(?:[ \t]|$)"),_rc(r"^ {0,3}(```+|~~~+)(.*)$")
_AJ=_rc(r"(?:https?://(?:dx\.)?doi\.org/|doi:\s*)(10\.\d{4,9}/(?:[^\s<>\"'\]\[,)()]|\([^\s<>\"'\]\[,)()]*\))+)",_I)
_A0=_rc(r"^10\.\d{4,9}/[-._;()/:A-Za-z0-9%]+$")
_AA=r"(?:[A-Za-z\-]+/\d{7}|\d{4}\.\d{4,5}(?:v\d+)?)"
_AO,_A6,_A7=_rc(rf"(?:arXiv:\s*|https?://(?:www\.)?arxiv\.org/(?:abs|pdf)/)({_AA})",_I),_rc(rf"^{_AA}$",_I),_rc(rf"^/(?:abs|pdf)/{_AA}(?:\.pdf)?$",_I)
_AI=_rc(r"--!?>")
_AB=_rc(r"<title[^>]*>[^<]*(?:\b404\b|(?:page|doi)\s+not\s+found|not\s+found\b|access\s+denied)[^<]*</title>",_I)
_UA={"User-Agent":"Stochology-Citation-Checker/1.0"}
@dataclass
class Finding:
 severity:str
 kind:str
 file:str
 line:int
 detail:str
def _ao(ch):return 0xFE00<=(cp:=ord(ch))<=0xFE0F or 0xE0100<=cp<=0xE01EF
def _ap(tok,ak=False):
 raw,sm=tok.strip("«»“”‘’*`$|{}_"),False
 for ch in tok:
  if (cp:=ord(ch))<128:continue
  cat=_ud.category(ch)
  if cat=="Nl" or (cat=="Lm" and cp not in _BA):return True
  nm=_ud.name(ch,"")
  if any(s in nm for s in ("CYRILLIC","CHEROKEE","ARMENIAN","COPTIC","FULLWIDTH LATIN","FULLWIDTH DIGIT")):return True
  if "MATHEMATICAL" in nm:sm=True
 if sm and raw!="\U0001d49f":return True
 for sub in re.split(r"[-–*`·×/|${}()\[\]«»]+",tok):
  if not sub:continue
  ha,sok=any(c.isascii() and c.isalnum() for c in sub),ak and _AT.fullmatch(sub) is not None
  for ch in sub:
   if (cp:=ord(ch))<128:continue
   cat=_ud.category(ch)
   if cat=="Lm" or not cat.startswith("L"):continue
   if cp in _A1 or (not sok and ha and "LATIN" not in _ud.name(ch,"")):return True
  for p in sub.split("_"):
   if (len(p)>=2 and all(c in _A3 for c in p)) or (len(p)==1 and p in _AU):return True
 return False
def _al(s,p):
 q=p
 while q>=0 and s[q]=="\\":q-=1
 return bool((p-q)&1)
def _ak(s,i):
 if not (m:=_AD.match(s,i)):return None
 ic,nm,j,n,alts=bool(m.group(1)),m.group(2).lower(),m.end(),len(s),[]
 def _ws(k):
  while k<n and s[k] in " \t\n":k+=1
  return k
 if ic:
  j=_ws(j)
  return (nm,True,False,j+1,alts) if j<n and s[j]==">" else None
 while (k:=_ws(j))<n:
  if s[k]==">":return nm,False,False,k+1,alts
  if s[k:k+2]=="/>":return nm,False,True,k+2,alts
  if k==j or not (am:=_A9.match(s,k)):return None
  a0,an,j=k,am.group(0).lower(),am.end()
  k=_ws(j)
  if k<n and s[k]=="=":
   k=_ws(k+1)
   if k>=n:return None
   if s[k] in ('"',"'"):
    if (q:=s.find(s[k],k+1))==-1:return None
    j=q+1
   else:
    v0=k
    while k<n and s[k]>" " and s[k] not in "\"'=<>`":k+=1
    if k==v0:return None
    j=k
   if an=="alt":alts.append((a0,j))
 return None
def _an(s,i,sv,stk):
 if not (m:=_AD.match(s,i)):return None
 ic,nm=bool(m.group(1)),m.group(2).lower()
 return (nm,ic,False,e+1,[]) if ((ic and nm=="svg") or _aj(nm,ic,sv,stk)) and (e:=s.find(">",m.end()))!=-1 else None
def _ah(s,esc=False):
 i=0
 while (k:=s.find("<",i))!=-1:
  if esc and _al(s,k-1):i=k+1
  elif (t:=_ak(s,k)) is not None:
   yield k,t
   i=t[3]
  else:i=k+1
def _ad(s):
 o,p=[],0
 for k,t in _ah(s,True):
  o.extend((s[p:k]," "))
  p=t[3]
 return "".join((*o,s[p:]))
def _at(ln):
 o,p=[],0
 for _,(nm,ic,_,_,alts) in _ah(ln,True):
  if not ic and nm in ("img","svg"):
   for a0,a1 in alts:
    o.extend((ln[p:a0],'alt=""'))
    p=a1
 return "".join((*o,ln[p:]))
def _aq(ln,c):
 if ln.startswith("<!-->",c):return c+5
 if ln.startswith("<!--->",c):return c+6
 return m.end() if (m:=_AI.search(ln,c+4)) else -1
def _ar(s,b):
 n,j=len(s),b
 while j<n and s[j]=="`":j+=1
 tl=j-b
 for m in re.finditer(r"`+",s[j:]):
  if len(m.group(0))==tl:return j,j+m.end()
 return j,-1
def _av(s,tl):return any(len(m.group(0))==tl for m in re.finditer(r"`+",s))
def _ay(s):
 o,i=[],0
 while (b:=s.find("`",i))!=-1:
  o.append(s[i:b])
  if _al(s,b-1):
   o.append("`")
   i=b+1
   continue
  j,ce=_ar(s,b)
  o.append(" "*(ce-b) if ce!=-1 else s[b:j])
  i=ce if ce!=-1 else j
 return "".join((*o,s[i:]))
def _aj(nm,ic,sv,stk):return sv and not any(t in _AE for t in stk) and ((not ic and nm in _AK) or (ic and nm in ("br","p")))
def _ag(nm,ic,sc,sv,stk):
 if _aj(nm,ic,sv,stk):return False,()
 if nm=="svg":return (False,()) if ic else ((sv,stk) if sc else (True,()))
 if sv and nm in _AF:
  if ic:
   if stk and stk[-1]==nm:return sv,stk[:-1]
   if nm in stk and nm in _AE:return sv,stk[:len(stk)-1-stk[::-1].index(nm)]
  elif not sc and len(stk)<64:return sv,(*stk,nm)
 return sv,stk
def _aw(ln,ic_,sv=False,stk=(),ml=False,bsv=False,hb=False):
 u,t,i,n=[],[],0,len(ln)
 _r=lambda:("".join(u),"".join(t))
 def _e(x,y=None):
  u.append(x)
  t.append(x if y is None else y)
 while i<n:
  if ic_:
   m=_AI.search(ln,i)
   ep=m.end() if m else n
   _e(" "*(ep-i))
   ic_,i=m is None,ep
   continue
  pr=(("c",ln.find("<!--",i)),("l",ln.find("<",i)))+((("b",ln.find("`",i)),) if not sv and not hb else ())
  cs=[(pos,k) for k,pos in pr if pos!=-1]
  if not cs:
   _e(ln[i:])
   break
  p,k=min(cs)
  if p>i:_e(ln[i:p])
  if k=="b":
   if _al(ln,p-1):
    _e("`")
    i=p+1
    continue
   j,ce=_ar(ln,p)
   if ce==-1 and ml:return *_r(),ic_,p,"bt",j-p
   _e(ln[p:ce]," "*(ce-p)) if ce!=-1 else _e(ln[p:j])
   i=ce if ce!=-1 else j
  elif not sv and _al(ln,p-1):
   _e("<"," ")
   i=p+1
  elif k=="c":
   ep=_aq(ln,p)
   ic_,ep=ep==-1,n if ep==-1 else ep
   _e(" "*(ep-p))
   i=ep
  elif (tg:=_ak(ln,p) or (_an(ln,p,sv,stk) if sv and bsv else None)):
   sv,stk=_ag(tg[0],tg[1],tg[2],sv,stk)
   _e(ln[p:tg[3]])
   i=tg[3]
  elif ml and (m:=_AD.match(ln,p)) and not m.group(1) and m.group(2).lower() in _AV:return *_r(),ic_,p,"tag",0
  else:
   _e("<")
   i=p+1
 return (*_r(),ic_,-1,"",0) if ml else (*_r(),ic_)
def _as(ln,sv,stk=(),tln=None,bsv=False):
 cs,p,s,i=[],0,ln if tln is None else tln,0
 while (k:=s.find("<",i))!=-1:
  if not (tg:=_ak(s,k) or (_an(s,k,sv,stk) if sv and bsv else None)):
   i=k+1
   continue
  nm,ic,sc,e,_=tg
  i=e
  if _aj(nm,ic,sv,stk):
   sv,stk,p=False,(),e
   cs.extend((" ",ln[k:e]))
   continue
  if nm not in _AF:continue
  sg=ln[p:k]
  if not sv:cs.append(sg)
  elif stk:cs.append(_ad(sg))
  ws=sv
  sv,stk=_ag(nm,ic,sc,sv,stk)
  if ic and (nm in ("svg","foreignobject") or ws):cs.append(" ")
  p=e
 tl=ln[p:]
 if not sv:cs.append(tl)
 elif stk:cs.append(_ad(tl))
 return "".join(cs),sv,stk
def _ax(ln,nm,hb=False,ah=False):
 if ah:return ""
 if nm=="index.md":
  for pat in (r'(<figcaption\b[^>]*class="[^"]*\bstoch-home-epigraph-attrib\b[^"]*"[^>]*>\s*)\u2014',r'(\bclass="[^"]*\bstoch-colophon-code\b[^"]*"[^>]*>App\.\s*[A-G]</span>\s*<a href="[^"]+">[^<]+</a>\s*)\u2014'):ln=_ru(pat,r"\1 ",ln)
 mk=_AR.sub("| ",_AX.sub(" ",_at(ln if hb else _ay(ln))))
 for q in _AQ.get(nm,()):mk=mk.replace(q," ")
 return mk
def scan_text_file(path:Path)->list[Finding]:
 fs:list[Finding]=[]
 rel,raw=str(path.relative_to(ROOT)),path.read_bytes()
 _ai=lambda sev,kind,lno,msg:fs.append(Finding(sev,kind,rel,lno,msg))
 if b"\r\n" in raw:_ai(_B,"crlf-line-ending",1,"File contains CRLF line endings.")
 ak=path.suffix.lower()==".svg"
 em_ex=ak or path.name in _EX
 au,aw,ap,ay,fbq,fc,fr=False,"",0,0,0,0,()
 ad,ah,ao,ag=False,(),0,False
 av,aa,aq,ai,ab,ae,hbq,hlc,hr,bqd,lst=False,False,0,False,False,None,0,0,(),0,[]
 pend,aj,al=[],"",0
 _pl=lambda mx=None:[e for e in lst if e[0]<bqd or (e[0]==bqd and (mx is None or e[1]<=mx))]
 _bs=lambda:max((e[1] for e in lst if e[0]==bqd),default=0)
 _rq=lambda d:tuple(max((e[1] for e in lst if e[0]==k),default=0) for k in range(d))
 _ce=lambda fn:[(*e[:3],False if fn(e) else e[3]) for e in lst]
 _in=lambda s:len(s)-len(s.lstrip(" "))
 def _am(all_svg=False):
  nonlocal aa,ai,ad,ah,ag
  if aa and not ai:aa=False
  if ad and (all_svg or not ag):ad,ah,ag=False,(),False

 def _ab(lno,txt,ml=False,blk=True,ct=None,iah=False):
  nonlocal aa,aq,ai,ad,ah,ao,ag,aj,al,hbq,hlc,hr
  wc,ws,hb,cx=aa,ad,ae is not None,txt if ct is None else ct
  if ml and not aa and not hb:u,t,aa,up,pk,tl=_aw(txt,aa,ad,ah,True,ag,False)
  else:
   u,t,aa=_aw(txt,aa,ad,ah,False,ag,hb)
   up,pk,tl=-1,"",0
  if not wc and aa:
   aq,ai=lno,blk and bool(_T2.match(cx) and _aq(cx,cx.find("<!--"))==-1)
   if ai:hbq,hlc,hr=bqd,_bs(),_rq(bqd)
  elif not aa:ai=False
  pr,ad,ah=_as(u,ad,ah,t,ag)
  if not ws and ad:
   t0=_ak(cx,cx.find("<")) if blk and (not ab or ae=="") and _S7.match(cx) else None
   ao,ag=lno,bool(t0 and not t0[1] and not t0[2] and not cx[t0[3]:].strip(" \t"))
  elif not ad:ag=False
  if pr.strip():
   if not em_ex and _EM in pr and _EM in _ax(pr,path.name,hb or ad or ws,iah and not (hb or ad or ws)):_ai(_B,"prose-em-dash",lno,f"Running-prose em-dash (U+2014) prohibited: {pr.strip()[:120]!r}")
   if path.name not in _CX:
    for lbl,pat in _A2:
     if pat.search(pr):_ai(_A,"llm-cliche",lno,f"Advisory style check matched {lbl!r}: {pr.strip()[:100]!r}")
  if up!=-1:
   pend.append((lno,txt[up:]))
   aj,al=pk,tl

 def _aa():
  nonlocal aj,al
  if pend:
   its,aj,al=pend[:],"",0
   pend.clear()
   for pl,pt in its:_ab(pl,pt,False,False)

 ar=raw.decode(_U8).replace("\r\n","\n").split("\n")
 if ar and ar[-1]=="":ar.pop()
 for lno,ln in enumerate(ar,1):
  for ch in ln:
   cat=_ud.category(ch)
   if ch in _AM or _ao(ch) or cat=="Cf":
    d=_AM.get(ch) or f"{_ud.name(ch,'FORMAT')} ({cat})"
    _ai(_B,"invisible-unicode",lno,f"Forbidden invisible/control character {d} (U+{ord(ch):04X}).")
   elif ch in _AN:_ai(_B,"anomalous-space",lno,f"Forbidden Unicode space {_AN[ch]} (U+{ord(ch):04X}).")
  if ak:ts=ln
  else:
   cp,pos,n=[],0,len(ln)
   while pos<n:
    if av:
     m=_AI.search(ln,pos)
     cp.append(ln[pos:m.start() if m else n])
     av,pos=m is None,m.end() if m else n
    else:
     if (ci_:=ln.find("<!--",pos))==-1:break
     if 0<=(li_:=ln.find("<",pos))<ci_:
      pos=tg[3] if (not _al(ln,li_-1) and (tg:=_ak(ln,li_))) else li_+1
      continue
     if _al(ln,ci_-1):
      pos=ci_+1
      continue
     ep=_aq(ln,ci_)
     cp.append(ln[ci_+4:n if ep==-1 else ep])
     av,pos=ep==-1,n if ep==-1 else ep
   at,st=" ".join((m.group(1) or m.group(2) or "") for m in _A8.finditer(ln)),_ad(ln)
   ts=f"{st} {at} {' '.join(cp)}" if (at or cp) else st
  for tok in _rf(r"[^\s<>()\[\]\"'`,.;:!?=+/\\]+",ts):
   if _ap(tok,ak):_ai(_B,"mixed-script-homoglyph",lno,f"Token contains Cyrillic or mixed ASCII/non-Latin homoglyph characters: {tok!r}")
  if ak:continue
  pbq,bqd,bi0,bi=bqd,0,ln.expandtabs(4),[]
  while not au or bqd<fbq:
   i0,bsa=_in(bi0),_bs()
   bsf=max((e[1] for e in lst if e[0]==bqd and e[1]<=i0),default=0)
   if (qm:=_A4.match(bi0[bsf:])):
    if bsf<bsa:lst=_pl(bsf)
    lst=_ce(lambda e:e[0]==bqd)
    bi.append(bsf)
    bqd,bi0=bqd+1,bi0[bsf+qm.end():]
   else:break
  if bqd>pbq:ab=False
  ind,nb=_in(bi0),bool(bi0.strip())
  _ok=lambda d,rq:bqd>=d and all(bi[k]>=rq[k] for k in range(d))
  if aa and ai:
   if _ok(hbq,hr) and (not nb or not hlc or ind>=hlc):
    _ab(lno,ln,True)
    continue
   aa=ai=False
   if nb:lst=_pl(ind)
  if au:
   ex=not _ok(fbq,fr) or (nb and fc and ind<fc)
   if ex or _rm(rf"^ {{{fc},{fc+3}}}{re.escape(aw)}{{{ap},}}[ \t]*$",bi0):
    au,aw,ap,fbq,fc,fr,ab=False,"",0,0,0,(),False
    lst=_pl(ind if nb else None)
   if not ex:continue
  if not _ok(hbq,hr) or (nb and ae is not None and hlc and ind<hlc):
   ae=None
   _am(True)
  if not nb:
   _aa()
   _am(True)
   ab=False
   if ae=="":ae=None
   lst=[e for e in _pl() if not (e[0]==bqd and e[3])]
   continue
  if not ab:lst=_pl(ind)
  lst=_ce(lambda e:e[0]==bqd and e[1]<=ind)
  lm_ok,af,ic5=False,"",False
  while True:
   bs=_bs()
   ind=_in(bi0)
   rind=ind-bs
   if rind>=4:break
   if (qm:=_A4.match(bi0[bs:])):
    bi.append(bs)
    bqd,bi0,ab=bqd+1,bi0[bs+qm.end():],False
    continue
   if not aa and not ad and ae is None and (lm:=_AW.match(bi0[ind:])):
    mk,sp,af_=lm.group(1),len(lm.group(2)),bi0[ind+lm.end():]
    if not ab or (af_ and (mk in ("-","*","+","1.","1)") or any(e[0]==bqd and e[2]==mk[-1] and ind<e[1] for e in lst))):
     lm_ok,af,nc=True,af_,ind+len(mk)+(1 if (not af_ or sp>=5) else sp)
     lst=_pl(ind)+[(bqd,nc,mk[-1],not af_)]
     if sp>=5 and af_:
      ab,ic5=False,True
      break
     if af_:
      bi0,ab=" "*nc+af_,False
      continue
   break
  if ic5:continue
  bs=_bs()
  ind=_in(bi0)
  rind,crst=ind-bs,bi0[ind:]
  r4=rind<4
  if not (ab or aa or ad or pend or ae is not None or r4):continue
  if ae is None and not ag and not aa and r4:
   fm=_AZ.match(crst)
   if fm and (fm.group(1)[0]=="~" or "`" not in fm.group(2)):
    _aa()
    _am()
    au,aw,ap,ay,fbq,fc,fr,ab,ae=True,fm.group(1)[0],len(fm.group(1)),lno,bqd,bs,_rq(bqd),False,None
    lst=_pl(bs)
    continue
  ohb=ae
  if ae is None and not aa and not ad and r4:
   if (he:=next((h for hs,h in _AY if hs.match(crst)),None)) is not None:ae,hbq,hlc,hr=he,bqd,bs,_rq(bqd)
   elif (not ab or lm_ok) and (t7:=_ak(crst,0)) is not None and t7[0] not in _H0.split("|") and not crst[t7[3]:].strip(" \t"):ae,hbq,hlc,hr="",bqd,bs,_rq(bqd)
  ibk=r4 and bool(_A5.match(crst))
  bi=bool(ibk or lm_ok or bqd!=pbq)
  if pend and not bi:
   pend.append((lno,ln))
   if (">" in ln) if aj=="tag" else _av(ln,al):
    jn=" ".join(t for _,t in pend)
    if (_ak(jn,0) is not None) if aj=="tag" else (_ar(jn,0)[1]!=-1):
     sl,aj,al=pend[0][0],"",0
     pend.clear()
     _ab(sl,jn,True,False)
   ab=True
   continue
  wp=bool(pend)
  if wp:_aa()
  if wp or (bi and ohb is None):_am()
  _ab(lno,ln,True,True,crst if r4 else "",ae is None and not aa and not ad and r4 and bool(_AH.match(crst)))
  if ae is None and r4 and _AH.match(crst):_am()
  if ae and ae.search(crst):ae=None
  ab=not (ag or ai or ae is not None or ibk or (lm_ok and not af))
 _aa()
 for flg,k,ol,lb in ((au,"code-fence",ay,"markdown code fence"),(ad,"svg-block",ao,"<svg> tag"),(aa,"html-comment",aq,"HTML comment")):
  if flg:_ai(_B,f"unclosed-{k}",ol,f"Unclosed {lb} hides downstream file content from style inspection.")
 return fs
def _hr(u,*,method="GET",headers=None,limit=None,opener=None):
 with (opener or _ur.urlopen)(_ur.Request(u,method=method,headers={**_UA,**(headers or {})}),timeout=6) as r:
  return r.status,b"" if method=="HEAD" else ((r.read() if limit is None else r.read(limit)) or b"")
_dq,_dc=lambda s,sf="":_up.quote(s,safe=sf),lambda b,e="replace":b.decode(_U8,e)
def _em(x,pfx=""):return f"{pfx}HTTP {x.code}" if isinstance(x,_ue.HTTPError) else f"{pfx}{type(x).__name__}: {x}"
def _cu(u,*,opener=None):
 try:
  st,b=_hr(u,limit=16384,opener=opener)
  ok=200<=st<400
  return (False,f"HTTP {st} soft-404 title") if ok and _AB.search(_dc(b,"ignore")) else (ok,f"HTTP {st}")
 except _ue.HTTPError as e:
  if e.code in (403,405):
   try:
    st,_=_hr(u,method="HEAD",opener=opener)
    return 200<=st<400,f"HTTP {st} (HEAD fallback)"
   except Exception as h:return False,f"HTTP {e.code} -> HEAD {_em(h)}"
  return False,f"HTTP {e.code}"
 except Exception as x:return False,_em(x)
def _vd(doi,*,opener=None):
 dl=doi.lower()
 try:
  _,b=_hr(f"https://api.crossref.org/works/{_dq(doi)}",headers={"Accept":"application/json"},opener=opener)
  m=json.loads(_dc(b)).get("message") or {}
  cd=(m.get("DOI") or "").strip().lower()
  return (True,"Crossref 200") if cd==dl and bool(m.get("title") or m.get("container-title")) else (False,f"Crossref metadata mismatch ({cd or 'missing DOI/title'})")
 except _ue.HTTPError as e:
  if e.code!=404:return False,f"Crossref HTTP {e.code}"
 except Exception as x:return False,_em(x,"Crossref ")
 try:
  st,b=_hr(f"https://doi.org/{_dq(doi,"/")}",headers={"Accept":"application/vnd.citationstyles.csl+json, application/json;q=0.9, text/html;q=0.5"},limit=32768,opener=opener)
  r=_dc(b,"ignore")
  if r.lstrip().startswith("{"):
   m=json.loads(r)
   return (True,"DOI CSL-JSON 200") if (m.get("DOI") or "").strip().lower()==dl and bool(m.get("title")) else (False,"DOI CSL-JSON metadata mismatch")
  return (False,"DOI landing soft-404") if _AB.search(r) else (200<=st<400,f"HTTP {st}")
 except Exception as x:return False,_em(x,"DOI ")
def _va(ax,*,opener=None):
 bid,ns=_ru(r"v\d+$","",ax,flags=_I).lower(),{"a":"http://www.w3.org/2005/Atom"}
 try:
  _,b=_hr(f"https://export.arxiv.org/api/query?id_list={_dq(ax)}",limit=65536,opener=opener)
  for e in ET.fromstring(_dc(b)).findall("a:entry",ns):
   eid,ttl=(e.findtext("a:id","",ns) or "").strip().lower(),(e.findtext("a:title","",ns) or "").strip()
   if bid in eid and ttl and ttl.lower()!="error":return True,"arXiv API 200"
  return False,"arXiv API missing matching entry/title"
 except Exception as x:return False,_em(x,"arXiv API ")
def _ac(raw):
 out=raw.strip()
 while True:
  p,out=out,out.rstrip(".,;>\"'")
  while out.endswith(")") and out.count(")")>out.count("("):out=out[:-1]
  if out==p:return out
def _au(url):
 p=_up.urlparse(url)
 h,path=(p.hostname or "").lower(),p.path or ""
 return h in {"stochology.org","www.stochology.org"} or (h in {"w3.org","www.w3.org"} and path.startswith(("/2000/svg","/1999/"))) or (h in {"doi.org","dx.doi.org"} and bool(_AJ.search(url))) or (h in {"arxiv.org","www.arxiv.org"} and bool(_A7.fullmatch(path)))
def _af(rev):
 pr=subprocess.run(["git","diff","--no-renames","-U0",rev,"--","book-src/","AGENTS.md","README.md","CONTRIBUTING.md"],cwd=ROOT,capture_output=True,text=True,check=False)
 if pr.returncode!=0:raise RuntimeError(f"git diff {rev} failed: {(pr.stderr or pr.stdout).strip()}")
 ds,ax,ex,seen,cur,lno=set(),set(),[],set(),None,0
 for ln in pr.stdout.splitlines():
  if ln.startswith("+++ b/"):
   c=ln[6:].strip()
   cur,lno=(c if c.endswith((".md",".svg")) else None),0
  elif ln.startswith("diff --git "):cur=None
  elif ln.startswith("@@ ") and (hm:=_rs(r"\+(\d+)",ln)):lno=int(hm.group(1))
  elif cur and ln.startswith("+") and not ln.startswith("+++ "):
   b=ln[1:]
   ds.update(_up.unquote(_ac(d)).lower() for d in _AJ.findall(b))
   ax.update(_ac(a).lower() for a in _AO.findall(b))
   for u in (_ac(r) for r in _AP.findall(b)):
    if not _au(u) and (cur,u) not in seen:
     seen.add((cur,u))
     ex.append((cur,max(1,lno),u))
   lno+=1
 return ds,ax,ex
def _ae(fs,sev,k,rel,lno,lb,fn,a):
 ok,st=fn(a)
 if not ok:fs.append(Finding(sev,k,rel,lno,f"{lb} {a} online check returned {st}"))
def scan_appendix_c_citations(verify_online:bool=False,strict_online:bool=False,citations_from_diff:str|None=None,diff_sets=None):
 vo,cfd=verify_online,citations_from_diff
 if not _AC.exists():return [Finding(_B,"missing-appendix-c","book-src/appendix-c-sources.md",1,"Missing Appendix C.")],{"doi_count":0,"arxiv_count":0}
 dd,da=diff_sets if diff_sets is not None else (_af(cfd)[:2] if cfd else (None,None))
 rel,fs,sd,cd,ca,dc,ac,sev=str(_AC.relative_to(ROOT)),[],{},set(),set(),0,0,_B if strict_online else _A
 for lno,ln in enumerate(_AC.read_text(_U8).splitlines(),1):
  _f2=lambda k,d:fs.append(Finding(_B,k,rel,lno,d))
  for rd in _AJ.findall(ln):
   doi,dc=_ac(rd),dc+1
   if not _A0.match(doi):_f2("invalid-doi-syntax",f"Malformed DOI: {doi!r}")
   nm=_up.unquote(doi).lower()
   if nm in sd and sd[nm]!=lno:_f2("duplicate-doi",f"DOI {nm!r} also appears on line {sd[nm]}")
   else:sd[nm]=lno
   if vo and (dd is None or nm in dd):
    cd.add(nm)
    _ae(fs,sev,"doi-online-check",rel,lno,"DOI",_vd,doi)
  for ra in _AO.findall(ln):
   ax,ac=_ac(ra),ac+1
   if not _A6.match(ax):_f2("invalid-arxiv-syntax",f"Malformed arXiv ID: {ax!r}")
   elif vo and (da is None or ax.lower() in da):
    ca.add(ax.lower())
    _ae(fs,sev,"arxiv-online-check",rel,lno,"arXiv",_va,ax)
 return fs,{"doi_count":dc,"arxiv_count":ac,"checked_dois":cd,"checked_arxivs":ca}
def analyze(files:list[Path]|None=None,verify_online:bool=False,strict_online:bool=False,citations_from_diff:str|None=None)->dict:
 vo,so,cfd=verify_online,strict_online,citations_from_diff
 tgts=[p if p.is_absolute() else ROOT/p for p in files] if files else sorted(_BS.glob("*.md"))+sorted(_FG.glob("*.svg"))+[_AG,_RM,_CT]
 fs=[f for p in tgts if p.exists() and p.is_file() for f in scan_text_file(p)]
 dd,da,du=_af(cfd) if cfd else (None,None,[])
 cs={"doi_count":0,"arxiv_count":0,"checked_dois":set(),"checked_arxivs":set()}
 if not files or any(p.resolve()==_AC.resolve() for p in tgts):
  cf,cs=scan_appendix_c_citations(vo,so,cfd,(dd,da) if cfd else None)
  fs.extend(cf)
 if vo and cfd:
  sev=_B if so else _A
  for k,its,fn,lb in (("doi-online-check",sorted((dd or set())-cs.get("checked_dois",set())),_vd,"Diff DOI"),("arxiv-online-check",sorted((da or set())-cs.get("checked_arxivs",set())),_va,"Diff arXiv")):
   for it in its:_ae(fs,sev,k,"book-src/",1,lb,fn,it)
  for rf,lno,u in du:_ae(fs,sev,"external-url-online-check",rf,lno,"External URL",_cu,u)
 blk,adv=[f for f in fs if f.severity==_B],[f for f in fs if f.severity==_A]
 return {"summary":{"files_scanned":len(tgts),"blocking_count":len(blk),"advisory_count":len(adv),"doi_count":cs["doi_count"],"arxiv_count":cs["arxiv_count"],"verdict":"PASS" if not blk else "FAIL"},"findings":[asdict(f) for f in fs]}
def format_markdown(report:dict)->str:
 s,fnd=report["summary"],report["findings"]
 ls=["## Style, Watermark & Citation Oracle (`.agents/scripts/check_style_and_citations.py`)","",f"- **Status:** `{s['verdict']}` | **Files scanned:** `{s['files_scanned']}` | **Blocking:** `{s['blocking_count']}` | **Advisory:** `{s['advisory_count']}` | **DOIs / arXiv checked:** `{s['doi_count']}` / `{s['arxiv_count']}`",""]
 if fnd:
  ls.extend(["| Severity | Kind | Location | Detail |","| :---: | :--- | :--- | :--- |",*(f"| `{f['severity']}` | `{f['kind']}` | `{f['file']}:{f['line']}` | {f['detail']} |" for f in fnd[:60])])
  if len(fnd)>60:ls.append(f"\n_... and {len(fnd)-60} more advisory items truncated._")
 else:ls.append("All em-dash, Unicode watermark, homoglyph, and citation-syntax checks clean.")
 return "\n".join(ls)+"\n"
def main(argv:list[str]|None=None)->int:
 p=argparse.ArgumentParser()
 pa=p.add_argument
 pa("--file",action="append",type=Path)
 pa("--out",type=Path)
 pa("--citations-from-diff",default=None)
 for fl in ("--markdown","--json","--verify-online","--strict-online"):pa(fl,action="store_true")
 a=p.parse_args(argv)
 try:rep=analyze(a.file,a.verify_online,a.strict_online,a.citations_from_diff)
 except RuntimeError as exc:
  sys.stderr.write(f"style check ERROR: {exc}\n")
  return 1
 text=(json.dumps(rep,indent=2)+"\n") if a.json else format_markdown(rep)
 if a.out:
  a.out.parent.mkdir(parents=True,exist_ok=True)
  a.out.write_text(text,_U8)
 sys.stdout.write(text)
 return 0 if rep["summary"]["blocking_count"]==0 else 1

if __name__=="__main__":
 raise SystemExit(main())
