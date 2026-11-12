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
    {rest : List U.Pid} {pi : PInfo U} {c : Contrib U} {fin : Bool} {l : Label U} :
    Frame (afterTurn σ w wi p rest pi c fin l) σ := by
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

end Annulus

end
