pub mod cpu;

#[cfg(feature="gpu")]
pub mod gpu;

pub use self::cpu::PpCPU;

#[cfg(feature="gpu")]
pub use self::gpu::PpGPU;
