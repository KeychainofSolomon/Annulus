import Annulus.State

noncomputable section
open Classical

namespace Annulus

variable {U : Universe}

/-- §9.2: the transitions and their enabling conditions, before the audit record is
appended. `Step` below adds the record, so that every transition appends to A. -/
inductive CoreStep (M : Core U) (R : RelRite U) : State U → Label U → State U → Prop where
  | launch {σ : State U}
      (hp : σ.phase = .preLaunch)
      (hc : LaunchChecks M σ) :
      CoreStep M R σ
        (.launch (fun p =>
          if (∃ m, σ.faculty p m) then some (M.mech.fpDatum (σ.world p).1, (σ.world p).2) else none))
        { σ with
          phase := .running,
          faculty := if σ.auth.wandPresent = true then σ.faculty else (fun _ _ => False),
          reserved := if σ.auth.wandPresent = true then σ.reserved else (fun _ _ => False) }

  | openC {σ : State U} {c : U.Circle} {e : U.Exec}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .openC c e)
      (hc : ¬ Label.openC c ∈ σ.audit) :
      CoreStep M R σ (.openC c)
        { σ with
          circles := c :: σ.circles,
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | closeC {σ : State U} {c : U.Circle} {rest : List U.Circle} {e : U.Exec}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .closeC e)
      (hs : σ.circles = c :: rest) :
      CoreStep M R σ (.closeC c)
        { (terminate σ (fun _ inst => inst.circle = c) .released) with
          circles := rest,
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | invokeOk {σ : State U} {i : U.Inv} {s : U.Seal} {w : U.Worker} {ok no : U.Exec}
      {c : U.Circle} {rest : List U.Circle} {n : Nat}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .invoke i s w ok no)
      (hs : σ.circles = c :: rest)
      (hfi : σ.invs i = none)
      (hfw : σ.workers w = none)
      (ha : R.admit σ (.invoke s c) true)
      (hb : R.bound σ (.ink s) n)
      (hx : ¬ Barred σ s)
      (hn : n ≤ σ.inkLeft) :
      CoreStep M R σ (.invoke i s c w true n)
        { σ with
          invs := upd σ.invs i (some ⟨s, c, .live, n, 0, w, []⟩),
          workers := upd σ.workers w (some ⟨some (.spirit i), some i, (fun _ => False), [], true, false⟩),
          inkLeft := σ.inkLeft - n,
          exec := upd σ.exec U.operator ok,
          sched := idle σ.sched }

  | invokeRefused {σ : State U} {i : U.Inv} {s : U.Seal} {w : U.Worker} {ok no : U.Exec}
      {c : U.Circle} {rest : List U.Circle} {b : Bool} {n : Nat}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .invoke i s w ok no)
      (hs : σ.circles = c :: rest)
      (ha : R.admit σ (.invoke s c) b)
      (hb : R.bound σ (.ink s) n)
      (hr : b = false ∨ Barred σ s ∨ σ.inkLeft < n) :
      CoreStep M R σ (.invoke i s c w b n) (endRun σ (.failure U.denial σ.audit.length))

  | requestSpirit {σ : State U} {i : U.Inv} {f : U.ReqForm} {q : U.Req} {p : U.Pid}
      {w : U.Worker} {e : U.Exec} {inst : Inst U} {wi : WInfo U} {asked : Power U → Prop}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .request (.spirit i) f q p w e)
      (hinv : σ.invs i = some inst)
      (hl : inst.status = .live)
      (hc : inst.circle ∈ σ.circles)
      (hm : σ.workers inst.main = some wi)
      (hq : σ.reqs q = none)
      (hp : σ.props p = none)
      (hsig : R.signature σ f asked)
      (hcon : ∀ r, asked r → Contained σ (.spirit i) r) :
      CoreStep M R σ (.request q p (.spirit i) f inst.main asked)
        { σ with
          reqs := upd σ.reqs q (some ⟨.spirit i, f, asked, p, inst.main⟩),
          props := upd σ.props p (some ⟨inst.spiritSeal, q, .empty, .pending, [inst.spiritSeal]⟩),
          workers := upd σ.workers inst.main (some { wi with queue := wi.queue ++ [p] }),
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | requestAngel {σ : State U} {a : U.Seal} {f : U.ReqForm} {q : U.Req} {p : U.Pid}
      {w : U.Worker} {e : U.Exec} {asked : Power U → Prop}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .request (.angel a) f q p w e)
      (hfw : σ.workers w = none)
      (hq : σ.reqs q = none)
      (hp : σ.props p = none)
      (hsig : R.signature σ f asked)
      (hcon : ∀ r, asked r → Contained σ (.angel a) r) :
      CoreStep M R σ (.request q p (.angel a) f w asked)
        { σ with
          reqs := upd σ.reqs q (some ⟨.angel a, f, asked, p, w⟩),
          props := upd σ.props p (some ⟨a, q, .empty, .pending, [a]⟩),
          workers := upd σ.workers w (some ⟨some (.angel a), none, asked, [p], true, false⟩),
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | turn {σ : State U} {w : U.Worker} {o : U.Outcome} {c : Contrib U} {fin : Bool}
      {wi : WInfo U} {p : U.Pid} {rest : List U.Pid} {pi : PInfo U} {ri : ReqInfo U}
      (hw : σ.sched.current = some w)
      (hwi : σ.workers w = some wi)
      (ha : wi.alive = true)
      (hq : wi.queue = p :: rest)
      (hp : σ.props p = some pi)
      (hst : pi.status = .pending)
      (hr : σ.reqs pi.req = some ri)
      (ho : R.willOpen σ w o)
      (hk : ¬ U.killOutcome o)
      (hsim : σ.mode = .simulated → o = M.sim.will σ w ∧ (c, fin) = M.sim.turn σ w)
      (hfin : fin = true → wi.isHelper = false)
      (hc : ContribOK R σ wi ri pi c) :
      CoreStep M R σ (.turn w o c fin) (afterTurn σ w wi p rest pi c fin (.turn w o c fin))

  | kill {σ : State U} {w : U.Worker} {o : U.Outcome} {wi : WInfo U} {i : U.Inv}
      (hw : σ.sched.current = some w)
      (hwi : σ.workers w = some wi)
      (ha : wi.alive = true)
      (hpar : wi.parent = some i)
      (ho : R.willOpen σ w o)
      (hk : U.killOutcome o)
      (hsim : σ.mode = .simulated → o = M.sim.will σ w) :
      CoreStep M R σ (.kill w o) (endRun σ .killed)

  | examineBy {σ : State U} {p : U.Pid} {k : WorkOutcome → U.Exec} {pi : PInfo U}
      {e : U.Seal} {v0 cb : Verdict}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi)
      (hst : pi.status = .complete ∨ pi.status = .judged .suspensum)
      (he : R.examiner σ p (some e))
      (hadm : e ≠ pi.author ∧ ¬ e ∈ pi.carries)
      (hv : R.verdict σ p e v0)
      (hcb : R.counterbalance σ p cb) :
      CoreStep M R σ (.examine p (.examined e v0 cb) (finalVerdict M σ pi v0 cb))
        { σ with index := noteSuspicion M σ pi.author, sched := idle σ.sched }

  | examineNone {σ : State U} {p : U.Pid} {k : WorkOutcome → U.Exec} {pi : PInfo U}
      {ea : Option U.Seal}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi)
      (hst : pi.status = .complete ∨ pi.status = .judged .suspensum)
      (he : R.examiner σ p ea)
      (hno : ea = none ∨ ∃ e, ea = some e ∧ (e = pi.author ∨ e ∈ pi.carries)) :
      CoreStep M R σ (.examine p (.noExaminer ea) .hold)
        { σ with index := noteSuspicion M σ pi.author, sched := idle σ.sched }

  | judge {σ : State U} {p : U.Pid} {a : ExamAns U} {v : Verdict}
      {k : WorkOutcome → U.Exec} {pi : PInfo U}
      (hl : σ.audit.getLast? = some (.examine p a v))
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi) :
      CoreStep M R σ (.judge p v.dest) (afterJudge σ p pi v.dest k)

  | commitDone {σ : State U} {p : U.Pid} {k : WorkOutcome → U.Exec} {pi : PInfo U}
      {ri : ReqInfo U}
      (hl : σ.audit.getLast? = some (.judge p .ratum))
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi)
      (hr : σ.reqs pi.req = some ri)
      (hok : Recheck σ ri (effectsOf pi.content))
      (hhc : ∀ e, e ∈ effectsOf pi.content → ¬ U.highConsequence e.verb) :
      CoreStep M R σ (.commit p (effectsOf pi.content) .done)
        { (afterCommit M σ pi) with exec := upd σ.exec U.operator (k .ratum) }

  | commitKeyed {σ : State U} {p : U.Pid} {k : WorkOutcome → U.Exec} {pi : PInfo U}
      {ri : ReqInfo U} {n : Nat}
      (hl : σ.audit.getLast? = some (.judge p .ratum))
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi)
      (hr : σ.reqs pi.req = some ri)
      (hok : Recheck σ ri (effectsOf pi.content))
      (hhc : ∃ e, e ∈ effectsOf pi.content ∧ U.highConsequence e.verb)
      (hkey : KeyConfirmed M σ (effectsOf pi.content) n) :
      CoreStep M R σ (.commit p (effectsOf pi.content) .done)
        { (afterCommit M σ pi) with
          keys := σ.keys ++ [.spend n],
          exec := upd σ.exec U.operator (k .ratum) }

  | commitOutside {σ : State U} {p : U.Pid} {k : WorkOutcome → U.Exec} {pi : PInfo U}
      {ri : ReqInfo U}
      (hl : σ.audit.getLast? = some (.judge p .ratum))
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi)
      (hr : σ.reqs pi.req = some ri)
      (hno : ¬ Recheck σ ri (effectsOf pi.content)) :
      CoreStep M R σ (.commit p (effectsOf pi.content) .outside)
        { (setStatus σ p pi .irritum) with
          vessel := (fun s => σ.vessel s ∨ s = .content pi.content),
          workers := endHelpersOf σ p,
          exec := upd σ.exec U.operator (k .irritum) }

  | commitAwait {σ : State U} {p : U.Pid} {k : WorkOutcome → U.Exec} {pi : PInfo U}
      {ri : ReqInfo U}
      (hl : σ.audit.getLast? = some (.judge p .ratum))
      (hi : instrOf M σ = .examine p k)
      (hp : σ.props p = some pi)
      (hr : σ.reqs pi.req = some ri)
      (hok : Recheck σ ri (effectsOf pi.content))
      (hhc : ∃ e, e ∈ effectsOf pi.content ∧ U.highConsequence e.verb)
      (hnokey : ¬ ∃ n, KeyConfirmed M σ (effectsOf pi.content) n) :
      CoreStep M R σ (.commit p (effectsOf pi.content) .awaitsKey)
        { (setStatus σ p pi .suspensum) with
          keys := σ.keys ++ [.petition (M.mech.fpEffects (effectsOf pi.content))],
          exec := upd σ.exec U.operator (k .suspensum) }

  | consecrateOk {σ : State U} {v : U.Value} {k : Bool → U.Exec} {kid : U.ValidatorId}
      {val : Validator U} {d : U.Datum}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .consecrate v k)
      (hm : σ.mark v = .quarantined)
      (hk : R.validator σ v (some kid))
      (hval : σ.constitution.validators kid = some val)
      (hd : σ.data v = some d) :
      CoreStep M R σ (.consecrate v (some kid) (accepts M val d (σ.origin v)))
        { σ with
          mark := if accepts M val d (σ.origin v) = true then upd σ.mark v .consecrated else σ.mark,
          exec := upd σ.exec U.operator (k (accepts M val d (σ.origin v))),
          sched := idle σ.sched }

  | consecrateRefused {σ : State U} {v : U.Value} {k : Bool → U.Exec}
      {kid : Option U.ValidatorId}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .consecrate v k)
      (hk : R.validator σ v kid)
      (hno : ¬ ∃ kid' val d, kid = some kid' ∧ σ.mark v = .quarantined ∧
        σ.constitution.validators kid' = some val ∧ σ.data v = some d) :
      CoreStep M R σ (.consecrate v kid false)
        { σ with
          exec := upd σ.exec U.operator (k false),
          sched := idle σ.sched }

  | graveOk {σ : State U} {v : U.Value} {g : Nat} {k : Bool → U.Exec}
      {kid : Option U.ValidatorId}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .consecrateGrave v g k)
      (hk : R.validator σ v kid)
      (hmatch : GraveMatch M σ v g kid) :
      CoreStep M R σ (.consecrateGrave v g kid true)
        { σ with
          mark := upd σ.mark v .consecratedGrave,
          grave := σ.grave ++ [.spend g],
          exec := upd σ.exec U.operator (k true),
          sched := idle σ.sched }

  | graveRefused {σ : State U} {v : U.Value} {g : Nat} {k : Bool → U.Exec}
      {kid : Option U.ValidatorId}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .consecrateGrave v g k)
      (hk : R.validator σ v kid)
      (hno : ¬ GraveMatch M σ v g kid) :
      CoreStep M R σ (.consecrateGrave v g kid false)
        { σ with
          exec := upd σ.exec U.operator (k false),
          sched := idle σ.sched }

  | releaseLive {σ : State U} {i : U.Inv} {e : U.Exec} {inst : Inst U}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .release i e)
      (hinv : σ.invs i = some inst)
      (hl : inst.status = .live) :
      CoreStep M R σ (.release i false)
        { (terminate σ (fun j _ => j = i) .released) with
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | releaseNoop {σ : State U} {i : U.Inv} {e : U.Exec} {inst : Inst U}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .release i e)
      (hinv : σ.invs i = some inst)
      (hl : inst.status ≠ .live) :
      CoreStep M R σ (.release i true)
        { σ with
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | relegate {σ : State U} {i : U.Inv} {e : U.Exec} {inst : Inst U}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .relegate i e)
      (hinv : σ.invs i = some inst)
      (hl : inst.status = .live) :
      CoreStep M R σ (.relegate i)
        { (terminate σ (fun j _ => j = i) .relegated) with
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | intrude {σ : State U} {a claimed : U.Seal} {c : U.Circle} {d : U.Datum}
      (hc : c ∈ σ.circles)
      (hn : ¬ Invited σ a c) :
      CoreStep M R σ (.intrude a claimed c d)
        { (terminate σ (fun _ inst => inst.spiritSeal = a) .relegated) with
          vessel := (fun s => termVessel σ (fun _ inst => inst.spiritSeal = a) s ∨ s = .datum d),
          index := damnIfMortal a σ.index }

  | compute {σ : State U} {e : U.Exec} {ds : List U.Decision} {rs : List U.Place}
      {made : List (Made U)}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .compute e ds rs made)
      (hr : ∀ p, p ∈ rs → σ.faculty p U.read ∧ ¬ σ.reserved p U.read)
      (hmr : ∀ m, m ∈ made → ∀ p, p ∈ m.places → p ∈ rs)
      (hf : ∀ m, m ∈ made → σ.data m.v = none ∧ σ.origin m.v = none) :
      CoreStep M R σ (.step (.go ds rs))
        { (made.foldl (addMade (.step (.go ds rs))) σ) with
          exec := upd σ.exec U.operator e,
          sched := idle σ.sched }

  | halt {σ : State U}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .halt) :
      CoreStep M R σ (.step .halt) (endRun σ .normal)

  | fail {σ : State U} {k : U.FailClass}
      (hw : σ.sched.current = some U.operator)
      (hi : instrOf M σ = .fail k) :
      CoreStep M R σ (.step (.fail k)) (endRun σ (.failure k σ.audit.length))

  | deny {σ : State U}
      (hw : σ.sched.current = some U.operator)
      (hd : Denied M R σ) :
      CoreStep M R σ (.step (.fail U.denial)) (endRun σ (.failure U.denial σ.audit.length))

  | schedule {σ : State U} {w : U.Worker}
      (hn : σ.sched.current = none)
      (he : Eligible M σ w)
      (hsim : σ.mode = .simulated → w = M.sim.pick σ) :
      CoreStep M R σ (.schedule w)
        { σ with
          sched := { σ.sched with
            current := some w,
            order := σ.sched.order ++ [w],
            entropy := σ.sched.entropy + 1 } }

/-- `judge` is enabled only immediately after `examine` (§9.2). -/
def JudgeOwed (σ : State U) : Prop :=
  ∃ p a v, σ.audit.getLast? = some (Label.examine p a v)

/-- Modelling choice (see README): `commit` immediately follows a `judge` to PARADISUS. -/
def CommitOwed (σ : State U) : Prop :=
  ∃ p, σ.audit.getLast? = some (Label.judge p .ratum)

/-- §9.2: one atomic transition, with its audit record appended. -/
def Step (M : Core U) (R : RelRite U) (σ : State U) (l : Label U) (σ' : State U) : Prop :=
  ∃ σ1, CoreStep M R σ l σ1 ∧
    (l.isLaunch = false → σ.phase = .running) ∧
    (JudgeOwed σ → l.isJudge = true) ∧
    (CommitOwed σ → l.isCommit = true) ∧
    σ' = { σ1 with audit := σ.audit ++ [l] }

/-- An initial state σ₀, before `launch`. -/
def Initial (M : Core U) (prog : M.lang.Program) (σ : State U) : Prop :=
  σ.phase = .preLaunch ∧
  σ.audit = [] ∧
  σ.circles = [] ∧
  (∀ p m, σ.reserved p m → σ.faculty p m) ∧
  σ.exec U.operator = M.lang.init prog ∧
  σ.workers U.operator = some ⟨none, none, (fun _ => False), [], true, false⟩ ∧
  (∀ w, w ≠ U.operator → σ.workers w = none) ∧
  (∀ i, σ.invs i = none) ∧
  (∀ q, σ.reqs q = none) ∧
  (∀ p, σ.props p = none) ∧
  (∀ v, σ.data v = none ∧ σ.origin v = none ∧ σ.valueCircle v = none) ∧
  σ.sched.current = none ∧
  σ.sched.order = []

/-- A finite execution: states `st 0 … st len` and labels `lb 0 … lb (len-1)`. -/
structure Exn (U : Universe) where
  len : Nat
  st : Nat → State U
  lb : Nat → Label U

def Exn.labels (x : Exn U) : List (Label U) := (List.range x.len).map x.lb

def Exn.final (x : Exn U) : State U := x.st x.len

/-- A run of `prog` that was launched (`launch` succeeded). -/
def Run (M : Core U) (R : RelRite U) (prog : M.lang.Program) (x : Exn U) : Prop :=
  Initial M prog (x.st 0) ∧
  0 < x.len ∧
  (∃ f, x.lb 0 = .launch f) ∧
  ∀ k, k < x.len → Step M R (x.st k) (x.lb k) (x.st (k + 1))

def Complete (x : Exn U) : Prop := ∃ e, x.final.phase = .ended e

def Reachable (M : Core U) (R : RelRite U) (σ : State U) : Prop :=
  ∃ prog x, M.lang.WellTyped prog ∧ Run M R prog x ∧ x.final = σ

end Annulus

end
