use packed_simd::*;

pub type Precision = f32;

/// Number of particles
pub const N_PARTICLES: usize = 1024;

pub type PrecisionSoA = f32x8;

/// Number of particles
pub const N_PARTICLES_SOA: usize = N_PARTICLES/8;

/// Number of time steps
pub const N_STEPS: usize = 20;

pub const DELTA_T: Precision = 0.002;
