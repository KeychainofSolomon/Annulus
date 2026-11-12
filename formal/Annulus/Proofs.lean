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

end Annulus

end
