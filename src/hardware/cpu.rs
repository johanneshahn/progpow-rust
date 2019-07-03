use std::path::Path;

use crate::types::{Hardware, PpCompute, ProgPowError, H256};
use progpow_cpu::cache::{NodeCacheBuilder, OptimizeFor};
use progpow_cpu::compute::{light_compute, PoW};

const CACHE_DIR: &str = "./cache";

pub struct PpCPU {
	cache_builder: NodeCacheBuilder,
}

impl PpCPU {
	pub fn new() -> Self {
		PpCPU {
			cache_builder: NodeCacheBuilder::new(OptimizeFor::Memory),
		}
	}
}

impl PpCompute for PpCPU {
	fn init(&mut self) -> Result<(), ProgPowError> {
		Ok(())
	}

	fn verify(
		&self,
		header_hash: &H256,
		height: u64,
		nonce: u64,
	) -> Result<([u32; 8], [u32; 8]), ProgPowError> {
		let light = self.cache_builder.light(Path::new(CACHE_DIR), height);
		Ok(light.compute(&header_hash, nonce, height))
	}

	fn compute(&self, header: [u8; 32], height: u64, epoch: i32, boundary: u64) {
		unimplemented!()
	}

	fn hardware(&self) -> Hardware {
		Hardware::CPU
	}
}
