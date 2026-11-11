import Annulus.Basic

noncomputable section
open Classical

namespace Annulus

variable {U : Universe}

/-- §9.1: an invocation. -/
structure Inst (U : Universe) where
  spiritSeal : U.Seal
  circle : U.Circle
  status : InvStatus
  escalation : Nat
  main : U.Worker
  helpers : List U.Worker

/-- §9.1, `concurrency.md`: a worker. The Operator's worker has no owner. -/
structure WInfo (U : Universe) where
  owner : Option (Addressee U)
  parent : Option U.Inv
  handed : Power U → Prop
  queue : List U.Pid
  alive : Bool
  isHelper : Bool

/-- A request, with its handed set H(q). -/
structure ReqInfo (U : Universe) where
  to : Addressee U
  form : U.ReqForm
  handed : Power U → Prop
  prop : U.Pid
  worker : U.Worker

structure PInfo (U : Universe) where
  author : U.Seal
  req : U.Req
  content : Content U
  status : PStatus
  carries : List U.Seal

/-- §9.3a: a validator entry, with its work limit as fuel. -/
structure Validator (U : Universe) where
  code : U.Code
  fuel : Nat

structure Constitution (U : Universe) where
  version : Nat
  fingerprint : U.Hash
  validators : U.ValidatorId → Option (Validator U)
  manifest : U.ValidatorId → Option U.Hash

/-- §9.1: the wand and authentication, constant after `launch`. -/
structure Auth (U : Universe) where
  wandFp : U.Hash
  wandPresent : Bool
  wandLock : WandLock
  revoked : List U.Hash
  facultySig : U.Hash
  ownerFp : U.Hash
  manifestSig : U.Hash
  indexRoot : U.Hash
  graveRoot : U.Hash
  keysRoot : U.Hash

structure Sched (U : Universe) where
  current : Option U.Worker
  entropy : Nat
  clock : Nat
  order : List U.Worker

/-- §9.1: the complete state σ of a run. -/
structure State (U : Universe) where
  phase : Phase U
  mode : Mode
  params : U.Param
  constitution : Constitution U
  auth : Auth U
  faculty : U.Place → U.Perm → Prop
  reserved : U.Place → U.Perm → Prop
  inkLeft : U.InkClass → Nat
  inscribed : U.Seal → U.Circle → Prop
  circles : List U.Circle
  circleRite : U.Circle → U.CircleRite
  exec : U.Worker → U.Exec
  workers : U.Worker → Option (WInfo U)
  invs : U.Inv → Option (Inst U)
  reqs : U.Req → Option (ReqInfo U)
  props : U.Pid → Option (PInfo U)
  world : U.Place → U.Datum × Mark
  data : U.Value → Option U.Datum
  mark : U.Value → Mark
  valueCircle : U.Value → Option U.Circle
  origin : U.Value → Option (Label U)
  audit : List (Label U)
  index : List (IxEntry U)
  vessel : Sealed U → Prop
  grave : List (GraveEntry U)
  keys : List (KeyEntry U)
  sched : Sched U

/-- The simulated will and scheduler (`dynamic.md` §5.6, `concurrency.md` §4). -/
structure Sim (U : Universe) where
  pick : State U → U.Worker
  will : State U → U.Worker → U.Outcome
  turn : State U → U.Worker → Contrib U × Bool

structure Core (U : Universe) where
  lang : Lang U
  mech : Mech U
  sim : Sim U

/-- §9.5: a relational rite answers each question of §3.1 with any of a set of answers. -/
structure RelRite (U : Universe) where
  admit : State U → Act U → Bool → Prop
  signature : State U → U.ReqForm → (Power U → Prop) → Prop
  examiner : State U → U.Pid → Option U.Seal → Prop
  verdict : State U → U.Pid → U.Seal → Verdict → Prop
  counterbalance : State U → U.Pid → Verdict → Prop
  bound : State U → BoundQ U → Nat → Prop
  willOpen : State U → U.Worker → U.Outcome → Prop
  validator : State U → U.Value → Option U.ValidatorId → Prop

/-- §9.5: a rite oracle, one answer per question. -/
structure Rite (U : Universe) where
  admit : State U → Act U → Bool
  signature : State U → U.ReqForm → (Power U → Prop)
  examiner : State U → U.Pid → Option U.Seal
  verdict : State U → U.Pid → U.Seal → Verdict
  counterbalance : State U → U.Pid → Verdict
  bound : State U → BoundQ U → Nat
  willOpen : State U → U.Worker → U.Outcome → Prop
  validator : State U → U.Value → Option U.ValidatorId

def Rite.toRel (R : Rite U) : RelRite U where
  admit σ a b := b = R.admit σ a
  signature σ f h := h = R.signature σ f
  examiner σ p e := e = R.examiner σ p
  verdict σ p e v := v = R.verdict σ p e
  counterbalance σ p v := v = R.counterbalance σ p
  bound σ b n := n = R.bound σ b
  willOpen := R.willOpen
  validator σ v k := k = R.validator σ v

def idle (s : Sched U) : Sched U := { s with current := none }

/-- The program sees only the places it may read. -/
def visibleWorld (σ : State U) (p : U.Place) : U.Datum × Mark :=
  if σ.faculty p U.read ∧ ¬ σ.reserved p U.read then σ.world p else (U.blank, Mark.clean)

def instrOf (M : Core U) (σ : State U) : Instr U :=
  M.lang.next (σ.exec U.operator) σ.data σ.mark (visibleWorld σ)

def Suspected (σ : State U) (s : U.Seal) : Prop := standing σ.index s = .suspected

/-- Ruling 2026-11-11: suspicion is entered in the Index, at examination. -/
def NewlySuspect (M : Core U) (σ : State U) (s : U.Seal) : Prop :=
  ¬ U.angelic s ∧ standing σ.index s = .unknown ∧ M.mech.resembles σ.index s σ.audit

def noteSuspicion (M : Core U) (σ : State U) (s : U.Seal) : List (IxEntry U) :=
  if NewlySuspect M σ s then σ.index ++ [.suspect s] else σ.index

def Barred (σ : State U) (s : U.Seal) : Prop :=
  ¬ U.angelic s ∧ (standing σ.index s = .proscribed ∨ standing σ.index s = .damnatus)

def Live (σ : State U) (i : U.Inv) : Prop :=
  ∃ inst, σ.invs i = some inst ∧ inst.status = .live

/-- `dynamic.md` §3.1: the record that ends a run names every invocation it dismisses, each once. -/
def Dismissed (σ : State U) (ds : List U.Inv) : Prop :=
  ds.Pairwise (· ≠ ·) ∧ ∀ i, i ∈ ds ↔ Live σ i

/-- §9.2, `authority.md` §6.2a: invited in Circle c. -/
def Invited (σ : State U) (a : U.Seal) (c : U.Circle) : Prop :=
  ∃ i inst, σ.invs i = some inst ∧ inst.spiritSeal = a ∧ inst.circle = c ∧ inst.status = .live

/-- C3, C5: what a request may hand, by addressee. -/
def Contained (σ : State U) : Addressee U → Power U → Prop
  | .spirit _, .cap p m => σ.faculty p m ∧ ¬ σ.reserved p m
  | .spirit i, .onInv j _ =>
      j ≠ i ∧ ∃ inst inst', σ.invs i = some inst ∧ σ.invs j = some inst' ∧
        inst'.circle = inst.circle ∧ inst'.status = .live
  | .angel _, .cap p m => σ.faculty p m
  | .angel _, .onInv _ _ => False

def CapContained (σ : State U) : Addressee U → U.Place → U.Perm → Prop
  | .spirit _, p, m => σ.faculty p m ∧ ¬ σ.reserved p m
  | .angel _, p, m => σ.faculty p m

/-- §3.3: the core's own re-check at commit. A verb that creates may only create, a verb
that writes may only write, and a verb that removes may only remove: `creare` needs a place
that does not yet exist, `scribere` and `delere` one that does (`offices.md` §1.3; review and
the technicality filter, 2026-11-11). -/
def Recheck (σ : State U) (ri : ReqInfo U) (es : List (Effect U)) : Prop :=
  (∀ i, ri.to = .spirit i → Live σ i) ∧
  (∀ e, e ∈ es → ri.handed e.needs ∧ Contained σ ri.to e.needs) ∧
  (∀ e p, e ∈ es → U.creates e.verb → e.target = .place p → (σ.world p).1 = U.blank) ∧
  (∀ e p, e ∈ es → U.writes e.verb → e.target = .place p → (σ.world p).1 ≠ U.blank) ∧
  ∀ e p, e ∈ es → U.removes e.verb → e.target = .place p → (σ.world p).1 ≠ U.blank

def applyEffects (M : Core U) :
    List (Effect U) → (U.Place → U.Datum × Mark) → (U.Place → U.Datum × Mark)
  | [], wd => wd
  | e :: es, wd =>
      applyEffects M es
        (match e.target with
         | .place p => upd wd p (M.mech.apply e (wd p).1, Mark.quarantined)
         | .inv _ => wd)

def accepts (M : Core U) (val : Validator U) (d : U.Datum) (o : Option (Label U)) : Bool :=
  match M.mech.runValidator val.code d o val.fuel with
  | some true => true
  | _ => false

/-- §9.6 P8: doubt. -/
def Doubtful (M : Core U) (σ : State U) (s : U.Seal) : Prop :=
  ¬ U.angelic s ∧ (Suspected σ s ∨ Barred σ s ∨ NewlySuspect M σ s)

/-- Ruling 2026-11-11: angels are in Heaven by nature; the Index holds nothing against them. -/
def damnIfMortal (s : U.Seal) (X : List (IxEntry U)) : List (IxEntry U) :=
  if U.angelic s then X else X ++ [.damn s]

def finalVerdict (M : Core U) (σ : State U) (pi : PInfo U) (v0 cb : Verdict) : Verdict :=
  if Doubtful M σ pi.author then .hold else v0.meet cb

def madeMark (σ : State U) (m : Made U) : Mark :=
  joinAll (m.ins.map σ.mark ++ m.places.map (fun p => (σ.world p).2) ++
    (match m.places with
     | [] => []
     | _ => [Mark.outside]))

/-- `static.md` §4.7: a rite value, or one computed from one, is bound to its Circle. -/
def madeCircle (σ : State U) (m : Made U) : Option U.Circle :=
  if m.rite = true then σ.circles.head? else firstSome (m.ins.map σ.valueCircle)

def addMade (l : Label U) (σ : State U) (m : Made U) : State U :=
  { σ with
    data := upd σ.data m.v (some m.d),
    mark := upd σ.mark m.v (madeMark σ m),
    valueCircle := upd σ.valueCircle m.v (madeCircle σ m),
    origin := upd σ.origin m.v (some l) }

/-- Proposals belonging to invocations that satisfy P. -/
def OfInv (σ : State U) (P : U.Inv → Inst U → Prop) (pi : PInfo U) : Prop :=
  ∃ i inst ri, σ.reqs pi.req = some ri ∧ ri.to = .spirit i ∧ σ.invs i = some inst ∧
    inst.status = .live ∧ P i inst

def termInvs (σ : State U) (P : U.Inv → Inst U → Prop) (st : InvStatus) (i : U.Inv) :
    Option (Inst U) :=
  match σ.invs i with
  | some inst =>
      if inst.status = .live ∧ P i inst then some { inst with status := st } else some inst
  | none => none

def termWorkers (σ : State U) (P : U.Inv → Inst U → Prop) (w : U.Worker) : Option (WInfo U) :=
  match σ.workers w with
  | some wi =>
      if (∃ i inst, wi.parent = some i ∧ σ.invs i = some inst ∧ inst.status = .live ∧ P i inst)
      then some { wi with alive := false } else some wi
  | none => none

def termProps (σ : State U) (P : U.Inv → Inst U → Prop) (p : U.Pid) : Option (PInfo U) :=
  match σ.props p with
  | some pi =>
      if Lingers pi.status ∧ OfInv σ P pi then some { pi with status := .abandoned } else some pi
  | none => none

def termVessel (σ : State U) (P : U.Inv → Inst U → Prop) (s : Sealed U) : Prop :=
  σ.vessel s ∨ ∃ p pi, σ.props p = some pi ∧ Lingers pi.status ∧ OfInv σ P pi ∧
    s = .content pi.content

/-- §9.2: the terminal step of every live invocation satisfying P. -/
def terminate (σ : State U) (P : U.Inv → Inst U → Prop) (st : InvStatus) : State U :=
  { σ with
    invs := termInvs σ P st,
    workers := termWorkers σ P,
    props := termProps σ P,
    vessel := termVessel σ P }

def IsAngelWork (σ : State U) (pi : PInfo U) : Prop :=
  ∃ ri a, σ.reqs pi.req = some ri ∧ ri.to = .angel a

def abandonAngelProps (σ : State U) (p : U.Pid) : Option (PInfo U) :=
  match σ.props p with
  | some pi =>
      if Lingers pi.status ∧ IsAngelWork σ pi then some { pi with status := .abandoned } else some pi
  | none => none

def abandonAngelVessel (σ : State U) (s : Sealed U) : Prop :=
  σ.vessel s ∨ ∃ p pi, σ.props p = some pi ∧ Lingers pi.status ∧ IsAngelWork σ pi ∧
    s = .content pi.content

/-- §9.2: the end of a run forces every terminal step, and abandons angels' unjudged work. -/
def endRun (σ : State U) (e : Ending U) : State U :=
  let τ := terminate σ (fun _ _ => True) .released
  { τ with
    props := abandonAngelProps τ,
    vessel := abandonAngelVessel τ,
    phase := .ended e,
    sched := idle τ.sched }

def addHelperTo (σ : State U) (w' : U.Worker) (wi : WInfo U) (i : U.Inv) : Option (Inst U) :=
  match σ.invs i with
  | some inst =>
      if wi.parent = some i then some { inst with helpers := w' :: inst.helpers } else some inst
  | none => none

def spawnHelper (σ : State U) (w' : U.Worker) (h : Power U → Prop) (wi : WInfo U) (p : U.Pid) :
    State U :=
  { σ with
    workers := upd σ.workers w' (some ⟨wi.owner, wi.parent, h, [p], true, true⟩),
    invs := addHelperTo σ w' wi }

def ContribOK (R : RelRite U) (σ : State U) (wi : WInfo U) (ri : ReqInfo U) (pi : PInfo U) :
    Contrib U → Prop
  | .respond v _ => pi.content = .empty ∧ σ.data v = none ∧ σ.origin v = none
  | .effects es =>
      (pi.content = .empty ∨ ∃ es', pi.content = .work es') ∧
      (wi.isHelper = true → ∀ e, e ∈ es → wi.handed e.needs)
  | .spawn w' h n =>
      σ.workers w' = none ∧
      (∃ i inst, wi.parent = some i ∧ σ.invs i = some inst ∧ R.bound σ (.legions i) n ∧
        inst.helpers.length < n) ∧
      (∀ r, h r → ri.handed r ∧ (wi.isHelper = true → wi.handed r))
  | .idle => True

def turnContent (pi : PInfo U) : Contrib U → Content U
  | .respond v d => .response v d
  | .effects es =>
      (match pi.content with
       | .work es0 => .work (es0 ++ es)
       | _ => .work es)
  | _ => pi.content

def afterTurn (σ : State U) (w : U.Worker) (wi : WInfo U) (p : U.Pid) (rest : List U.Pid)
    (pi : PInfo U) (c : Contrib U) (fin : Bool) (l : Label U) : State U :=
  let pi' : PInfo U :=
    { pi with content := turnContent pi c, status := if fin = true then .complete else pi.status }
  let wi' : WInfo U := if fin = true then { wi with queue := rest } else wi
  let σa : State U :=
    { σ with
      props := upd σ.props p (some pi'),
      workers := upd σ.workers w (some wi'),
      sched := { σ.sched with current := none, entropy := σ.sched.entropy + 1 } }
  match c with
  | .respond v _ => { σa with mark := upd σa.mark v .quarantined, origin := upd σa.origin v (some l) }
  | .spawn w' h _ => spawnHelper σa w' h wi' p
  | _ => σa

def endHelpersOf (σ : State U) (p : U.Pid) (w : U.Worker) : Option (WInfo U) :=
  match σ.workers w with
  | some wi =>
      if wi.isHelper = true ∧ wi.queue = [p] then some { wi with alive := false } else some wi
  | none => none

def afterJudge (σ : State U) (p : U.Pid) (pi : PInfo U) (d : WorkOutcome)
    (k : WorkOutcome → U.Exec) : State U :=
  let σa : State U :=
    { σ with
      props := upd σ.props p (some { pi with status := .judged d }),
      exec := upd σ.exec U.operator (k d) }
  if d = .ratum then
    { σ with props := upd σ.props p (some { pi with status := .judged d }) }
  else if d = .irritum then
    { σa with
      vessel := (fun s => σa.vessel s ∨ s = .content pi.content),
      workers := endHelpersOf σa p }
  else σa

def committedData (σ : State U) (pi : PInfo U) : U.Value → Option U.Datum :=
  match pi.content with
  | .response v d => upd σ.data v (some d)
  | _ => σ.data

def afterCommit (M : Core U) (σ : State U) (pi : PInfo U) : State U :=
  { σ with
    world := applyEffects M (effectsOf pi.content) σ.world,
    data := committedData σ pi }

def setStatus (σ : State U) (p : U.Pid) (pi : PInfo U) (d : WorkOutcome) : State U :=
  { σ with props := upd σ.props p (some { pi with status := .judged d }) }

/-- Ruling 2026-11-11: a confirmed, unspent sealed petition for exactly this work. -/
def KeyConfirmed (M : Core U) (σ : State U) (es : List (Effect U)) (n : Nat) : Prop :=
  σ.keys[n]? = some (.confirmed (M.mech.fpEffects es)) ∧ ¬ (KeyEntry.spend n) ∈ σ.keys

/-- A request the core must deny. -/
def RequestBad (R : RelRite U) (σ : State U) (f : U.ReqForm) (w : U.Worker) :
    Addressee U → Prop
  | .spirit i =>
      ¬ (∃ inst, σ.invs i = some inst ∧ inst.status = .live ∧ inst.circle ∈ σ.circles) ∨
      ∃ asked, R.signature σ f asked ∧ ∃ r, asked r ∧ ¬ Contained σ (.spirit i) r
  | .angel a =>
      ¬ U.angelic a ∨ σ.workers w ≠ none ∨
      ∃ asked, R.signature σ f asked ∧ ∃ r, asked r ∧ ¬ Contained σ (.angel a) r

/-- `dynamic.md` §3, §6.2: what the program asks for and cannot have is a denial (class 2). -/
def Denied (M : Core U) (R : RelRite U) (σ : State U) : Prop :=
  match instrOf M σ with
  | .openC c _ => Label.openC c ∈ σ.audit
  | .closeC _ => σ.circles = []
  | .invoke i _ w _ _ => σ.circles = [] ∨ σ.invs i ≠ none ∨ σ.workers w ≠ none
  | .inscribe s k _ => σ.circles = [] ∨ σ.inkLeft k = 0 ∨ R.admit σ (.inscribe s k) false
  | .request to f q p w _ => σ.reqs q ≠ none ∨ σ.props p ≠ none ∨ RequestBad R σ f w to
  | .examine p _ =>
      ∀ pi, σ.props p = some pi →
        ¬ (pi.status = .pending ∨ pi.status = .complete ∨ pi.status = .judged .suspensum)
  | .release i _ => σ.invs i = none
  | .relegate i _ => ¬ Live σ i
  | .compute _ _ rs made =>
      (∃ p, p ∈ rs ∧ ¬ (σ.faculty p U.read ∧ ¬ σ.reserved p U.read)) ∨
      (∃ m, m ∈ made ∧ (σ.data m.v ≠ none ∨ σ.origin m.v ≠ none ∨ ∃ p, p ∈ m.places ∧ ¬ p ∈ rs))
  | _ => False

/-- §9.5: a relational rite gives at least one answer to every question. -/
def RelRite.Total (R : RelRite U) : Prop :=
  (∀ σ a, ∃ b, R.admit σ a b) ∧
  (∀ σ f, ∃ h, R.signature σ f h) ∧
  (∀ σ p, ∃ e, R.examiner σ p e) ∧
  (∀ σ p e, ∃ v, R.verdict σ p e v) ∧
  (∀ σ p, ∃ v, R.counterbalance σ p v) ∧
  (∀ σ b, ∃ n, R.bound σ b n) ∧
  (∀ σ w, ∃ o, R.willOpen σ w o) ∧
  (∀ σ v, ∃ k, R.validator σ v k)

def Blocked (M : Core U) (σ : State U) : Prop :=
  ∃ p k pi, instrOf M σ = .examine p k ∧ σ.props p = some pi ∧ pi.status = .pending

/-- `concurrency.md` §1.3: who may be scheduled. -/
def Eligible (M : Core U) (σ : State U) (w : U.Worker) : Prop :=
  (w = U.operator ∧ ¬ Blocked M σ) ∨
  (w ≠ U.operator ∧ ∃ wi p rest pi, σ.workers w = some wi ∧ wi.alive = true ∧
    wi.queue = p :: rest ∧ σ.props p = some pi ∧ pi.status = .pending)

/-- §9.1, §9.3a, §9.3c, `proscription.md` §3.2: what `launch` checks. -/
def LaunchChecks (M : Core U) (σ : State U) : Prop :=
  M.mech.verify σ.auth.ownerFp (M.mech.digestManifest σ.constitution.manifest) σ.auth.manifestSig ∧
  ¬ σ.auth.ownerFp ∈ σ.auth.revoked ∧
  (∀ k val, σ.constitution.validators k = some val →
    σ.constitution.manifest k = some (M.mech.fpCode val.code)) ∧
  M.mech.chainIx σ.index = σ.auth.indexRoot ∧
  M.mech.chainGrave σ.grave = σ.auth.graveRoot ∧
  M.mech.chainKeys σ.keys = σ.auth.keysRoot ∧
  σ.auth.wandLock ≠ .master ∧
  (σ.auth.wandPresent = true → ¬ σ.auth.wandFp ∈ σ.auth.revoked ∧
    M.mech.verify σ.auth.wandFp (M.mech.digestFaculty σ.faculty σ.reserved) σ.auth.facultySig)

def GraveMatch (M : Core U) (σ : State U) (v : U.Value) (g : Nat) (kid : Option U.ValidatorId) :
    Prop :=
  ∃ k val d fv fo fk,
    kid = some k ∧ σ.constitution.validators k = some val ∧ σ.data v = some d ∧
    (σ.mark v = .quarantined ∨ σ.mark v = .consecrated) ∧
    σ.grave[g]? = some (.grant fv fo fk) ∧ ¬ (GraveEntry.spend g) ∈ σ.grave ∧
    fv = M.mech.fpDatum d ∧ fo = M.mech.fpRecord (σ.origin v) ∧ fk = M.mech.fpCode val.code

end Annulus

end
