noncomputable section
open Classical

namespace Annulus

/-- The carriers the model leaves abstract, and the core mechanics that are fixed
data rather than rite (`core.md` §1: the effect verbs and the permissions they need). -/
structure Universe where
  Place : Type
  Perm : Type
  Verb : Type
  InvRight : Type
  Seal : Type
  Value : Type
  Inv : Type
  Req : Type
  Pid : Type
  Worker : Type
  Circle : Type
  ValidatorId : Type
  Hash : Type
  Code : Type
  Exec : Type
  Datum : Type
  Outcome : Type
  FailClass : Type
  Param : Type
  ReqForm : Type
  CircleRite : Type
  Decision : Type
  InkClass : Type
  RiteAct : Type
  actInv : RiteAct → Option Inv
  actInk : RiteAct → Option InkClass
  needs : Verb → Perm
  invNeeds : Verb → InvRight
  read : Perm
  operator : Worker
  blank : Datum
  killOutcome : Outcome → Prop
  angelOpen : Outcome → Prop
  declarable : Outcome → Prop
  highConsequence : Verb → Prop
  creates : Verb → Prop
  writes : Verb → Prop
  removes : Verb → Prop
  denial : FailClass
  angelic : Seal → Prop

def upd {α β : Type} (f : α → β) (a : α) (b : β) : α → β :=
  fun x => if x = a then b else f x

def IsPre {α : Type} (a b : List α) : Prop := ∃ t, a ++ t = b

def Compat {α : Type} (a b : List α) : Prop := IsPre a b ∨ IsPre b a

def firstSome {α : Type} : List (Option α) → Option α
  | [] => none
  | some a :: _ => some a
  | none :: rest => firstSome rest

def concatAll {α : Type} : List (List α) → List α
  | [] => []
  | l :: ls => l ++ concatAll ls

/-- §9.3: ordered from least to most restricted. -/
inductive Mark where
  | clean
  | outside
  | consecratedGrave
  | consecrated
  | quarantined
  deriving DecidableEq, Repr

def Mark.rank : Mark → Nat
  | .clean => 0
  | .outside => 1
  | .consecratedGrave => 2
  | .consecrated => 3
  | .quarantined => 4

def Mark.join (a b : Mark) : Mark := if a.rank ≤ b.rank then b else a

def joinAll : List Mark → Mark
  | [] => .clean
  | a :: as => a.join (joinAll as)

inductive Verdict where
  | approve
  | hold
  | reject
  deriving DecidableEq, Repr

def Verdict.rank : Verdict → Nat
  | .approve => 2
  | .hold => 1
  | .reject => 0

/-- §3.1: counterbalances can only lower a verdict. -/
def Verdict.meet (a b : Verdict) : Verdict := if a.rank ≤ b.rank then a else b

inductive WorkOutcome where
  | ratum
  | suspensum
  | irritum
  deriving DecidableEq, Repr

def Verdict.dest : Verdict → WorkOutcome
  | .approve => .ratum
  | .hold => .suspensum
  | .reject => .irritum

/-- §9.2, a proposal's life. -/
inductive PStatus where
  | pending
  | complete
  | judged (d : WorkOutcome)
  | abandoned
  deriving DecidableEq, Repr

def Unjudged : PStatus → Prop
  | .pending => True
  | .complete => True
  | _ => False

/-- Ruling 2026-11-11: held work does not outlive its invocation. -/
def Lingers : PStatus → Prop
  | .pending => True
  | .complete => True
  | .judged .suspensum => True
  | _ => False

/-- §9.2, an invocation's life. -/
inductive InvStatus where
  | live
  | released
  | relegated
  deriving DecidableEq, Repr

inductive Mode where
  | simulated
  | real
  deriving DecidableEq, Repr

inductive WandLock where
  | locked
  | master
  | rite
  deriving DecidableEq, Repr

/-- §9.2, an agent's standing across runs. -/
inductive Standing where
  | unknown
  | suspected
  | proscribed
  | damnatus
  deriving DecidableEq, Repr

variable {U : Universe}

/-- `core.md` §9.2: what the record knows of a turn's outcome. In simulated mode the
scheduler drew it, so it is known; in real mode it is only the outside will's own
declaration, and the outcome itself (a lie, malice) is not known. -/
inductive Will (U : Universe) where
  | known (o : U.Outcome)
  | declared (o : U.Outcome)

def Will.outcome : Will U → U.Outcome
  | .known o => o
  | .declared o => o

def Will.of (m : Mode) (o : U.Outcome) : Will U :=
  match m with
  | .simulated => .known o
  | .real => .declared o

/-- `offices.md` §3.1: a handed set holds (place, permission) pairs and invocation rights. -/
inductive Power (U : Universe) where
  | cap (p : U.Place) (m : U.Perm)
  | onInv (i : U.Inv) (r : U.InvRight)

inductive Target (U : Universe) where
  | place (p : U.Place)
  | inv (i : U.Inv)

structure Effect (U : Universe) where
  verb : U.Verb
  target : Target U

def Effect.needs (e : Effect U) : Power U :=
  match e.target with
  | .place p => .cap p (U.needs e.verb)
  | .inv i => .onInv i (U.invNeeds e.verb)

inductive Addressee (U : Universe) where
  | spirit (i : U.Inv)
  | angel (a : U.Seal)

/-- `offices.md` §1.4: a response or a work, never both. -/
inductive Content (U : Universe) where
  | empty
  | response (v : U.Value) (d : U.Datum)
  | work (es : List (Effect U))

def effectsOf : Content U → List (Effect U)
  | .work es => es
  | _ => []

inductive Contrib (U : Universe) where
  | respond (v : U.Value) (d : U.Datum)
  | effects (es : List (Effect U))
  | spawn (w : U.Worker) (h : Power U → Prop) (legions : Nat)
  | idle

inductive ExamAns (U : Universe) where
  | noExaminer (e : Option U.Seal)
  | examined (e : U.Seal) (v0 cb : Verdict)

inductive CommitResult where
  | done
  | outside
  | awaitsKey
  deriving DecidableEq, Repr

inductive StepOut (U : Universe) where
  | go (ds : List U.Decision) (reads : List U.Place)
  | halt (dismissed : List U.Inv)
  | fail (k : U.FailClass) (dismissed : List U.Inv)

/-- §9.4: how a run ends. -/
inductive Ending (U : Universe) where
  | normal
  | failure (k : U.FailClass) (n : Nat)
  | killed

inductive Phase (U : Universe) where
  | preLaunch
  | running
  | ended (e : Ending U)

structure Made (U : Universe) where
  v : U.Value
  d : U.Datum
  ins : List U.Value
  places : List U.Place
  rite : Bool

/-- What the Operator's worker does next, as its program decides. -/
inductive Instr (U : Universe) where
  | compute (e : U.Exec) (ds : List U.Decision) (reads : List U.Place) (made : List (Made U))
  | openC (c : U.Circle) (e : U.Exec)
  | closeC (e : U.Exec)
  | inscribe (s : U.Seal) (k : U.InkClass) (e : U.Exec)
  | act (a : U.RiteAct) (e : U.Exec)
  | invoke (i : U.Inv) (s : U.Seal) (w : U.Worker) (ok no : U.Exec)
  | request (to : Addressee U) (f : U.ReqForm) (q : U.Req) (p : U.Pid) (w : U.Worker) (e : U.Exec)
  | examine (p : U.Pid) (k : WorkOutcome → U.Exec)
  | consecrate (v : U.Value) (k : Bool → U.Exec)
  | consecrateGrave (v : U.Value) (g : Nat) (k : Bool → U.Exec)
  | release (i : U.Inv) (e : U.Exec)
  | relegate (i : U.Inv) (e : U.Exec)
  | halt
  | fail (k : U.FailClass)

/-- §9.2: the transitions. Each label is also the audit record of its step. -/
inductive Label (U : Universe) where
  | launch (fp : U.Place → Option (U.Hash × Mark))
  | openC (c : U.Circle)
  | closeC (c : U.Circle)
  | inscribe (s : U.Seal) (k : U.InkClass) (c : U.Circle) (admitted : Bool)
      (dismissed : List U.Inv)
  | act (a : U.RiteAct) (c : U.Circle) (admitted : Bool) (dismissed : List U.Inv)
  | invoke (i : U.Inv) (s : U.Seal) (c : U.Circle) (w : U.Worker) (admitted : Bool)
      (dismissed : List U.Inv)
  | request (q : U.Req) (p : U.Pid) (to : Addressee U) (f : U.ReqForm) (w : U.Worker)
      (asked : Power U → Prop)
  | turn (w : U.Worker) (v : Will U) (c : Contrib U) (finish : Bool)
  | kill (w : U.Worker) (o : U.Outcome) (dismissed : List U.Inv)
  | examine (p : U.Pid) (a : ExamAns U) (v : Verdict)
  | judge (p : U.Pid) (d : WorkOutcome)
  | commit (p : U.Pid) (es : List (Effect U)) (r : CommitResult)
  | consecrate (v : U.Value) (k : Option U.ValidatorId) (accepted : Bool)
  | consecrateGrave (v : U.Value) (g : Nat) (k : Option U.ValidatorId) (accepted : Bool)
  | release (i : U.Inv) (noop : Bool)
  | relegate (i : U.Inv)
  | intrude (a : U.Seal) (claimed : U.Seal) (c : U.Circle) (sent : U.Datum)
  | step (out : StepOut U)
  | schedule (w : U.Worker)

def Label.isLaunch : Label U → Bool
  | .launch _ => true
  | _ => false

def Label.isJudge : Label U → Bool
  | .judge _ _ => true
  | _ => false

def Label.isCommit : Label U → Bool
  | .commit _ _ _ => true
  | _ => false

/-- `proscription.md` §3: the Index, append-only. -/
inductive IxEntry (U : Universe) where
  | damn (s : U.Seal)
  | proscribe (s : U.Seal)
  | lift (s : U.Seal)
  | suspect (s : U.Seal)
  | clear (s : U.Seal)

/-- Ruling 2026-11-11: sealed petitions for the second key, append-only. -/
inductive KeyEntry (U : Universe) where
  | petition (fp : U.Hash)
  | confirmed (fp : U.Hash)
  | spend (n : Nat)

/-- §9.3c: the store of grave grants, append-only. -/
inductive GraveEntry (U : Universe) where
  | grant (fv fo fk : U.Hash)
  | spend (g : Nat)

inductive Sealed (U : Universe) where
  | content (c : Content U)
  | datum (d : U.Datum)

inductive Act (U : Universe) where
  | invoke (s : U.Seal) (c : U.Circle)
  | inscribe (s : U.Seal) (k : U.InkClass)
  | rite (a : U.RiteAct)

inductive BoundQ (U : Universe) where
  | legions (i : U.Inv)

/-- The program's semantics, left abstract. `WellTyped` stands for `static.md`. -/
structure Lang (U : Universe) where
  Program : Type
  WellTyped : Program → Prop
  init : Program → U.Exec
  next : U.Exec → (U.Value → Option U.Datum) → (U.Value → Mark) →
    (U.Place → U.Datum × Mark) → Instr U

/-- Fingerprints, hash chains, the validator interpreter and effect application. -/
structure Mech (U : Universe) where
  fpDatum : U.Datum → U.Hash
  fpRecord : Option (Label U) → U.Hash
  fpCode : U.Code → U.Hash
  chainIx : List (IxEntry U) → U.Hash
  chainGrave : List (GraveEntry U) → U.Hash
  chainKeys : List (KeyEntry U) → U.Hash
  fpEffects : List (Effect U) → U.Hash
  verify : U.Hash → U.Hash → U.Hash → Prop
  /-- Fourth outside review: the wand signs the faculty together with the store roots, and
  the owner signs the manifest together with the revocations. -/
  digestFaculty : (U.Place → U.Perm → Prop) → (U.Place → U.Perm → Prop) →
    U.Hash → U.Hash → U.Hash → U.Hash
  digestManifest : (U.ValidatorId → Option U.Hash) → List U.Hash → U.Hash
  resembles : List (IxEntry U) → U.Seal → List (Label U) → Prop
  runValidator : U.Code → U.Datum → Option (Label U) → Nat → Option Bool
  apply : Effect U → U.Datum → U.Datum

def standingStep (s : U.Seal) (st : Standing) : IxEntry U → Standing
  | .damn s' => if s' = s then .damnatus else st
  | .proscribe s' => if s' = s ∧ st ≠ .damnatus then .proscribed else st
  | .lift s' => if s' = s ∧ st = .proscribed then .unknown else st
  | .suspect s' => if s' = s ∧ st = .unknown then .suspected else st
  | .clear s' => if s' = s ∧ st = .suspected then .unknown else st

def standing (X : List (IxEntry U)) (s : U.Seal) : Standing :=
  X.foldl (standingStep s) .unknown

end Annulus

end
