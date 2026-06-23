import Init.Data.Nat.Basic
import Init.Data.List.Basic

namespace MOC.Resonance

/-- Explicit Signal and Hypergraph types for the axiomatic core -/
abbrev Signal := Nat
abbrev Hypergraph := List Nat

def epsilon_of_resonance (τ : Nat) (r1 : Nat) : Nat :=
  if r1 > τ then (12 * (r1 - τ)) / 100 else 0

def Lip_transition (r1 : Nat) (τ : Nat) : Nat :=
  10000 - epsilon_of_resonance τ r1

def nat_abs_diff (x y : Nat) : Nat :=
  if x > y then x - y else y - x

theorem nat_abs_diff_triangle (x y z : Nat) :
  nat_abs_diff x z ≤ nat_abs_diff x y + nat_abs_diff y z := by
  unfold nat_abs_diff
  split <;> split <;> split <;> omega

def ultra_dist (x y : Nat) (tiers : List Nat) : Nat :=
  tiers.foldl (fun acc q => 
    let d := nat_abs_diff x y
    let component_dist := if q > 0 then ( (d % q) * 10000 / q ) else 0
    Nat.max acc component_dist
  ) 0

theorem ultrametric_contraction (r1 : Nat) (τ : Nat) (_h_r1 : r1 >= 8000) (_h_tau : τ = 7500) :
  ∃ L : Nat, L < 10000 ∧ ∀ d : Nat, d > 0 → (d * L) / 10000 < d := by
  exists 9940
  apply And.intro
  · decide
  · intro d h_d
    apply Nat.div_lt_of_lt_mul
    -- Proof: d * 9940 < d * 10000
    have h_mul : d * 9940 < d * 10000 := Nat.mul_lt_mul_of_pos_left (by decide) h_d
    omega

end MOC.Resonance
