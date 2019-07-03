use ffi::*;
use libc::c_void;

const MINER_UNINITIALIZED: &str = "Miner is not initialized";

type H256 = [u8; 32];

#[derive(Debug, Clone)]
pub enum Driver {
	CUDA = 1,
	OCL = 2,
}

#[derive(Debug)]
pub struct GPU {
	pub driver: Driver,
	miner: Option<*mut c_void>,
}

impl GPU {
	pub fn new(driver: Driver) -> Self {
		GPU {
			driver,
			miner: None,
		}
	}

	pub fn init(&mut self) -> Option<*mut c_void> {
		let driver = self.driver.clone() as u32;

		let miner = unsafe {
			progpow_gpu_configure(1);
			progpow_gpu_init(0, driver)
		};

		self.miner = Some(miner);
		self.miner
	}

	pub fn compute(
		&self,
		hash: [u8; 32],
		height: u64,
		epoch: i32,
		target: u64,
	) -> Result<(), &str> {
		if let None = self.miner {
			return Err(MINER_UNINITIALIZED);
		}

		let miner = self.miner.unwrap();

		unsafe {
			progpow_gpu_compute(
				miner,
				hash.as_ptr() as *const c_void,
				hash.len() as usize,
				height,
				epoch,
				target,
			);
		}

		Ok(())
	}

	pub fn solutions(&self) -> Result<Option<(u64, [u8; 32])>, &str> {
		if let None = self.miner {
			return Err(MINER_UNINITIALIZED);
		}

		let miner = self.miner.unwrap();
		let mut result = [0u8; 40];

		let found: bool =
			unsafe { progpow_gpu_get_solutions(miner, result.as_mut_ptr() as *mut c_void) };

		if found {
			let mut n = [0u8; 8];
			n.copy_from_slice(&result[0..8]);

			let mut mix = [0u8; 32];
			mix.copy_from_slice(&result[8..40]);

			let nonce: u64 = unsafe { ::std::mem::transmute(n) };

			Ok(Some((nonce, mix)))
		} else {
			Ok(None)
		}
	}
}

impl Drop for GPU {
	fn drop(&mut self) {
		if let Some(miner) = self.miner {
			unsafe {
				progpow_destroy(miner);
			}
		}
	}
}
