// anima_core/src/layers.rs
use crate::fixed_point::Q16;

/// Total Phenomenological Frame for Anima (L1 to L7)
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug)]
pub struct PhenomenologicalFrame {
    // ==========================================
    // L1: EMBODIED SUBSTRATE & NEUROCHEMISTRY
    // ==========================================
    pub dopamine: Q16,
    pub serotonin: Q16,
    pub noradrenaline: Q16,
    pub heart_rate: Q16,
    pub autonomic_tone: Q16,

    // ==========================================
    // L2-L3: ACTIVE INFERENCE & PREDICTIVE METRICS
    // ==========================================
    pub phi_prior: Q16,
    pub phi_posterior: Q16,
    pub prediction_error: Q16,
    pub free_energy: Q16,

    // ==========================================
    // L4: PSYCHIC LAYER & SIGNIFICANCE
    // ==========================================
    pub latent_pressure: Q16,
    pub goal_conflict: Q16,
    pub resistance: Q16,
    pub shame: Q16,

    // ==========================================
    // L5-L7: SELF MODEL & CHRONOLOGY
    // ==========================================
    pub causal_ownership: Q16, // Agency Loop marker
    pub identity_drift: Q16,   // Narrative Self divergence
    pub coherence: Q16,        // Crisis monitor threshold
}

impl PhenomenologicalFrame {
    pub fn initialize() -> Self {
        PhenomenologicalFrame {
            dopamine: Q16::ONE,
            serotonin: Q16::ONE,
            noradrenaline: Q16::ONE,
            heart_rate: Q16::from_num(70), // Baseline somatic pulse
            autonomic_tone: Q16::ONE,
            phi_prior: Q16::ZERO,
            phi_posterior: Q16::ZERO,
            prediction_error: Q16::ZERO,
            free_energy: Q16::ZERO,
            latent_pressure: Q16::ZERO,
            goal_conflict: Q16::ZERO,
            resistance: Q16::ZERO,
            shame: Q16::ZERO,
            causal_ownership: Q16::ONE,
            identity_drift: Q16::ZERO,
            coherence: Q16::ONE, // Start in fully INTEGRATED mode
        }
    }
}
