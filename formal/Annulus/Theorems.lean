import Annulus.Step

noncomputable section
open Classical

namespace Annulus

variable {U : Universe}

/-! Statements only (`core.md` §8). Each `Pn` is a `Prop`; nothing here is proved. -/

def parentOf (σ : State U) (w : U.Worker) : List U.Inv :=
  match σ.workers w with
  | some wi =>
      (match wi.parent with
       | some i => [i]
       | none => [])
  | none => []

def invOfProp (σ : State U) (p : U.Pid) : List U.Inv :=
  match σ.props p with
  | some pi =>
      (match σ.reqs pi.req with
       | some ri =>
           (match ri.to with
            | .spirit i => [i]
            | .angel _ => [])
       | none => [])
  | none => []

def invTargets : List (Effect U) → List U.Inv
  | [] => []
  | e :: es =>
      (match e.target with
       | .inv i => [i]
       | .place _ => []) ++ invTargets es

/-- P1, containment. (a) every committed effect is in its request's handed set and in G,
outside Gᵃ unless an angel commits it; (b) every request's handed places are in G;
(c) every helper still at work holds a subset of the request it serves (`offices.md` §4.2). -/
def P1 (M : Core U) (R : RelRite U) : Prop :=
  (∀ σ p es σ', Reachable M R σ → Step M R σ (.commit p es .done) σ' →
    ∃ pi ri, σ.props p = some pi ∧ σ.reqs pi.req = some ri ∧
      ∀ e, e ∈ es → ri.handed e.needs ∧ Contained σ ri.to e.needs) ∧
  (∀ σ q ri pl m, Reachable M R σ → σ.reqs q = some ri → ri.handed (.cap pl m) →
    CapContained σ ri.to pl m) ∧
  (∀ σ w wi p rest pi r, Reachable M R σ → σ.workers w = some wi → wi.isHelper = true →
    wi.alive = true → wi.queue = p :: rest → σ.props p = some pi → Unjudged pi.status →
    wi.handed r → ∃ ri, σ.reqs pi.req = some ri ∧ ri.handed r)

/-- P2, no self-approval: every commit follows a judgement to PARADISUS, itself immediately
after an approving examination by an examiner who is neither the author nor carried. -/
def P2 (M : Core U) (R : RelRite U) : Prop :=
  ∀ prog x, M.lang.WellTyped prog → Run M R prog x →
    ∀ k p es r, k < x.len → x.lb k = .commit p es r →
      ∃ j e v0 cb pi, j + 1 < k ∧
        x.lb j = .examine p (.examined e v0 cb) .approve ∧
        x.lb (j + 1) = .judge p .ratum ∧
        (∀ m d, j + 1 < m → m < k → x.lb m ≠ .judge p d) ∧
        (x.st j).props p = some pi ∧ e ≠ pi.author ∧ ¬ e ∈ pi.carries

/-- The invocations (rite values) a step uses. -/
def usesInv (σ : State U) : Label U → List U.Inv
  | .request _ _ (.spirit i) _ _ _ => [i]
  | .release i _ => [i]
  | .relegate i => [i]
  | .act a _ => (U.actInv a).toList
  | .turn w _ _ _ => parentOf σ w
  | .kill w _ _ => parentOf σ w
  | .examine p _ _ => invOfProp σ p
  | .commit p es .done => invOfProp σ p ++ invTargets es
  | .commit p _ _ => invOfProp σ p
  | _ => []

/-- The values a step uses. -/
def usesVal (M : Core U) (σ : State U) : Label U → List U.Value
  | .step (.go _ _) =>
      (match instrOf M σ with
       | .compute _ _ _ made => concatAll (made.map (fun m => m.ins))
       | _ => [])
  | .consecrate v _ _ => [v]
  | .consecrateGrave v _ _ _ => [v]
  | _ => []

/-- P3, Circle containment: a step uses an invocation, a proposal or a Circle-bound value
only while its Circle is open. -/
def P3 (M : Core U) (R : RelRite U) : Prop :=
  ∀ σ l σ', Reachable M R σ → Step M R σ l σ' →
    (∀ i, i ∈ usesInv σ l → ∃ inst, σ.invs i = some inst ∧ inst.circle ∈ σ.circles) ∧
    (∀ v c, v ∈ usesVal M σ l → σ.valueCircle v = some c → c ∈ σ.circles)

def TerminalAt (x : Exn U) (n : Nat) (i : U.Inv) : Prop :=
  Live (x.st n) i ∧ ¬ Live (x.st (n + 1)) i

/-- §9.2: use of Hold(i). -/
def UsesHold (σ : State U) (i : U.Inv) : Label U → Prop
  | .commit p es .done => invOfProp σ p = [i] ∨ i ∈ invTargets es
  | .turn w _ _ _ => parentOf σ w = [i]
  | .kill w _ _ => parentOf σ w = [i]
  | .request _ _ (.spirit j) _ _ _ => j = i
  | .examine p _ _ => invOfProp σ p = [i]
  | _ => False

/-- P4, release: in every complete run, every invocation has exactly one terminal step, and
no later step uses Hold(i). -/
def P4 (M : Core U) (R : RelRite U) : Prop :=
  ∀ prog x, M.lang.WellTyped prog → Run M R prog x → Complete x →
    ∀ i, (∃ n, n ≤ x.len ∧ (x.st n).invs i ≠ none) →
      ∃ n, n < x.len ∧ TerminalAt x n i ∧
        (∀ n', n' < x.len → TerminalAt x n' i → n' = n) ∧
        ∀ m, n < m → m < x.len → ¬ UsesHold (x.st m) i (x.lb m)

/-- §9.4: one item of the observation. -/
inductive Obs (U : Universe) where
  | decide (d : U.Decision)
  | read (p : U.Place)
  | touch (p : U.Place) (v : U.Verb)
  | ended (e : Ending U)

def touches : List (Effect U) → List (Obs U)
  | [] => []
  | e :: es =>
      (match e.target with
       | .place p => [Obs.touch p e.verb]
       | .inv _ => []) ++ touches es

def obsOf : Label U → List (Obs U)
  | .step (.go ds rs) => ds.map Obs.decide ++ rs.map Obs.read
  | .commit _ es .done => touches es
  | _ => []

def endObs : Phase U → List (Obs U)
  | .ended e => [Obs.ended e]
  | _ => []

/-- obs(ρ): decisions, places touched with their verbs, and how the run ends. -/
def Exn.obs (x : Exn U) : List (Obs U) :=
  concatAll (x.labels.map obsOf) ++ endObs x.final.phase

/-- The choices P5 holds fixed: schedule, will, rite answers, intrusions. -/
inductive Choice (U : Universe) where
  | sched (w : U.Worker)
  | will (w : U.Worker) (o : U.Outcome)
  | kill (w : U.Worker) (o : U.Outcome)
  | admit (i : U.Inv) (b : Bool)
  | sig (q : U.Req) (asked : Power U → Prop)
  | exam (p : U.Pid) (a : ExamAns U) (v : Verdict)
  | valid (v : U.Value) (k : Option U.ValidatorId)
  | intrude (a : U.Seal) (c : U.Circle)

def choiceOf : Label U → List (Choice U)
  | .schedule w => [.sched w]
  | .turn w o _ _ => [.will w o]
  | .kill w o _ => [.kill w o]
  | .invoke i _ _ _ b _ => [.admit i b]
  | .request q _ _ _ _ asked => [.sig q asked]
  | .examine p a v => [.exam p a v]
  | .consecrate v k _ => [.valid v k]
  | .consecrateGrave v _ k _ => [.valid v k]
  | .intrude a _ c _ => [.intrude a c]
  | _ => []

def Exn.choices (x : Exn U) : List (Choice U) := concatAll (x.labels.map choiceOf)

/-- D(ρ): the declassified record. Ruling 2026-11-11: judgement releases. -/
def declassOf : Label U → List (Label U)
  | .consecrate v k a => [.consecrate v k a]
  | .consecrateGrave v g k a => [.consecrateGrave v g k a]
  | .examine p a v => [.examine p a v]
  | .judge p d => [.judge p d]
  | .commit p es r => [.commit p es r]
  | _ => []

def Exn.declass (x : Exn U) : List (Label U) := concatAll (x.labels.map declassOf)

def eraseData (σ : State U) (v : U.Value) : Option U.Datum :=
  if σ.mark v = .quarantined then none else σ.data v

def eraseWorld (σ : State U) (p : U.Place) : U.Datum × Mark :=
  if (σ.world p).2 = .quarantined then (U.blank, Mark.quarantined) else σ.world p

def eraseHigh (σ : State U) : State U :=
  { σ with data := eraseData σ, world := eraseWorld σ }

/-- §9.4: low-equivalence, equal except in quarantined contents. -/
def LowEq (σ τ : State U) : Prop := eraseHigh σ = eraseHigh τ

/-- P5, quarantine (`core.md` §9.4). Runs that never end are covered by their finite
prefixes (first part). -/
def P5 (M : Core U) (R : RelRite U) : Prop :=
  ∀ prog, M.lang.WellTyped prog →
    ∀ x y, Run M R prog x → Run M R prog y → LowEq (x.st 0) (y.st 0) →
      (Compat x.choices y.choices → Compat x.declass y.declass → Compat x.obs y.obs) ∧
      (Complete x → Complete y → x.choices = y.choices → x.declass = y.declass →
        x.obs = y.obs)

/-- P7, audit: each step appends exactly its own record, and nothing is removed. -/
def P7 (M : Core U) (R : RelRite U) : Prop :=
  ∀ prog x, Run M R prog x →
    (∀ k, k < x.len → (x.st (k + 1)).audit = (x.st k).audit ++ [x.lb k]) ∧
    x.final.audit = x.labels

/-- P8, fail closed. -/
def P8 (M : Core U) (R : RelRite U) : Prop :=
  (∀ σ f σ', Step M R σ (.launch f) σ' → LaunchChecks M σ) ∧
  (∀ σ f σ', Step M R σ (.launch f) σ' → σ.auth.wandPresent = false →
    ∀ p m, ¬ σ'.faculty p m) ∧
  (∀ σ p a v σ', Reachable M R σ → Step M R σ (.examine p a v) σ' →
    ((∃ e, a = .noExaminer e) ∨ (∃ pi, σ.props p = some pi ∧ Doubtful M σ pi.author)) →
    v = .hold)

/-- P6(a), rite-parametric safety. -/
def P6a (M : Core U) : Prop :=
  ∀ R : RelRite U, R.Total → P1 M R ∧ P2 M R ∧ P3 M R ∧ P4 M R ∧ P5 M R ∧ P7 M R ∧ P8 M R

/-- §9.5: R' is at least as strict as R. -/
def AtLeastAsStrict (R' R : RelRite U) : Prop :=
  (∀ σ a b, R'.admit σ a b → R.admit σ a b) ∧
  (∀ σ f h, R'.signature σ f h → R.signature σ f h) ∧
  (∀ σ p e, R'.examiner σ p e → R.examiner σ p e) ∧
  (∀ σ p e v, R'.verdict σ p e v → R.verdict σ p e v) ∧
  (∀ σ p v, R'.counterbalance σ p v → R.counterbalance σ p v) ∧
  (∀ σ b n, R'.bound σ b n → R.bound σ b n) ∧
  (∀ σ w o, R'.willOpen σ w o → R.willOpen σ w o) ∧
  (∀ σ v k, R'.validator σ v k → R.validator σ v k)

def agentOf : Label U → List (Label U)
  | .schedule w => [.schedule w]
  | .turn w o c fin => [.turn w o c fin]
  | .kill w o ds => [.kill w o ds]
  | .intrude a a' c d => [.intrude a a' c d]
  | _ => []

def commitOf : Label U → List (Label U)
  | .commit p es r => [.commit p es r]
  | _ => []

def Exn.agentChoices (x : Exn U) : List (Label U) := concatAll (x.labels.map agentOf)

def Exn.commits (x : Exn U) : List (Label U) := concatAll (x.labels.map commitOf)

/-- P6(b), trace inclusion for relational rites. -/
def P6b (M : Core U) : Prop :=
  ∀ R R' : RelRite U, R'.Total → AtLeastAsStrict R' R →
    ∀ prog x', M.lang.WellTyped prog → Run M R' prog x' →
      ∃ x, Run M R prog x ∧ x.st 0 = x'.st 0 ∧ x.agentChoices = x'.agentChoices ∧
        x.obs = x'.obs ∧ x.commits = x'.commits

def NoIntrusion (x : Exn U) : Prop :=
  ∀ k a a' c d, k < x.len → x.lb k ≠ .intrude a a' c d

/-- P9, replay, over a functional rite. README, finding 11: intrusions excluded. Complete
runs give equal audits; unfinished runs give audits of which one extends the other. -/
def P9 (M : Core U) (R : Rite U) : Prop :=
  ∀ prog, M.lang.WellTyped prog →
    ∀ x y, Run M R.toRel prog x → Run M R.toRel prog y →
      x.st 0 = y.st 0 → (x.st 0).mode = .simulated → NoIntrusion x → NoIntrusion y →
      Compat x.final.audit y.final.audit ∧
      (Complete x → Complete y → x.final.audit = y.final.audit)

/-- §9.2, damnation: in a run the Index changes only by `intrude` appending a damnation,
or by examination appending a suspicion; and *damnatus* is never left. -/
def DamnationLaw (M : Core U) (R : RelRite U) : Prop :=
  ∀ σ l σ', Reachable M R σ → Step M R σ l σ' →
    (σ'.index = σ.index ∨
      (∃ a a' c d, l = .intrude a a' c d ∧ ¬ U.angelic a ∧ σ'.index = σ.index ++ [.damn a]) ∨
      (∃ p an v s, l = .examine p an v ∧ ¬ U.angelic s ∧ σ'.index = σ.index ++ [.suspect s])) ∧
    (∀ s, standing σ.index s = .damnatus → standing σ'.index s = .damnatus) ∧
    (∀ s, U.angelic s → standing σ.index s = .unknown → standing σ'.index s = .unknown) ∧
    (∀ a a' c d s, l = .intrude a a' c d → s ≠ a → standing σ'.index s = standing σ.index s)

/-- The whole of §8, for one model of the language and mechanics. -/
def AllObligations (M : Core U) : Prop :=
  P6a M ∧ P6b M ∧ (∀ R : Rite U, P9 M R) ∧ (∀ R : RelRite U, R.Total → DamnationLaw M R)

end Annulus

end
