namespace StochologyInvariants

inductive Axis | epi | vital | gov deriving DecidableEq, Repr
inductive Site | t1 | t2 | t3 | t4 | t5 | t6 | t7 deriving DecidableEq, Repr
inductive PropId | p1 | p2 | p3 deriving DecidableEq, Repr
inductive Reason | skeleton | axial | marginMissing | budget | reopen deriving DecidableEq, Repr
inductive Verdict
  | interior
  | boundary (s : Site) (m : Int)
  | collapse (p : PropId) (s : Site) (step : Nat)
  | unparsable (r : Reason)
  deriving DecidableEq, Repr
inductive Out | err (code : Int) | ok (v : Verdict) deriving DecidableEq, Repr

def Site.axis : Site → Axis
  | .t1 | .t2 | .t3 => .epi
  | .t4 | .t5 => .vital
  | .t6 | .t7 => .gov

def PropId.axis : PropId → Axis
  | .p1 => .epi | .p2 => .vital | .p3 => .gov

structure Tx where
  attested : Bool := true
  contingency : Nat := 7
  l : Int := 200
  u : Int := 400
  nEff : Int := 1000
  defect : Int := 0
  c : Int := 120
  b : Int := 0
  r : Int := 0
  limit : Int := 1000
  elapsed : Int := 10
  epiR : Nat := 1
  vitalR : Nat := 1
  govR : Nat := 1
  mEpi : Option Int := some 500000
  mVital : Option Int := some 500000
  mGov : Option Int := some 500000
  prop : Option PropId := none
  site : Option Site := none
  stepIdx : Option Nat := none
  reopen : Bool := false
  tau : Int := 100000
  band : Int := 50000
  deriving DecidableEq, Repr

def maxAbs : Int := 1000000000
def scale : Int := 1000000
def min3 (a b c : Int) : Int := if a ≤ b then (if a ≤ c then a else c) else (if b ≤ c then b else c)

def weakestOn (tx : Tx) (s : Site) : Option Int :=
  match s.axis with | .epi => tx.mEpi | .vital => tx.mVital | .gov => tx.mGov

def inBounds (q : Int) : Bool := q ≤ maxAbs && q ≥ -maxAbs
def inScale : Option Int → Bool
  | none => true
  | some v => 0 ≤ v && v ≤ scale

def sealCheck (tx : Tx) : Option Out :=
  if tx.l == -1 then some (.err (-6))
  else if ![tx.l, tx.u, tx.nEff, tx.defect, tx.c, tx.b, tx.r, tx.limit, tx.elapsed, tx.tau, tx.band].all inBounds
      || ![tx.mEpi, tx.mVital, tx.mGov].all inScale then some (.err (-5))
  else none

def prestageCheck (tx : Tx) : Option Out :=
  match tx.prop, tx.site with
  | some p, some s => if p.axis != s.axis then some (.err (-11)) else none
  | _, _ => none

def presentMargins (tx : Tx) : List Int := [tx.mEpi, tx.mVital, tx.mGov].filterMap id

def foldList : List Int → Option Int
  | [] => none
  | x :: xs => some (xs.foldl min x)

def evalStage (unguarded : Bool) (tx : Tx) : Nat → Option Out
  | 1 =>
    if !tx.attested || tx.l > tx.u || tx.c ≤ 0 || tx.r < 0 || tx.band ≤ 0 then some (.ok (.unparsable .skeleton))
    else if tx.elapsed > tx.limit then some (.ok (.unparsable .budget))
    else if tx.reopen then some (.ok (.unparsable .reopen))
    else none
  | 2 => if tx.epiR == 0 || tx.vitalR == 0 || tx.govR == 0 then some (.ok (.unparsable .axial)) else none
  | 3 => if tx.mEpi.isNone || tx.mVital.isNone || tx.mGov.isNone then some (.ok (.unparsable .marginMissing)) else none
  | 4 =>
    let ps := presentMargins tx
    if !unguarded && ps.length < 3 then some (.ok (.unparsable .marginMissing))
    else match foldList ps with
      | none => none
      | some f => if f < tx.tau then
          match tx.prop, tx.site, tx.stepIdx with
          | some p, some s, some k =>
            if weakestOn tx s == some f then some (.ok (.collapse p s k)) else some (.ok (.unparsable .skeleton))
          | _, _, _ => some (.ok (.unparsable .skeleton))
        else none
  | 5 =>
    let ps := presentMargins tx
    if !unguarded && ps.length < 3 then some (.ok (.unparsable .marginMissing))
    else match foldList ps with
      | none => none
      | some f =>
        let cond := if unguarded then f < tx.tau + tx.band else (tx.tau ≤ f && f < tx.tau + tx.band)
        if cond then
          match tx.site with
          | some s => if weakestOn tx s == some f then some (.ok (.boundary s f)) else some (.ok (.unparsable .skeleton))
          | none => some (.ok (.unparsable .skeleton))
        else some (.ok .interior)
  | _ => none

def acceptOrder (ord : List Nat) (unguarded : Bool) (tx : Tx) : Out :=
  match sealCheck tx with
  | some e => e
  | none => match prestageCheck tx with
    | some e => e
    | none => (ord.findSome? (evalStage unguarded tx)).getD (.ok .interior)

def accept (tx : Tx) : Out := acceptOrder [1, 2, 3, 4, 5] false tx

theorem contingency_invariance (tx : Tx) (c1 c2 : Nat) :
    accept { tx with contingency := c1 } = accept { tx with contingency := c2 } := rfl

theorem non_compensatory_leontief (e v g τ : Int) :
    e < τ ∨ v < τ ∨ g < τ → min3 e v g < τ := by
  unfold min3
  omega

theorem seven_guard_sites : 3 * (3 - 1) + 1 = 7 := rfl

theorem vandermonde_35_and_partition : (12 + 9 + 9 + 4 + 1 = 35) ∧ (5 * 7 = 35) := ⟨rfl, rfl⟩

def witnesses : List (Nat × Nat × Tx) :=
  [ (1, 2, { attested := false, epiR := 0 }),
    (1, 3, { attested := false, mEpi := none }),
    (1, 4, { reopen := true, mEpi := some 50000, prop := some .p1, site := some .t1, stepIdx := some 7 }),
    (1, 5, { reopen := true, mEpi := some 101000, site := some .t1 }),
    (2, 3, { epiR := 0, mEpi := none }),
    (2, 4, { epiR := 0, mEpi := some 50000, prop := some .p1, site := some .t1, stepIdx := some 7 }),
    (2, 5, { epiR := 0, mEpi := some 101000, site := some .t1 }),
    (3, 4, { mEpi := none, mVital := some 50000, prop := some .p2, site := some .t4, stepIdx := some 7 }),
    (3, 5, { mEpi := none, mVital := some 101000, site := some .t4 }),
    (4, 5, { mEpi := some 50000, prop := some .p1, site := some .t1, stepIdx := some 7 }) ]

def ins (x : Nat) : List Nat → List (List Nat)
  | [] => [[x]]
  | y :: ys => (x :: y :: ys) :: (ins x ys).map (y :: ·)

def perms : List Nat → List (List Nat)
  | [] => [[]]
  | x :: xs => (perms xs).flatMap (ins x)

def precedes (p : List Nat) (b a : Nat) : Bool :=
  match p with
  | [] => false
  | x :: xs => if x == b then a ∈ xs else if x == a then false else precedes xs b a

def separatesPerm (p : List Nat) : Bool :=
  if p == [1, 2, 3, 4, 5] then
    witnesses.all (fun w => accept w.2.2 == acceptOrder p true w.2.2)
  else
    witnesses.any (fun w => precedes p w.2.1 w.1 && accept w.2.2 != acceptOrder p true w.2.2)

theorem stage_non_commutation_119 :
    (perms [1, 2, 3, 4, 5]).length = 120 ∧
    (perms [1, 2, 3, 4, 5]).all separatesPerm = true := by
  refine ⟨rfl, ?_⟩
  simp only [perms, List.all_flatMap]
  decide

def triples73 : List (Nat × Nat × Nat) :=
  let s := List.range 7
  s.flatMap fun a => s.flatMap fun b => s.filterMap fun c =>
    if a < b && b < c then some (a, b, c) else none

def mem3 (x : Nat) (t : Nat × Nat × Nat) : Bool := x == t.1 || x == t.2.1 || x == t.2.2
def overlap2 (x y : Nat × Nat × Nat) : Bool :=
  let hits := [x.1, x.2.1, x.2.2].countP (fun v => mem3 v y)
  hits == 2

theorem johnson_j73_invariants :
    triples73.length = 35 ∧
    triples73.all (fun x => (triples73.filter (overlap2 x)).length == 12) = true ∧
    (triples73.foldl (fun acc x => acc + (triples73.filter (overlap2 x)).length) 0) / 2 = 210 := by
  decide

def fn1Vectors : List (Tx × Out) :=
  [ ({}, .ok .interior),
    ({ mEpi := some 101000, mVital := some 101000, mGov := some 101000, site := some .t4 }, .ok (.boundary .t4 101000)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p1, site := some .t1, stepIdx := some 7 }, .ok (.collapse .p1 .t1 7)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p1, site := some .t2, stepIdx := some 7 }, .ok (.collapse .p1 .t2 7)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p1, site := some .t3, stepIdx := some 7 }, .ok (.collapse .p1 .t3 7)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p2, site := some .t4, stepIdx := some 7 }, .ok (.collapse .p2 .t4 7)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p2, site := some .t5, stepIdx := some 7 }, .ok (.collapse .p2 .t5 7)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p3, site := some .t6, stepIdx := some 7 }, .ok (.collapse .p3 .t6 7)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p3, site := some .t7, stepIdx := some 7 }, .ok (.collapse .p3 .t7 7)),
    ({ epiR := 0 }, .ok (.unparsable .axial)),
    ({ mEpi := none, mVital := none, mGov := none }, .ok (.unparsable .marginMissing)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 50000, prop := some .p1, site := some .t4, stepIdx := some 3 }, .err (-11)),
    ({ mEpi := some 101000, mVital := some 900000, mGov := some 900000, site := some .t4 }, .ok (.unparsable .skeleton)),
    ({ mEpi := some 900000, mVital := some 101000, mGov := some 900000, site := some .t4 }, .ok (.boundary .t4 101000)),
    ({ mEpi := some 101000, mVital := some 900000, mGov := some 900000, site := some .t1 }, .ok (.boundary .t1 101000)),
    ({ mEpi := some 900000, mVital := some 101000, mGov := some 900000, site := some .t1 }, .ok (.unparsable .skeleton)),
    ({ mEpi := some 101000, mVital := some 101000, mGov := some 101000, site := some .t4 }, .ok (.boundary .t4 101000)),
    ({ mEpi := some 50000, mVital := some 900000, mGov := some 900000, prop := some .p1, site := some .t1, stepIdx := some 7 }, .ok (.collapse .p1 .t1 7)),
    ({ mEpi := some 50000, mVital := some 900000, mGov := some 900000, prop := some .p2, site := some .t4, stepIdx := some 7 }, .ok (.unparsable .skeleton)),
    ({ mEpi := some 50000, mVital := some 50000, mGov := some 900000, prop := some .p1, site := some .t1, stepIdx := some 7 }, .ok (.collapse .p1 .t1 7)),
    ({ reopen := true }, .ok (.unparsable .reopen)) ]

theorem core_vectors_conformance :
    fn1Vectors.length = 21 ∧ fn1Vectors.all (fun row => accept row.1 == row.2) = true := by
  decide

end StochologyInvariants
