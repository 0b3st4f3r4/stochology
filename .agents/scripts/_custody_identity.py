"""Identity & blind-review checks."""
from __future__ import annotations
from pathlib import Path
from re import compile as _rc,findall as _a9,finditer as _bz,fullmatch as _a8,match as _rm,search as _rs,split as _bq,sub as _ru
import html as _hl,posixpath as _px,re,unicodedata as _ud
_hu,_uc,_un0=_hl.unescape,_ud.category,_ud.normalize
_ah,_ri,_asw=lambda p:_rc(_nc(p),re.I),lambda p,s:_rs(p,s,re.I),lambda p:_rc(_sw(p),re.I)
_sp=lambda p,s:[x for x in _bq(p,s) if x]
_ar=lambda s:_ru(r"\s+"," ",s).strip()
_aa=lambda xs:list(dict.fromkeys(xs))
def _nc(s):
 o,i,n,ic,st=[],0,len(s),False,-1
 while i<n:
  c=s[i]
  if c=="\\" and i+1<n:
   o.append(s[i:i+2])
   i+=2
   continue
  if not ic and c=="[":ic,st=True,i+1+(i+1<n and s[i+1]=="^")
  elif ic and c=="]" and i>st:ic=False
  elif not ic and c=="(" and (i+1>=n or s[i+1]!="?"):
   o.append("(?:")
   i+=1
   continue
  o.append(c)
  i+=1
 return "".join(o)
_a3,_dv,_sw=lambda *w:"".join(f"(?<!{x.replace(' ','\\x20')})" for x in w),lambda *w:"|".join(f"(?<={x.replace(' ','\\x20')})" for x in w),lambda s:_nc(s.replace(" ",r"\s+"))
_SK0=r"skip(?:s|ped|ping)?|omit(?:s|ted|ting)?"
_E1,_E2,_E3,_E4,_E5,_E6,_E7,_E8=r"(?:s|ed|ing)?",r"(?:e|es|ed|ing)",r"(?:es|ed|ing)?",r"(?:ed|ing)?",r"(?:s|ed)?",r"(?:e|es|ed)",r"(?:e|es|ing)",r"(?:ed|ing)"
_Y3,_Y4,_H3,_W0,_AW,_Z0,_NV=r"(?:y|ies|ied)",r"(?:y|ies|ied|ying)",r"(?:has|have|had)",r"(?:[a-z0-9_-]+\s+)",r"(?:[a-z-]+\s+)",r"(?:0|zero|no)",r"(?:not|never)"
_DA0,_HTN=r"(?<=\d)(?=[A-Za-z]{2,})"," \t\n"
_L,_AL,_AP,_NL,_GC,_LB,_LA=r"[A-Za-z]",r"[A-Za-z0-9]",r"[A-Za-z0-9_./-]",r"(?![A-Za-z])",r"(?![A-Za-z0-9])",r"(?<=[A-Za-z])",r"(?=[A-Za-z])"
_D5,_GG,_D2,_FC,_EP,_HT,_S=r"(?!\s*-?\s*closed\b)",r"(?:ignored|skipped|omitted|waived|failed|errors?|failures?)",r"(?:this|that|these|those|it|them|the\s+above)",r"(?:above|below|earlier|prior|listed|cited)",r"earlier|previous|prior"," \t",r"[\x20\t]"
_CN,_A5,_CK,_CJ=r"run|executed|verified|tested|performed|completed",r"blocking|structural|critical(?!\s+(?:archival|theory|studies|epistemolog\w*|tradition|lineage|review\s+constitution)\b)|severe|major|un(?:resolved|fixed)",r"perform|complet|conduct|check|test|audit|attempt",r"execut|invok|validat"
_SI,_YI,_AR,_CO,_NB,_P0=r"(?:s|ing)?",r"(?:y|ies)",r"(?:actual|real)ly",r"carried\s+out",r"non[-\s]?blocking",r"[,;:\-?!]?"
_BF,_H0,_BP=_a3(r"\bif\s",r"\bwhen\s",r"\bunless\s"),r"(?<!\\)(?:\\\\)*",r"(?<!\\)(?:\\\\)*+"
_b2=lambda s:(s or "").replace("\r\n","\n").replace("\r","\n")
_IC=r"\u00ad\u034f\u115f\u1160\u17b4\u17b5\u180b-\u180f\u200b-\u200f\u202a-\u202e\u2060-\u206f\u3164\ufe00-\ufe0f\ufeff\uffa0\ufff0-\ufff8\U000e0000-\U000e0fff"
_FM=_rc(rf"[{_IC}]")
_FQ=_rc(rf"[\x00-\x08\x0b-\x1f\x7f\x85\u00a0\u1680\u2000-\u200a\u2028\u2029\u202f\u205f\u2800\u3000{_IC}]")
_AD,_DA="in blind-review artifact","disallowed "
_A3=f"{_DA}Unicode/control char {_AD}"
def _c6(text):
 if _FQ.search(text):return True
 for ch in text:
  if ord(ch)>=128:
   cat=_uc(ch)
   if cat in ("Cf","Cc","Cs","Cn","Mn","Me","Zl","Zp","Zs") or cat[0]=="N":return True
   if (cat=="Po" and ch not in "\u00a7\u00b7\u2026\u2022\u2020\u2021\u2032\u2033\u00a1\u00bf") or ch in "\u00a6\u2223\u2236\u2237" or "\ufe10"<=ch<="\ufe6f" or "\uff01"<=ch<="\uff65" or "\u3001"<=ch<="\u303f":return True
   if cat.startswith("L") and ch not in ("\u02be","\u02bf"):
    nm=_ud.name(ch,"")
    if nm.startswith("LATIN "):
     if not _un0("NFKD",ch)[0].isascii():return True
    elif not (nm.startswith("GREEK ") and ("\u0391"<=ch<="\u03a9" or "\u03b1"<=ch<="\u03c9")):return True
 return False
_E9=str.maketrans("авеіјкмнорстухѕһӏԁԛԝАВЕІЈКМНОРСТХУЅαβεικνορτυχΑΒΕΖΗΙΚΜΝΟΡΤΥΧϲϹϳͿıƖɑɡɢɩɪɴʀʋʏʙʜʟᴀᴄᴅᴇᴊᴋᴍᴏᴘᴛᴜᴠᴡᴢꜰꜱꞮꞯՕօհոսᎠᎪᏴᏟᎬᏀᎻᏆᏦᏞᎷᏢᏒᏚᎢᏙᎳᏃ","abeijkmnopctyxshldqwABEIJKMHOPCTXYSabeikvoptuxABEZHIKMNOPTYXcCjJiIagGiINRvyBHLACDEJKMOPTUVWZFSIQOohnuDABCEGHIKLMPRSTVWZ")
def _strip_invisible_and_fold(text):
 if not text:return ""
 if text.isascii():return _ru(r"[\x00-\x08\x0b-\x1f\x7f]","",text)
 c=_FM.sub("",_un0("NFKC",text.translate(_E9))).translate(_E9)
 o=[]
 for ch in _un0("NFKD",c):
  k=_uc(ch)
  if k in ("Cf","Mn","Me") or (k=="Cc" and ch not in ("\n","\t")):continue
  o.append("-" if k=="Pd" or ch=="\u2212" else " " if k=="Zs" or ch=="\u2800" else "_" if k=="Pc" else ch)
 return "".join(o).translate(_E9)
_fd=_strip_invisible_and_fold
def _ab(text):
 s=_fd(text)
 if s.isascii():return s
 return "".join("_" if ord(ch)>=128 and (_uc(ch) in ("Po","Sm","So","Sk") or ch in ("\ua789","\ua78a")) else ch for ch in s)
def _at(text,pos):
 p=pos
 while p>=0 and text[p]=="\\":p-=1
 return (pos-p)%2==1
def _un(text):
 for _ in range(32):
  if (nxt:=_hu(text))==text:break
  text=nxt
 return text
def _aj(text,col=0):
 for c in text:col=(col+4-(col%4)) if c=="\t" else col+1
 return col
_L0=r"(?<![A-Za-z0-9])"
_R0=r"(?=$|[^A-Za-z0-9])"
_PH=r"pending|unset|placeholder"
_O3,_DC0,_GA,_FD=r"o[134]",r"devin|cursor",rf"{_PH}|anonymous|redacted",r"claude|gemini|mistral|opus|sonnet|haiku|cursor|devin|grok"
PLACEHOLDER_RE=_ah(rf"<(?![\w.+-]+@[\w.-]+\.{_L}{{2,}}>)[^>]+>|\b(TODO|TBD|{_PH})\b")
NON_HUMAN_SIGNOFF_RE=_ah(rf"(^|[\s,;:/(\[])(na|n\s*[./]\s*a|none|false|null|nil|self|auto|tbd|todo|{_GA}|exempt|trivial)($|[\s,;:/\)\].])|{_L0}(bot(\d+|[-_]{_AL}+)?|aibot|{_O3}(bot|mini|pro|nano|preview)|github-actions|dependabot|renovate|chatgpt\d*|gpt-?\d*[a-z]*|{_O3}[-_](mini|pro|preview|nano)|(codex|copilot|antigravity|llm|openai|anthropic|(ai[-_]?)?agent)\d*|deepseek([-_.]?[rv]?\d+[a-z0-9.]*)?|llama[-_.]?\d+[a-z0-9.]*|qwen([-_.]?\d+[a-z0-9.]*)?|anthropic[-_.]?claude|({_FD})[-_.]?\d+(\.\d+)?[a-z]*|({_DC0})[-_]?(ai|bot|agent)|{_GA}){_R0}|(?<={_AL}[-_])(bot|ai)(\d+)?{_R0}")
_EM=_ah(rf"{_L0}(github|noreply|no[-_]?reply){_R0}")
CAMEL_BOT_RE=_rc(r"[a-z0-9](?:Bot|BOT)(?:\d+)?(?=$|[^a-z])")
_AR0=r"review(?:er)?|assistant|helper|auditor|checker|oracle|verifier"
UPPER_AI_RE=_rc(_nc(rf"({_L0}|[a-z0-9])AI(\d+)?(?=$|[^A-Za-z]|[A-Z][a-z]|(bot|agent|reviewer|assistant|dev|ops|helper|model|BOT|AGENT|REVIEWER){_R0})|(^|[\s(\[@])[A-Z0-9]{{3,}}AI(\d+)?{_R0}|(?<![\w.\-])[\w.\-]++\s+(ai([-_.]?\d+)?|Ai[-_.]?\d+){_R0}|{_L0}(ai([-_.]?\d+)?|Ai[-_.]?\d+)(?=\s+[\w.\-])"))
CAMEL_AI_BRAND_RE=_rc(rf"{_L0}(?:[Cc]laude|[Gg]emini|[Mm]istral|GPT|gpt|OpenAI|openai|[Aa]nthropic|[Dd]eep[Ss]eek|[Qq]wen|[Ll]lama|[Cc]opilot|[Cc]odex|[Gg]rok|[Cc]hatGPT|[Dd]evin|[Cc]ursor)(?:[A-Z]{_AL}*|(?:bot|agent|{_AR0}|ai)\d*{_R0})")
NUMBERED_BOT_SUFFIX_RE=_ah(r"(?<![a-z0-9])(?!([a-z]?ab|sab|[a-z]?tal|[a-z]?ca|cha|cor|mar|bar|tur)bot(?=$|[^a-z0-9]))[a-z0-9]+bot\d*(?=$|[^a-z0-9])")
_AM=rf"{_FD}|gpt|llama|qwen|deepseek|openai|anthropic|chatgpt|copilot|codex"
_AC=r"cli|research|team|studio|labs?|dev|ops"
_AS=r"opus|sonnet|haiku|pro|flash|ultra|mini|nano|coder?|engine|think(?:ing)?|large|small|reason(?:er|ing)?|max|plus|maverick|scout|heavy|medium|fast|turbo|lite|preview|instruct|chat|base|distill|vision|audio|omni"
_FB=rf"ai|{_AM}|antigravity|llm|agent|bot"
AMBIGUOUS_SINGLE_AI_RE=_ah(rf"^(@)?({_O3}([-_.]\d+)?|({_FB})([-_.]?\d+)?)$")
_AI_SEG_DIGIT_RE,_AI_WORD_DIGIT_RE=_ah(rf"{_O3}|({_FB})\d+"),_ah(rf"{_O3}([-_.]\d+)?|({_FB})[-_.]?\d+")
_FN={"ai",*_FD.split("|")}
_AG=r"\-_./:|+\u00b7\u2022\u2027\u2215\u2219\u2044\u22c5\ua789\ua78a"
_AT=rf"{_AS}|{_AC}|agent|bot|{_AR0}|model|v\d[\w.]*|\d[\w.]*"
AI_MODEL_COMBO_RE=_ah(rf"{_L0}({_O3}|{_AM})[\s{_AG}]+(ai|{_AT}){_R0}|{_L0}(model|meta|openai|anthropic|google|xai|{_AR0}|agent|bot|code|{_AC})[\s{_AG}]+({_O3}|llama|qwen|deepseek|claude|gemini|mistral|gpt|grok|ai)([{_AG}\s]*(v?\d[\w.]*))?{_R0}|{_L0}(ai\d*|{_DC0})[\s{_AG}]+({_DC0}|grok|{_AT}){_R0}")
EMAIL_TOKEN_RE=_rc(r"(?<![\w.+-])[\w.+-]++@[\w.-]+\.[A-Za-z]{2,}")
CREDENTIAL_SUFFIX_RE=_ah(r"(,\s*(jr|sr|ii|iii|iv|v|phd|ph\.d|md|m\.d|jd|j\.d|esq|pe|p\.e|cpa|rn|dds|dvm|mba|msc|ms|ma|ba|bs|prof|dr)|\s+(jr\.?|sr\.?|iii|iv|ph\.?d\.?|m\.d\.?|j\.d\.?|esq\.?|p\.e\.?|cpa|dds|dvm|mba|msc))\.?\s*$")
_GH0="users.noreply.github.com"
NOREPLY_EMAIL_RE=_rc(rf"^(?:\d+\+)?([^@]+)@{_GH0.replace('.','\\.')}$",re.I)
COAUTHOR_TRAILER_RE=_rc(r"(?im)^Co-authored-by:\s*(.+)$")
IDENTITY_QUOTE_RE=_rc(r"[\'\"`\u2018\u2019\u201c\u201d]")
_NRE,_IQR,_ETR=NOREPLY_EMAIL_RE,IDENTITY_QUOTE_RE,EMAIL_TOKEN_RE
SPECIALIST_HEADER_RE=_rc(r"(?im)^\s*-\s*\*\*Specialist:\*\*\s*`?([a-z][a-z0-9_-]+)\b`?")
LEONTIEF_VERDICT_RE=_rc(r"(?im)^\s*-\s*\*\*Leontief Verdict:\*\*\s*`?(BLOCKED|STRUCTURAL-REVISION|EDITORIAL-ONLY|CLEAN)\b`?")
TARGET_FILES_HEADER_RE=_rc(r"(?im)^\s*-\s*\*\*Target File\(s\):\*\*\s*(.+)$")
CANONICAL_SPECIALIST_NAMES={"formal-stat","logic-systems","philosophy-sources","coherence-editor","orders-homomorphism","redteam-gov"}
_DP={"(":")","[":"]","{":"}","«":"»"}
_DP0=r"()\[\]{}«»"
_C3,_HP=_rc(rf"[{_DP0}]+"),_rc(rf"\(([^{_DP0}]*)\)|\[([^{_DP0}]*)\]|\{{([^{_DP0}]*)\}}|«([^{_DP0}]*)»")
def _strip_delimited_groups(text):
 st,sp,rs=[],[],text or ""
 for i,ch in enumerate(rs):
  if ch in _DP:st.append((i,_DP[ch]))
  elif st and ch==st[-1][1]:
   si,_=st.pop()
   if not st:sp.append((si,i+1))
 out,gs,pos,seen=[],[],0,set()
 def _ag(c):
  if (c:=c.strip()) and c not in seen:
   seen.add(c)
   gs.append(c)
 _a2g=lambda x:(_ag(_C3.sub("",x)),_ag(_C3.sub(" ",x)))
 for a,b in sp:
  out+=[rs[pos:a]," "]
  pos=b
  _a2g(rs[a+1:b-1])
 out.append(rs[pos:])
 if st:_a2g(rs[st[0][0]+1:])
 for m in _HP.finditer(rs):_ag(next((g for g in m.groups() if g is not None),""))
 return _ar(_C3.sub(" ","".join(out))),gs
_sd2=_strip_delimited_groups
def _norm_identity(name):
 f=_ab(name)
 s0=_ru(r"<[^>]+>","",f)
 c=_ar(_IQR.sub("",_sd2(s0 if _rs(r"<[^>]*@[^>]*>",f) else _ETR.sub("",s0))[0])).lstrip("@").rstrip(",;.").strip().lower()
 while (m:=CREDENTIAL_SUFFIX_RE.search(c)):
  b=c[:m.start()].strip().rstrip(",;.").strip()
  if b and len(b.split())>=2:c=b
  else:break
 return c
_ni2=_norm_identity
def _canonical_display_name(raw_norm):
 p=[_ar(x) for x in raw_norm.split(",") if x.strip()]
 return f"{p[1]} {p[0]}" if len(p)==2 else raw_norm
_cd2=_canonical_display_name
def _dq(e):
 if "@" not in e:return e
 l,d=(m.group(1),_GH0) if (m:=_NRE.match(e)) else e.split("@",1)
 return f"{l.split('+',1)[0]}@{d}"
def _extract_email_identities(raw):
 o=set()
 for e in _ETR.findall(_fd(raw or "")):
  l=e.lower()
  o.update((l,_dq(l)))
  if (m:=_NRE.match(l)):
   h=m.group(1).lower()
   o.update((h,h.split("+",1)[0]))
 return o
_ee2=_extract_email_identities
def _extract_coauthors(text):
 o=set()
 for m in COAUTHOR_TRAILER_RE.finditer(text or ""):
  p=m.group(1).strip()
  if (n:=_ni2(p)):o.add(n)
  o|=_ee2(p)
 return o
_HM=(CAMEL_BOT_RE,CAMEL_AI_BRAND_RE,NUMBERED_BOT_SUFFIX_RE,AI_MODEL_COMBO_RE)
_GL=_rc(rf"{_L0}[A-Z][a-z]+{_R0}")
_af=lambda s:_ru(r"\bn\s*[./]\s*a\.?\b","na",s,flags=re.I)
_ad=lambda e:(m.group(1) if (m:=_NRE.match(e)) else e.split("@",1)[0]).lower()
def _is_non_human_reviewer(rev,*,verified_human_names=None):
 rev=_ab(rev)
 vhns,fn,am,ht,sd,nr,et,iq,ph,ua,cb,wd,nhr=verified_human_names,_is_non_human_reviewer,AMBIGUOUS_SINGLE_AI_RE.match,_FN,_AI_SEG_DIGIT_RE.fullmatch,_NRE.fullmatch,_ETR,_IQR,PLACEHOLDER_RE.search,UPPER_AI_RE.search,CAMEL_AI_BRAND_RE.search,_AI_WORD_DIGIT_RE.fullmatch,NON_HUMAN_SIGNOFF_RE.search
 _b9,_m4,_nh=lambda xs:sum(x in ht for x in xs)>=2,lambda s:any(rx.search(s) for rx in _HM),lambda s:nhr(s) or _HM[3].search(s)
 _gh=f"@{_GH0}"
 ok_h=lambda nm,hd:not fn(nm) and not fn(f"{nm} <{hd}{_gh}>")
 if vhns:
  gv=lambda k:(vhns.get(k) or "").strip()
  vk=rev.strip().lstrip("@").strip().lower()
  if not (vn:=gv(vk)) and (mnr:=nr(vk)) and "+" not in mnr.group(1):vk,vn=mnr.group(1).lower(),gv(mnr.group(1).lower())
  if (vn and ok_h(vn,vk)) or (vk in vhns and not vn and not fn(vk)):return False
  if (ma:=_rm(rf"^\s*@?([^\s<>{_DP0}]+)\s*<([^<>]+)>\s*$",rev)):
   ha,ea=ma.group(1).lower(),ma.group(2).strip()
   if (va:=gv(ha)) and ok_h(va,ha):return fn(f"{va} <{ea}>")
   if ha in vhns and not va and (mna:=nr(ea.lower())) and mna.group(1)==ha:return fn(ha)
  if (mp:=_rm(rf"^\s*@([^\s<>{_DP0}]+)\s*(?:\(([^)]+)\)|\[([^\]]+)\]|\{{([^}}]+)\}}|«([^»]+)»)\s*$",rev)):
   hp,pp=mp.group(1).lower(),next((g for g in mp.groups()[1:] if g),"").strip()
   if hp in vhns and not fn(pp) and (not (vp:=gv(hp)) or not fn(vp)):return False if not vp else fn(f"{pp} <{hp}{_gh}>")
 if ua(rev) or _m4(rev):return True
 r_outer,dgs=_sd2(rev)
 ruq=iq.sub("",r_outer).strip().rstrip(".")
 rdd=_af(ruq)
 rb=et.sub("",_ru(r"<[^>]+>","",rdd)).strip()
 bws=_a9(r"[a-z0-9]+",rb.lower())
 if _EM.search(rb) or _b9(bws):return True
 bset,cws,cin=set(bws),{w.lower() for w in _GL.findall(rb)},{w.lower() for w in _a9(rf"{_L0}[A-Z](?=\.(?:\s|$)|\s|$)",rb)}
 at=_cd2(rb) in {_cd2(v) for v in (vhns or {}).values() if v and not fn(v)}
 _cm=lambda sg:len(bws)>=2 and bool(sg) and (any(s in bset and len(s)>=2 for s in sg) or "".join(sg)=="".join(bws)) and all(s in bset or (len(s)==1 and any(b.startswith(s) for b in bws)) or any(b in cin and s.startswith(b) for b in bws) for s in sg)
 _ac=lambda s,sg:am(s) and s in ht and (s in cws or at or any(b in cin and s.startswith(b) for b in bws)) and _cm(sg)
 for em in et.findall(rev):
  rl=_ad(em.lower())
  if _EM.search(rl):return True
  pp=[p for p in rl.split("+") if p]
  bl=pp[0] if pp else ""
  bs=_sp(r"[-_.]+",bl)
  if _b9(bs) or (am(bl) and not _ac(bl,bs)) or any(sd(s) or (am(s) and not _ac(s,bs)) for s in bs):return True
  for tag in pp[1:]:
   if _nh(tag) or _AI_SEG_DIGIT_RE.search(tag) or any(am(s) for s in _sp(r"[-_.]+",tag)):return True
 for pt in dgs:
  pn=_af(pt)
  ps=_sp(rf"[\s{_AG}]+",pn.strip().lstrip("@"))
  pl,pc=[s.lower() for s in ps],{w.lower() for w in _GL.findall(pt)}
  if _b9(pl) or len({*bws,*pl}&ht)>=2 or ph(pt) or _EM.search(pt) or _nh(pn) or _m4(pt):return True
  for seg in ps:
   if am(seg) and not ((sl_:=seg.lower()) in ht and len(ps)>1 and (sl_ in pc or sl_ in cws or at) and (_cm(pl) or set(pl)==bset)):return True
 for tok in _bq(r"[\s,;:/|]+",rb):
  tc=tok.strip("@()[]{}«»").rstrip(".")
  if not tc:continue
  ts_=tc.strip("*~")
  wr,segs=bool(_rm(r"^[*~_-]|[*~_-]$",tc)),_sp(rf"[{_AG}*~]+",ts_)
  if not segs:continue
  if ts_.lower()=="ai" or wd(ts_) or (wr and any(am(s) for s in segs)):return True
  s0,sl=segs[0].lower(),segs[-1].lower()
  if len(segs)==1 and (am(segs[0]) or wd(segs[0])) and not (s0 in ht and len(bws)>=2 and segs[0] in rb and s0 in cws):return True
  if len(segs)>1:
   hc=sl=="claude" and sl in cws and len(bws)>=3 and bws[-1] not in ht and sl!=bws[-1] and s0 in cws
   if any(sd(s) or (am(s) and ((u:=s.lower()) not in ht or len(sl)<3 or (u==sl and not hc))) for s in segs) or sl=="ai" or any(segs[i+1].isdigit() and wd(f"{segs[i]}-{segs[i+1]}") for i in range(len(segs)-1)):return True
 rvs=_sp(rf"[{_AG}]+",rb.lstrip("@"))
 _do=len(rb.split())<=1 and bool(rb and (am(rb) or (len(rvs)==1 and am(rvs[0])) or (len(rvs)>1 and (any(sd(s) or (am(s) and s.lower() not in ht) for s in rvs) or rvs[-1].lower()=="ai" or len(rvs[-1])<3))))
 return bool(not rdd or _do or ph(rdd) or ua(ruq) or cb(ruq) or any(_nh(s) for s in (ruq,rdd)) or nhr(rev))
def _reviewer_identity_keys(rev):
 rc=_sd2(_ab(rev))[0]
 keys,na=set(),_ni2(rc)
 ka,ku=keys.add,keys.update
 def _ew(ws):
  mg=_aa([*ws,*(f"{ws[i]}{ws[i+1]}" for i in range(len(ws)-1))])[:24]
  if len(ws)>=2:ku(("".join(ws),f"{ws[0]} {ws[-1]}",f"{ws[0]}{ws[-1]}"))
  for i in range(len(mg)):
   for j in range(i+1,len(mg)):ku((f"{mg[i]} {mg[j]}",f"{mg[i]}{mg[j]}"))
 if na:
  cn=_cd2(na)
  ku((na,cn))
  for c in {na,cn}:
   if (col:=_ar(_ru(rf"[,;@{_AG}]+"," ",c))):
    ka(col)
    _ew(col.split())
   if (wj:=[p for p in (_ru(rf"[,;@{_AG}]+","",pc) for pc in c.split()) if p]):
    ka(" ".join(wj))
    _ew(wj)
 for em in _ee2(rev):
  ka(em)
  if "@" in em:
   rl=_ad(em)
   for loc in (rl,rl.split("+",1)[0]):
    if loc:
     ka(loc)
     if (cl:=_ru(r"[._-]+"," ",loc).strip()):ka(cl)
 return {k for k in keys if k}
def _reviewer_equiv_key(rev):
 rc=_sd2(_ab(rev))[0]
 for em in sorted(e for e in _ee2(rc) if "@" in e):
  return _ru(r"[._-]+"," ",_ad(em).split("+",1)[0]).strip()
 return _ru(rf"[,;@{_AG}]+"," ",_cd2(_ni2(rc))).strip()
def _split_address_list(raw):
 qm={'"':'"',"`":"`","'":"'","\u201c":"\u201d","\u2018":"\u2019"}
 def _sp(seps):
  o,b,qc,ad,es,pv,ss=[],[],None,0,False,"",False
  for ch in raw or "":
   if es:es=False
   elif ch=="\\" and qc is not None:es=True
   elif qc is not None:
    if ch==qc:qc=None
   elif ch in '"`\u201c' or (ch in "'\u2018" and not pv.isalnum()):qc=qm[ch]
   elif ch=="<":ad+=1
   elif ch==">" and ad>0:ad-=1
   else:
    if not ad and ch==";":ss=True
    if not ad and ch in seps:
     if (t:="".join(b).strip()):o.append(t)
     b.clear()
     pv=""
     continue
   b.append(ch)
   pv=ch
  if (t:="".join(b).strip()):o.append(t)
  return o,ss
 semis,saw=_sp({";"})
 if saw:return semis
 cm,mg,i=_sp({","})[0],[],0
 while i<len(cm):
  t1=cm[i]
  if i+1<len(cm):
   c1,c2=t1.strip(),cm[i+1].strip()
   if "@" not in c1 and "<" not in c1 and not _ee2(c1) and not _IQR.search(c1) and len(_ni2(c1).split())==1 and _ee2(c2) and len(_ni2(c2).split())==1:
    mg.append(f"{c1}, {c2}")
    i+=2
    continue
  mg.append(t1)
  i+=1
 return mg
_parse_reviewers=lambda r:_split_address_list(c) if (c:=r.split("#",1)[0].strip()) else []
def _expand_simple_braces(spec):
 ms=[*_bz(r"\{([^{}]+)\}",spec)]
 if not ms:return [spec]
 if len(ms)>1 or "{" in spec[ms[0].end():] or "}" in spec[:ms[0].start()]:return []
 opts=[o.strip() for o in ms[0].group(1).split(",") if o.strip()]
 return [f"{spec[:ms[0].start()]}{o}{spec[ms[0].end():]}" for o in opts] if 0<len(opts)<=8 else []
def _split_target_tokens(line):
 toks,buf,bd=[],[],0
 for ch in _ru(r"\[[^\]]*\]\(([^)]+)\)",r" \1 ",line)+" ":
  if ch=="{":bd+=1
  elif ch=="}" and bd>0:bd-=1
  elif bd==0 and (ch.isspace() or ch in ",;`"):
   if (t:="".join(buf).strip().strip("`'\"()[]")):toks.append(t)
   buf=[]
   continue
  buf.append(ch)
 return toks
_cs=lambda r:f"{r.as_posix().rstrip('/')}/"
def _target_header_covers_path(target_line,req_path,root):
 req,rp=_px.normpath(req_path.strip().replace("\\","/")),_cs(root)
 for raw in _split_target_tokens(target_line):
  t=raw.replace("\\","/")
  if t.startswith("file://"):
   if not (t:=t[7:]).startswith(rp):continue
   t=t[len(rp):]
  elif _rm(r"^[A-Za-z][A-Za-z0-9+.-]*://",t):continue
  elif t.startswith(rp):t=t[len(rp):]
  t=_ru(r"^(?:\./)+|:\d.*$|#.*$","",t)
  for e in _expand_simple_braces(t):
   if not e.endswith("/") and not any(w in e for w in "*?[") and (p:=_px.normpath(e)) not in ("",".","..","/") and req==p:return True
 return False
_B8Q=r"(?<!\\)(?:\\\\\\\\)*+\\\\\\?"
_BD=_rc(rf"{_BP}\$\$((?:[^$\\]|\$(?!\$)|\\[\s\S])+?)\$\$|{_BP}\$((?:[^$\\\n]|\n(?!{_S}*\n)|\\[\s\S])+?)\$|{_BP}\\\(((?:[^\\\n]|\n(?!{_S}*\n)|\\(?![()])[\s\S])+?)\\\)|{_BP}\\\[((?:[^\\]|\\(?![\[\]])[\s\S])+?)\\\]|{_B8Q}\(((?:\\\\\\\\|\\\\\\(?![\\()])|\\\\(?![\\()])|\\(?!\\)|[^\\\n]|\n(?!{_S}*\n))+?)\\\\\\?\)|{_B8Q}\[((?:\\\\\\\\|\\\\\\(?![\\\[\]])|\\\\(?![\\\[\]])|\\(?!\\)|[^\\])+?)\\\\\\?\]")
_DN=r"(?:defect|finding|issue|error|bug|flaw|failure|problem|violation|regression)s?|anomal(?:y|ies)"
_CF=r"(?:item|gate|claim)s?"
_ZN=rf"{_DN}|{_CF}"
_M0=r"will|would|must|may|might|can|could|should|shall"
_MA=rf"(?:{_H3}|{_M0})"
_D0=r"timed?\s+out|timed-out|timeout(?!\s*=)"
_E0,_FV,_PV=r"exit(?:ed)?\s*(?:code\s*)?0",rf"fail{_E1}",rf"pass{_E4}"
_EQ=_nc(rf"(refut|disput|invalidat){_E2}|den{_Y3}|(reject|contradict){_E1}|rebut(s|ted|ting)?|falsif{_Y4}|disprov(e|es|ed|en|ing)")
_B0,_CU,_W1,_Z2=rf"\b{_Z0}\s+",_a3(r"\b0\s",r"\bno\s",r"\bzero\s",r"\bnot\x20a\x20single\x20",r"\bnot\x20one\x20"),r"[a-z-]{2,20}",r"(?:[A-Za-z0-9/_-]+\s+){0,2}"
_W2,_EV,_AA=rf"(?:{_W1}\s+){{0,2}}",r"one|two|three|four|five|six|seven|eight|nine|ten",r"(?:was|were|is|are)"
_A4,_AQ,_AZ=r"(?:isn|aren|wasn|weren)'t",r"(?:hasn|haven|hadn)'t",r"cannot|(?:can|won|wouldn|couldn|shouldn|mustn|shan|doesn|didn|don)'t"
_SN=r"un(?:true|founded|sound|tenable|trustworthy|reliable)|bogus|fake|(?:fabricat|falsifi|retract|debunk|repudiat|discredit)ed|disprov(?:ed|en)|erroneous|mistaken|indefensible"
_FR=r"(?<!\bright\x20answer\x20to\x20the\x20)wrong|(?<!\b5\x20)inaccurate|incorrect|misleading|unverified"
_NC=_nc(rf"{_SN}|{_FR}|unacceptable|invalid|flawed|questionable|over(ridden|ruled|turned)|withdrawn|rescinded|revoked")
_NP=rf"actually|never|{_NC}|(?:disput|contradict|refut|rebutt|reject)ed|denied|(?:contradict|refut|disprov|retract)s?|falsif{_YI}|{_CU}(?:fails|failed|failing)"
_PA=_nc(rf"(true|accurate|correct|valid|clean|zero|so|passing|upheld|proven|sound|real|{_AR}|sure|certain|(cred|plaus|defens)ible|(reli|believ|ten|accept)able|trustworthy|(verifi|confirm|support|sustain|accept|establish|demonstrat|prov|settl|convinc|appli|inspect|review|{_CK}|{_CJ})ed|the\s+case)")
_GK=r"(?:deemed|considered|found|judged|marked|shown|declared|prov(?:ed|en))"
_F5=_sw(rf"( {_GK}( to be)?)?")
_IA=_sw(rf"(still|now|((?!(only|merely|solely)\b)[a-z]{{3,18}}ly)|always|very|quite|altogether|even|yet|ever|once|at all|in any way|by any means|{_GK}( to be)?)")
_A6,_A8=rf"(?:{_IA}\s+)",r"the|our|this|that|these|those"
_A61,_A62=rf"{_A6}{{0,1}}",rf"{_A6}{{0,2}}"
_A6V=rf"(?:(?:{_IA}|first|also|both|all|each|next|then|again|further)\s+)"
_AV1,_AV2=rf"{_A6V}{{0,1}}",rf"{_A6V}{{0,2}}"
_DW=_sw(rf"doubt(s|ed|ing|ful)?|skeptical|dubious|implausible|unconvinced|unsure|uncertain|suspicious|hard to believe|disbeliev{_E2}|distrust{_E1}|suspect{_E1}(?= (that|{_Z0}|{_A8}|it|any|such|everything|none|what)\b)|question{_E1}(?= (whether|if|that|{_Z0}|this|these|those|it|our|any|such|everything|the ({_Z0}|claim|verdict|count|result|finding|report|thes[ei]s|argument|method|conclusion|premise|analysis|evidence))\b)")
_BJ=_sw(rf"((have )?(been{_F5}|((seem|appear|remain)(ed)?|prov(e|ed|en)|turn(ed)? out|st(and|ood))( to be)?)|be(ing)?{_F5})")
_BX=_sw(rf"({_D2}|everything|anything|what|{_AP}+\.{_AL}{{1,8}}|({_A8}|any|all|most|some|many|several|both|such|my|their|an?|one|each|every)( of ({_A8}))? {_W0}{{0,3}}[a-z0-9_.-]{{2,24}})")
_ED=r"(?:ed)?"
_EY=r"pass|work|runn?|function|validat|operat|execut|block|catch|detect|prevent|flagg?|reject|enforc|protect|guard|audit|check|screen|scann?|mitigat|eliminat|remediat|fix|clos|resolv"
_GI=rf"(?:{_EY}|verif(?:y|ie))(?:e|s|es|ed|ing)?|caught"
_OP,_G0V=r"active|effective|operational|functional",rf"(?:{_GI}|(?:hold|appl(?:y|ie)|stand|complet|succeed|match)(?:s|es|ed|ing)?)"
_AV0,_Z5,_Z4=r"available|working|ready|reachable|online",r"(?:0|zero)",rf"(?:(?:out\s+)?of\s*(?:\d+|{_EV})|in\s*(?:\d+|{_EV})|/\s*\d+)"
_ST0=rf"{_PA}|{_OP}|safe|intact|green|{_AV0}"
_G0P=rf"(?:be {_A61}(?:{_ST0}|{_GI}|ran|patched)|{_G0V})"
_BO=_sw(rf"({_NV}|far from|hardly|barely|unlikely to be|{_A4}|('s|'re) not) {_A62}((hav(e|ing) )?be(ing|en)? )?{_A62}({_ST0}|ran|proceeded|started|begun)|never {_A62}(held|holds?|{_G0V}|(happen|proceed|start|finish)ed|ran|beg(an|un)|(exist|remain){_E5}|occurred|was|were)|no longer {_A61}(be(ing|en)? )?({_ST0}|needed|applicable|required|{_G0V}|able to {_A61}{_G0P})|(unable|not able) to {_A61}{_G0P}|incapable of {_A61}(being {_A61}({_ST0}|{_GI}|patched)|{_G0V})|(stopped|ceased|quit) {_A61}((being|to be) {_A61}({_ST0}|{_GI})|(to )?{_G0V})|((do(es)?|did|{_MA}) {_NV}|{_AZ}|{_AQ}|never) {_A62}((have )?((up)?h(old|eld)|st(and|ood)|r[au]n|beg(in|an|un)|(pass|succeed|work|test|check|audit|perform|confirm|trust|accept|attest|sustain|proceed|start|finish|happen|occur){_ED}|(execut|complet|believ|agre|endors|validat|materializ)ed?|(appl|verif|certif)(y|ied)|vouch{_ED} for|st(and|ood) by|((support|back|defend|inspect|review|examin){_ED}|read|tell) (whether|if|{_BX})|(think|thought|f(ind|ound)|(consider|deem|regard){_ED}|(judg|believ)ed?) (that )?{_BX} ((to be|as|{_AA}|{_MA} {_BJ}) )?{_A62}{_ST0})|{_BJ} {_A62}(trusted|relied on|{_ST0}))|((must|should|shall|may) {_NV}|(must|should)n't|can('t|not)|ought {_NV} to) {_AV2}(remain|persist|linger|stay|continue|exist)")
_FS=r"false(?!\s*[-\s]?(?:positive|negative|alarm|certainty|precision|sense|discover(?:y|ies)|claim|belief))"
_NQ=_sw(r"(eleven|twelve|(thir|four|fif|six|seven|eigh|nine)teen|twenty|thirty|forty|fifty|(dozen|hundred)s?( of)?|scores? of|some|a (single|few|dozen|couple( of)?|(handful|number|pair) of)|multiple|numerous|several|many)")
_NW=rf"(?:[1-9]\d*+|{_EV}|{_NQ})\b"
_G9=_rc(_nc(r"(?i:\b([a-z]\.|e\.g|i\.e|cf|viz|vs|etc|(et\s+)?al|(fig|ch(ap)?|sec|app|eq|no|vol|ref|ed|para|def|tab|col|spec)s?|subsec|pp|resp|approx|(in|ex)cl|rev|esp|lit|orig|prob|art|par|thm|prop|lem|cor|rem|ver|num|est|std|misc|fn|pt|dr|prof|tbl|ex|mrs?|ms|st|sr|jr)\.(?=\s))|(?<![A-Za-z])[A-Z][a-z]{1,4}\.(?=\s+(\d|[IVXLC]+\b|[A-Z]\b))"))
_C1=_NP.replace("actually|","").replace("|never|","|").replace(rf"|{_CU}(?:fails|failed|failing)","")
def _mg(text):
 out,ds,op,cl=[],[0,0,0,0],"([{«",")]}»"
 for ch in text:
  if (i:=op.find(ch))>=0:ds[i]+=1
  elif (i:=cl.find(ch))>=0:ds[i]=max(0,ds[i]-1)
  elif ch in ".;!?|" and any(ds):ch=" "
  out.append(ch)
 return "".join(out)
_ms=lambda s:_mg(_G9.sub(lambda m:m.group(0)[:-1]+" ",_ru(r"\.{2,}",lambda m:" "*len(m.group(0)),s).replace("\u2026"," ")))
_dm=lambda s,p=True,*,masked=False:_sb(_bq(r"\.(?:\s|$)",s if masked else _ms(s),maxsplit=1 if p else 0)[0 if p else -1])
_CY=_sw(r"except( for)?|besides|(apart|aside) from|excluding|beyond|save for")
_AY=r"but|however|yet|though|although|despite|whereas"
_FL2,_CW,_TH,_GF=r"(?:fewer|less)",r"in\s+(?:fact|reality)",r"there\s+(?:are|is|were|was)",r"(?:false|wrong|erroneous|mistaken|incorrect)ly"
_DX=_sw(rf"{_NV}|non|hardly|barely|above|over|beyond|past|{_GF}|exceed{_E1}|surpass{_E3}|(more|greater|higher|other|rather|better|fewer) than|at least|least|than|min(imum)?|up to|no (less|fewer) than|({_FS}|unlikely|{_DW}|{_C1}|{_EQ})( that)?|{_A4}|{_AQ}|{_AZ}|(all|anything|everything|nothing) but|except|besides|far from|plus|almost|(near|rough)ly|approx(imately)?")
_NM=_nc(r"(\s*+[:;,\-]\s*+(\([^)\n]{1,96}\)|\[[^\]\n]{1,96}\]|[^,;:\-\n]{1,64}[,;:\-])|\s*+\([^)\n]{1,96}\)|\s*+\[[^\]\n]{1,96}\]|\s++(?!(applicable|relevant|affected|touched|modified|changed)\b(?!\s+(to|that|as|with|by|in|for|of|have|has|had|be|been)\b))[\"\x27\u201c\u201d]?[a-z0-9][a-z0-9\x27\-_/]{0,23}[\"\x27\u201c\u201d]?){0,8}")
_FP=_ah(rf"([A-Za-z0-9\-<>≥≤≠=!+~/%\.]|([<>≥≤≠!]=?|[+~±])\s*+|\b({_DX}){_NM}(\s*+[:=|\-]\s*+|\s++))$")
_RA=rf"(?:{_EQ}|(?:retract|overturn|rescind){_E1}|withdraw(?:s|n|ing)?|revok{_E2})"
_AN=_sw(rf"((do(es)?|did|{_M0})( (still|(actual|real|simp|mere)ly|yet|ever|even|just)){{0,2}} not|{_AZ})")
_DU=_asw(rf"\b({_RA}|{_C1}|{_FS}|{_BO}|{_DW}|unlikely|far from|hardly|barely|{_GF}|({_AN}|never|{_FV} to) ((show|mean|establish|prov|impl|confirm|guarante|indicat|demonstrat|support|verif|find|hold|pass|clear|claim|yield|leave)\w*|meant|found|have|had|held|left))\b")
_LV=r"(?:Level|Severity|L)"
_L3=rf"{_LV}[\s:=\-]*3"
_EL=rf"(?:{_L3}|advisor[iy]\w*|untouched|deferred|tracked|oos|excluded)(?![A-Za-z0-9\-/])"
_G8=rf"(?:editorial(?![A-Za-z0-9\-])|{_EL})"
_F2=rf"(?:{_EL}|(?:passed|prior|closed|fixed|resolved|remediated|historical)(?![A-Za-z0-9\-/]))"
_HA=rf"(?:editorial(?![A-Za-z0-9\-])|{_F2})"
_ZS=r"(?:0|zero|no|none|without|not|neither|nor)\b"
_HN=_ah(rf"{_LV}[\s:=\-]*$")
_S0=rf"fatal|serious|{_A5}|broken|fail{_E8}|invalid|flawed|erroneous|corrupt\w*|defective|{_FR}|un(?:addressed|sound)"
_BA=rf"(?:real|open|new|remaining|additional|extra|active|{_S0})[-\w/]*"
_C2=rf"(?:remain{_E1}|exist{_E5}|found|persists?|detected|unresolved|open|unaddressed|unfixed|fail{_E8}|broken|invalid|blocking)"
_BZ=r"any|the|new|known|residual|latent|verified|potential|possible|(?:introduc|hav|leav|caus|creat)ing"
_HD=_ah(_a3(r"\bnot\s",r"\bnever\s")+_sw(rf"\b({_Z0}|none|not|without|never|neither|nor|non[-\s]?|(free|absence) of) (({_BZ}) ){{0,2}}$"))
_PV0=r"prevent|avoid|block|eliminat|fix|clos|resolv|remediat|forbid|reject"
_HE=_asw(rf"\b(({_PV0})\w*|guard\w* against|to (catch|detect|hunt|check|screen|scan)) ((all|both|(find|emitt|report)ing|{_BZ}) ){{0,2}}$")
_HX=_rc(r"(?:[\n;!?|]+|\.(?=\s|$))")
_as=lambda s:_HX.split(_ms(s))
_DL=_asw(rf"\b({_NV}|no longer|far from|hardly|barely|(false|wrong|erroneous)ly|unlikely|{_DW}|{_AN}|{_A4}|{_AQ}|{_FV} to)\b")
_DQ=_asw(rf"\b({_NV}|no|without|{_AN}|{_FV}|unable|must|should|need{_E5}|ought|yet|(attempt|neglect|intend|claim|purport){_E1}|tr{_Y3}|hardly|barely|(hav(e|ing)|has|had) to|forg(ot(ten)?|et(s|ting)?)|miss{_E3}|(refus|postpon|suppos|waiv){_E2}|defer(s|red|ring)?|meant|{_SK0}|in(sufficient|adequate|capable|effective)|weak|powerless|blind|reluctant|hesitant|un(willing|fit)|impossible|nothing|ill-?equipped|too [a-z-]+( to)?|before|rather than|instead of|lack{_SI}|short of)\b")
_CS=_sw(rf"({_AA}|{_H3}|remain{_E1}|stay{_E1}|went|bec(ame|omes?)|gets?|got|st(ands?|ood)|(prov(e|es|ed)|(seem|appear|look|sound){_E5}|feel(s)?|felt|turn(s|ed)? out)( to be)?)")
_CM=_sw(rf"(({_MA} (have been|{_BJ})|{_CS}( being)?|('s|'re))( |(?=,))){{1,2}}")
_CV=rf"(?:{_CS}|be(?:en)?|deemed|considered|marked)"
_FA=rf"un(?:{_CN}|started|finished|attempted|conducted|invoked|checked|audited|launched|initiated|triggered|dispatched)|in(?:effective|adequate|sufficient|complete)"
_GP=_nc(r"(execution|run(ning)?|verif\w*|test(s|ing)?|complet\w*|checks?|ci|audits?|reviews?|validat\w*|builds?|results?|inspect\w*|confirm\w*|remediat\w*|resol\w*|repairs?|patch(es|ing)?|fix(es|ing)?|sign-?off|closure|triage|mitigat\w*|investigat\w*)")
_BM=_sw(rf"(bypass|disabl|skipp|omitt|waiv|deferr|postpon|halt|suspend|paus|stall|interrupt|abort|cancell?|stopp|queu)ed|unavailable|offline|unreachable|pending|on hold|awaiting {_GP}|in abeyance|dormant")
_OW=_sw(r"un(necessary|needed|required)|inapplicable|redundant|superfluous|out[\s-]+of[\s-]+scope|not (in|within) scope|(outside|beyond) (the )?scope|left out|withheld|(ignor|barr|dropp)ed|irrelevant")
_U0=rf"{_BM}|{_FA}"
_C5=rf"{_FS}|{_C1}|{_U0}|{_OW}"
_QP=r",\s*(?:[^,.;!?()\n]|\.(?=\S)){1,64},"
_EU=rf"(?!(?:{_AA}|be(?:en)?)\b)"
_HF=_sw(rf"( ((we|i|they|you|one|(the|this|that|our|each|an?) {_W1}) ({_EU}[a-z'-]{{1,20}} ){{0,3}}{_EU}[a-z'-]{{1,20}}|({_AA} )?([a-z]{{3,16}}(ed|en)|shown|given|made|found|run|built|set|left|kept|held|sent|put|above|below|earlier|previously|herein)( {_EU}[a-z0-9'.-]{{1,20}}){{0,4}}))")
_C0J=_sw(r"(\s*,\s*((and|or) )?| (and|or) )")
_A7=_nc(rf"(\s*\([^()\n]{{1,64}}\)|{_QP}|\s+(in|on|at|within|for|of|across|under|from|with|by|regarding|concerning|involving|affecting|that|which|where|when|who|whose)((\s++|,\s*((and|or)\s+)?)[^\s;!?(),]{{1,64}}(?<!\.)){{1,8}}|{_HF}){{0,2}}")
_DV=_asw(rf"\b({_FV}{_D5}|(bypass|crash){_E3}|{_SK0}|(waiv|disabl){_E2}|brok(e|en)|abort{_E1}|{_D0}|unimplemented|missing|stalled|absent|non-?existent|{_FA}|{_CS} {_W2}({_NV}|broken|{_BM}|{_FS}|{_C1}|missing|absent)|{_AN} ({_W1} )?(run|pass|work|execute|succeed|hold|catch|detect|prevent|block|fix|close|exist|apply|remain))\b")
_FE=_asw(rf"(\b({_NV}|{_AN}|without|far from|no longer|instead of|rather than|other than|(more|greater) than|({_FV}|unable) to|{_DW}|above|over|(exceed|surpass)\w*)\b|[><≥≠!]=?\s*$)")
_BB=r"(?:\s+(?:itself|themselves|alone|here|there|above|below|overall)){0,2}"
_C7=_asw(rf"^\s*+{_A7}{_BB}\s*+((('s|'re)|({_MA} {_BJ}|{_CS})) {_W2}(not|never|{_C5})|{_BO}|{_AN} ({_W1} )?(hold|occur|happen|pass|apply|materialize|obtain|exist)|{_FV} to\b)")
_HQ=_ah(r"\b(non-blocking|fail-closed|fail-safe|non-failing|false-(positive|negative|alarm)s?|(defect|error|bug|issue|failure)-free|non-finding)\b")
def _dh(s):
 t=_HQ.sub(lambda m:m.group(0).replace("-"," ") if m.group(0).lower().startswith(("non-b","fail-c","false-")) else " ",s)
 return _ru(rf"{_LB}\s*(?<!-)-(?!-)\s*{_LA}","",t)
_am=lambda s:(s,_dh(s)) if "-" in s else (s,)
def _au(s,rx):
 def _k(m):
  rem=_sb(s[m.end():])
  pc=_as(rem)[0]
  return m.group(0) if (_FE.search(_as(s[:m.start()])[-1]) or _C7.match(pc) or _HC.search(rem) or (rem and _CT.search(". "+rem)) or (pc and _C4.search(pc[:200]))) else ""
 s1=rx.sub(_k,s)
 return rx.sub(_k,s1) if s1!=s else s1
def _a0(pre,post=""):
 _cx=bool(post and _DV.search(_au(_as(post)[0],_CZ)))
 if (mn:=_HD.search(pre)):return not _DL.search(_as(pre[:mn.start()])[-1]) and not _cx
 return bool((m:=_HE.search(pre)) and not _DQ.search(_as(pre[:m.start()])[-1]) and not _cx)
_EI=_nc(r"found|finds?|(detect|uncover|discover|report)(s|ed)?|identif(y|ies|ied)|(introduc|observ)(e|es|ed)|log(s|ged)?|confirmed|(exhibit|contain|create|cause|show|reveal|expose|trigger|produce|yield|manifest|retain|leave)s?|carries?|left|has|have|had")
_SV=rf"(?:{_EI}|confirms?|there\s+(?:is|are|was|were|remains?|exists?)|{_CY}|plus|despite)"
_CX=r"(?:(?:fix|resolv|clos|address|remediat|repair|remov|mitigat|clear|eliminat|patch|correct|verifi)ed|clean)"
_ES,_CA=r"(?<![A-Za-z0-9\-/:#§])\b",r"outstanding|lingering|un(?:resolved|fixed|patched|mitigated|remediated|verified|corrected|closed|checked|tested|implemented)"
_AB=_nc(rf"({_S0}|{_U0}|{_CA}|missing|absent|unconfirmed|un(prov(en|ed)|established|reproduc(ed|ible)|replicat(ed|able)|successful|feasible|acceptable|substantiated|corroborated|authori[sz]ed)|impossible|in(feasible|accessible)|disallowed|prohibited|forbidden|non-?(reproducible|functional|operational|compliant))")
_AW0=rf"{_AB}|{_OW}"
_B4=rf"(?:{_BA}|{_C2}|{_AB}|{_FV}{_D5}|false)"
_BL=r"(?:file|gate|check|test|proof|suite|assertion|case)s?"
_AX,_AH=rf"(?:{_BL}|(?:chapter|line|byte|word)s?)",rf"(?:{_CS}|{_IA}|left|kept|be(?:en|ing)?)"
_AH3=rf"(?:{_AH}\s+){{0,3}}"
def _al(_sa,_sc):
 quot,pb,s0=r"[\"\'\u201c\u201d]?",_ES,rf"(?!{_sa}|{_ZS})"
 _nt=r"[^\s.;:!?()]++\s++"
 st,sa=rf"(?:{s0}{_nt})",rf"(?:(?!{_sc}|{_ZS}){_nt})"
 _q0=rf"{pb}(?:{_NW}|an?|another){quot}\s+"
 ns,nb0=rf"{_q0}{s0}{st}{{0,4}}",_a3(r"\bno\s",r"\bnot\s",r"\bwithout\s",r"\bnever\s",r"\bnor\s")
 l3_t=_nc(r"(editorial|styl(e|istic)|clarity|notation|tooling|parity|alignment|terminolog(y|ical)|prose|citation|wording|typo(graph(y|ic(al)?))?|minor|non[-\s]?blocking|verified|remaining|residual|latent|known|other|further|additional)")
 l3_alt=rf"|(?:{_L3}|editorial)\s+(?:{l3_t}[/\-\s]*+){{0,5}}" if "editorial" in _sc else ""
 p2_m=rf"(?:verified|new|open|active|remaining|residual|latent|known|{_A5}|serious|real|unaddressed|other|further|additional|editorial|{_NB}|minor|level\s*[123]|l[123]|or|and)"
 ao=rf"{_sa}|{_ZS}|(?:for|of|against|absence|lack|free|to|which|if|whether|how|when|where|why)\b"
 mod,sep,oa=rf"({_MA} {_AV1})",_sw(rf"(\s*:\s*| \([^()\n]{{1,64}}\) |{_QP} | )"),rf"((?!{_Z0}\b)[a-z-]{{2,24}}\s+){{0,2}}"
 return (
  _ah(rf"{pb}{_NW}{quot}\s+{s0}{st}{{0,6}}({_DN})\b"),
  _ah(rf"(\b{_SV}{sep}(an?|another)\s+{s0}{st}{{0,4}}({_DN})\b|(?<!\bto\s){nb0}\b({_EI}|there\s+(are|were|remain|exist)){sep}(?!{ao})((?!{ao}){_nt}){{0,4}}({_DN})\b|{_q0}(?!{_sc}|{_ZS}){sa}{{0,4}}({_DN})\s+(({_AA}|still|currently)\s+){{0,2}}{_C2}\b)"),
  _ah(rf"({ns}{_BA}\s+{st}{{0,2}}({_CF})\b|{nb0}{_q0}{s0}{st}{{0,1}}{_AB}\s+{st}{{0,2}}{_BL}\b|{nb0}{_q0}(?!{_sc}|{_ZS}){sa}{{0,4}}({_CF}|{_AX})\s+{_AH3}({_C2}|{_AB}|{_B8P})\b)"),
  _ah(rf"{nb0}(?<!\bneither\s){pb}(an?|another)\s+{s0}{st}{{0,3}}({_DN})\b"),
  _ah(rf"\b({_BA})\s+{s0}{st}{{0,2}}({_DN})\b"),
  _asw(rf"\b({_DN}){_A7} {_AV1}((({mod}|{_H3} {_AV1})?((remain|appear|affect|need|await){_E1}|(persist|exist){_E5}|occur(s|red|ring)?|aris(e|es|en|ing)|arose|(surfac|violat|evad|requir|escap){_E2}|bypass{_E3}|break{_SI}|slip(s|ped|ping)?)|({_H3}|{_AA}) (still )?yet to (be )?{_CX}|(({_AA}|do(es)?|did|{_MA}) {_AV1}{_NV}|{_A4}|{_AQ}|{_AN}) {oa}((be(en)?|get|become) )?{oa}{_CX}|({_CS}|{_H3} (been|remained|stayed|proved|become)|{_MA} be) {_AV1}(open|present|{_CA}|{_S0}|un(handled|true|detected|covered)|persistent|active|false|found|(miss|overlook|detect|discover|identifi|observ|logg|report|confirm|introduc|trigger|expos|reveal)ed)|{_B8P}))\b"),
  _ah(rf"\b(({_Z0}|none)\s+({p2_m}\s+){{0,4}}{l3_alt})$"),
 )
def _a4(text,ed=True,_if0=False):
 bc,sg,it,sgf,pl1,pl2,pl2_ok=_CI if ed else _BV
 t=_ru(r"(?i)\blaminar\s+guard-?site\s+findings\b","",text)
 if any(not _HN.search(t[:m.start()]) for rx in (bc,it) for m in rx.finditer(t)) or sg.search(t):return True
 if any(not _a0(t[:m.start()],t[m.end():]) for m in pl1.finditer(t)) or any(not ((mp:=pl2_ok.search(t[:m.start()])) and not _DL.search(_as(t[:mp.start()])[-1]) and not _C7.match(t[m.start()+len(m.group(0).split()[0]):])) for m in pl2.finditer(t)):return True
 return bool(_if0 and sgf.search(t))
_CP=_sw(r"cargo (\+\S+ )*(miri )?test")
_BI=_sw(r"cargo (\+\S+ )*(miri )?(test|check|build)")
_FT=_sw(r"structure\.py|(check|verify)_[a-z_]+\.py|surface\.py|build_pdf\.py|wasm_gates\.py|repro_aot\.py|security_audit\.py|mdbook build|node --test")
_LC=r"[`'\"]?\s*(?:[:#§]|,?\s*\(\s*(?:lines?\b|L\d|§)|,?\s*(?:at\s+)?lines?\s+\d|\bL\d)"
_LP=_sw(rf"(\blake (build|test|exe|env lean)\b|(?<![/.A-Za-z0-9_-])lean {_AP}+\.lean(?![A-Za-z0-9_./:#\]-]|{_LC}))")
_A2=r"(?:doc|note|comment|derivation|proof|claim|output|log|table|section|paragraph|quote|report|paper|article|citation|file|module|crate|helper|wrapper|target|binary|spec|result|record|line|artifact)s?|prose|text|discussion|code|summar(?:y|ies)"
_MP=_nc(rf"((?<![/.A-Za-z0-9_-])\b(sympy|numpy|scipy)(?!(\.py)?+([:#\]/]|{_LC}|\.(py|md|rs|txt|json)\b)|\s+({_A2}|documentation|manual|reference|api|guide|source|package|library|repo(sitory)?|codebase|script|readme|changelog|(description|example|snippet|excerpt|formula|equation)s?)\b)\b|(?<![A-Za-z0-9_-])({_AP}+/)?(check_symbolic_math|verify_formal_invariants)(\.py)?+(?![A-Za-z0-9_.:#\]-]|{_LC})|{_LP})")
_BH=r"crashed|aborted|panicked|halted|stalled|froze|frozen|hung|died|errored|timed\s+out"
_TA=r"actually|fully|completely|properly|successfully"
_PI=r"(?:,\s*[^,.;!?()\n]{1,40},|\([^()\n]{1,32}\)|-{1,2}\s*[^-;!?\n]{1,40}\s*-{1,2})"
_WA=_sw(rf"(({_PI}|{_IA}) ){{0,2}}(to be (({_PI}|{_IA}) ){{0,2}})?")
_EJ=_nc(rf"([^\w\s\-']|(?<![A-Za-z0-9_])-)\s*{_WA}({_FS}|{_NC}|(not|never)\s+({_IA}\s+)?{_PA})")
_NR=_sw(rf"(without|instead of|rather than) (({_TA}) )?((runn|test|check|audit|perform|execut|invok|verify)ing|having (({_TA}) )?(run|executed|verified|tested|performed|checked|audited))\b")
_GB=_nc(rf"{_BF}\b({_NV}\s+((yet|ever|{_TA})\s+){{0,2}}({_CN})\b|{_NR})")
_CR=r"(?:(?:run|suite|check|test|build|gate|script|command|cmd|job|step|stage|harness|oracle|pipeline|audit|proof|workflow|invocation|process|task|execution|validation|evaluation|probe|runner|sanitizer|linter|prover|verifier|checker|analyzer|scanner|compiler)s?|verification|ci|automation|tooling)"
_EO=rf"(?:{_CR}|invariants?)"
_DE=r"(?:skipp|omitt|bypass|waiv|deferr|abort|cancell?)ed"
_B5=_nc(r"((?!((edit|file|line|word|byte|char|defect|finding|issue|error|warning|bug)s?|and|or|but|with|while|where|that|which|(verifi|confirm|check|audit|inspect|review)ed)\b)[a-z0-9_-]++(,\s*+|\s++))")
_CQ=_sw(rf"{_EO}({_C0J}{_B5}{{0,2}}{_EO}){{0,3}}")
_AV=_nc(rf"(r[au]n|carr(y|ied)\s+out|undert(ake|aken|ook)|({_CK})(ed)?|({_CJ})(e|ed)|verif(y|ied))")
_AO=_sw(rf"(({_A8}|all|every|any|an?|one|each)( of ({_A8}))? )?{_B5}{{0,6}}({_CQ}|propert{_YI}|(file|crate|module|claim)s?|anything|it|them)( ({_FC}))?")
_W3=r"we|i|this (?:review|audit|report)|the (?:reviewer|auditor"
_H1=_sw(rf"(({_W3}|author|harness|script|runner|subagent|agent|oracle|check|pipeline|suite)) (({_TA}|(effective|intentional|deliberate)ly) )?({_DE}( (runn|execut|invok|perform|check|test)ing)?|((do(es)?|did|{_MA}) {_NV}|{_AZ}|{_AQ}|{_H3} ({_NV}|yet to)|yet to|never) {_AV}) {_AO}|(nobody|no one|none of us) ((ever|{_TA}) )?{_AV} {_AO}|{_CU}{_DE} ((runn|execut|invok|perform)ing )?({_A8}|an?|any) {_W0}{{0,4}}{_EO}( ({_FC}))?)")
_AE=rf"ran|{_CN}|{_CO}|undertaken|begun|occurred|(?:{_CK}|{_CJ}|initiat|launch|trigger|dispatch|start|proceed|finish|happen|issu|inspect|review|pass|succeed|implement)ed|{_CX}"
_N3=rf"(?:(?:need|await){_E1}|requir{_E2})"
_OB=rf"(must|should|shall|ought to|(({_M0}) )?(ha(ve|s|d)|need(s|ed)?) to|having to)"
_AE2,_VB0,_VB1=rf"{_A62}(?:{_AE})",r"fix|patch|repair|correct|address|verify|check|test|audit|inspect|review|implement",r"resolv|clos|remediat|mitigat|validat|execut|complet"
_B8V=rf"((be|get|become|stay|remain) {_AV2}({_ST0}|(re-?)?({_AE}|built|done|cleared|enforced|restored|updated|hardened|compiled|reproduced))|(re-?)?(pass|run|work|succeed|hold|clear|finish|compile|build|apply|match|proceed|start|begin|converge|reproduce|confirm|enforce|restore|update|harden|{_VB0}|({_VB1})e))"
_B8=_sw(rf"(((remain{_E1}|{_N3}|yet|still|left) |({_AA}|{_H3} been|{_MA} be) {_AV1}((expected|supposed|meant|scheduled|queued|planned) )?)to {_AV1}{_B8V}|(remain{_E1}|{_N3}|yet|still|left) (re-?)?({_VB0}|{_VB1}|runn)ing|{_OB} {_AV2}{_B8V}|({_AA}|{_H3} been|{_MA} be) {_AV1}(required|needed|necessary|mandatory|due|overdue)|{_N3} ((an?|further|more|manual) )?((fix|patch)(es)?|repairs?|remediation|resolution|closure|mitigation|correction|(verific|valid)ation(?! of\b)|audits?|inspection|review(?! of\b)|testing|execution|completion|implementation))")
_B8P=_sw(rf"{_B8}|{_BO}|({_NV}|no longer) ({_AH} ){{0,2}}({_ST0}|{_AE}|passing)")
_BV,_CI=_al(_F2,_EL),_al(_HA,_G8)
_AU=_sw(rf"((still|as) )?yet to (be {_AE2}|r[au]n|proceed|start|begin|take place|happen|occur|execute|finish|complete|pass|succeed)")
_HZ,_FU=rf"{_AE}|{_BM}|{_FA}|broken",r"pass|compile|build|verify|match|proceed|start|begin|happen|occur|materialize"
_NV2=rf"{_NV} {_A62}"
_P2=rf"(?:passed|{_AE}|effective|working)"
_D6=_sw(rf"({_NV2}(having {_A62}been|being)|having {_A62}{_NV2}been) {_WA}{_P2}")
_ch=lambda s,wm=_W0:_sw(rf"(without ({_A8} )?{wm}{{0,3}}({s}){_A7}{_BB} ((having been|being) {_WA}{_P2}|(runn|pass|execut|complet|work)ing)|before ({_A8} )?{wm}{{0,3}}({s}){_A7}{_BB} (({_AA}|{_H3} been|{_MA} be) {_WA}(passed|{_AE})|r[au]ns?|(proceed|complet|finish|pass|start|execut)(s|es|ed)?))")
_D3=r"(?:now|still|currently|presently|recently|lately|again|newly|freshly|today|remaining|staying|left|kept)"
_BQ=_sw(rf"({_D3} {_WA}({_AW0})|already {_WA}{_AB}|(awaiting|pending) {_GP}|on hold|in abeyance)")
_GU=_asw(rf"\b((({_PV0}|catch|detect|flag|mitigat|prohibit|disallow)\w*|guard\w* against|(check|scan|test|hunt)\w* for) ((any|all|the|such) )?{_BQ}|({_Z0}|not (a single|one)) ({_AX}|{_ZN}) {_AA} {_BQ}|(pre-?push|pre-?commit|local hooks?) ((also|still) )?(permit|allow|exempt)\w* ((any|the) )?pending reviews?)\b")
_bn=lambda s:_GU.sub(lambda m:m.group(0) if _DQ.search(_as(s[:m.start()])[-1]) else " ",s)
_D8=_sw(rf"({_C0J}(({_A8}|its|their|all|any) )?((?!tools?\b)[a-z0-9/_-]+ ){{0,2}}(?!tools?\b)[a-z0-9/_-]+){{0,2}}")
_EN0=r"sandbox|runtime|env(?:ironment)?|container|network|host|coverage"
_BC=_nc(rf"({_CQ}|status|results?|outcomes?|verdicts?|state|{_EN0}|network\s+access|worker|daemon|evidence|logs?|outputs?|reproduction|replication|repro|assertions?|parity|confirmation|conformance|compliance)")
_CC,_DF,_EZ=r"(?:shell|tools?)",rf"broken|halted|offline|unreachable|pending|skipped|bypassed|disabled|failed|corrupt\w*|{_BH}|{_FA}",r"(?:[:=(\u2014–→⇒]|[-=]>|(?<=\s)-|-(?=\s))"
_D4=_nc(rf"{_AE}|{_ST0}|needed|applicable|required|done|started|reproduc(ed|ible)|replicat(ed|able)|possible|feasible|successful|permitted|allowed|achiev(ed|able)|satisfi(ed|able)|fulfilled|met|obtained|observed")
_FW=_nc(rf"{_BH}|dead|down|terminated|killed|unresponsive|blocked(?!\s+(by\s+(?:guard|check|gate|rule|filter|hook)|attack|bypass|exploit|payload))")
_P3=r"passed|verified|run|executed|completed|checked"
_P3T=rf"(?:{_P3}|tested)"
_HO=_sw(rf"n/?a|none(?! executed directly\b)|null|nil|empty|blank|unknown|tbd|{_Z5}\b(?=\s*({_P3T}|checks?|tests?|runs?|gates?|{_Z4}))")
_D7=rf"(?:{_CM}|{_D3}\s+){_WA}"
_V1=rf"{_BC}{_D8}{_A7}{_BB}\s*{_EZ}\s*{_WA}"
_GT=_asw(rf"(\b(exit(ed)?( with)?( code| status)?|return\s*code|status|rc)\s*[:=]?\s*[1-9]\d*\b|\bnon-?zero (exit|return|status)\b|\b{_Z5}(\s*{_Z4})? (({_BL} )?{_P3T})(?!\s+({_ZN}|warnings?|mismatches?|gaps?|collisions?|discrepanc(y|ies)|bypasses?|alerts?)\b)\b|\b{_AN} ({_FU}|complete|finish|succeed|run|execute)\b|\b({_D0})\b|\berror\[E\d+\]|\berror: test failed\b|\bpanicked at\b|\btraceback \(most recent call|\btest result: failed\b|{_ES}[1-9]\d* ({_BL} )?fail(ed|ures?|ing)\b|\b{_FV} (on|with|at|to ({_FU}|{_G0P}|avoid))\b|{_BF}\b({_H1}|{_ch(_CQ,_B5)}|{_CU}(({_EP}|last|above|{_A8}) )?({_BC}{_D8}({_A7}|( ({_FC}|previously|in question)){{0,2}}) ({_D7}({_NV2}({_D4})|{_AW0}|{_AU})|{_D6}|({_MA} {_NV}|{_AZ}) {_BJ} {_WA}({_HZ})|{_H3} {_WA}{_AU}|never {_A62}({_AE}|began)|failed|{_BH})|{_CC}{_A7} ({_D7}({_DF})|{_BH}|failed)))\b)|\b({_V1}((({_C5}|{_AB}|{_NV} {_A61}(passed|{_D4})|{_AU}|failed|{_BH}|{_HO})\b|(0(\.0+)?|zero)\s*(%|percent\b)(?!\d)))|(checks?|tests?|runs?|suites?|gates?|proofs?|coverage)( (passed|verified|run|executed))?\s*:\s*{_Z5}(?=\s*($|[,;.)]))|{_CC}{_A7}\s*{_EZ}\s*{_WA}({_DF})\b)")
_EB=_ah(rf"(\b({_BI}|pytest|sympy|python3?|{_FT})\b|{_LP})")
_CZ=_ah(rf"{_B0}{_Z2}({_FV}|failures?|errors?|(error|crash|panick|abort)ed|{_BM}|{_FA})(/{_GG})?\b")
_EA=_ah(r'\bgrep(\s+-[A-Za-z]+)*\s+("[^"]*"|\x27[^\x27]*\x27)')
_B9=rf"(?:comput|verif|deriv|evaluat|solv|prov|calculat|{_CK}|{_CJ})ed|done|{_CO}"
_MW=_sw(r"manually|by hand|(on|(pen|pencil) and) paper|from memory|mentally|in (my|our) head")
_LY0=r"(?:[a-z]{3,16}ly|straight)"
_GM=_sw(rf"(cop(y|ies|ied)|tak(e|en)|past(e|ed)|quot(e|ed)|lift(ed)?|reus(e|ed)|(extract|transcrib|pull|grab|fetch)ed?) ({_LY0} )?from ([^\s,.;!?()\[\]]+ ){{0,4}}(log|output|report|run|build|pr|ci|commit|thread|comment|artifact|record|transcript|cache|snapshot|stdout|stderr|terminal|console|issue)s?")
_DO=r"as\s+(?:(?:report|claim|stat|not|describ|logg)ed|shown)\s+(?:by|in)"
_F7=r"pass|succeed|run|verify|hold"
_GH=_sw(rf"((expect|assum|presum|suppos|believ)ed|thought|likely) (to ({_F7}|clear|compile|build)|(pass|succeed)ing)|(based on|on the basis of) ((an?|the|our) )?(ci|{_EP}|cached|stale|old(er)?|existing|pr|author\x27?s?)( (ci )?(run|build|log|output|report|result|check|test|pr|commit)s?)?|(would|could|should|might) ({_F7}|confirm|match|clear)|({_DO}|according to) (ci|the author|an? ({_EP}) (run|build|ci|log|pr))")
_A0=r"(?:mock|stubb|fak|hard-?cod|fabricat|forg)ed|canned|simulated|dummy"
_BY=r"(?:run|test|check|execution|pass|output|result|value|suite|gate|build|stub|oracle|harness|response|verdict|number|figure|assertion|proof|fixture|metric|count|table|log|trace|benchmark|record)s?|data|verification"
_EK=_sw(rf"simulat(ed|ion) mode|({_A0}|mock|fake|placeholder) ({_BY})|({_BY}) (({_AA}|remained|stayed) )?(an? )?({_A0}|placeholders?)")
_DY=_asw(rf"\b({_NV}|no|without|rather than|instead of) (being )?(({_B9}|(copi|tak|past|quot|extract|transcrib|pull|grab|fetch|bas|expect|assum|us|rel[iy])(e|ed|ing|en)?( on)?) )?((via |with |by )?({_MW})( ({_MW}))?|(in )?(an? )?(dry[-\s]?run|{_EK}|{_A0})|({_LY0} )?(based on|on the basis of|from) [^,.;!?()]{{1,48}})( or ((via |with |by )?{_MW}|{_EK}|{_A0}|dry[-\s]?run))?")
_DR=_asw(rf"\b(({_B9}) ((via|with|by) )?({_MW})|({_MW}) ({_B9})|(?<!-)dry[-\s]?run(s|ning)?|{_EK}|{_GM}|{_GH})\b")
_ci=lambda s:bool(_DR.search(_DY.sub(" ",_ar(s))))
_B2,_B3=rf"errored|crash{_E3}|panic(?:ked|s|king)?|abort{_E1}|killed|segfault\w*|oom",r"(?:test|doc-?test|build|check|link(?:er|ing)|compil(?:e|er|ation)|runtime|assertion|parse|syntax|type|script|gate|suite|proof|verification|ci|automation|tooling)"
_EC=_asw(rf"((?<!\bnon-)\b({_FV}{_D5}|{_B2}|timeout(?!\s*=)|un(run|executed|performed|tested))\b|\b({_Z0} {_Z2}|{_B3} )(failures?|errors?)\b|(^|\s)error:\s|{_EJ}\b|{_GB}|[:,(]\s*{_WA}({_C5})\b|\b({_DW}|{_BO}|never (ran|{_CN}|passed|succeeded)|(left|leav{_E7}|kept|keep{_SI}) {_W0}{{0,3}}({_U0})|{_CV}( |(?=,))\s*{_WA}({_C5})|(({_AA}|be(en)?) {_WA}{_NV}|{_MA} {_WA}{_NV} ((yet|ever) )?be(en)?) ((yet|ever|{_TA}) ){{0,2}}({_CN}|applicable|needed|required))\b)")
_SB=_ah(r"(bloat[\s_-]?alert|spec[\s_-]?shrinking)\s*([:=]|is|was)\s*false\b")
_sb=lambda s:_SB.sub("",s)
_ms0=lambda s:_am(_sb(_EA.sub("",s)))
_df=lambda s:any(_EC.search(_au(v,_CZ)) for v in _ms0(s))
_ZQ=_sw(rf"({_Z0}|none of ({_A8})|not (a single|one))")
_C6=_sw(rf"(r[au]n|running|did|do(es|ing)?|carr{_Y4} out|undert(ook|ak(e|es|en|ing))|m(ade|ak{_E7})|{_H3}|having|({_CJ}|initiat|issu){_E2}|({_CK}|inspect|review|trigger|launch|dispatch|start){_E4}|verif{_Y4}) (nothing\b|{_ZQ} {_B5}{{0,6}}{_CQ}\b)")
_FF=_sw(rf"((un(able|willing)|incapable|reluctant|hesitant|fail{_E8}|(declin|refus){_E6}|forb(idden|ade)|prevented|blocked) (to|from)|(refrain|abstain|desist){_E1} from|(could|can|do(es)?|did|will|would) {_NV}|{_AZ}) {_A62}(run(ning)?|({_CJ}|complet){_E7}|({_CK})(ing)?|carr(y|ying) out|undertak(e|ing)|verif(y|ying)) ((any|all|{_A8}|such|an?) )?{_B5}{{0,4}}({_CQ}|{_BI}|pytest|sympy|anything|it|them)\b")
_GS=_asw(rf"({_BF}\b({_NR}|{_C6}|{_FF}|({_ZQ}|neither( of ({_A8}|them))?) {_B5}{{0,6}}{_CQ} (({_AA}|{_H3}) (be(en|ing) )?)?{_AE2}\b|nothing (({_AA}|{_H3}) (be(en|ing) )?)?{_AE2}\b)|(?<!\bnon-)\b(un(run|executed|performed|tested))\b|\b(left|leav{_E7}|kept|keep{_SI}) {_B5}{{0,4}}({_U0})\b|(?<!\bnon-)\b({_B2})\b|\b{_B3} (failures?|errors?)\b|\b{_D6}\b|\b{_BQ}\b|{_BF}{_CU}\b{_BC}{_D8}{_A7} (({_CM}|{_CS} |{_D3} )?{_WA})({_AW0}|{_FW}|{_B8}|{_NV2}({_D4}))\b)|{_ES}{_NW} {_AX} {_AH3}({_B4}|{_B8})\b|{_CU}(^|\s)(errors?|fatals?|failures?|failed|blocked|aborted|crashed|panicked|timed out):\s*+(?!0\b|none\b|no\b|zero\b|n/?a\b|false\b|pass)|[:,(\[]\s*{_WA}({_C5})\b|([:=(\[→⇒]|[-=]>|(?<=\s)-(?=\s))\s*{_WA}({_AB}|{_FW}|{_NV} {_A61}(passed|{_D4}))\b|[\(\[]\s*{_Z5}(\s*{_Z4}(?=\s*([)\]]|{_P3T}\b))| {_P3T}\b)|:\s*{_Z5}\s*{_Z4}(?=\s*($|[,;.)\]\u2014–-]|{_P3T}\b))|\b({_BC})\s*[\(\[]\s*{_Z5}\s*[\)\]]|\b({_BC})(?<!\bexit\sstatus)(?<!\breturn\sstatus)( ({_P3}))?\s*([:=→⇒]|[-=]>)\s*{_Z5}(?=\s*($|[,;.)\](\u2014–])| -(?!\w))|\b({_BC}|{_CC}){_D8}{_A7}{_BB}\s*{_EZ}\s*{_WA}({_FW})\b")
_de=lambda s:any(_GS.search(_bn(_au(v,_CZ))) for v in _ms0(s))
def _vf(seg):
 vs=_ms0(seg)
 return any(_GT.search(v) for v in vs) or (any(_EB.search(v) for v in vs) and any(_EC.search(_au(v,_CZ)) for v in vs))
_DD=rf"{_L3}\b|editorial\b|{_NB}\b|advisor[iy]\w*\b|untouched\b|deferred\b"
_BW=r"(?:(?:declared|explicit|mandatory)\s+)?(?:\[\s*)?"
_B7=_sw(rf"({_BW}(tracked|oos)\b|(wave-\d+\s+)?({_DD})|(local\s+symbol\s+collisions?|summary\s+clauses?)\b|(minor|polish)\b|((passed|cargo|sympy|layer[-\s]?0)(\s++{_AX})?+|{_AX})\b(?!\s+{_AH3}({_B4}|{_B8P})\b)|/\d+)")
_BT=_nc(rf"{_S}*((the|these|those|our)\s+)?({_NW})(?!\s*{_BW}({_DD}|minor\b|polish\b|tracked\b|oos\b))")
_CD,_OD=r"[\)\]\}\"'»\u201d\u2019]*+",r"[\(\[\{\"'\u201c\u2018«]*+"
_CH=_sw(rf"or ({_NW}|(more|above|greater|higher|extra|additional|further|{_A5}|fatal|real|open|false|wrong|invalid)\b|[+±~>→⇒])|(to|through|thru|plus|up to) {_NW}|(more|above(?! {_L3}\b)|up)( (than {_NW}|{_DN}))?\.?\s*$|\+(\s*{_NW}|\s*$)|([±~>→⇒]|->|=>|\.\.)\s*{_NW}")
_DG=_sw(rf"({_AY}|{_CW}|{_TH}|still|total|only|rather|instead|now|revised|correction|wait|no|nope|also|meaning|namely|i\.e\.|viz\.?|plus|actual(ly)?|(real|tru|rough|near)ly|almost|approx(imately)?|about|around|at least|no {_FL2} than|more than|over|up to|above(?! {_L3}\b)|(we )?found)")
_BS=rf"(?:of\s+them|{_C2}|{_BA}|left|present|still|more|above|{_CA})"
_DH=rf"(?=\s*{_CD}(?:$|[.?!](?!\d)|[,;)\]]|\s+{_BS}\b))"
_DJ,_DI=_asw(rf"(\b(plus|{_CY}|other than)\b|(?<![A-Za-z0-9_])\+){_BT}"),_asw(rf"(\b(({_CY}|other than)\b{_BT}|plus (({_A8}) )?{_NW} (other|more|additional|extra|remaining|open|{_A5}|{_DN})s?\b)|(^|[.?!] |[(,;:\-]\s*+)(at least|no {_FL2} than|more than|over|actually|in fact|correction\s*:?|wait\s*,?)\s*[,;:\-]?\s*{_BT}(?!\s*{_B7})|[(,;:\-]\s*+plus {_BT}(?!\s*{_B7})|(^|[.?!] |[(,;:\-]\s*+)({_DG}\s*[,;:\-]?\s*)+((the|these|those|our) )?{_NW}\+?(?!:\d|-\d){_DH}|(?<!\d)[,;]\s++{_OD}({_NQ}(?!\s*{_B7}){_DH}|([1-9]\d?\+?|{_EV}|{_CH})([.?!]?\s*$|(?= {_BS}\b)))|^\s*+{_CD}\s*+([,;:\-/]\s*+)?({_CH}|(({_DG}|and|about|around|approx\w*)\s*[,;:\-]?\s*)?{_OD}{_NW}\+?(?!:\d|-\d)(?=\s*{_CD}($|[.?!](?!\d)| ({_BS}|or (more|above|greater|higher|{_NW}))\b)))|(^|[.?!] ){_OD}{_NW}\+?(?!\s*([.)] )?{_B7}|:\d|-\d)(?=\s*{_CD}($|[.?!](?!\d)\s*$| {_BS}\b)))")
_F1,_BSE=r"(?:\.{2,}|\u2026|[,:\-=/;|.?!])",r"blocking|structural|editorial"
_ZP=_nc(rf"({_F1}\s*|\b(({_ZN}|(report|verdict|statement|count|total|result|check)s?)\s+)?{_AA}\s+|(?<=[\s.?!\u2026]))")
_Z3,_N7=rf"{_ZP}{_OD}\s*+(?:(?:{_AY}|{_CW}|{_AR}|still|no|nope|{_TH})\s*{_P0}\s*)",rf"{_NW}(?!\s*(?:[.)] )?{_B7}|-\d{{2}}-\d{{2}}\b|:\d+\b)"
_C4=_asw(rf"^\s*+{_CD}\s*(or (?!{_LV}\b|L[123]\b|({_BSE}|other|verified|remaining|unparsable)\b)|(to|through|thru|plus) \d|(more|above(?! {_L3}\b)|up)\b|\+|([+±~>→⇒]|->|=>|\.\.)\s*[1-9]|([.,;:?!]\s*+){{1,2}}({_CH})|({_F1}\s*+{_CD}\s*+{_OD}\s*){{0,2}}(({_DG}|and|{_CY}|other than|the|these|those|or|remain{_E1}){_P0} ){{0,6}}{_N7}|{_Z3}{{1,3}}{_N7}|{_Z3}{{0,3}}(({_FS}|{_NP})\b|{_NQ}\b(?!\s*{_B7}))|{_ZP}{_OD}\s*+(({_AY}|{_CW}|{_AR})\s*{_P0}\s*){{0,2}}{_BO}\b)")
_SD=rf"(?:all|any|{_A8}|such|which|some|most|many|several|both|every|each|an?|another|{_EV}|[1-9]\d*|key|core|critical|primary|certain)"
_B6K=r"(?:invariant|assertion|guarantee|condition|standard|contract|mechanism|claim|axiom|theorem|bound|rule|guard|control|safeguard|brake|spec)s?"
_B6=_nc(rf"{_CR}|{_EN0}|{_B6K}|(verdict|(assump|sec|func)tion|review|inspection|count|total|number|figure|premise|validator|metric|report|(assess|argu|state)ment|conclusion|(declar|certific|justific|explan|cit|deriv|equ|implement)ation|finding|result|constraint|limit|ceiling|budget|threshold|barrier|table|row|line|record|outcome|decision|ruling|approval|(infer|refer)ence|account|narrative|method|diff|file|module|crate|hook|source|formula|estimate|model|system|layer|chapter|parser|matcher|commit|trailer|artifact)s?|(test|check|audit)ing|(tall|propert|polic|summar|entr|identit){_YI}|(hypo)?thes[ei]s|(diagno|analy)s[ei]s|criteria|text|prose|status|reasoning|logic|methodology|code(base)?|(approach|patch)(es)?|appendi(x|ces)")
_F8,_B6Z=rf"above|{_EP}|preceding|(?:stat|report|claim|alleg)ed",rf"{_AW}{{0,2}}(?:{_B6}|{_ZN})"
_GJ=_sw(rf"({_SD} {_B6Z}\b|all of {_D2}\b|(this|that|these|those|such|which|it|they|everything|anything|what(ever)?)\b( ({_F8}|here|below))?|the ({_F8}|foregoing))")
_DS=_sw(rf"({_GJ}|({_Z0}|none) {_B6Z}\b|none of {_D2}\b|nothing\b|the zero)")
_ER=_nc(r"(we|i|((the|our|these|those)\s+)?([A-Za-z-]+\s+){0,2}((reviewer|auditor|check|test|inspection|audit|oracle|script|diff|counterexample|suite|gate|build|run|log|output|result|proof)s?|evidence|analys[ei]s|code|data|(patch|harness)(es)?))")
_MR=rf"(?:(?:{_MA}|hereby|now)\s+){{0,2}}"
_PN=rf"(?:\b{_CM}{_WA}(?:{_FS}|{_NC}|{_DW}|{_EQ})|{_EJ})\b"
_FL=rf"(?:[.,?!;:\u2026]|--?)\s*+{_CD}\s*+{_OD}\s*+"
_HG=_nc(rf"holds?|held|appl{_Y3}|(pass|match)(es|ed)?|(work|succeed|finish|clear|exist|remain|happen){_E5}|st(ands?|ood)|r[au]ns?|(execut|complet)(es?|ed)|occur(s|red)?|verif{_Y3}")
_ET=_nc(rf"{_A62}({_CM}{_WA}({_PA}|{_CN}|(pass|succeed|work|exist|remain|occur|happen|finish|clear)ed|{_CO}|undertaken)|(({_MA}|do(es)?|did)\s+{_A62})?({_HG}))")
_CE=_sw(rf"(({_A4}|{_AQ}|{_AZ})|({_AA}|{_H3}|{_MA}) {_NV}) {_A62}((have been|be(en|ing)?) )?{_AE2}|({_AA}|{_H3}|{_MA} be) {_WA}{_AU}|({_AN}|never) {_A62}(proceed(ed)?|start(ed)?|beg(in|an)|finish(ed)?|complet(e|ed)|r[au]n|execut(e|ed)|happen(ed)?|occur(red)?)")
_B1=_sw(rf"neither( of ({_A8}|them))?( {_B6Z})?{_A7}{_BB} {_ET}")
_DK=_sw(rf"(nothing|none of {_D2}|{_ZQ} {_AW}{{0,2}}({_CR}|{_B6K}|(propert|polic){_YI}|criteria|evidence))")
_AJ=_sw(rf"(({_MA} )?((have|be(en)?) )?|{_AA} )?({_FV}{_D5}|break{_SI}|brok(e|en)|(crash|error|abort|stall)ed|{_D0})")
_BK=rf"{_BF}{_a3(r'\bwhether\s',r'\bwhere\s')}(?:(?:it|this|that|they|these|those){_A7}{_BB}\s*(?:{_CE}|{_D7}(?:{_AW0}|{_AU})|{_B8}|{_AJ})|{_B1}|{_DK}{_A7}{_BB}\s+{_ET}|{_ER}\s+{_A62}{_C6})"
_F9=_sw(rf"(({_AY}|{_CY}|(except|save|only) that|{_CW}|and|{_AR}|in truth|correction|wait|no|nope){_P0} ){{0,3}}({_GJ}{_A7}{_BB}\s*({_CE}|{_D7}({_AW0}|{_AU})|{_B8}|{_AJ})|{_DS}{_A7}{_BB}\s*{_CM}{_WA}{_BO}|{_B1}|{_DK}{_A7}{_BB} ({_ET}|(exist|remain){_E5})|(which|that) {_ER} {_MR}{_RA}|{_ER} {_MR}({_RA} {_DS}|{_DW})|{_DW}|{_PN}|{_WA}((refut|contradict|rebutt|invalidat)ed|{_FS}|{_NC}|not {_A62}run)(?=$|[.,;:!?()\[\]]| by\b))\b")
_HC,_CT=_ah(rf"^\s*+{_CD}{_FL}{_F9}"),_ah(rf"(^\s*+{_CD}|([.,?!;:\u2026]|--?))\s*+{_CD}\s*+{_OD}\s*+{_F9}")
_HY=_ah(r"^([A-Za-z0-9/%x\-+±~]|(?<![a-z])\.(\d|\.))")
_F3=_sw(rf"({_AY}|still)\s*[,;:\-]?\s*{_NW} (remain{_E1}|exist{_E5}|persists?|{_AA} (still )?(open|present|un(resolved|fixed|addressed)|found))")
_HW=_ah(rf"\b({_FS}|{_NP}|{_BO}|{_RA}|{_DW}|{_F3}|{_BK})\b")
_RV=_sw(rf"({_EI}|(({_AA}|be(en)?) )?((based|built|founded|predicated|dependent) on|(grounded|rooted) in)|(rest|rel(y|ie)|depend|draw|build)s? on|(proceed|stem|follow|deriv)s? from|us{_E2}|((as|pre)sum|stat|invok){_E6}|mak{_E7}|made|(assert|present){_E5}|giv(e|es|en)|gave)")
_RN=_sw(rf"({_DS}{_A7}{_BB} (({_MA}|{_H3}) )?{_RV} ((an?|the|some|many|several|multiple|very|quite|(fundamental|deep|fatal|severe|plain|clear|manifest|obvious)ly) ){{0,2}}({_NC}|{_FS})\b)")
_CB=_ah(rf"(\b({_NC}|{_FS}|{_BO}|{_DW}|{_BK})\b|{_PN}|{_RN})")
_LT=".,;:!? \t)]}\"\x27\u00bb\u201d\u2019"
_F4=_sw(rf"({_DS}|(clean|passing|valid|sound|full|blind|independent|complete|formal) ({_B6}|status|adequacy|soundness|validity|compliance|parity)\b|(passing )?status\b|(adequac|validit|veracit|integrit|sufficienc|viabilit|safet)y\b|(sound|correct)ness\b|accuracy\b|truth\b|compliance\b|parity\b|it\b|them\b|zero\b)")
_HB=_sw(rf"(?:(?:{_W3})) {_MR}(?:reject|den{_YI}|withhold|(?:declin|refus)e (?:passing|clean|approval|certification|to (?:certify|approve|pass)))\b|")+_RA.replace("|ing","").replace("(?:reject|contradict)(?:s|ed)?",rf"contradict(?:s|ed)?|rejected|rejects?\s+{_F4}").replace("den(?:y|ies|ied)",rf"denied|den{_YI}\s+{_F4}").replace("|ying","").replace("|ting","")+")"
_GQ=r"(?:[a-z-]+(?:,\s*+|\s++))"
_G7=_nc(rf"({_a3(*(r'\b'+w for w in 'none of |neither of |no |0 |zero |without |if |whether |unless |when '.split('|')))}{_SD}\s+|(^|{_dv(*r'\. |; |, |: |because |since |while |although |though |whereas |that |and |but |yet |as |despite |notwithstanding |regardless of |in spite of |given |where |so |after |with '.split('|'))}))((?!({_Z0}|if|whether|unless|when|none|neither|without)\b){_GQ}){{0,3}}({_B6})({_C0J}{_GQ}?({_B6})){{0,2}}")
_D9=_asw(rf"(\b{_BO}|\b{_DW}|\b{_BK}|{_PN}|{_CU}(?<!\bwithout\s)\b{_HB}|{_G7}{_A7}{_BB}\s*({_CE}|{_B8}|{_AJ}|({_CM}({_WA}|{_W2})|{_D3} {_WA}|({_NV} )?(having been|being) {_WA})({_NV} (passed|{_D4})|{_AW0}|{_AU})|{_D6}|({_AN}|{_AQ}|{_H3} {_NV}) ({_W1} )?({_GI}|(succeed|proceed|start|finish){_ED}|h(old|eld)|r[au]n|beg(in|an)|appl(y|ied)|st(and|ood)))|\b{_D6}|\b{_BQ}|(permit|allow|exempt|tolerat|accept|ignor)\w* ((any|all|the|such) )?({_AW0})|(despite|with) ({_A8} )?{_BC} ({_AH} ){{0,2}}({_AW0})|\b(we|i|((the|our|an?) )?(author|maintainer|reviewer|auditor|contributor|developer|team)s?) {_OB} {_A61}({_VB0}|({_VB1})e|re-?run|run)|\b{_NR}|\b{_ch(_B6)}|\b{_F3})\b")
def _d0(text,ed=True,*,masked=False):
 ap=[_au(_sb(x),_CZ) for x in _bq(r"\.(?:\s|$)",text if masked else _ms(text))]
 cl,tail=ap[0],". ".join(x for x in ap[1:] if x.strip(_LT))
 cl2,tl2=_bn(cl),_bn(tail) if tail else ""
 return bool(_HW.search(cl) or _D9.search(cl2) or _DJ.search(cl) or _DI.search(text) or _a4(cl,ed) or (tail and (_CB.search(tail) or _D9.search(tl2) or _DJ.search(tail) or _CT.search(". "+tail) or _a4(tail,ed))))
_c5=lambda s,ed=True,*,masked=False:any(_d0(v,ed,masked=masked) for v in _am(s))
_LM,_GE=r"(?:[-*+]|\d{1,9}[.)])",rf"{_S}*+(?:>{_S}*+)*+"
_EW=_rc(rf"{_LM}(?:{_S}+|$)")
_EX=_nc(rf"(\|{_S}*+:?-++:?{_S}*+(\|{_S}*+:?-++:?{_S}*+)*+\|?|:?-++:?{_S}*+\|({_S}*+:?-++:?{_S}*+\|)*+{_S}*+(:?-++:?{_S}*+)?){_S}*+")
_FO=_rc(rf"^{_GE}{_EX}$")
_BU=rf"(?:-{_S}*+){{3,}}+$|(?:\*{_S}*+){{3,}}+$|(?:_{_S}*+){{3,}}+$"
_EM0=r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}"
_CG=_rc(_nc(rf"^(#{{1,6}}({_S}|$)|>|`{{3,}}|~{{3,}}|{_LM}({_S}|$)|{_BU}|<(?!(https?://|[A-Za-z0-9._%+-]+@))[A-Za-z/?!])"))
_ZT=r"(?:0|no\b|zero\b)"
_CM_UNESC_RE=_rc(r"\\([!\"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{|}~])")
_un2=lambda s:_CM_UNESC_RE.sub(r"\1",s)
_H5=_ah(rf"<br[ \t]*/?>|<https?://[^\s<>]+>|<{_EM0}>")
_H4=_ah(r"<!--|<\?|<!|</|<[A-Za-z][A-Za-z0-9:-]*+([\s/>=]|$)")
_HS=_rc(r"&(?:#[0-9]+|#[xX][0-9A-Fa-f]+|[A-Za-z][A-Za-z0-9]+);")
_FJ=_rc(rf"{_H0}%")
_AK=r"\\text(?:rm|bf|it|sf|tt|md|up|sl|sc|normal)?"
_EE=r"rm|bf|it|sf|tt|cal|mit|oldstyle|bold|bm|boldsymbol|pmb|Bbb|frak|scr"
_FX=r"underline|overline|tilde|hat|bar|vec|acute|grave|breve|check|d{1,4}ot|widetilde|widehat|(?:over|under)(?:right|left|leftright)arrow|Overrightarrow|mathring"
_TW=rf"\\(?:text|math|sym|operatorname|{_EE}|{_FX}){_L}*"
_SP0=r"(?:thin|med|thick)space|q?quad|en(?:space|skip)"
_A9=rf"(?:\\(?:[!,:;>/@\- ]|mkern\s*(?:\{{[^{{}}]*\}}|\d+\s*mu)|{_SP0})|~)"
_C9,_DB=_rc(_TW),_rc(_A9)
_AF,_CL=rf"{_TW}\s*\{{{_L}+\}}",rf"{_TW}\s*\{{{_L}{{2,}}\}}"
_BR=_nc(rf"\\(cdot|times|circ|ast|star|bullet|diamond|pm|mp|div|o(plus|times|dot|minus|slash)|wedge|vee|cap|cup|sq(cap|cup)|uplus|amalg|wr|triangle|(small)?setminus){_NL}")
_GZ=r"(?:[A-Z]?[a-z]{2,5}|[A-Z]{2,5})"
_BG=r"(?<![\\A-Za-z])"
_G5=_rc(_nc(rf"{_BG}({_GZ}\s*{_BR}\s*([A-Z]?[a-z]{{1,6}}|[A-Z]{{2,6}})|{_L}\s*{_BR}\s*([A-Z]?[a-z]{{2,6}}|[A-Z]{{2,6}})|({_L}{{1,4}}\s*{_BR}\s*){{3,}}{_L}{{1,6}}|[a-z]{{2,4}}[_^][a-z]{{1,4}}\s*{_BR}\s*{_L}{{2,6}}){_NL}"))
_EN,_EF=r"\{(?:[^{}]++|\{[^{}]*+\})++\}",rf"{_TW}\s*\{{{_L}{{3,}}\}}"
_FG=rf"{_TW}\s*\{{(?:{_L}{{2,}}\}}{_L}+|{_L}\}}{_L}{{2,}})|(?<![\\A-Za-z0-9]){_L}{{2,}}{_TW}\s*\{{{_L}+\}}"
_C8=_sw(rf"{_CL}\s*{_AF}|{_AF}\s*{_CL}|{_AF} {_L}++ {_TW}{_NL}|{_BG}{_L} {_EF}|{_TW}\s*\{{(?!mod\}})({_L}+\}}\s*{_A9}\s*(?!d{_L}\b){_L}{{2,}}|([a-z]{{3,}}\}}{_A9}\s*[a-z]|[A-Z]{{3,}}\}}{_A9}\s*[A-Z]){_NL})|{_FG}|{_TW}\s*\{{{_L}\}} ([a-z]{{3,}}|[A-Z]{{3,}}){_NL}")
_GY=_rc(_sw(rf"{_BP}(\\([/@\-]|!(?!\s*(\\(left|right|[Bb]igg?[lr]?)\b|[\(\[\{{])))|{_AK}\s*\{{[^{{}}\s]+\}}\s*({_A9}|\\(text|math|sym|{_EE}){_L}*{_NL}|{_L}+\s*{_AK})|{_CL} {_L}{{2,}}|{_BG}({_L}{{2,}} ({_CL}|{_AF} {_L}+)|{_L}+ {_AF} {_L}{{2,}})|{_AF} {_L}{{2,}} {_L}{{2,}}|{_CL}\s*(\\([;:>\x20]|(med|thick)space|q?quad|en(space|skip))|~)|{_AF}\s*{_A9}\s*({_AF}|(?!d{_L}\b){_L}\s*({_L}|{_AF}|{_A9}))|(?<![\\_A-Za-z]){_L}+\s*{_A9}\s*({_CL}|(?!d{_L}\b){_L}{{2,}}|{_L} {_L})|((?<![\\A-Za-z0-9])(?!d{_L}\b){_L}+\s*{_A9}\s*){{2,}}(?!d{_L}\b){_L}+|{_AK}\s*\{{{_L}{{2,}}\}}\s+{_L}{_NL}|{_BG}{_L}{{2,}} {_AK}\s*\{{{_L}+\}}|{_EF} {_L}{_NL}|{_C8}|{_BG}({_L}{{2,}}[_^]{_EN}{_L}++|{_L}++[_^]({_EN}[a-z]{{2,}}|\{{[A-Za-z_^]++\}}{_L}++)))"))
_BN=rf"(?:{_AL}++|{_EN})"
_BN2=rf"(?:[_^]{_BN}{_L}*+)*+"
_D1,_FZ=rf"{_L}++{_BN2}",rf"[A-Za-z]?[a-z]{{2,}}{_BN2}"
_DC,_FY=rf"(?:{_L}{{2,}}{_BN2}|{_L}++[_^]{_BN}{_L}++)",rf"(?:{_L}{{3,}}{_BN2}|{_L}{{2,}}[_^]{_BN}{_L}++|{_L}++[_^]{_BN}{_L}{{2,}})"
_G1=_rc(_nc(rf"(?<![\\A-Za-z0-9]){_AL}++((_{_BN}){{2,}}|(\^{_BN}){{2,}}|([_^]{_BN}){{3,}})"))
_GX=_rc(rf"(?<![\\A-Za-z_^])(?:(?!(?:{_L}\s+)?d{_L}\s+d{_L}\b){_D1}(?:\s++{_D1}){{2,}}|(?!d{_L}\s+d{_L}\b){_FZ}\s++{_FZ}){_NL}")
_FI=_rc(rf"(?<![\\A-Za-z_^])(?:{_FY}\s++{_D1}|{_D1}\s++{_FY}|(?!d{_L}\s+d{_L}\b){_DC}\s++{_DC}){_NL}")
_G6,_G4=_rc(r"(\{[^{}\\_^]*)\{([^{}\\]*)\}"),_rc(r"(?<![\\A-Za-z_^])\{([A-Za-z]+)\}")
_G2,_DT=_rc(rf"(?<![_^\\])\{{\s*({_TW}\s*\{{[^{{}}]*\}})\s*\}}"),_rc(r"(\\(?:[dtc]?frac|[dt]?binom)\s*\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\})\s*(\{[^{}]*(?:\{[^{}]*\}[^{}]*)*\})")
_G3=_rc(rf"(?:{_AF}\s*){{3,}}|{_EF}\s+(?![aI]\b){_L}{_NL}|{_C8}|{_AK}\s*\{{[^{{}}\s]+\}}\s*(?:{_A9}|{_L}+\s*{_AK})")
def _hto(raw,bt=False):
 rm=_ru(r"\\mkern\s*\{([^{}]*)\}",r"\\mkern \1",raw)
 if _G6.search(rm) or _G1.search(raw):return True
 nr=len(rm)
 def _bw(k):
  d,md=1,1
  while k<nr and d>0:
   if rm[k]=="\\" and k+1<nr:k+=2
   else:
    d+=(rm[k]=="{")-(rm[k]=="}")
    if d>md:md=d
    k+=1
  return k,d,md
 if _rs(r"\{\s+\}",rm) or _rs(rf"{_BG}{_L}+\{{\s*\}}{_L}",_DZ.sub(" ",rm)):return True
 for sm in _bz(rf"{_L}+[_^]\{{",rm):
  k,d,md=_bw(sm.end())
  if d==0 and k<nr and rm[k].isalpha():
   q=k
   while q<nr and rm[q].isalpha():q+=1
   if md>=2 or (_rs(_L,rm[sm.end():k-1]) and (sm.end()-sm.start()>3 or q-k>=2)):return True
 for m in _C9.finditer(rm):
  j=m.end()
  while j<nr and rm[j] in _HTN:j+=1
  if j<nr and rm[j]=="{":
   k,_,_=_bw(j+1)
   inn,p,q=rm[j+1:k-1],m.start()-1,k
   while p>=0 and rm[p] in "{}":p-=1
   while q<nr and rm[q] in "{}":q+=1
   if _rs(rf"(?:{_L}\s*{_A9}\s*{_L}|{_A9}\s*{_L}{{2,}})",_ru(r"(?<!\\)[{}]","",inn)) or not inn.strip() or (inn!=inn.lstrip() and p>=0 and rm[p].isalpha()) or (inn!=inn.rstrip() and q<nr and rm[q].isalpha()):return True
 ub=_ru(rf"\{{({_A9})\}}",r"\1",_DT.sub(r"(\1 / \2)",rm))
 for _ in range(3):ub=_G2.sub(r"\1",ub)
 ub=_G4.sub(r"\1",ub)
 if bt:return bool(_G3.search(ub) or _rs(rf"{_BG}(?!d{_L}\s+d{_L}\b)(?:{_L}\s+){{3,}}{_L}{_NL}",raw))
 _nb=lambda s:_ru(r"(?<!\\)[{}]","",s)
 uw,rnt=_nb(_C0.sub("",_EH.sub("",rm))),_ru(rf"{_AK}\s*\{{[^{{}}]*\}}"," , ",rm)
 unt,nb=_nb(_C0.sub("",rnt)),_nb(rm)
 return bool(any(_GY.search(s) for s in (raw,ub,nb,uw)) or any(_G5.search(s) for s in (raw,nb,_nb(_C0.sub("",rm)))) or _GX.search(_DB.sub(" ",unt)) or _FI.search(_DB.sub(" ",rnt)) or _rs(rf"{_L}\s*{_BP}\\[%#&$]\s*{_L}",raw) or not _da(raw,True,6,False).strip())
_FK=_rc(rf"{_BP}(?:\$|\\[()\[\]])")
_HC0=r"[hv]?phantom|class|style|cssId|href|(?:text|page|math)?color|colorbox|html(?:Class|Id|Style|Data)"
_DM=_rc(_nc(rf"\\(begin|end|ref|eqref|{_HC0}|[gex]?def|let|(re|provide)?newcommand|require|unicode|fcolorbox|definecolor|(math)?[rlc]lap|kern|[hv]skip|hspace|m(skip|space)|mkern(?!({_S}*[1-6]mu(?!\d)))|choose|brace|brack|l?eqno|(nobreak)?space){_NL}|\\([Bb]igg?[lr]?\s*(\.|\{{\s*\}})|(left|right)\s*\{{\s*\}})|(?<={_L})\s*\\(left|right)\s*\.\s*{_L}"))
_EH=_rc(rf"{_AK}{_NL}")
_C0=_rc(_nc(rf"\\(math(rm|bf|it|sf|tt|cal|bb|frak|scr|normal)|mathbf(it|sf(up)?|scr|cal|frak)|mathsf(bfit|up|it)|sym(rm|bf|it|sf|tt)|{_EE}|[Bb]igg?[lr]?|left|right|{_FX}|[dtc]?frac|[dt]?binom|root|of|operatorname|(under|over)brace|stackrel|(over|under)set|x(right|left)arrow){_NL}"))
_GW=_rc(rf"\\(?:mkern|{_SP0})")
_G0=_rc(rf"{_BP}(\\{_L}+)")
_DZ=_rc(_nc(r"\\(alpha|beta|[Gg]amma|[Dd]elta|zeta|eta|iota|[Ll]ambda|mu|nu|[Xx]i|tau|[Uu]psilon|chi|[Pp]si|[Oo]mega|(var)?(epsilon|theta|kappa|pi|rho|sigma|phi)|Theta|Pi|Sigma|Phi|[lg]{2,3}|[lg]e(q(slant)?)?|neq?|less(sim|eqgtr)|gtr(sim|eqless)|approx|sim(eq)?|equiv|cong|propto|asymp|doteq|bowtie|models|n?v[Dd]ash|dashv|Vdash|notin|in|ni|owns|(sq)?(sub|sup)set(n?eq)?|(prec|succ)(eq)?|(big)?(sq)?(cup|cap)|(big)?(uplus|o(times|plus|dot)|wedge|vee)|(small)?setminus|cdot|[clvd]?dots|times|o(minus|slash)|pm|mp|div|ast|star|circ|bullet|d?dagger|amalg|wr|diamond|triangle|(black)?square|l(and|or|not)|not|neg|to|gets|rightleftharpoons|(([Ll]ong|hook)?([Rr]ight|[Ll]eft|[Ll]eftright)|[uU]p(down)?|[dD]own|[ns][ew])arrow|implies|iff|(long)?mapsto|n?(mid|parallel|exists)|perp|bot|top|forall|infty|partial|nabla|emptyset|varnothing|ell|hbar|[ij]math|wp|Re|Im|aleph|beth|gimel|daleth|(measured|spherical)?angle|(back)?prime|surd|flat|natural|sharp|(club|diamond|heart|spade)suit|sum|(co)?prod|i{1,4}nt|idotsint|oint|(var)?(inj|proj)?lim(sup|inf)?|sup|inf|max|min|arg|det|dim|exp|ln|log|lg|(arc)?(sin|cos|tan|cot|sec|csc)h?|gcd|deg|ker|hom|Pr|[bp]?mod|pod|sqrt|[lr](angle|floor|ceil|[vV]ert|brace)|[vV]ert|backslash|[ul][lr]corner)(?![A-Za-z])\*?[ \t]*"))
_FH=_rc(r"\\[A-Za-z]+\*?[ \t]*")
_EG,_F0,_MK0=frozenset(("frac","dfrac","tfrac","cfrac","binom","dbinom","tbinom")),frozenset(("stackrel","overset","underset")),r"(\\(?:mkern|mskip|kern|[hv]skip))\s*(\{[^{}]*\}|[-+]?(?:\d+(?:\.\d*)?|\.\d+)\s*(?i:[a-z]{2}|mu))"
_ak=lambda pat:rf"\\mkern\s*(?:\{{\s*{pat}\s*(?i:mu)\s*\}}|{pat}\s*(?i:mu))(?!\d){_S}*"
_HK,_HJ,_HI=_rc(_ak("[12]")),_rc(rf"\\,[ \t]*|\\thinspace{_NL}[ \t]*|{_ak('3')}"),_rc(rf"\\[:>;][ \t]*|\\(?:(?:med|thick)space|q?quad|enspace|enskip){_NL}[ \t]*|{_ak('[456]')}")
def _da(text,_bp=False,_ct=0,_ks=False,_a7=False):
 t=text.replace("\x01","").replace("\x02","")
 if _bp:t=_ru(rf"({_H0})%[^\n]*(?:\n{_S}*)?",r"\1",t)
 t,cm=_fd(t),5 if _ct is True else int(_ct)
 sp_c="\x02" if (cm==6 or (cm==5 and not _ks)) else "\x01"
 t=_ru(rf"\\(?:fcolorbox|rule|resizebox)(?:\s*\[[^\]]*\])?(?:\s*\{{[^{{}}]*\}}){{2}}|\\(?:label|raisebox|scalebox|{_HC0}|hspace\*?|mspace|unicode)\*?(?:\s*\[[^\]]*\])?\s*\{{[^{{}}]*\}}(?:\s*\[[^\]]*\]){{0,2}}|\\smash\s*\[[^\]]*\]|\\(?:left|right|middle|[Bb]igg?[lrm]?)\s*\.","",t)
 t=_ru(rf"\\[!/@\-]{_S}*","\x02",_ru(rf"\\\\(?:\s*\[[^\]]*\])?|({_H0})&",lambda m:(m.group(1) or "")+sp_c,_ru(r"\\\\([>/\-@!,:; \t\n])",r"\\\1",t)))
 for rx,cs in ((_HK,(1,2,5,6)),(_HJ,(2,3,5,6)),(_HI,(3,4,5,6))):t=rx.sub("\x02" if cm in cs else "\x01",t)
 s1=_ru(r"\\[ \t\n]|~|\\(?:nobreak)?space(?![A-Za-z])[ \t]*",sp_c,_ru(rf"{_MK0}[ \t]?",sp_c,t))
 _su=r"[\^_][ \t\n]*"
 s1=_ru(_su+r"[\x01\x02]+","",s1).replace("\x02","") if cm>=5 else _ru(_su+r"(\x01+)",r"\1",_ru(rf"(?<=\d)\x02+{_LA}","\x01",_ru(_su+r"\x02+","",s1)).replace("\x02",""))
 res,stk,idx,n=[],[],0,len(s1)
 ra=res.append
 _bp= lambda:len(res) if (res and res[-1] in "^_" and not any(f[2]==len(res) for f in stk)) else -1
 while idx<n:
  ch=s1[idx]
  if ch=="\\" and idx+1<n:
   if s1[idx+1] in "()[]":
    if s1[idx+1] in "([":stk.append(("math",")",-1))
    elif stk and stk[-1][1]==")":stk.pop()
    idx+=2
    continue
   is_txt=bool(tm:=_EH.match(s1,idx))
   if (wm:=tm or _C0.match(s1,idx)):
    cmd,j=wm.group(0)[1:].rstrip("*"),wm.end()
    while j<n and s1[j] in _HTN:j+=1
    if j<n and s1[j]=="{":
     if (cmd in _EG or cmd in _F0) and (not res or res[-1] not in "^_"):ra("\x01")
     stk.append(("text" if is_txt else "math","\x01" if (cmd in _F0 or (cmd in _EG and cm>=5)) else ("/" if cmd in _EG else ""),_bp()))
     idx=j+1
     continue
    idx=j
    continue
   if s1[idx+1].isalpha() and (sm:=_FH.match(s1,idx)):
    if _DZ.fullmatch(sm.group(0)):ra("·")
    elif res and res[-1] in "^_":res.pop()
    idx=sm.end()
    continue
   ra("·" if (cm==6 or s1[idx+1] in "{}%#&$") else ("\x01" if s1[idx+1]=="_" else s1[idx:idx+2]))
   idx+=2
   continue
  if ch=="$":
   if stk and stk[-1][1]=="$":stk.pop()
   elif stk and stk[-1][0]=="text":stk.append(("math","$",-1))
   idx+=1
   continue
  if ch=="{":
   stk.append((stk[-1][0] if stk else "math","",_bp()))
   idx+=1
   continue
  if ch=="}":
   if stk:
    _,sep,sp=stk.pop()
    if sp>0 and len(res)>=sp and res[sp-1] in "^_" and not "".join(res[sp:]).replace("\x01","").replace("\x02","").strip():res[sp-1:]=[]
    elif sep and sep not in ("$",")"):ra(sep)
   idx+=1
   continue
  if ch in _HTN:
   while idx<n and s1[idx] in _HTN:idx+=1
   if _ks or (cm<6 and stk and stk[-1][0]=="text"):ra("\x01")
   continue
  ra(ch)
  idx+=1
 out="".join(res).replace("\x01"," ")
 if cm==6:out=_ru(rf"{_LB}[ \t]*·+[ \t]*{_LA}","",out)
 elif cm==5 and not _ks:out=_ru(rf"{_LB}·+{_LA}","",out)
 if cm>=5 and _ks:out=out.replace("·"," ")
 return _a6(out) if _a7 else _ru(rf"(?<={_L}{{2}})\s*[_^](?={_AL})|{_DA0}" if _ks else _DA0," ",out)
_HV=_rc(r"[ ]{0,3}(`{3,}|~{3,})([^\n]*)")
_H6=_rc(r"^(`{3,}|~{3,})([^\n]*)$")
_GN=_rc(rf"<(?:[A-Za-z][A-Za-z0-9+.-]{{1,31}}:[^\s<>]*|{_EM0})>")
def _sf(ln,fn,*,_ai0=True):
 rx=_HV if (fn or _ai0) else _H6
 if not (fm:=(rx.fullmatch(ln) if fn else rx.match(ln))):return fn,False
 fc,nf=fm.group(1),fm.group(2)
 if fn:return (None,True) if fc[0]==fn[0] and len(fc)>=fn[1] and not nf.strip(_HT) else (fn,False)
 return ((fc[0],len(fc)),True) if (fc[0]=="~" or "`" not in nf) else (None,False)
def _d5(tx):
 fn=None
 for r in _b2(tx).split("\n"):
  if fn:fn,_=_sf(r,fn)
  else:
   fn,op=_sf(r,None,_ai0=False)
   if not op and _sf(_ba(r,True)[1],None,_ai0=False)[1]:return f"{_DA}nested fenced code {_AD}"
 return f"{_DA}unclosed fenced code {_AD}" if fn else None
_HL=_rc(_nc(rf"(?m)\n({_S}*+\n|{_S}*+(#{{1,6}}({_S}|$)|>|`{{3,}}|~{{3,}}|{_LM}({_S}|$)|<|{_BU}|(=++{_S}*+|-++{_S}*+)$)|{_GE}{_EX}$)"))
_AI=_rc(r"^#{1,6}(?:[ \t]|$)")
def _an(ln):
 i,col,n,ag,bq=0,0,len(ln),False,0
 while i<n:
  s0=i
  while i<n and ln[i] in _HT:i+=1
  w=_aj(ln[s0:i],col)-col
  eff=max(0,w-1) if ag else w
  if i<n and ln[i]==">" and eff<=3:
   col,i,ag,bq=col+w+1,i+1,True,bq+1
   continue
  p=s0+(1 if ag and s0<n and ln[s0]==" " else 0)
  return bq,eff,col+(p-s0),p
 return bq,0,col,n
def _co(rr):
 c=rr.strip(_HT)
 if c.startswith("|"):c=c[1:]
 if c.endswith("|") and not _at(c,len(c)-2):c=c[:-1]
 return c
def _gc(r):
 c=_co(r)
 return sum(1 for k,ch in enumerate(c) if ch=="|" and not _at(c,k-1))+1
def _ay(s,j,tl,lim,mt,cm=True):
 if cm and tl in mt:return 0
 k=j
 while (p:=s.find("`"*tl,k,lim))!=-1:
  q=p+tl
  while q<lim and s[q]=="`":q+=1
  if q-p==tl:return q
  k=q
 if cm:mt.add(tl)
 return 0
def _dn(s,_a1=None):
 s=_b2(s)
 tr=_b7(s.split("\n"))
 def _c1(ln,li):
  _,ef,col,p=_an(ln)
  if ef>=4 or li in tr or _GD.match(ln):return True
  sub=ln[p:]
  return (_EW.match(sub.lstrip(_HT)) and _b4(sub,col)[0]) or bool(_AI.match(_ba(ln,True)[1]))
 o,fb,i,n,fn,ih,hb,cl,mt,li,le,sl=[],[],0,len(s),None,False,-1,0,set(),-1,0,False
 oa=o.append
 while i<n:
  if i==0 or s[i-1]=="\n":
   li+=1
   le=n if (le:=s.find("\n",i))==-1 else le
   ln=s[i:le]
   if ih and (not ln.strip(_HT) or _an(ln)[0]<hb):ih=False
   if not fn and not ih:
    b0,e0,_,p0=_an(ln)
    if e0<=3 and _rm(r"(?i)<br(?:[\s/>]|$)",ln[p0:].lstrip(_HT)):ih,hb=True,b0
   sl,tg=_c1(ln,li),False
   if not ih:fn,tg=_sf(ln,fn)
   if fn or tg:
    if _a1 is not None:
     if tg and fb:
      _a1.append("\n".join(fb))
      fb.clear()
     elif not tg and fn:fb.append(ln)
    i=le+1 if le<n else n
    continue
  if ih:
   oa(s[i:le]+("\n" if le<n else ""))
   i=le+1 if le<n else n
   continue
  if s[i]=="\n":
   oa("\n")
   i+=1
   continue
  if s[i]=="<" and not _at(s,i-1) and (am:=_GN.match(s,i)):
   oa(am.group(0))
   i=am.end()
   continue
  if s[i]=="`":
   if _at(s,i-1):
    oa("`")
    i+=1
    continue
   j=i
   while j<n and s[j]=="`":j+=1
   if j>=cl:
    mb=_HL.search(s,j)
    cl=mb.start() if mb else n
    mt.clear()
   lim=min(cl,le) if sl else cl
   if li in tr:
    pk=j
    while (pp:=s.find("|",pk,lim))!=-1:
     if not _at(s,pp-1):
      lim=pp
      break
     pk=pp+1
   q=_ay(s,j,j-i,lim,mt,lim==cl)
   if q and "\n" in s[j:q]:
    el=li+s.count("\n",j,q)
    if li not in tr and any(ki in tr for ki in range(li+1,el+1)):q=0
    else:
     li,ls=el,s.rfind("\n",j,q)+1
     le=n if (le:=s.find("\n",q))==-1 else le
     sl=_c1(s[ls:le],li)
   if q:
    if _a1 is not None:_a1.append(s[j:q-(j-i)])
    i=q
    continue
   oa(s[i:j])
   i=j
   continue
  nx=[x for c in "`\n<" if (x:=s.find(c,i+1))!=-1]
  np=min(nx) if nx else n
  oa(s[i:np])
  i=np
 if _a1 is not None and fb:_a1.append("\n".join(fb))
 return "".join(o)
def _ba(ln,_d7=False):
 p,n,bq=0,len(ln),0
 while p<n:
  while p<n and ln[p] in _HT:p+=1
  if p<n and ln[p]==">":
   bq,p=bq+1,p+1
   continue
  if _d7 and (lm:=_EW.match(ln,p)):
   p=lm.end()
   continue
  break
 return bq,ln[p:]
def _b7(lines):
 ix,n,i=set(),len(lines),1
 if n<2:return ix
 bi=[(b,e,ln[p:].lstrip(_HT)) for ln in lines for b,e,_,p in (_an(ln),)]
 while i<n:
  b1,e1,sr=bi[i]
  b0,e0,hr=bi[i-1]
  if e1<=3 and _FO.match(sr) and not _CG.match(sr) and b0==b1 and e0<=3 and hr and not _CG.match(hr) and _gc(hr)==_gc(sr):
   ix.update((i-1,i))
   j=i+1
   while j<n and bi[j][0]==b1 and bi[j][2] and not (bi[j][1]<=3 and _CG.match(bi[j][2])):
    ix.add(j)
    j+=1
   i=j
   continue
  i+=1
 return ix
def _jw(pr,nxt):
 pr=pr.rstrip(_HT)
 if _at(pr,len(pr)-1):return pr[:-1]+" "+nxt
 if _rs(r"[A-Za-z][-\u2010-\u2015\u2212]$",pr) and _rm(r"^[a-z]",_hu(nxt)):return pr+nxt
 return pr+" "+nxt
_A1=_rc(rf"^(?:{_BU})$")
def _b4(raw,col=0,bq_d=0,mc0=None):
 p,n,es=0,len(raw),[]
 while True:
  rs=raw[p:]
  if p>0 and _A1.match(rs.strip(_HT)):return True,es
  if (m:=_rm(rf"({_S}*){_LM}({_S}+|$)",rs)):
   ind=_aj(m.group(1),col)-col
   if ind>3 and not (not es and mc0 is not None and mc0<=col+ind<=mc0+3):break
   mc=_aj(rs[:m.start(2)],col)
   wl,pe=_aj(m.group(2),mc)-mc,p+m.end()
   te=pe>=n or not raw[pe:].strip(_HT)
   cc=(mc+1) if (te or wl>=5) else (mc+wl)
   mt=rs[m.end(1):m.start(2)]
   es.append([bq_d,"-" if mt in "-*+" else mt[-1],cc,te])
   col,p=cc,pe
   if te or wl>=5:return True,es
   continue
  bn,eb,_,pb=_an(rs)
  if bn>0:
   if eb>=4:return True,es
   bq_d,p,col=bq_d+bn,p+pb,0
   if p>=n:return True,es
   continue
  break
 rem=raw[p:].strip(_HT)
 return (not rem or bool(_A1.match(rem) or _rm(r"^(?:#{1,6}(?:[ \t]|$)|`{3,}|~{3,})",rem))),es
def _av(text,hm=0):
 doc=_b2(text)
 if hm in (1,2):doc=_ru(r"(?i)<br[ \t]*/?>"," " if hm==1 else "",doc)
 doc=_BD.sub(lambda m:_ru(rf"({_H0})\\\n{_S}*",r"\1\\ ",_ru(rf"({_H0})%[^\n]*\n{_S}*",r"\1",m.group(0)) if m.lastindex>2 else m.group(0)),doc)
 rls=doc.split("\n")
 tr,out,fn,ihb,hbq,phb,ls=_b7(rls),[],None,False,-1,True,[]
 oa,_st,_ls=out.append,lambda s:s.strip(_HT),lambda s:s.lstrip(_HT)
 for idx,rl in enumerate(rls):
  if not (st:=_st(rl)):ls=[e for e in ls if e[0]==0 and not e[3]]
  fn,tg=_sf(rl,fn)
  if tg:
   oa(rl)
   phb,ls,ihb=True,[],False
   continue
  bqc,efc,_,psc=_an(rl)
  if ihb and (not st or bqc<hbq):ihb=False
  if not fn and not ihb and efc<=3 and _rm(r"(?i)<br(?:[\s/>]|$)",_ls(rl[psc:])):ihb,hbq=True,bqc
  ibb=bool(st) and _a8(r">[> \t]*",st) is not None
  if st and not fn and not ihb:
   while True:
    sub=rl[psc:]
    we=len(sub)-len(_ls(sub))
    wl=efc if bqc>0 else _aj(rl[:we])
    if phb and not ibb:
     while ls and (ls[-1][0]>bqc or (ls[-1][0]==bqc and wl<ls[-1][2])):ls.pop()
    if not (cd:=[i for i,e in enumerate(ls) if e[0]==bqc and e[2]<=wl]):break
    ent=ls[cd[-1]]
    if not ibb:ent[3]=False
    if wl<=ent[2]+3 and sub[we:].startswith(">"):
     rs=wl-ent[2]
     b2,e2,_,p2=_an(" "*rs+sub[we:])
     if b2>0:
      ent[3],bqc,efc,psc=False,bqc+b2,e2,psc+we+p2-rs
      continue
    break
   if ibb:ls=[e for e in ls if e[0]<=bqc and not (e[0]==bqc and e[3])]
  ibq=st.startswith(">") or bqc>0
  _lc=lambda dp,ic:next((e[2] for e in reversed(ls) if e[0]==dp and e[2]<=ic<=e[2]+3),None)
  sm,am=_rm(r"^([ \t]*)(?:=+|-+)[ \t]*$",rl),_rm(r"^([ \t]*)#{1,6}(?:[ \t]|$)",rl)
  _li=lambda m:((v:=_aj(m.group(1)))<=3 or _lc(0,v) is not None)
  isx,ia=bool(sm) and _li(sm),bool(am) and _li(am)
  ba=ibq and (_rm(r"^(?:>[ \t]*)+#{1,6}(?:[ \t]|$)",st) is not None or _AI.match(_ls(rl[psc:])) is not None)
  hb=bool(fn) or ihb or not st or ia or (idx in tr) or bool(_A1.match(st)) or isx or ibb or ba
  bm,om=_rm(r"^([ \t]*)[-*+](?:[ \t]+|$)",rl),_rm(r"^([ \t]*)(\d{1,9})([.)])(?:[ \t]+|$)",rl)
  ib_=bool(bm) and (bool(_st(rl[bm.end():])) or phb) and _li(bm)
  io_=bool(om) and ((ind:=_aj(om.group(1)))<=3 or _lc(0,ind) is not None) and (phb or (bool(_st(rl[om.end():])) and (om.group(2)=="1" or any(e[0]==0 and e[1]==om.group(3) and ind<e[2] for e in ls))))
  il=ib_ or io_
  pbq=_ba(out[-1],True)[0] if out else 0
  mb=next((e[2] for e in reversed(ls) if e[0]==bqc and e[2]<=efc),None) if bqc>0 else None
  ref=(efc-mb) if mb is not None else efc
  if ibq and bqc>=1 and out and bqc<=pbq and not phb and not hb and not il:
   ib=_ls(rl[psc:])
   if ib and (ref>=4 or (not _AI.match(ib) and not (_A1.match(ib) or _rm(r"^(?:[-*+](?:[ \t]+|$)|1[.)][ \t]+)",ib) or (_rm(r"^[> \t]*\d{1,9}[.)][ \t]+",out[-1]) and _rm(r"^\d{1,9}[.)][ \t]+",ib))))):
    out[-1],phb=_jw(out[-1],ib),False
    continue
  if not phb and not hb and not il and not ibq and out and _st(out[-1]):out[-1]=_jw(out[-1],st)
  else:oa(rl)
  lnp=False
  if il:
   bi=_aj(rl[:len(rl)-len(_ls(rl))])
   lnp,ne=_b4(rl,0,0,_lc(0,bi))
   ls=[e for e in ls if e[0]==0 and e[2]<=bi]+ne
  elif ibq and not hb:
   sa=rl[psc:]
   si=_aj(sa[:len(sa)-len(_ls(sa))])
   mb=_lc(bqc,si)
   if _EW.match(_ls(sa)) and (si<=3 or mb is not None):
    lnp,ne=_b4(sa,0,bqc,mb)
    ls=[e for e in ls if e[0]<bqc or (e[0]==bqc and e[2]<=si)]+ne
  phb=hb or lnp
 return "\n".join(_fd(_ru(r"[\r\n]+"," ",_ru(r"[\u115f\u1160\u3164\uffa0]"," ",_un(ln)))).replace("\u2018","'").replace("\u2019","'") for ln in out)
_HH=_rc(r"(?<!\\)((?:\\\\)*+)\\([`*~_])|[`*~]+|__+|(?<![A-Za-z0-9\\}])_|_(?![A-Za-z0-9{])")
_GD=_rc(rf"(?m)^(?:{_S}*+(?:>{_S}*+|{_LM}{_S}+))*+{_S}*+\[((?:\\.|[^\[\]\\\n]++)++)\]:")
def _mp(s,*,q=False):
 pm,st,i,n={},[],0,len(s)
 while i<n:
  ch=s[i]
  if ch=="\\":
   i+=2
   continue
  if q and st and st[-1][1] and i>0:
   if ch=="<" and (i==st[-1][0]+1 or (i==st[-1][0]+2 and s[i-1]==" ")):
    k=i+1
    while k<n and s[k] not in "<>\n":k+=2 if s[k]=="\\" else 1
    if k<n and s[k]==">":
     i=k+1
     continue
   elif ch in ('"',"'") and s[i-1] in _HT:
    k=i+1
    while k<n and s[k]!=ch and s[k]!="\n":k+=2 if s[k]=="\\" else 1
    if k<n and s[k]==ch:
     t=k+1
     while t<n and s[t] in _HT:t+=1
     if t<n and s[t]==")":
      pm[st.pop()[0]],i=t+1,t+1
      continue
  if ch=="(":st.append((i,i>0 and s[i-1]=="]"))
  elif ch==")" and st:pm[st.pop()[0]]=i+1
  i+=1
 return pm
def _unwrap_md_links(text,ref_defs=frozenset(),*,use_quoted=True):
 if "[" not in text or "]" not in text:return text
 if "  " in text or "\t" in text:text=_ru(r"[ \t]+"," ",text)
 hp="](" in text
 pq,pp=(_mp(text,q=True) if use_quoted else {}) if hp else {},_mp(text) if hp else {}
 mr=max((len(r) for r in ref_defs),default=0) if ref_defs else 0
 out,bk,mt,i,n,vis=[],[],set(),0,len(text),0
 oa=out.append
 def _c9():
  for e in bk:
   if not e[1]:e[3]=False
 def _az(p,img):
  out[p]=""
  if img:out[p-1]=""
  else:_c9()
 while i<n:
  ch=text[i]
  if ch=="\\" and i+1<n:
   oa(text[i:i+2])
   vis,i=vis+2,i+2
   continue
  if ch=="`":
   j=i
   while j<n and text[j]=="`":j+=1
   end=_ay(text,j,j-i,n,mt) or j
   oa(text[i:end])
   vis,i=vis+end-i,end
   continue
  if ch=="<" and (am:=_GN.match(text,i)):
   _c9()
   oa(am.group(0))
   vis,i=vis+am.end()-i,am.end()
   continue
  if ch=="[":
   bk.append([len(out),bool(out and out[-1]=="!"),vis,True])
   oa("[")
   vis,i=vis+1,i+1
   continue
  if ch=="]" and bk:
   pos,img,pv,act=bk.pop()
   if act:
    rh=lambda s:any(k in ref_defs for r in ((s,_un2(s)) if "\\" in s else (s,)) for k in (_ar(r).lower(),_a2(r).lower()))
    if i+1<n and text[i+1]=="(" and (end:=pq.get(i+1) or pp.get(i+1) or 0):
     _az(pos,img)
     i=end
     continue
    if i+1<n and text[i+1]=="[":
     k=i+2
     while k<n and text[k] not in "[]\n":k+=2 if text[k]=="\\" else 1
     if k<n and text[k]=="]" and ref_defs:
      lbl=text[i+2:k]
      if rh(lbl if lbl.strip() else "".join(out[pos+1:])):
       _az(pos,img)
       i=k+1
       continue
    if ref_defs and vis-pv<=mr+4 and rh("".join(out[pos+1:])):
     _az(pos,img)
     i+=1
     continue
  oa(ch)
  vis,i=vis+1,i+1
 return "".join(out)
_ul=_unwrap_md_links
_a2=lambda s:_ar(_HH.sub(lambda m:m.group(0) if m.group(2) else "",s))
_hm=lambda s:"$" in s or r"\(" in s or r"\[" in s
def _a6(s):
 s=_ru(rf"{_LB}\s*[_^]\s*{_LA}","",s)
 s=_ru(r"(?<=\d)\s*[_^]\s*(?=[A-Za-z]{2,})"," ",s)
 s=_ru(r"(?i)(\[\s*L)\s*[_^]\s*([123]\s*\])|\b(Level|Severity)\s*[_^]\s*([123])\b|\b(L)(?:[ \t]+[_^]|\s*\^)\s*([123])\b",lambda m:"".join(g for g in m.groups() if g),s)
 return _ru(_DA0," ",s)
def _ai(text,_ct=0,_ks=False,_a7=False):
 pe=[-1]
 def _b0(m):
  ri=m.group(m.lastindex)
  aa=_a2(_da(ri,m.lastindex>2,_ct,_ks,_a7)).replace("|"," ")
  if _rs(rf"~|{_S}+[_^]|\\(?:text|math|rm|bf|it|sf|tt|mkern|{_SP0}|[>,:; !/@\-])",ri):
   aa=_ru(rf"(?<![A-Za-z0-9#])L{_S}+([123]){_GC}",r"L\1",aa)
  pre_s=" " if (_ks and aa and (pe[0]==m.start() or (m.start()>0 and text[m.start()-1].isalnum() and aa[0].isalnum()))) else ""
  post_s=" " if (_ks and aa and m.end()<len(text) and text[m.end()].isalnum() and aa[-1].isalnum()) else ""
  pe[0]=m.end()
  return pre_s+aa+post_s
 out=_ru(r"#L\d+(?:[-–]L?\d+)?\b","",_BD.sub(_b0,text))
 return _ru(rf"{_LB}[ \t]*·+[ \t]*{_LA}","",out) if _ct==6 else out
_dc=lambda t:((True,False) if ("](" in t or ("\\" in t and "(" in t and "]" in t)) else (True,),"\\" in t,(0,1,2,3,4,5) if (any(c in t for c in "\\~·&") and _hm(t)) else (0,))
_HU=_rc(r"<!--(?:[\s\S]*?--!?>|-?>)")
_HR=_ah(r"<\?[\s\S]*?\?>|<![^>\n]*>|<(?!https?://)/?[A-Za-z][A-Za-z0-9:-]*+([\s/]++([^>\n\"']++|\"(\\.|[^\"\\\n])*+\"|'(\\.|[^'\\\n])*+')*+)?/?>")
_ao=lambda s,rh="":_HR.sub(rh,t) if "<" in (t:=_HU.sub(rh,s) if "<!--" in s and any(x in s for x in ("-->","--!>","<!-->","<!--->")) else s) else t
_F6,_GO=_rc(r"\\[^\n]"),_rc(rf"(?<!{_L})(?:(?:{_L}{{1,12}}\s+)+{_L}{{1,12}}|(?:{_L}{{1,12}}[_.])+{_L}{{1,12}}|L\s+[123](?![A-Za-z0-9])){_NL}")
_FE0=r"\.(?:md|py|rs|yml|yaml|toml|svg|js|json|sh|lean)\b"
def _dg(seg):
 oseg=seg
 seg=_ru(rf"{_L0}(?:(?:{_L}[_.]){{4,}}{_L}|(?:{_L}{{1,12}}[_.]){{2,}}{_L}{{1,12}}){_GC}",lambda m:_ru(r"[_.]"," ",m.group(0)),seg)
 seg=_ru(r"(\\[A-Za-z]+)\s+(\{)",r"\1\2",_ru(_MK0,lambda m:m.group(1)+_ru(r"\s+","",m.group(2)),_ru(r"\s*([_^])\s*",r"\1",_ru(r"\$|\\[()\[\]]"," ",seg))))
 its,cur,d,i,n=[],[],0,0,len(seg)
 while i<n:
  ch=seg[i]
  if ch=="\\" and i+1<n:
   cur.append(seg[i:i+2])
   i+=2
   continue
  if ch=="{":d+=1
  elif ch=="}" and d>0:d-=1
  elif d==0 and ch in _HT:
   if cur:
    its.append((True,"".join(cur)))
    cur.clear()
   j=i
   while j<n and seg[j] in _HT:j+=1
   its.append((False,seg[i:j]))
   i=j
   continue
  cur.append(ch)
  i+=1
 if cur:its.append((True,"".join(cur)))
 tks=[k for k,(w,_) in enumerate(its) if w]
 nt,m,_tk=len(tks),len(its),lambda j:its[tks[j]][1]
 def _x(a):
  for p in range(1,nt-1):
   if not a[p] and a[p-1] and a[p+1]:a[p]=True
  for p in range(1,nt):
   if a[p-1] and not a[p] and len(_tk(p))<=1:a[p]=True
  for p in range(nt-2,-1,-1):
   if a[p+1] and not a[p] and len(_tk(p))<=1:a[p]=True
  return a
 def _ap(base,dl,dr,ex=True):
  rs,p,ids=[],0,[0]*nt
  while p<nt:
   if base[p]:
    q=p
    while q+1<nt and base[q+1]:q+=1
    rs.append((p,q))
    p=q+1
   else:p+=1
  for ri,(a,b) in enumerate(rs,1):
   lo,hi=max(0,a-dl),min(nt-1,b+dr)
   if ex:
    while lo>0 and not ids[lo-1] and len(_tk(lo-1))<=1:lo-=1
    if ex is True or ri==len(rs):
     while hi+1<nt and len(_tk(hi+1))<=1:hi+=1
   for q in range(lo,hi+1):
    if not ids[q] or a<=q<=b:ids[q]=ri
  km,out,run,cr={tks[p]:ids[p] for p in range(nt)},[],[],0
  def _fl():
   out.append(f"${''.join(run)}$")
   run.clear()
  for k,(w,v) in enumerate(its):
   if w and km[k]:
    if cr and cr!=km[k]:_fl()
    cr=km[k]
    run.append(v)
   elif not w and run and k+1<m and its[k+1][0] and km[k+1]==cr:run.append(v)
   else:
    if run:_fl()
    cr=0
    out.append(v)
  if run:_fl()
  return "".join(out)
 m1=[bool(_F6.search(its[k][1])) for k in tks]
 m2=[m1[p] and not _DZ.fullmatch(_tk(p)) for p in range(nt)]
 ws=[its[k][1] for k in tks]
 _j2=lambda p,q,r,s=ws:f"{' '.join(s[:p])} ${' '.join(s[p:q])}${' $'+' '.join(s[q:r])+'$' if q<r else ''} {' '.join(s[r:])}".strip()
 out=[]
 if any(m1):
  b1,ds=_x(list(m1)),[(dl,dr) for dl in range(7) for dr in range(7-dl)]
  out=[_ap(b1,dl,dr) for dl,dr in ds]+[_ap(m1,0,dr,2) for dr in (0,1)]+[_ap(m1,dl,dr,False) for dl,dr in ds if dl+dr<=5]
  if any(m2) and m2!=m1:out.extend(_ap(m2,dl,dr,False) for dl,dr in ds if dl+dr<=5)
  if nt>=3:
   wl=[len(_ru(rf"\\{_L}+|\W","",ws[j])) if m1[j] else len(ws[j]) for j in range(nt)]
   _ok=lambda a,b:all(m1[j] or wl[j]<=12 for j in range(a,b)) and sum(wl[j] for j in range(a,b))<=16
   ks=[i for i in range(nt) if (m1[i] and (i==0 or i==nt-1 or not m1[i-1] or not m1[i+1])) or (i%2==0 and min(wl[max(0,i-1):min(nt,i+3)])<=8)]
   for k in ks:
    out.extend(_j2(p,q,r) for r in range(k+1,min(nt+1,k+4)) for q in range(max(1,k-5),k+1) if _ok(q,r) for p in range(max(0,q-6),q) if _ok(p,q))
    out.extend(_j2(p,q,r) for q in range(k+1,min(nt+1,k+4)) for p in range(max(0,k-5),k+1) if _ok(p,q) for r in range(q+1,min(nt+1,q+7)) if _ok(q,r))
 if nt>=4:
  i=0
  while i<nt:
   j=i
   while j<nt and not m1[j] and 1<=len(ws[j])<=3 and ws[j].isalpha():j+=1
   if j-i>=4:
    e=min(j,i+24)
    out.extend(_j2(p,b,e) for p in range(i,min(i+3,e-2)) for b in range(p+1,e+1))
   i=j+(j==i)
 if nt>=2:
  wsc=[_ru(rf"{_LB}[-_.]{_LA}","",w) if (not m1[k] and not _HQ.search(w) and not _ri(_FE0,w)) else w for k,w in enumerate(ws)]
  out.extend(_tl(wsc,[not m1[k] and 1<=len(wsc[k])<=18 and wsc[k].isalpha() for k in range(nt)],("no","not","zero","non","nor","neither","without","where","whether")))
 if any(c in oseg for c in "_.-"):
  js=_ru(rf"{_L0}(?:{_L}{{1,12}}[-_.])+{_L}{{1,12}}{_GC}",lambda m:g if (len(g:=m.group(0))>32 or _HQ.search(g) or _ri(_FE0,g)) else _ru(r"[-_.]","",g),oseg)
  if js!=oseg:out.append(js)
 if not any(c in oseg for c in "\\<>=/") and (ls_:=_ru(rf"(?i){_L0}L{_S}+([123]){_GC}",r"L\1",oseg))!=oseg:out.append(ls_)
 return _aa(out)
def _ds(s):
 ps,mt,i,n,mw,chg=[],set(),0,len(s),0,False
 pa=ps.append
 while (k:=s.find("`",i))!=-1:
  pa((False,s[i:k]))
  if _at(s,k-1):
   pa((False,"`"))
   i=k+1
   continue
  j=k
  while j<n and s[j]=="`":j+=1
  if (q:=_ay(s,j,j-k,n,mt,True)) and ((hp:=bool(_F6.search(inn:=s[j:q-(j-k)]))) or _GO.search(inn)):
   ws=_dg(inn)
   if hp or ws:
    chg,i=True,q
    vs=(f" ${inn.replace('$',' ')}$ ",*ws) if hp else tuple(ws)
    mw=max(mw,len(vs))
    pa((True,vs))
   else:
    pa((False,s[k:q]))
    i=q
  else:
   e=q or j
   pa((False,s[k:e]))
   i=e
 if not chg:return None
 pa((False,s[i:]))
 return _aa("".join(v[min(bi,len(v)-1)] if f else v for f,v in ps) for bi in range(mw+1))
_nf=lambda s:_a2(_fd(s))
_bo,_u2=lambda s:_ru(rf"{_LB}\s*[_^]\s*{_LA}"," ",s),lambda s:"_" in s or "^" in s
_ns=lambda s:_ru(r"#L\d+(?:[-–]L?\d+)?\b","",_nf(s))
def _tl(ws,ok,ng=()):
 nt,out=len(ws),[]
 _ok=lambda s,alt:(n:=len(s)) and sum(ls:=[len(w) for w in s])<=(22 if n>=4 else 28) and min(ls)<=6 and not (ls[0]>=2 and len(set(s))==1) and (not alt or max(ls)<=3) and (n<4 or max(ls)<=(2 if n>=8 else 5)) and not (ng and min(ls)>=2 and ((lw:=[w.lower() for w in s])[0] in ng or lw[-1] in ng or "where" in lw or "whether" in lw))
 for ks in (*((L,) for L in range(2,min(nt,14)+1)),(2,3),(3,4),(2,4),(4,5)):
  if nt<(pr:=sum(ks)):continue
  alt=len(ks)>1
  for r in range(pr):
   rs,i,ch,st=list(ws[:r]),r,False,0
   while i+(L:=ks[st])<=nt:
    if all(ok[k] for k in range(i,i+L)) and _ok(sl:=ws[i:i+L],alt):
     rs.append("".join(sl))
     ch=True
    else:rs.extend(ws[i:i+L])
    i,st=i+L,(st+1)%len(ks)
   if ch:out.append(" ".join((*rs,*ws[i:])))
 return _aa(out)
_GR=rf"(?:{_BP}(?<!\$)\$(?!\$)(?:[^$\\\n]|\\[^\n]){{1,96}}{_BP}\$(?!\$)|{_BP}\\\((?:[^\\\n]|\\(?!\))){{1,96}}\\\))"
_H2=_rc(rf"{_GR}(?:(?:{_S}+|(?<=\\\))|(?=\\\()){_GR})+")
_M4=((0,True),(5,False),(5,True),(6,False))
def _cp(src,_cu,_cj,has_bs,_di):
 srcs=_aa((src,_ao(src,""),_ao(src," "))) if "<" in src else [src]
 vs=[]
 va=vs.append
 def _ua(txt,hbs):
  res=[]
  for uq in _cj:
   u=_ul(txt,_cu,use_quoted=uq)
   res.append(u)
   if hbs:
    ux=_un2(u)
    for uq2 in _cj:res.append(_ul(ux,_cu,use_quoted=uq2))
  return _aa(res)
 for s in srcs:
  va(_ns(s))
  hm,hbs=_hm(s),has_bs or ("\\" in s)
  tt=(0,1,2,3,4,5) if (any(c in s for c in "\\~·&") and hm) else _di
  if hm:
   for mf in _aa([_ai(s,0,True),_ai(s,0,False),*[_ai(s,5,ks) for ks in (False,True) if 5 in tt]]):
    for u in _ua(mf,hbs):va(_ns(u))
  for u in _ua(s,hbs):
   va(_ns(u))
   if "`" in u and ("\\" in u or _GO.search(u)) and (pms:=_ds(u)):
    hu="\\" in u
    for pi,um in enumerate(pms):
     if hu and pi<2:vs.extend((_nf(_ai(um,0,True)),_nf(_ai(um,5,True,True))))
     va(_nf(_ai(um,6,False,True)))
     if hu and _u2(um):va(_bo(_nf(_ai(um,6,False,False))))
   if hm or "\\" in u:
    ef=lambda ct,ks,sc=False:_nf(_ai(u,ct,ks,sc))
    if hm and (" " in u or r"\)\(" in u or r"$\(" in u or r"\)$" in u):
     _cn=lambda m,k:(t:=m.group(m.lastindex).split()) and (f"{''.join(t[:k])} {''.join(t[k:])}" if 2<=len(t)<=24 and k<len(t) and all(w.isalnum() for w in t) else m.group(0))
     if (al_:=_ru(r"(?i)(?<![\\A-Za-z0-9])\$\s*L\s*\$\s*\$\s*([123])\s*\$",r"L\1",u))!=u:va(_nf(al_))
     saj,sts=set(),set()
     for rx_s,tk_r in ((r"\\[A-Za-z]+",r"[A-Za-z]+"),(r"\\[A-Za-z]+|[_^](?:\{[^{}\n]{1,12}\}|[A-Za-z0-9])",r"[A-Za-z]+"),(r"\\[A-Za-z]+",r"[A-Za-z0-9]+")):
      aj=_H2.sub(lambda m:"$"+" ".join(_a9(tk_r,_ru(rx_s," ",m.group(0))))+"$",u)
      if aj!=u and aj not in saj:
       saj.add(aj)
       va(_nf(_ai(aj,6,False,True)))
       mk=min(24,max((len(m.group(m.lastindex).split()) for m in _BD.finditer(aj)),default=0))
       for k in range(1,mk):
        if (sv:=_BD.sub(lambda m:_cn(m,k),aj))!=aj:va(_nf(sv))
      for hm_ in _H2.finditer(u):
       if len(ts:=tuple(_a9(tk_r,_ru(rx_s," ",hm_.group(0)))))>=3 and (ky:=(hm_.start(),ts)) not in sts:
        sts.add(ky)
        p_,q_=u[:hm_.start()],u[hm_.end():]
        for tv in _tl(ts,[w.isalnum() for w in ts]):va(_nf(f"{p_}{tv}{q_}"))
     for k in range(1,16):
      if (sv:=_BD.sub(lambda m:_cn(m,k),u))!=u:va(_nf(sv))
    va(ef(0,True))
    if _C9.search(u):
     tw=u
     for _ in range(4):tw=_ru(rf"{_TW}\s*\{{([^{{}}\n]{{1,64}})\}}",r"\1",tw)
     va(ntw:=_nf(tw))
     if _u2(ntw):vs.extend((_a6(ntw),_bo(ntw)))
    for ct in tt:va(ef(ct,False))
    if 5 in tt:vs.extend((ef(5,True),ef(6,False),ef(6,True)))
    if _u2(u):
     for ct,ks in _M4:
      ev=ef(ct,ks,False)
      vs.extend((_a6(ef(ct,ks,True)) if ks else _a6(ev),_bo(ev)))
 return _aa(vs)
_bj=lambda s,rd=frozenset():tuple(_cp(s,rd,*_dc(s)))
_BE=_asw(rf"(\b{_NB}\b|{_ZT} (verified |new |remaining )?(bl-?ock(ing|ed)|un-?parsable|st-?ructural-?revision)( ({_ZN}))?\b)")
def _sg(pat,text,_xs=None,ed=True):
 if not pat.search(text):return text
 mt=_ms(text)
 def _b0(m):
  post,aq,pm=text[m.end():],_dm(mt[:m.start()],False,masked=True),mt[m.end():]
  ps=_sb(post)
  mg=m.group(0)
  if _FP.search(text[max(0,m.start()-200):m.start()]) or _DU.search(aq) or _a4(aq,ed) or _C4.search(ps[:200]) or _CT.search(ps) or _c5(pm,ed,masked=True):return mg
  if _rs(r"\b0$",mg.strip()) and _HY.search(post):return mg
  rest=post
  while (mp:=_rm(rf"^\s*+{_CD}\s*+(?:[:=/,;\-]\s*+)?(?:\(([^)\n]+)\)|\[([^\]\n]+)\]|\{{([^}}\n]+)\}})",rest)):
   par=_ru(rf"(?i)\b(?:{_L3}\s*[:=]\s*[1-9]\d*|(?:[1-9]\d*\s+)?{_L3}(?:\s*\(\s*editorial\s*\))?)\b","",next(g for g in mp.groups() if g is not None))
   if _xs is not None:par=_sg(_xs,par,_xs,ed)
   par=_sg(_BE,par,_xs,ed)
   if _rs(rf"(?i)[1-9]|\b(?:actual\w*|real(?:ly)?|true|false|several|many|multiple|some|few|plus|more|not|non|but|however|except|yet|{_NP}|{_NW}|{_BA}|{_C2}|block(?:ing|ed)|unpars\w*|struct\w*)\b",par):return mg
   rest=rest[mp.end():]
  if rest!=post and (_C4.search((rs:=_sb(rest))[:200]) or _CT.search(rs) or _c5(mt[len(text)-len(rest):],ed,masked=True)):return mg
  return ""
 return pat.sub(_b0,text)
_dl=lambda text,ed=True:_sg(_BE,text,ed=ed)
_PD=(("code",r"^(?:(?:crates|adapters|consumers|\.cargo)/|rust-toolchain|.*Cargo\.(?:toml|lock)$)"),("gov",r"^(?:\.github/|\.agents/(?:scripts|rules|skills)/|\.pre-commit-config\.yaml|theme/|(?:AGENTS|CONTRIBUTING)\.md$|\.git(?:message|ignore|attributes)$|book\.toml$|mathjax\.js$|CITATION\.cff$|book-src/(?:SUMMARY\.md$|(?:0[78]|19|20|4[0189]|50|7[6-9]|8[0-5]|9[1-9])-))"),("math",r"^(?:formal/|\.agents/scripts/(?:check_(?:symbolic_math|graphs|notation)|verify_formal_invariants)\.py$|book-src/(?:appendix-[ab]-|(?:0[23569]|1[0-8]|2[1-68]|3\d|4[1-9]|5\d|6\d|7[05-9]|8\d|9[0-6])-))"),("src",r"^book-src/(?:appendix-[cd]-|(?:00|1[6-9]|2[05-9]|3\d|4[02-589]|6[1-9]|7[0-5]|9[7-9])-)"))
_ax=lambda p:{k for k,r in _PD if _rm(r,p)} or {"any" if _rm(r"^book-src/(?:index\.md|part-|stratum-|appendix-[efg]-|0[14]-)",p) else "gov"}
def _validate_blind_review_artifact(tpath,text,root,*,required_paths=None,out_domains=None):
 pra,prh="review ","requires single '- **"
 if _px.basename(tpath).startswith(("00-deterministic-report","triage")):return "triage/deterministic report invalid"
 if not text or not text.strip():return f"{pra}is empty"
 if _c6(text.replace("\r\n","\n")):return _A3
 sph,lvr,tgh,lref=SPECIALIST_HEADER_RE,LEONTIEF_VERDICT_RE,TARGET_FILES_HEADER_RE,r"(?::[1-9]\d*|#L[1-9]\d*)"
 _h1=lambda rx:ms[0] if len(ms:=[*rx.finditer(text)])==1 else None
 if not (spm:=_h1(sph)):return f"{prh}Specialist:**'"
 if (sp:=spm.group(1).strip().lower()) not in CANONICAL_SPECIALIST_NAMES or _is_non_human_reviewer(sp):return f"bad specialist {sp!r}"
 if not (vrm:=_h1(lvr)):return f"{prh}Leontief Verdict:**'"
 if (vd:=vrm.group(1).strip().upper()) not in ("CLEAN","EDITORIAL-ONLY"):return f"bad verdict {vd!r}"
 tb,fb,vb=([*_bz(rf"(?ims)^##\s+{h}\b[^\n]*\n(.*?)(?=^##\s+|\Z)",text)] for h in ("Audited Central Thesis","Findings","Verification"))
 if any(len(b)!=1 for b in (tb,fb,vb)):return f"{pra}missing core sections"
 if not (tb[0].start()<fb[0].start()<vb[0].start()):return f"{pra}bad section order"
 if len(text.splitlines())>450 or len(text.encode())>35000:return f"{pra}>450L/35KB"
 if not 5<=(twc:=len(tb[0].group(1).split()))<=150:return f"{pra}Thesis {twc}w"
 vext=r"(?:md|py|rs|yml|yaml|toml|svg|js|cff|json|lock|ts|mjs|kv|hbs|css|sh|lean)"
 bman=r"(?:\.gitmessage|\.gitignore|\.gitattributes|rust-toolchain|lean-toolchain|CODEOWNERS)"
 pstm=rf"(?<!{_AP})(?:{_AP}+\.{vext}|(?:{_AP}+/)?{bman})"
 ed,vt=vd!="CLEAN",_av(vb[0].group(1),1).strip()
 if len(vt.split())<15 or not _ri(_sw(rf"({pstm}{lref}|\b({_E0}|[1-9]\d* passed|sympy|cargo test|pytest|search_web|{_FT})\b)"),vt):return f"{pra}Verification lacks evidence"
 af={"any"}|({"gov"} if sp=="redteam-gov" else {"src"} if sp=="philosophy-sources" else set())
 if sp in ("formal-stat","logic-systems"):
  _ex,_py,_stq=r"(?:execut|invok)","python3?",lambda s,e="":s.strip(" `'\"()"+e)
  py0=rf"{_py}(?=\s+(?:-[cm]|[\w./-]+\.py\b))"
  _d2,_dj=rf"{py0}|ran|run|running|{_ex}(?:ed|ing)",r"while|whereas|although|though|but|however"
  _cw=rf"(?:{_PV}|{_E0}|[1-9]\d*\s*(?:passed|/\s*[1-9]\d*))"
  _bv=_nc(rf"({_cw}|{_B0}([a-z-]+\s+)?(fail(ed|ures?)|errors?))")
  _cr=_sw(r"mdbook (build|test)|node (--test|\S+\.mjs)")
  _by,_br,_bx=r"claim|report|assert|document|record|mention|suggest",r"stat|alleg|indicat|demonstrat|describ|cit|quot|summariz",r"validat|demonstrat|corroborat|substantiat"
  _c0=_asw(rf"\b({_AN}|{_NV}|no|zero|neither|nor|without|{_AQ}|{_A4}|{_DW}|(rather|other) than|instead of|in lieu of|(prior|as opposed) to|sans|short of|{_CY}|before|{_U0}|disabl{_E8}|{_SK0}|(bypass|suppress){_E3}|(avoid|exempt|prohibit|suspend|halt|abandon|eschew){_E1}|bar(s|red|ring)?|leav{_E7} out|left out|withh(old{_SI}|eld)|(ignor|elid|preclud){_E2}|(drop|sidestep)(s|ped|ping)?|forbid(s|ding)?|forbade|cancel(s|l?ed|l?ing)?|dispens{_E2} with|pass{_E3} over|set(s|ting)? aside|(declin|refus){_E2}( to)?|fore?go(es|ing|ne)?|forwent|(abstain|refrain){_E1} from|(decid{_E2}|opt{_E1}) against|rul{_E2} out|(unable|{_FV}) to|(neglect{_E1}|forg(ot(ten)?|et(s|ting)?))( to)?|unavailabl\w*|{_OW})\b")
  _ta=_nc(rf"(ran|{_CN}|needed|required|available|applicable|used|invoked|(pass|succeed|finish){_ED}|compil(e|ed)|buil(d|t))")
  _cz=_asw(rf"({_EJ}\b|\b({_NR}|{_BO}|({_NV}|cannot|[a-z]+n't) ((yet|ever|even|{_TA}) ){{0,2}}(be(en)? )?(({_TA}) )?{_ta}|(neither|none) (({_AA}|has|have) )?(be(en)? )?{_ta}|({_FV}|unable|refused) to {_ta}|{_U0}|{_OW}|{_DW}|{_CV}( |(?=,))\s*{_WA}({_FS}|{_C1})|(abort|crash|panick)ed|{_D0}|n/?a)\b)")
  _cg=_ah(rf"{_B0}{_Z2}((defect|finding|issue|error|bug|failure|warning|regression|drift|violation)s?|mismatches?|blocking|(skipp|omitt|fail|waiv)ed|untested)(/{_GG})?\b")
  _tc=_asw(rf"(\b({_BI}|{_py}( -m \S+)?|pytest|{_cr}|rustc|npm test|[A-Za-z0-9_-]+\.(py|sh|mjs))\b|{_MP})")
  _cl=r"sympy|numpy|scipy|cargo|lean|lake"
  _ev=_ah(rf"\b({_d2}|{_bv}|({_cl})\s+((confirm|verifi|prov)ed|yields?)|via\s+({_cl}))\b")
  _cv=_sw(r"(regarding|concerning|about|involving|(pertaining|related|with respect|as|in relation) to|(target|touch|referenc|mention|cit|cover|address)ing)")
  _bf=_nc(rf"(inspect|review|audit|view|examin|open|cit|scann|pars|compar|referenc|mention|list|document|target|touch|consult|claim|assert|recount|repeat|trust|accept|expect|insist|contend|maintain|suggest){_E4}|read(ing)?|(say|tell){_SI}|said|told|writ(e|es|ing)|wrote|written|hear(s|d|ing)?|see(s|n|ing)?|saw|infer(s|red|ring)?|cop{_Y4}|(not|observ|trac|brows|perus|glanc|analyz|paraphras|transcrib|assum|believ|impli|assur|argu|presum|suppos|hop|{_br}){_E2}|(discuss|address|assess|echo){_E3}|(explain|report|record|survey|cover|support|reflect|reaffirm|show){_E1}|skimm{_E8}|{_cv}")
  _b1,_pv=_ah(rf"\b({_bf})\b"),_ah(rf"\b({_bf}|(check|evaluat|confirm){_E4}|verif{_Y4}|({_bx}){_E2}|establish{_E3})\b")
  _cq,_dx=_ah(rf"\b({_dj}|after|before|when|since|because|until|unless)\b"),_ah(rf"\b(re-?ran|r[au]n|running|{_ex}(ed|ing)|via)\b")
  _bd=r"(?:previous|historical|remote|external|alleged|reported|supposed|apparent|presumab)ly|earlier|yesterday|upstream|elsewhere|cached|stale"
  _dr=_ah(rf"\b(where|who(m|se)?|that|which|(say|tell){_SI}|said|told|wrote|heard|saw|show(s|n|ing)|(confirm|verif|validat|establish|prov)ing|({_by}|expect){_E1}|({_br}){_E2}|{_bd}|third-?part{_YI}|some(one|body)|they|colleagues?)\b")
  _li0=r"independent|direct|local|fresh|clean"
  _dd=rf"(?:and|then|also|plus|we|i|having|{_H3}|to|by|after|before|when|while|(?:separate|{_li0}|explicit|final|subsequent|successful)ly|again|still|further|next|now|already|first|offline|both|re-?r[au]n)"
  _cy=_ah(rf"(^|[,;:\-\(\[\{{]|\b{_dd})\s*$")
  _bk=rf"{_dd}|{_li0}|in-?memory|custom|full|all|the|an?|our|my|symbolic|algebraic|numerical|workspace|unit|cargo|{_CR}|via"
  _be=_ah(rf"^\s*+([`'\"(\[]\s*+)*+{_py}(\s+-[A-Za-z]+)++\s+('[^']*'?|\"[^\"]*\"?|\S+)")
  _dw=_ah(rf"({_bk}|crates?|{_py}|uv|bash|sh|using|with|on|in|for|against|under|from|import|-[A-Za-z0-9-]+|[\w./-]+\.(py|sh|rs|toml|lean))")
  _ew=_ah(rf"({_bk}|{_py}|-[cm]|(verifi|confirm|check|validat|evaluat|test|execut|us|invok)ed|(us|runn|invok)ing|battery|parity|or)")
  _vw=_ah(rf"(and|then|also|we|i|({_li0})ly|(execut|verifi|confirm|check|comput|solv|deriv|evaluat|test|prov){_E8}|verifying|r[au]n|running)")
  _b3=_asw(r"^\s*(([-*+]|\d{1,3}[.)]) )*(\*\*([^*\n:]{1,64}):\*\*\s*|(commands?(\s*/\s*checks?)?( run)?|verification|core parity|rust(\s*/\s*cargo)?|layer-?0|(check|test|proof)s?|math|sympy|python|lean|cargo|parity)\s*:\s*)?")
  _b5=_asw(rf"\b((imagin|hypothesiz|fantasiz|simulat|fak){_E2}|(pretend|mock){_E1}|dream(s|ed|t|ing)?|fictional|counterfactual|{_MW}|in (theory|principle)|(theoretic(al)?|conceptual|abstract|hypothetical)ly|(?<!-)dry[-\s]?run(s|ning)?|{_GM}|{_GH}|{_DO}|if (we (r[au]n|{_ex}ed)|possible|needed|necessary|applicable|available|feasible|required|it (runs?|passes?|works?))|(suppos|assum){_E2} (we|it|that|cargo|pytest|sympy|(test|check)s?))\b")
  _bc=lambda s:_a9(r"-[A-Za-z0-9-]+|[A-Za-z0-9_]+(?:[./-][A-Za-z0-9_]+)*",s)
  _bl=lambda rx,s,mx:len(ts:=_bc(s))<=mx and all(rx.fullmatch(t) for t in ts)
  _d4=lambda tl:bool(_be.fullmatch(tl)) or ("," not in tl and _bl(_dw,tl,8))
  _c8=lambda s:_bl(_ew,_be.sub(" ",s),10)
  def _bs(seg,asb=False):
   for d in _dx.finditer(seg):
    p0,tl=seg[:d.start()],seg[d.end():]
    if _dr.search(p0) or _b5.search(_DY.sub(" ",p0)):continue
    if d.group(0).lower()=="via":
     vts=_bc(_BD.sub(" ",_b3.sub("",p0)))
     if not _stq(tl) and not _b1.search(p0) and 1<=len(vts)<=4 and all(_vw.fullmatch(t) for t in vts):return True
    elif _cy.search(p0) and _d4(tl):
     pb=_b3.sub("",p0)
     if asb and "," in pb:pb=pb.rsplit(",",1)[1]
     if not _stq(pb,",-*+") or _c8(pb):return True
   return False
  _dy=_asw(rf"(\b({_BI}|pytest|\S+\.(py|sh|mjs)|{_cr}|{_py} -m \S+)\b[^.\n]{{0,160}}?{_bv}\)?|[1-9]\d*\s*(passed|/\s*[1-9]\d*)\s*:|{_bv}(?!\s*(\([^)]*\)|\))?\s*:)[^.:\n]{{0,240}}?(,\s*| (and|plus|then) ))")
  _d3=_asw(rf"(\band\b|(,\s*| (to|which|that|thereby|thus) | )(confirm|not|observ|show|reflect|argu|cover|support|establish|{_by}|{_br}|{_bx})\w*\b)")
  _du=_ah(rf"^[`'\"]?\s*({_A2}|{_CR}|(assertion|diagnostic|invariant|benchmark|fixture|trace|metric)s?)\b")
  _tm=rf"(?:(?!(?:r[au]n|running|{_ex}\w*|via|using)\b)[^\s;!?]++\s++){{0,12}}[`'\"]?$"
  _ck=_ah(rf"\b({_cv}|whether|if|where|that|in|on|over|against|for|of|to|into|from|around|towards?|alongside|under|per|within|besides?|beyond|versus|vs\.?|unlike|among|between|before|after|during|pending|amid|across|through|thru)\s+{_tm}")
  _cd=_ah(rf"\b(with|by)\s+{_tm}")
  _nx=_asw(rf"\b({_CR}|{_A2}|(hook|rule|function|tool|program|patch|diff|commit|pr|post|thread|issue|review|audit|entr(y|ie))s?|evidence) ((that|which) (({_AA}|can|will|would|should|may|might) )?)?(r[au]n|run(s|ning)|say(s|ing)|said|shown|proven|(impl|verif){_Y4}|establish{_E3}|(call|trigger|test|check|show|confirm|attest|{_by}){_E1}|({_ex}|not|prov|{_br}|{_bx}){_E2})\s*[`'\"]?$")
  _c7=_asw(rf"\b({_bd}|according to|per (the )?(ci|{_A2}|(author|commit|pr|issue|thread)s?|upstream)|in (ci|upstream|github actions|an? ({_EP}|older|past|stale|cached) (run|commit|build|pr|session|audit|review))|on an? ({_EP}|older) (commit|branch|run|build)|(by|from) (ci|some(one|body)|others|colleagues?|the author|third[-\s]part{_YI}|upstream))\b")
  _dt=_ah(rf"({_c0.pattern}|{_cz.pattern}|{_GB}|{_EC.pattern}|{_c7.pattern}|{_b5.pattern})")
  _bb=_as(vt)
  _cb=any(_ci(c) for c in _bb)
  def _c2(rx):
   if _cb:return False
   pos=False
   for rcl in _bb:
    for m in rx.finditer(rcl):
     pre,post=rcl[:m.start()],rcl[m.end():]
     if (tm:=_tc.search(post)):
      gap,tp=post[:tm.start()],post[tm.end():]
      def _ca(o,c):
       idx=gap.rfind(o)
       inn=gap[idx+1:]+tm.group(0)+tp.split(c,1)[0]
       return gap[:idx]+tp.split(c,1)[1] if not (rx.search(inn) or _dt.search(inn)) else post
      if gap.count("(")>gap.count(")") and ")" in tp:post=_ca("(",")")
      elif "," in gap and tp.lstrip().startswith(","):post=_ca(",",",")
     if any(_c0.search(_au(v,_cg)) for v in _am(pre)) or any(_cz.search(_au(_sb(v),_cg)) for v in _am(post)):return False
     if _ri(r"\bwhether\b",pre) or _c7.search(rcd:=_DY.sub(" ",rcl)) or _b5.search(rcd):continue
     ep,ak,cuts=pre,False,[]
     if (scm:=[*_cq.finditer(pre)]):
      cuts.append((pre[:scm[-1].start()],True,True))
      ep=pre[scm[-1].end():]
     if (pd:=[x for x in _dy.finditer(ep) if not rx.search(x.group(0))]):
      cuts.append((ep[:pd[-1].start()],True,False))
      ep,ak=ep[pd[-1].end():],True
     if (ot:=[x for x in _tc.finditer(ep) if not rx.fullmatch(x.group(0)) and not _a8(rf"(?i){_py}",x.group(0))]):
      cuts.append((ep[:ot[-1].start()],True,True))
      ep,ak=ep[ot[-1].end():],True
     _bh=lambda seg:any(not _bs(seg[p0.end():]) for p0 in _b1.finditer(seg))
     _dz=lambda seg:any(not _bs(seg[p0.end():]) for p0 in _pv.finditer(seg))
     odx=_bs(ep,bool(scm) and scm[-1].group(0).lower() not in ("unless","until","before","because"))
     if scm and not odx:continue
     if _ck.search(ep) or _nx.search(ep) or (_cd.search(ep) and (ak or _ri(rf"\b{_bv}\b",ep) or not _ri(rf"\b(?:ran|run|running|{py0})\b",ep))):continue
     if not odx and any(c and ((_dz(c) if sub else _bh(c)) or (ck and (_ck.search(c) or _nx.search(c)))) for c,ck,sub in cuts):continue
     eq=post[:sm.start()] if (sm:=_cq.search(post)) else post
     if (o2:=next((x for x in _tc.finditer(eq) if not rx.fullmatch(x.group(0))),None)):eq=eq[:o2.start()]
     _bu=_b3.sub("",ep)
     if not odx and _stq(_bu,",-*+") and not _c8(_bu):continue
     if not odx and (_bh(ep) or any(not _bs(ep[p.end():]) and (_b1.fullmatch(p.group(0)) or not _cy.search(_bu[:max(0,p.start()-(len(ep)-len(_bu)))]) or bool(_stq(ep[p.end():])) or _du.match(eq) or not _rs(rf"(?i)\b{_cw}\b",_d3.split(eq,1)[0].replace("`",""))) for p in _pv.finditer(ep))):continue
     if _ev.search(f"{ep} {m.group(0)} {eq}".replace("`","")):pos=True
   return pos
  if _c2(_ah(rf"\b{_CP}\b")):af.add("code")
  if _c2(_ah(_MP)):af.add("math")
 ae=rf"(?:{_LV}[\s:=\-]*[123]|L[123]\b)(?:\s*\(\s*(?:{_BSE})\s*\))?"
 zmod,_sj=rf"(?:(?:verified|new|{_BSE}|remaining)\s+)?",r"(?:\s*(?:,\s*(?:(?:or|and) )?|/\s*| (?:or|and) ))"
 szr=_asw(rf"(\b{_ZT} {zmod}{ae}({_sj}({_ZT} {zmod})?{ae}){{0,2}}( ({_ZN}))?|\b{_LV}[\s:=\-]*[123]\s*[:=]\s*0\b)")
 cd_mod=_sw(rf"((Ch\.?|Chapter|Part|Stratum|Layer|Section|Appendix) {_AL}+ )?((verified|new|{_BSE}|remaining|open|active|critical|severe|major|unresolved|unaddressed|unfixed) ){{0,4}}")
 cdr=_asw(rf"\b({_ZT} {cd_mod}({ae}({_sj}{ae})* )?({_DN})|none found)\b")
 _tp=r"[A-Za-z0-9_./*{}?-]"
 tpr=_rc(_nc(rf"`?(?<!{_tp})({_tp}+\.{vext}|({_tp}+/)?{bman})((:|#L)[0-9,\-–#L]+)?`?"))
 _b6=lambda m,r="":"\n"*m.group(0).count("\n")+"Hdr: "+r
 bt=tgh.sub(lambda m:_b6(m,tpr.sub(" ",m.group(1))),lvr.sub(_b6,sph.sub(_b6,text)))
 if (_d1:=_d5(bt)):return _d1
 _b8=[]
 nc=_dn(bt,_b8)
 if "<!--" in nc:return f"{_DA}HTML comment {_AD}"
 if _H4.search(_H5.sub(" ",nc)):return f"{_DA}HTML tag {_AD}"
 ncu=_un(nc)
 ncm=_un2(ncu)
 if _c6(ncu):return _A3
 _a5=f"{_DA}TeX macro {_AD}"
 if any(_DM.search(s) for s in (nc,ncu,ncm)):return _a5
 _cf=lambda s:("$"+s) if (len(_a9(rf"{_BP}\$(?!\$)",s))%2 and _rm(r"^\s*[^$\n]*\\[A-Za-z]+[^$\n]*(?<!\s)\$(?=\s|$)",s)) else s
 _bm=_rc(rf"(?:{_TW}|\\(?:stackrel|overset|underset))\s*\{{")
 _spv=[cs for sp in _b8 for cs in _aa((sp,_un(sp),_ao(_un(sp),"")))]
 cs_tex=[(_cf(_ru(r"(?<=\d)\\unicode\{x25\}","",cs)),False,False) for cs in _spv if _hm(cs)]
 if any(_hto(cs,True) for cs in _spv if not _hm(cs) and _bm.search(cs)):return _a5
 for _dk,_d6,is_cm in ((nc,True,False),(ncu,False,False),(ncm,False,True),*cs_tex):
  for mm in _BD.finditer(_dk):
   ri=mm.group(mm.lastindex)
   if _DM.search(ri) or _rs(rf"(?m)\n{_S}*(?:>|#|`{{3}}|~{{3}}|{_LM}(?:{_S}|$)|(?:=+|-+){_S}*$|{_BU}|{_EX}$|\n)",ri):return _a5
   if _FJ.search(ri) and (not is_cm or "\n" in ri or mm.lastindex>2):return f"{_DA}TeX %"
   if _d6 and _HS.search(ri):return f"{_DA}HTML entity"
   if _FK.search(ri):return f"{_DA}nested math"
   if _hto(ri):return _a5
   for cm in _G0.finditer(ri):
    tok=cm.group(1)
    if not any(rx.fullmatch(tok) for rx in (_EH,_C0,_DZ,_GW)) or (tok==r"\mkern" and not _rm(r"[ \t]*[1-6]mu(?!\d)",ri[cm.end():])):return _a5
 an=lambda d,opt="":_ah(rf"\b(?:Level|Severity)\s*[:=\-]*\s*\(?\s*{opt}{d}\s*\)?{_GC}|\[\s*L\s*-?\s*{d}\s*\]|(?<![A-Za-z0-9#])L-?{d}{_GC}")
 al=lambda d:_ah(rf"\b[1-9]\d*\s+{_LV}[\s:=\-]*{d}\b")
 l12,l3,pl12,pl3=an("[12]",r"\+?\s*"),an("3"),al("[12]"),al("3")
 btr=_ah(r"\b(block(ing|ed)|unparsable|structural-?revision)\b")
 def _bt(seg):
  sc=_dl(seg,ed)
  return any(btr.search(v) for v in _am(sc))
 rds=frozenset(k for sr in (bt,_av(bt,0)) for lbl in (m.group(1).strip() for m in _GD.finditer(sr)) for rl in (lbl,_un2(lbl)) for k in (_ar(rl).lower(),_a2(rl).lower()) if k)
 _up=lambda c:_ru(r"((?:\\\\)*)\\\|",r"\1|",c)
 def _dp(_aq):
  tr,ab,buf=_co(_aq[_an(_aq)[3]:]),[],[]
  for pos,ch in enumerate(tr):
   if ch=="|" and not _at(tr,pos-1):
    ab.append(_up("".join(buf)))
    buf.clear()
   else:buf.append(ch)
  return [*ab,_up("".join(buf))]
 def _ce(_aq):
  dims,rc=_dc(_aq),[c.strip() for c in _dp(_aq)]
  cv=[_cp(c,rds,*dims) for c in rc]
  if all(_a8(r":?-+:?",c.replace(" ","")) or not c for vs in cv for c in vs):return True,[]
  mv=max((len(vs) for vs in cv),default=0)
  rows,hm=[" ".join(t for vs in cv if vs and (t:=vs[min(k,len(vs)-1)])) for k in range(mv)],_hm(_aq)
  mm=((0,True),*((ct,False) for ct in dims[2]),*((5,True),(6,False))*(5 in dims[2])) if hm else ((0,True),)
  def _cc(c,uq,ct,ks,mf,sc=False):
   s=_ul(_ai(c,ct,ks,sc) if mf else c,rds,use_quoted=uq)
   if "\\" in s:s=_ul(_un2(s),rds,use_quoted=uq)
   return _ns(_ai(s,ct,ks,sc) if (not mf and (_hm(s) or "\\" in s)) else s)
  re_=rows.extend
  re_(" ".join(t for c in rc if (t:=_cc(c,uq,ct,ks,mf))) for uq in dims[0] for ct,ks in mm for mf in ((False,True) if hm else (False,)))
  if hm and _u2(_aq):
   re_(" ".join(t for c in rc if (t:=_cc(c,True,ct,ks,False,True))) for ct,ks in _M4)
   re_([_a6(r) for r in rows]+[_bo(r) for r in rows])
  nv=[vs for vs in ([v for v in cs if v] for cs in cv) if vs]
  for u,v in zip(nv,nv[1:]):
   if len(u)>1 or len(v)>1:re_([*(f"{x} {y}" for x in u for y in (v[0],v[-1])),*(f"{x} {y}" for x in (u[0],u[-1]) for y in v)])
  rows=_aa(rows)
  _a7=ed and len(cv)>=3 and any(l3.search(r) for r in rows)
  ab=_aa(c for ci,vs in enumerate(cv) for c in vs if not (_a7 and ci==0 and _a8(r"[1-9]\d{0,2}[.)]?",c.strip())))
  col0=[vs[0] for vs in cv]
  ith=len(col0)>=2 and col0[0].lower() in ("id","finding","#") and any(k in " ".join(col0[1:4]).lower() for k in ("severity","level","tag")) and not any(any(r.search(s) for r in (pl12,pl3,l12,l3,_CB,_D9)) or _bt(s) or _a4(s,ed) or _CT.search(". "+s) or _C4.search(". "+s[:200]) for s in (*rows,*ab)) and all(len(c)<=45 and len(c.split())<=6 and "[" not in c and "$" not in c and "\\" not in c and not _rs(rf"(?:\.{vext}|{bman})(?::|\b)",c,re.I) for c in ab if c)
  return (True,[]) if ith else (False,_aa((*rows,*ab)))
 def _bi(seg,kind,*,inf=False):
  for sv in _am(seg):
   sc=_sg(szr,sv,szr,ed)
   cd_hit=bool(cdr.search(sc))
   sc=_sg(cdr,sc,szr,ed)
   if (cd_hit and bool(cdr.search(sc))) or pl12.search(sv) or l12.search(sc) or _bt(sc) or _a4(sc,ed,_if0=inf):
    return f"{_DA}L1/L2 {kind} despite {vd}"
   if vd=="CLEAN" and (pl3.search(sv) or l3.search(sc)):return f"CLEAN despite L3 {kind}"
  return None
 rca={}
 def _bg(body):
  lines=body.split("\n")
  tr,fn,wp=_b7(lines),None,[]
  def _ww(seg,idx,is_f):
   if wp and _rs(r"[.?!]$",_ms(wp[-1]+" ").rstrip()) and ("<!--" not in (wj:=" ".join(wp)) or "-->" in wj.rsplit("<!--",1)[1]):wp.clear()
   wp.append(seg)
   while len(wp)>1 and sum(len(x.split()) for x in wp)>80:wp.pop(0)
   for k in (*range(2,min(len(wp),5)+1),*(len(wp),)*(len(wp)>5)):
    j=" ".join(wp[-k:])
    for sv in (_bj(j,rds) if (_hm(j) or "<" in j) else (_ns(_ul(j,rds)),)):yield ("finding",sv,idx,is_f)
  for idx,line in enumerate(lines):
   fn,tg=_sf(line,fn)
   if tg:
    wp.clear()
    continue
   if not (st:=line.strip(_HT)):continue
   if not fn and idx in tr:
    wp.clear()
    skip,segs=rca[st] if st in rca else rca.setdefault(st,_ce(st))
    if not skip:
     for seg in segs:yield ("table row",seg,idx,False)
    continue
   kind="finding" if fn else ("heading" if (_an(line)[1]<=3 and _AI.match(_a2(st))) else "finding")
   bf=bool(fn)
   for seg in _bj(st,rds):yield (kind,seg,idx,bf)
   if not fn and (kind=="heading" or _a8(r">[> \t]*",st)):wp.clear()
   elif (pl:=st if fn else _ba(line,True)[1].strip()):yield from _ww(pl,idx,bf)
 ok_s,no_r=set(),set()
 has_vc=any(_EB.search(ln) and _ri(rf"\b(?:{_BI}|pytest|sympy|python3?|passed|exit|ran|run|running|executed|invoked)\b|{_LP}|→|->",ln) for ln in vt.splitlines())
 _c3=lambda x:(0,1,2) if _ri(r"<br[ \t]*/?>",x) else (0,1)
 _Z1="0 Level 1 or Level 2 defects"
 _hz=lambda s:bool(szr.search(s) or _BE.search(s))
 _ae=lambda pz,s,k,h:_bi(f"{(pz if (_hz(pz) and (l12.search(pz) or _bt(pz))) else _Z1).rstrip('.?!;:\u2026 ')}. {s}",k,inf=h)
 _fn_p=lambda ps:next((e for nk,ns,nh,it in ps if it and (e:=_ae(_Z1,ns,nk,nh))),None)
 for prep in dict.fromkeys(_av(bt,m) for m in _c3(bt)):
  pidx,cz1,cz3,sz1,sz3,np_,sec,cln,cfn= -1,[],[],[],[],[],"",-9,False
  for kind,seg,lidx,ah in _bg(prep):
   is_h=kind=="heading"
   chk=_ru(r"^#{1,6}\s+","",seg) if is_h else seg
   if not ah:cfn=False
   if is_h and (hm:=_rm(r"^##\s+(.+)",seg)):
    if (err:=_fn_p(np_)):return err
    sz1,sz3,cz1,cz3,np_,cln,cfn,ht=[],[],[],[],[],-9,False,hm.group(1).strip().lower()
    sec="f" if ht.startswith("findings") else "v" if ht.startswith("verification") else "t" if ht.startswith("audited central thesis") else "o"
   is_v,in_f=sec=="v",sec=="f"
   if is_v and _EB.search(chk):
    cln=lidx
    if ah:cfn=True
   if _vf(chk) or (is_v and (_de(chk) or _ci(chk) or ((ah or cfn or cln>=0 or has_vc) and _df(chk)))):return f"bad verification {kind} despite {vd}"
   sk=(kind,in_f,chk)
   if sk not in ok_s:
    if (err:=_bi(chk,kind,inf=in_f)):return err
    ok_s.add(sk)
   if lidx!=pidx:
    if cz1:sz1=_aa(sz1+cz1)[-12:]
    if cz3:sz3=_aa(sz3+cz3)[-12:]
    cz1,cz3,pidx=[],[],lidx
   if _hz(chk) or (in_f and cdr.search(chk)):
    (cz1 if (l12.search(chk) or _bt(chk) or (in_f and cdr.search(chk))) else cz3).append(chk)
    for nk,ns,nh,_ in np_:
     if (err:=_ae(chk,ns,nk,nh)):return err
    np_.clear()
   rk=(is_v,kind,chk)
   if (sz1 or sz3 or (sec in "ftv" and len(np_)<16)) and rk not in no_r:
    ac=chk if kind=="table row" else _a2(_ba(chk,True)[1])
    pvs=_am(_sb(ac))
    _apv=lambda fn:any(fn(v) for v in pvs)
    hdp,hit_t,hit_m,hit_b=_apv(lambda v:_a4(v,ed)),_apv(lambda v:_CT.search(". "+v) or _CB.search(v)),_apv(lambda v:_DI.search(v) or _D9.search(_bn(v))),_apv(lambda v:_C4.search(". "+v[:200]))
    if hit_t or hit_m or (not is_v and (hdp or hit_b)):
     if sz1 or sz3:
      for pz in (*sz1,*sz3):
       if (err:=_ae(pz,ac,kind,hdp)):return err
     elif not (_hz(chk) or cdr.search(chk)):np_.append((kind,ac,hdp,True))
    else:no_r.add(rk)
  if (err:=_fn_p(np_)):return err
 _c4=lambda ft:any(kind!="heading" and (pl3.search(s) or l3.search(_sg(szr,s,szr,ed))) for kind,s,_,_ in _bg(ft))
 fr=fb[0].group(1)
 fb2,bv=_av(fr,1),_av(_ru(r"(?m)^Hdr:[ \t]*","",bt),1)
 if vd=="CLEAN" and any(_c4(_av(fr,m)) for m in _c3(fr)):return "CLEAN despite L3"
 if vd=="EDITORIAL-ONLY" and not _c4(fb2):return "EDITORIAL-ONLY needs L3"
 if vd=="CLEAN" and not cdr.search(fb2):return "CLEAN missing non-finding"
 if len(bv.split())<80:return f"{pra}too thin (<80w)"
 cb=_av(bt.replace(f"file://{_cs(root)}",""),1)
 if _ri(rf"{pstm}(?::0\b|#L0\b)",cb):return f"{pra}bad :0/#L0"
 if not (tgm:=_h1(tgh)):return f"{prh}Target File(s):**'"
 if required_paths:
  _aw,th,tc_=[_px.normpath(rp.strip().replace("\\","/")) for rp in required_paths if rp and rp.strip()],tgm.group(1),_target_header_covers_path
  _db=lambda p:bool(_rs(rf"(?<!{_AP}){re.escape(p)}{lref}",cb))
  if out_domains is None:
   if (mc:=[p for p in _aw if not _db(p)]):return f"{pra}missing file:line: {mc}"
   if (mt:=[p for p in _aw if not tc_(th,p,root)]):return f"Target omits Tier-1: {mt}"
   if (md:={d for p in _aw for d in (_ax(p)-af)}):return f"{sp!r} lacks Tier-1 domain(s) {sorted(md)} for {_aw[:3]}"
  else:
   rp_={(p,d) for p in _aw for d in _ax(p)}
   out_domains["req"]|=rp_
   cp_={(p,d) for p in _aw if _db(p) and tc_(th,p,root) for d in (_ax(p)&af)}
   if rp_ and not cp_:return f"{sp!r} covers no Tier-1 path/domain in {_aw[:3]}"
   out_domains["cov"]|=cp_
 elif not _ri(rf"{pstm}{lref}",cb):return f"{pra}missing file:line"
 return None
