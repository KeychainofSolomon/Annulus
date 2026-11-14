import Annulus.Theorems

noncomputable section
open Classical

namespace Annulus

variable {U : Universe}

/-! The first proofs (`core.md` §8). Each theorem below is proved from the transition
rules alone; none assumes anything about the rite beyond `RelRite`. -/

/-- P1 (a), containment at commit: every effect a commit carries out is in its request's
handed set and inside the faculty (outside the reserved part, for a spirit). The core's
re-check at commit (`Recheck`) is what makes this hold at every step. -/
theorem P1_commits (M : Core U) (R : RelRite U) :
    ∀ σ p es σ', Reachable M R σ → Step M R σ (.commit p es .done) σ' →
      ∃ pi ri, σ.props p = some pi ∧ σ.reqs pi.req = some ri ∧
        ∀ e, e ∈ es → ri.handed e.needs ∧ Contained σ ri.to e.needs := by
  intro σ p es σ' _ h
  obtain ⟨σ1, hc, _⟩ := h
  cases hc
  all_goals
    obtain ⟨-, h2, -⟩ := ‹Recheck _ _ _›
    exact ⟨_, _, ‹_›, ‹_›, h2⟩

/-- An outside will's turn is recorded as its declaration, never as known, and it may
declare only what is declarable (`core.md` §9.2, 2026-11-12). -/
theorem real_turn_declared (M : Core U) (R : RelRite U) :
    ∀ σ w v c fin σ', Step M R σ (.turn w v c fin) σ' → σ.mode = .real →
      ∃ o, v = .declared o ∧ U.declarable o := by
  intro σ w v c fin σ' h hm
  obtain ⟨σ1, hc, _⟩ := h
  cases hc
  first
    | (simp only [hm, Will.of]; exact ⟨_, rfl, ‹σ.mode = .real → U.declarable _› hm⟩)
    | (rw [hm]; exact ⟨_, rfl, ‹σ.mode = .real → U.declarable _› hm⟩)

/-- In simulated mode every turn is recorded as known. -/
theorem simulated_turn_known (M : Core U) (R : RelRite U) :
    ∀ σ w v c fin σ', Step M R σ (.turn w v c fin) σ' → σ.mode = .simulated →
      ∃ o, v = .known o := by
  intro σ w v c fin σ' h hm
  obtain ⟨σ1, hc, _⟩ := h
  cases hc
  first
    | (simp only [hm, Will.of]; exact ⟨_, rfl⟩)
    | (rw [hm]; exact ⟨_, rfl⟩)

/-- No outside will ends a run by killing: `kill` happens only in simulated mode. -/
theorem kill_only_simulated (M : Core U) (R : RelRite U) :
    ∀ σ w o ds σ', Step M R σ (.kill w o ds) σ' → σ.mode = .simulated := by
  intro σ w o ds σ' h
  obtain ⟨σ1, hc, _⟩ := h
  cases hc
  assumption

/-! ### P1 (b): every request's handed places lie in the faculty

An invariant over runs: requests are made only through the two request rules, whose
hypothesis `hcon` puts every handed power inside the faculty, and nothing after `launch`
changes the faculty or the reserved part. -/

def ReqsContained (σ : State U) : Prop :=
  ∀ q ri pl m, σ.reqs q = some ri → ri.handed (.cap pl m) → CapContained σ ri.to pl m

def ReqInv (σ : State U) : Prop :=
  ReqsContained σ ∧ (σ.phase = .preLaunch → ∀ q, σ.reqs q = none)

theorem cap_of_contained {σ : State U} {to : Addressee U} {pl : U.Place} {m : U.Perm}
    (h : Contained σ to (.cap pl m)) : CapContained σ to pl m := by
  cases to <;> exact h

abbrev Frame (σ1 σ : State U) : Prop :=
  σ1.reqs = σ.reqs ∧ σ1.faculty = σ.faculty ∧ σ1.reserved = σ.reserved ∧ σ1.phase = σ.phase

theorem frame_out {σ1 σ : State U} (h : Frame σ1 σ) :
    σ1.reqs = σ.reqs ∧ σ1.faculty = σ.faculty ∧ σ1.reserved = σ.reserved ∧
      (σ1.phase = σ.phase ∨ ∃ e, σ1.phase = .ended e) :=
  ⟨h.1, h.2.1, h.2.2.1, Or.inl h.2.2.2⟩

theorem frame_ite {c : Prop} [Decidable c] {a b σ : State U} (ha : Frame a σ)
    (hb : Frame b σ) : Frame (if c then a else b) σ := by
  by_cases h : c
  · rw [if_pos h]; exact ha
  · rw [if_neg h]; exact hb

theorem afterTurn_frame {σ : State U} {w : U.Worker} {wi : WInfo U} {p : U.Pid}
    {rest : List U.Pid} {pi : PInfo U} {o : U.Outcome} {c : Contrib U} {fin : Bool}
    {l : Label U} : Frame (afterTurn σ w wi p rest pi o c fin l) σ := by
  unfold afterTurn
  cases c <;> exact ⟨rfl, rfl, rfl, rfl⟩

theorem afterJudge_frame {σ : State U} {p : U.Pid} {pi : PInfo U} {d : WorkOutcome}
    {k : WorkOutcome → U.Exec} : Frame (afterJudge σ p pi d k) σ := by
  unfold afterJudge
  exact frame_ite ⟨rfl, rfl, rfl, rfl⟩ (frame_ite ⟨rfl, rfl, rfl, rfl⟩ ⟨rfl, rfl, rfl, rfl⟩)

theorem foldl_addMade_frame (l : Label U) :
    ∀ (ms : List (Made U)) (σ : State U), Frame (ms.foldl (addMade l) σ) σ
  | [], _ => ⟨rfl, rfl, rfl, rfl⟩
  | m :: ms, σ => by
      have h := foldl_addMade_frame l ms (addMade l σ m)
      exact ⟨h.1.trans rfl, h.2.1.trans rfl, h.2.2.1.trans rfl, h.2.2.2.trans rfl⟩

/-- Every step other than `launch` and `request` leaves the requests, the faculty and the
reserved part as they were, and either keeps the phase or ends the run. -/
theorem step_frame {M : Core U} {R : RelRite U} {σ σ1 : State U} {l : Label U}
    (h : CoreStep M R σ l σ1)
    (hl : ∀ f, l ≠ .launch f)
    (hr : ∀ q p to f w a, l ≠ .request q p to f w a) :
    σ1.reqs = σ.reqs ∧ σ1.faculty = σ.faculty ∧ σ1.reserved = σ.reserved ∧
      (σ1.phase = σ.phase ∨ ∃ e, σ1.phase = .ended e) := by
  cases h
  all_goals first
    | exact (hl _ rfl).elim
    | exact (hr _ _ _ _ _ _ rfl).elim
    | exact ⟨rfl, rfl, rfl, Or.inl rfl⟩
    | exact ⟨rfl, rfl, rfl, Or.inr ⟨_, rfl⟩⟩
    | exact frame_out afterTurn_frame
    | exact frame_out afterJudge_frame
    | exact frame_out (foldl_addMade_frame _ _ _)

theorem capContained_congr {σ σ1 : State U} {to : Addressee U} {pl : U.Place} {m : U.Perm}
    (hf : σ1.faculty = σ.faculty) (hr : σ1.reserved = σ.reserved)
    (h : CapContained σ to pl m) : CapContained σ1 to pl m := by
  cases to <;> simpa only [CapContained, hf, hr] using h

theorem reqInv_step {M : Core U} {R : RelRite U} {σ σ' : State U} {l : Label U}
    (hi : ReqInv σ) (h : Step M R σ l σ') : ReqInv σ' := by
  obtain ⟨σ1, hc, hrun, -, -, rfl⟩ := h
  obtain ⟨hcont, hpre⟩ := hi
  show ReqsContained σ1 ∧ (σ1.phase = .preLaunch → ∀ q, σ1.reqs q = none)
  by_cases hL : ∃ f, l = .launch f
  · obtain ⟨f, rfl⟩ := hL
    cases hc with
    | launch hp _ =>
        have hnone := hpre hp
        constructor
        · intro q ri pl m hq
          rw [hnone q] at hq
          cases hq
        · intro hph
          cases hph
  by_cases hR : ∃ q p to f w a, l = .request q p to f w a
  · obtain ⟨q, p, to, f, w, a, rfl⟩ := hR
    have hph : σ.phase = .running := hrun rfl
    cases hc with
    | requestSpirit _ _ _ _ _ _ _ _ _ hcon =>
        constructor
        · intro q' ri pl m hq hh
          simp only [upd] at hq
          split at hq
          · cases hq
            exact cap_of_contained (hcon _ hh)
          · exact hcont q' ri pl m hq hh
        · intro h'
          rw [hph] at h'
          cases h'
    | requestAngel _ _ _ _ _ _ hcon _ =>
        constructor
        · intro q' ri pl m hq hh
          simp only [upd] at hq
          split at hq
          · cases hq
            exact cap_of_contained (hcon _ hh)
          · exact hcont q' ri pl m hq hh
        · intro h'
          rw [hph] at h'
          cases h'
  have hnl : l.isLaunch = false := by
    cases l with
    | launch f => exact (hL ⟨f, rfl⟩).elim
    | _ => rfl
  have hph : σ.phase = .running := hrun hnl
  obtain ⟨hq, hf, hres, hphase⟩ := step_frame hc (fun f e => hL ⟨f, e⟩)
    (fun q p to f w a e => hR ⟨q, p, to, f, w, a, e⟩)
  constructor
  · intro q ri pl m h1 h2
    rw [hq] at h1
    exact capContained_congr hf hres (hcont q ri pl m h1 h2)
  · intro h'
    rcases hphase with h2 | ⟨e, h2⟩
    · rw [h2, hph] at h'
      cases h'
    · rw [h2] at h'
      cases h'

theorem reqInv_run {M : Core U} {R : RelRite U} {prog : M.lang.Program} {x : Exn U}
    (hrun : Run M R prog x) : ∀ k, k ≤ x.len → ReqInv (x.st k) := by
  obtain ⟨hinit, -, -, hsteps⟩ := hrun
  intro k
  induction k with
  | zero =>
      intro _
      obtain ⟨hp, -, -, -, -, -, -, -, -, hreqs, -⟩ := hinit
      constructor
      · intro q ri pl m hq
        rw [hreqs q] at hq
        cases hq
      · intro _
        exact hreqs
  | succ k ih =>
      intro hk
      exact reqInv_step (ih (Nat.le_of_succ_le hk)) (hsteps k (Nat.lt_of_succ_le hk))

/-- P1 (b), proved: every request's handed places are in the faculty (and outside its
reserved part, for a spirit), in every reachable state. -/
theorem P1_requests (M : Core U) (R : RelRite U) :
    ∀ σ q ri pl m, Reachable M R σ → σ.reqs q = some ri → ri.handed (.cap pl m) →
      CapContained σ ri.to pl m := by
  intro σ q ri pl m hr hq hh
  obtain ⟨prog, x, -, hrun, rfl⟩ := hr
  exact (reqInv_run hrun x.len (Nat.le_refl _)).1 q ri pl m hq hh

/-- `operator.md` §8.8, proved: a launch whose faculty the master did not sign grants
nothing but reading. So every work still needs the master; a binding asks only. -/
theorem launch_without_master_only_reads (M : Core U) (σ : State U)
    (hc : LaunchChecks M σ) (hp : σ.auth.wandPresent = true)
    (hn : ¬ M.mech.verify σ.auth.wandFp
      (M.mech.digestFaculty σ.faculty σ.reserved σ.auth.indexRoot σ.auth.graveRoot
        σ.auth.keysRoot) σ.auth.facultySig) :
    (∀ p m, σ.faculty p m → m = U.read) ∧ (∀ p m, ¬ σ.reserved p m) := by
  obtain ⟨-, -, -, -, -, -, -, h⟩ := hc
  rcases (h hp).2 with hm | ⟨_k, _t, _c, -, -, hr, hres, -⟩
  · exact absurd hm hn
  · exact ⟨hr, hres⟩

/-! ### P7: the audit

Every step appends exactly its own record, so the audit of a run is the list of its
labels. Nothing is assumed of the rite. -/

theorem step_audit {M : Core U} {R : RelRite U} {σ σ' : State U} {l : Label U}
    (h : Step M R σ l σ') : σ'.audit = σ.audit ++ [l] := by
  obtain ⟨σ1, -, -, -, -, rfl⟩ := h
  rfl

theorem run_audit {M : Core U} {R : RelRite U} {prog : M.lang.Program} {x : Exn U}
    (hrun : Run M R prog x) :
    ∀ k, k ≤ x.len → (x.st k).audit = (List.range k).map x.lb := by
  obtain ⟨hinit, -, -, hsteps⟩ := hrun
  intro k
  induction k with
  | zero =>
      intro _
      rw [hinit.2.1]
      rfl
  | succ k ih =>
      intro hk
      rw [step_audit (hsteps k (Nat.lt_of_succ_le hk)), ih (Nat.le_of_succ_le hk),
        List.range_succ, List.map_append]
      rfl

/-- P7, proved. -/
theorem P7_holds (M : Core U) (R : RelRite U) : P7 M R := by
  intro prog x hrun
  exact ⟨fun k hk => step_audit (hrun.2.2.2 k hk), run_audit hrun x.len (Nat.le_refl _)⟩

/-! ### P8: fail closed -/

/-- P8, proved: `launch` happens only when its checks hold; without the wand the faculty
is empty; and an examination with no lawful examiner, or of a doubtful author, holds. -/
theorem P8_holds (M : Core U) (R : RelRite U) : P8 M R := by
  refine ⟨?_, ?_, ?_⟩
  · intro σ f σ' h
    obtain ⟨σ1, hc, -⟩ := h
    cases hc
    assumption
  · intro σ f σ' h hw p m
    obtain ⟨σ1, hc, -, -, -, rfl⟩ := h
    cases hc
    simp [hw]
  · intro σ p a v σ' _ h hd
    obtain ⟨σ1, hc, -⟩ := h
    cases hc with
    | examineBy hw hi hp hst he hadm hv hcb =>
        rcases hd with ⟨e, he'⟩ | ⟨pi', hp', hdoubt⟩
        · cases he'
        · rw [hp] at hp'
          cases hp'
          simp [finalVerdict, hdoubt]
    | examineNone => rfl

/-! ### P2: no self-approval

A commit is enabled only right after a judgement to PARADISUS, which is enabled only
right after an examination; an examination that approves was made by an examiner who is
neither the author nor carried. With P7, the audit is the run's labels, so these are the
two steps before the commit. -/

theorem getLast_range_map {α : Type} (f : Nat → α) (k : Nat) (v : α)
    (h : ((List.range k).map f).getLast? = some v) : ∃ j, k = j + 1 ∧ f j = v := by
  cases k with
  | zero => simp at h
  | succ j =>
      rw [List.range_succ, List.map_append] at h
      simp at h
      exact ⟨j, rfl, h⟩

theorem coreStep_commit_owed {M : Core U} {R : RelRite U} {σ σ1 : State U} {p : U.Pid}
    {es : List (Effect U)} {r : CommitResult} (h : CoreStep M R σ (.commit p es r) σ1) :
    σ.audit.getLast? = some (.judge p .ratum) := by
  cases h <;> assumption

theorem coreStep_judge_owed {M : Core U} {R : RelRite U} {σ σ1 : State U} {p : U.Pid}
    {d : WorkOutcome} (h : CoreStep M R σ (.judge p d) σ1) :
    ∃ a v, σ.audit.getLast? = some (.examine p a v) ∧ v.dest = d := by
  cases h
  exact ⟨_, _, ‹_›, rfl⟩

theorem coreStep_examine_approve {M : Core U} {R : RelRite U} {σ σ1 : State U} {p : U.Pid}
    {a : ExamAns U} {v : Verdict} (h : CoreStep M R σ (.examine p a v) σ1)
    (hv : v = .approve) :
    ∃ e v0 cb pi, a = .examined e v0 cb ∧ σ.props p = some pi ∧ e ≠ pi.author ∧
      ¬ e ∈ pi.carries := by
  cases h with
  | examineBy hw hi hp hst he hadm hv' hcb => exact ⟨_, _, _, _, rfl, hp, hadm.1, hadm.2.1⟩
  | examineNone => cases hv

/-- P2, proved. -/
theorem P2_holds (M : Core U) (R : RelRite U) : P2 M R := by
  intro prog x _ hrun k p es r hk hlb
  obtain ⟨σ1, hc, -⟩ := hrun.2.2.2 k hk
  rw [hlb] at hc
  have h1 := coreStep_commit_owed hc
  rw [run_audit hrun k (Nat.le_of_lt hk)] at h1
  obtain ⟨j1, rfl, hj1⟩ := getLast_range_map _ _ _ h1
  have hk1 : j1 < x.len := by omega
  obtain ⟨σ2, hc2, -⟩ := hrun.2.2.2 j1 hk1
  rw [hj1] at hc2
  obtain ⟨a, v, h2, hd⟩ := coreStep_judge_owed hc2
  rw [run_audit hrun j1 (Nat.le_of_lt hk1)] at h2
  obtain ⟨j, rfl, hj⟩ := getLast_range_map _ _ _ h2
  have hk2 : j < x.len := by omega
  obtain ⟨σ3, hc3, -⟩ := hrun.2.2.2 j hk2
  rw [hj] at hc3
  have hv : v = .approve := by
    cases v with
    | approve => rfl
    | hold => simp [Verdict.dest] at hd
    | reject => simp [Verdict.dest] at hd
  obtain ⟨e, v0, cb, pi, rfl, hp, h3, h4⟩ := coreStep_examine_approve hc3 hv
  subst hv
  exact ⟨j, e, v0, cb, pi, by omega, hj, hj1, fun m d h5 h6 => by omega, hp, h3, h4⟩

/-! ### The damnation law -/

theorem standing_append (X : List (IxEntry U)) (e : IxEntry U) (s : U.Seal) :
    standing (X ++ [e]) s = standingStep s (standing X s) e := by
  simp [standing, List.foldl_append]

theorem noteSuspicion_cases (M : Core U) (σ : State U) (s : U.Seal) :
    noteSuspicion M σ s = σ.index ∨
      (¬ U.angelic s ∧ noteSuspicion M σ s = σ.index ++ [.suspect s]) := by
  unfold noteSuspicion
  by_cases h : NewlySuspect M σ s
  · rw [if_pos h]; exact Or.inr ⟨h.1, rfl⟩
  · rw [if_neg h]; exact Or.inl rfl

theorem damnIfMortal_cases (a : U.Seal) (X : List (IxEntry U)) :
    damnIfMortal a X = X ∨ (¬ U.angelic a ∧ damnIfMortal a X = X ++ [.damn a]) := by
  unfold damnIfMortal
  by_cases h : U.angelic a
  · rw [if_pos h]; exact Or.inl rfl
  · rw [if_neg h]; exact Or.inr ⟨h, rfl⟩

theorem foldl_addMade_index (l : Label U) :
    ∀ (ms : List (Made U)) (σ : State U), (ms.foldl (addMade l) σ).index = σ.index
  | [], _ => rfl
  | m :: ms, σ => foldl_addMade_index l ms (addMade l σ m)

theorem index_ite {c : Prop} [Decidable c] {a b : State U} {X : List (IxEntry U)}
    (ha : a.index = X) (hb : b.index = X) : (if c then a else b).index = X := by
  by_cases h : c
  · rw [if_pos h]; exact ha
  · rw [if_neg h]; exact hb

theorem afterTurn_index {σ : State U} {w : U.Worker} {wi : WInfo U} {p : U.Pid}
    {rest : List U.Pid} {pi : PInfo U} {o : U.Outcome} {c : Contrib U} {fin : Bool}
    {l : Label U} : (afterTurn σ w wi p rest pi o c fin l).index = σ.index := by
  unfold afterTurn
  cases c <;> rfl

theorem afterJudge_index {σ : State U} {p : U.Pid} {pi : PInfo U} {d : WorkOutcome}
    {k : WorkOutcome → U.Exec} : (afterJudge σ p pi d k).index = σ.index := by
  unfold afterJudge
  exact index_ite rfl (index_ite rfl rfl)

/-- The Index changes only by an intrusion damning a mortal seal, or by an examination
entering a suspicion against one. -/
theorem coreStep_index {M : Core U} {R : RelRite U} {σ σ1 : State U} {l : Label U}
    (h : CoreStep M R σ l σ1) :
    σ1.index = σ.index ∨
      (∃ a a' c d, l = .intrude a a' c d ∧ ¬ U.angelic a ∧ σ1.index = σ.index ++ [.damn a]) ∨
      (∃ p an v s, l = .examine p an v ∧ ¬ U.angelic s ∧
        σ1.index = σ.index ++ [.suspect s]) := by
  cases h
  all_goals first
    | exact Or.inl rfl
    | exact Or.inl afterTurn_index
    | exact Or.inl afterJudge_index
    | exact Or.inl (foldl_addMade_index _ _ _)
    | exact (noteSuspicion_cases M _ _).elim Or.inl
        (fun h => Or.inr (Or.inr ⟨_, _, _, _, rfl, h.1, h.2⟩))
    | exact (damnIfMortal_cases _ _).elim Or.inl
        (fun h => Or.inr (Or.inl ⟨_, _, _, _, rfl, h.1, h.2⟩))

/-- The damnation law, proved: *damnatus* is never left, nothing is entered against an
angel, and an intrusion touches no standing but the intruder's. -/
theorem damnationLaw_holds (M : Core U) (R : RelRite U) : DamnationLaw M R := by
  intro σ l σ' _ h
  have hi : σ'.index = σ.index ∨
      (∃ a a' c d, l = .intrude a a' c d ∧ ¬ U.angelic a ∧ σ'.index = σ.index ++ [.damn a]) ∨
      (∃ p an v s, l = .examine p an v ∧ ¬ U.angelic s ∧
        σ'.index = σ.index ++ [.suspect s]) := by
    obtain ⟨σ1, hc, -, -, -, rfl⟩ := h
    exact coreStep_index (σ1 := σ1) hc
  refine ⟨hi, ?_, ?_, ?_⟩
  · intro s hs
    rcases hi with h0 | ⟨a, _, _, _, -, -, h1⟩ | ⟨_, _, _, s', -, -, h1⟩
    · rw [h0]; exact hs
    · rw [h1, standing_append, hs]; simp [standingStep]
    · rw [h1, standing_append, hs]; simp [standingStep]
  · intro s hang hs
    rcases hi with h0 | ⟨a, _, _, _, -, hna, h1⟩ | ⟨_, _, _, s', -, hna, h1⟩
    · rw [h0]; exact hs
    · have hne : a ≠ s := fun e => hna (e ▸ hang)
      rw [h1, standing_append, hs]; simp [standingStep, hne]
    · have hne : s' ≠ s := fun e => hna (e ▸ hang)
      rw [h1, standing_append, hs]; simp [standingStep, hne]
  · intro a a' c d s hl hne
    rcases hi with h0 | ⟨b, _, _, _, hl', -, h1⟩ | ⟨_, _, _, _, hl', -, -⟩
    · rw [h0]
    · rw [hl] at hl'
      cases hl'
      have hne' : a ≠ s := fun e => hne e.symm
      rw [h1, standing_append]; simp [standingStep, hne']
    · rw [hl] at hl'
      cases hl'

/-! ### P1 (c): a helper holds a subset of the request it serves

An invariant over runs. A helper is made only by `spawn`, which hands it a subset of the
request its maker serves; a helper never finishes a proposal (`hfin`), so the head of its
queue never changes; a proposal keeps its request, and a request is never rewritten. -/

theorem upd_eq {α β : Type} {f : α → β} {a x : α} {b : β} (e : x = a) : upd f a b x = b := by
  show (if x = a then b else f x) = b
  rw [if_pos e]

theorem upd_ne {α β : Type} {f : α → β} {a x : α} {b : β} (e : x ≠ a) :
    upd f a b x = f x := by
  show (if x = a then b else f x) = f x
  rw [if_neg e]

def HInv (ws : U.Worker → Option (WInfo U)) (ps : U.Pid → Option (PInfo U))
    (qs : U.Req → Option (ReqInfo U)) : Prop :=
  ∀ w wi, ws w = some wi → wi.isHelper = true →
    ∃ p rest pi ri, wi.queue = p :: rest ∧ ps p = some pi ∧ qs pi.req = some ri ∧
      ∀ r, wi.handed r → ri.handed r

def HelperInv (σ : State U) : Prop := HInv σ.workers σ.props σ.reqs

/-- Requests are only added. -/
def QExt (f f1 : U.Req → Option (ReqInfo U)) : Prop :=
  ∀ q ri, f q = some ri → f1 q = some ri

/-- Proposals are only added, and each keeps its request. -/
def PExt (g g1 : U.Pid → Option (PInfo U)) : Prop :=
  ∀ p pi, g p = some pi → ∃ pi', g1 p = some pi' ∧ pi'.req = pi.req

/-- Every helper was a helper before, with the same handed set and the same head of queue. -/
def WExt (h h1 : U.Worker → Option (WInfo U)) : Prop :=
  ∀ w wi1, h1 w = some wi1 → wi1.isHelper = true →
    ∃ wi, h w = some wi ∧ wi.isHelper = true ∧ wi1.handed = wi.handed ∧
      ∀ p rest, wi.queue = p :: rest → ∃ rest', wi1.queue = p :: rest'

theorem hinv_ext {ws ws1 : U.Worker → Option (WInfo U)} {ps ps1 : U.Pid → Option (PInfo U)}
    {qs qs1 : U.Req → Option (ReqInfo U)} (hi : HInv ws ps qs) (hw : WExt ws ws1)
    (hp : PExt ps ps1) (hq : QExt qs qs1) : HInv ws1 ps1 qs1 := by
  intro w wi1 h1 hh
  obtain ⟨wi, hwi, hhw, hhand, hqu⟩ := hw w wi1 h1 hh
  obtain ⟨p, rest, pi, ri, hq0, hp0, hr0, hsub⟩ := hi w wi hwi hhw
  obtain ⟨rest', hq'⟩ := hqu p rest hq0
  obtain ⟨pi', hp', hreq⟩ := hp p pi hp0
  refine ⟨p, rest', pi', ri, hq', hp', ?_, ?_⟩
  · rw [hreq]; exact hq _ _ hr0
  · intro r h
    rw [hhand] at h
    exact hsub r h

theorem hinv_spawn {ws : U.Worker → Option (WInfo U)} {ps : U.Pid → Option (PInfo U)}
    {qs : U.Req → Option (ReqInfo U)} {w' : U.Worker} {o : Option (Addressee U)}
    {par : Option U.Inv} {h : Power U → Prop} {p : U.Pid} {pi : PInfo U} {ri : ReqInfo U}
    (hi : HInv ws ps qs) (hp : ps p = some pi) (hr : qs pi.req = some ri)
    (hh : ∀ r, h r → ri.handed r) :
    HInv (upd ws w' (some ⟨o, par, h, [p], true, true⟩)) ps qs := by
  intro w wi h1 hhelp
  by_cases e : w = w'
  · rw [upd_eq e] at h1
    cases h1
    exact ⟨p, [], pi, ri, rfl, hp, hr, hh⟩
  · rw [upd_ne e] at h1
    exact hi w wi h1 hhelp

theorem qext_refl (f : U.Req → Option (ReqInfo U)) : QExt f f := fun _ _ h => h

theorem qext_upd {f : U.Req → Option (ReqInfo U)} {q : U.Req} {x : Option (ReqInfo U)}
    (h : f q = none) : QExt f (upd f q x) := by
  intro q' ri h'
  by_cases e : q' = q
  · rw [e, h] at h'; cases h'
  · rw [upd_ne e]; exact h'

theorem pext_refl (g : U.Pid → Option (PInfo U)) : PExt g g := fun _ pi h => ⟨pi, h, rfl⟩

theorem pext_trans {g g1 g2 : U.Pid → Option (PInfo U)} (a : PExt g g1) (b : PExt g1 g2) :
    PExt g g2 := by
  intro p pi h
  obtain ⟨pi1, h1, e1⟩ := a p pi h
  obtain ⟨pi2, h2, e2⟩ := b p pi1 h1
  exact ⟨pi2, h2, e2.trans e1⟩

theorem pext_upd_none {g : U.Pid → Option (PInfo U)} {p : U.Pid} {x : Option (PInfo U)}
    (h : g p = none) : PExt g (upd g p x) := by
  intro p' pi h'
  by_cases e : p' = p
  · rw [e, h] at h'; cases h'
  · exact ⟨pi, by rw [upd_ne e]; exact h', rfl⟩

theorem pext_upd_same {g : U.Pid → Option (PInfo U)} {p : U.Pid} {pi0 pi1 : PInfo U}
    (h : g p = some pi0) (hr : pi1.req = pi0.req) : PExt g (upd g p (some pi1)) := by
  intro p' pi h'
  by_cases e : p' = p
  · rw [e, h] at h'
    cases h'
    exact ⟨pi1, upd_eq e, hr⟩
  · exact ⟨pi, by rw [upd_ne e]; exact h', rfl⟩

theorem pext_term {σ : State U} {P : U.Inv → Inst U → Prop} :
    PExt σ.props (termProps σ P) := by
  intro p pi h
  simp only [termProps, h]
  split <;> exact ⟨_, rfl, rfl⟩

theorem pext_abandon {τ : State U} : PExt τ.props (abandonAngelProps τ) := by
  intro p pi h
  simp only [abandonAngelProps, h]
  split <;> exact ⟨_, rfl, rfl⟩

theorem wext_refl (h : U.Worker → Option (WInfo U)) : WExt h h :=
  fun _ wi hw hh => ⟨wi, hw, hh, rfl, fun _ rest e => ⟨rest, e⟩⟩

theorem wext_upd_nonhelper {h : U.Worker → Option (WInfo U)} {w : U.Worker} {wi' : WInfo U}
    (hn : wi'.isHelper = false) : WExt h (upd h w (some wi')) := by
  intro w1 wi1 h1 hh
  by_cases e : w1 = w
  · rw [upd_eq e] at h1
    cases h1
    rw [hn] at hh
    cases hh
  · rw [upd_ne e] at h1
    exact ⟨wi1, h1, hh, rfl, fun _ rest e => ⟨rest, e⟩⟩

theorem wext_upd_same {h : U.Worker → Option (WInfo U)} {w : U.Worker} {wi wi' : WInfo U}
    (hw : h w = some wi) (hh' : wi'.handed = wi.handed) (hi : wi'.isHelper = wi.isHelper)
    (hq : ∀ p rest, wi.queue = p :: rest → ∃ rest', wi'.queue = p :: rest') :
    WExt h (upd h w (some wi')) := by
  intro w1 wi1 h1 hh
  by_cases e : w1 = w
  · rw [upd_eq e] at h1
    cases h1
    exact ⟨wi, by rw [e]; exact hw, by rw [← hi]; exact hh, hh', hq⟩
  · rw [upd_ne e] at h1
    exact ⟨wi1, h1, hh, rfl, fun _ rest e => ⟨rest, e⟩⟩

theorem wext_upd_append {h : U.Worker → Option (WInfo U)} {w : U.Worker} {wi : WInfo U}
    {p : U.Pid} (hw : h w = some wi) :
    WExt h (upd h w (some { wi with queue := wi.queue ++ [p] })) :=
  wext_upd_same hw rfl rfl (fun p0 rest e => ⟨rest ++ [p], by
    show wi.queue ++ [p] = p0 :: (rest ++ [p])
    rw [e]
    rfl⟩)

theorem pext_endRun {σ : State U} {e : Ending U} : PExt σ.props (endRun σ e).props :=
  pext_trans pext_term (pext_abandon (τ := terminate σ (fun _ _ => True) .released))

theorem wext_term {σ : State U} {P : U.Inv → Inst U → Prop} :
    WExt σ.workers (termWorkers σ P) := by
  intro w wi1 h1 hh
  cases hw : σ.workers w with
  | none => simp [termWorkers, hw] at h1
  | some wi =>
      simp only [termWorkers, hw] at h1
      split at h1 <;> cases h1 <;> exact ⟨_, rfl, hh, rfl, fun _ rest e => ⟨rest, e⟩⟩

theorem wext_endHelpers {σ : State U} {p : U.Pid} :
    WExt σ.workers (endHelpersOf σ p) := by
  intro w wi1 h1 hh
  cases hw : σ.workers w with
  | none => simp [endHelpersOf, hw] at h1
  | some wi =>
      simp only [endHelpersOf, hw] at h1
      split at h1 <;> cases h1 <;> exact ⟨_, rfl, hh, rfl, fun _ rest e => ⟨rest, e⟩⟩

theorem helperInv_ite {c : Prop} [Decidable c] {a b : State U} (ha : HelperInv a)
    (hb : HelperInv b) : HelperInv (if c then a else b) := by
  by_cases h : c
  · rw [if_pos h]; exact ha
  · rw [if_neg h]; exact hb

theorem helperInv_foldl (l : Label U) :
    ∀ (ms : List (Made U)) (σ : State U), HelperInv σ → HelperInv (ms.foldl (addMade l) σ)
  | [], _, hi => hi
  | m :: ms, σ, hi => helperInv_foldl l ms (addMade l σ m) hi

theorem helperInv_afterTurn {R : RelRite U} {σ : State U} {w : U.Worker} {wi : WInfo U}
    {p : U.Pid} {rest : List U.Pid} {pi : PInfo U} {ri : ReqInfo U} {o : U.Outcome}
    {c : Contrib U} {fin : Bool} {l : Label U}
    (hi : HelperInv σ) (hwi : σ.workers w = some wi) (hp : σ.props p = some pi)
    (hr : σ.reqs pi.req = some ri) (hfin : fin = true → wi.isHelper = false)
    (hc : ContribOK R σ wi ri pi c) :
    HelperInv (afterTurn σ w wi p rest pi o c fin l) := by
  have hW : WExt σ.workers
      (upd σ.workers w (some (if fin = true then { wi with queue := rest } else wi))) := by
    cases fin with
    | true =>
        rw [if_pos rfl]
        exact wext_upd_nonhelper (hfin rfl)
    | false =>
        rw [if_neg (by simp)]
        exact wext_upd_same hwi rfl rfl (fun _ rest e => ⟨rest, e⟩)
  unfold afterTurn
  cases c with
  | spawn w' h n =>
      obtain ⟨-, -, hh⟩ := hc
      exact hinv_spawn (hinv_ext hi hW (pext_upd_same hp rfl) (qext_refl _)) (upd_eq rfl) hr
        (fun r x => (hh r x).1)
  | _ => exact hinv_ext hi hW (pext_upd_same hp rfl) (qext_refl _)

theorem helperInv_afterJudge {σ : State U} {p : U.Pid} {pi : PInfo U} {d : WorkOutcome}
    {k : WorkOutcome → U.Exec} (hi : HelperInv σ) (hp : σ.props p = some pi) :
    HelperInv (afterJudge σ p pi d k) := by
  unfold afterJudge
  exact helperInv_ite (hinv_ext hi (wext_refl _) (pext_upd_same hp rfl) (qext_refl _))
    (helperInv_ite (hinv_ext hi wext_endHelpers (pext_upd_same hp rfl) (qext_refl _))
      (hinv_ext hi (wext_refl _) (pext_upd_same hp rfl) (qext_refl _)))

theorem helperInv_coreStep {M : Core U} {R : RelRite U} {σ σ1 : State U} {l : Label U}
    (hi : HelperInv σ) (h : CoreStep M R σ l σ1) : HelperInv σ1 := by
  cases h with
  | invokeOk hw hi' hs hfi hfw ha hx hsl hna =>
      exact hinv_ext hi (wext_upd_nonhelper rfl) (pext_refl _) (qext_refl _)
  | requestSpirit hw hi' hinv hl hc hm hq hp hsig hcon =>
      exact hinv_ext hi (wext_upd_append hm) (pext_upd_none hp) (qext_upd hq)
  | requestAngel hw hi' hfw hq hp hsig hcon hang =>
      exact hinv_ext hi (wext_upd_nonhelper rfl) (pext_upd_none hp) (qext_upd hq)
  | turn hw hwi ha hq hp hst hr ho hk hang hsim hdecl hfin hc =>
      exact helperInv_afterTurn hi hwi hp hr hfin hc
  | judge hl hi' hp => exact helperInv_afterJudge hi hp
  | commitOutside hl hi' hp hr hno =>
      exact hinv_ext hi wext_endHelpers (pext_upd_same hp rfl) (qext_refl _)
  | commitAwait hl hi' hp hr hok hhc hnokey =>
      exact hinv_ext hi (wext_refl _) (pext_upd_same hp rfl) (qext_refl _)
  | compute hw hi' hr hmr hf =>
      exact helperInv_foldl _ _ _ hi
  | _ =>
      first
        | exact hi
        | exact hinv_ext hi wext_term pext_term (qext_refl _)
        | exact hinv_ext hi wext_term pext_endRun (qext_refl _)

theorem helperInv_run {M : Core U} {R : RelRite U} {prog : M.lang.Program} {x : Exn U}
    (hrun : Run M R prog x) : ∀ k, k ≤ x.len → HelperInv (x.st k) := by
  obtain ⟨hinit, -, -, hsteps⟩ := hrun
  intro k
  induction k with
  | zero =>
      intro _ w wi hw hh
      obtain ⟨-, -, -, -, -, -, hop, hothers, -⟩ := hinit
      by_cases e : w = U.operator
      · rw [e, hop] at hw
        cases hw
        cases hh
      · rw [hothers w e] at hw
        cases hw
  | succ k ih =>
      intro hk
      obtain ⟨σ1, hc, -, -, -, he⟩ := hsteps k (Nat.lt_of_succ_le hk)
      rw [he]
      exact helperInv_coreStep (σ1 := σ1) (ih (Nat.le_of_succ_le hk)) hc

/-- P1 (c), proved. -/
theorem P1_helpers (M : Core U) (R : RelRite U) :
    ∀ σ w wi p rest pi r, Reachable M R σ → σ.workers w = some wi → wi.isHelper = true →
      wi.alive = true → wi.queue = p :: rest → σ.props p = some pi → Unjudged pi.status →
      wi.handed r → ∃ ri, σ.reqs pi.req = some ri ∧ ri.handed r := by
  intro σ w wi p rest pi r hr hw hh _ hq hp _ hhand
  obtain ⟨prog, x, -, hrun, rfl⟩ := hr
  obtain ⟨p', rest', pi', ri, hq', hp', hr', hsub⟩ :=
    helperInv_run hrun x.len (Nat.le_refl _) w wi hw hh
  cases hq.symm.trans hq'
  cases hp.symm.trans hp'
  exact ⟨ri, hr', hsub r hhand⟩

/-- P1, proved whole. -/
theorem P1_holds (M : Core U) (R : RelRite U) : P1 M R :=
  ⟨P1_commits M R, P1_requests M R, P1_helpers M R⟩

/-! ### P6 (b): a stricter rite adds no behaviour

The rules ask the rite only for answers it gives; none asks what it withholds. So a step
taken under a rite at least as strict is a step under the looser one, and the same
execution is a run of both. -/

theorem requestBad_mono {R R' : RelRite U}
    (hSig : ∀ σ f h, R'.signature σ f h → R.signature σ f h) {σ : State U} {f : U.ReqForm}
    {w : U.Worker} {to : Addressee U} (h : RequestBad R' σ f w to) : RequestBad R σ f w to := by
  cases to with
  | spirit i => exact h.imp id (fun ⟨asked, h1, h2⟩ => ⟨asked, hSig _ _ _ h1, h2⟩)
  | angel a =>
      exact h.imp id (Or.imp id (fun ⟨asked, h1, h2⟩ => ⟨asked, hSig _ _ _ h1, h2⟩))

theorem denied_mono {M : Core U} {R R' : RelRite U}
    (hSig : ∀ σ f h, R'.signature σ f h → R.signature σ f h) {σ : State U}
    (h : Denied M R' σ) : Denied M R σ := by
  unfold Denied at h ⊢
  generalize instrOf M σ = ins at h ⊢
  cases ins with
  | request to f q p w e => exact h.imp id (Or.imp id (requestBad_mono hSig))
  | _ => exact h

theorem contribOK_mono {R R' : RelRite U} (hB : ∀ σ b n, R'.bound σ b n → R.bound σ b n)
    {σ : State U} {wi : WInfo U} {ri : ReqInfo U} {pi : PInfo U} {c : Contrib U}
    (h : ContribOK R' σ wi ri pi c) : ContribOK R σ wi ri pi c := by
  cases c with
  | spawn w' hd n =>
      obtain ⟨h1, ⟨i, inst, h2, h3, h4, h5⟩, h6⟩ := h
      exact ⟨h1, ⟨i, inst, h2, h3, hB _ _ _ h4, h5⟩, h6⟩
  | _ => exact h

theorem coreStep_mono {M : Core U} {R R' : RelRite U} (hS : AtLeastAsStrict R' R)
    {σ σ1 : State U} {l : Label U} (h : CoreStep M R' σ l σ1) : CoreStep M R σ l σ1 := by
  obtain ⟨hA, hSig, hEx, hV, hCb, hB, hW, hVal⟩ := hS
  cases h with
  | invokeOk hw hi hs hfi hfw ha hx hsl hna =>
      exact .invokeOk hw hi hs hfi hfw (hA _ _ _ ha) hx hsl hna
  | invokeRefused hw hi hs ha hr hd => exact .invokeRefused hw hi hs (hA _ _ _ ha) hr hd
  | inscribe hw hi hs hn ha => exact .inscribe hw hi hs hn (hA _ _ _ ha)
  | act hw hi hs hinv hk ha => exact .act hw hi hs hinv hk (hA _ _ _ ha)
  | inscribeRefused hw hi hs hn ha hd => exact .inscribeRefused hw hi hs hn (hA _ _ _ ha) hd
  | actRefused hw hi hs hinv hk ha hd => exact .actRefused hw hi hs hinv hk (hA _ _ _ ha) hd
  | requestSpirit hw hi hinv hl hc hm hq hp hsig hcon =>
      exact .requestSpirit hw hi hinv hl hc hm hq hp (hSig _ _ _ hsig) hcon
  | requestAngel hw hi hfw hq hp hsig hcon hang =>
      exact .requestAngel hw hi hfw hq hp (hSig _ _ _ hsig) hcon hang
  | turn hw hwi ha hq hp hst hr ho hk hang hsim hdecl hfin hc =>
      exact .turn hw hwi ha hq hp hst hr (hW _ _ _ ho) hk hang hsim hdecl hfin
        (contribOK_mono hB hc)
  | kill hw hwi ha hpar ho hk hm hsim hd =>
      exact .kill hw hwi ha hpar (hW _ _ _ ho) hk hm hsim hd
  | examineBy hw hi hp hst he hadm hv hcb =>
      exact .examineBy hw hi hp hst (hEx _ _ _ he) hadm (hV _ _ _ _ hv) (hCb _ _ _ hcb)
  | examineNone hw hi hp hst he hno => exact .examineNone hw hi hp hst (hEx _ _ _ he) hno
  | consecrateOk hw hi hm hk hval hd =>
      exact .consecrateOk hw hi hm (hVal _ _ _ hk) hval hd
  | consecrateRefused hw hi hk hno => exact .consecrateRefused hw hi (hVal _ _ _ hk) hno
  | graveOk hw hi hk hmatch => exact .graveOk hw hi (hVal _ _ _ hk) hmatch
  | graveRefused hw hi hk hno => exact .graveRefused hw hi (hVal _ _ _ hk) hno
  | deny hw hd hds => exact .deny hw (denied_mono hSig hd) hds
  | _ => constructor <;> assumption

/-- P6 (b), proved: every run under the stricter rite is, step for step, a run under the
looser one. -/
theorem P6b_holds (M : Core U) : P6b M := by
  intro R R' _ hS prog x' _ hrun
  refine ⟨x', ⟨hrun.1, hrun.2.1, hrun.2.2.1, fun k hk => ?_⟩, rfl, rfl, rfl, rfl⟩
  obtain ⟨σ1, hc, h⟩ := hrun.2.2.2 k hk
  exact ⟨σ1, coreStep_mono hS hc, h⟩

end Annulus

end
