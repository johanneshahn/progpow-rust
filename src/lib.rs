extern crate libc;
#[macro_use]
extern crate lazy_static;
extern crate bigint;

extern crate progpow_cpu;

#[cfg(feature="gpu")]
extern crate progpow_gpu;

pub mod types;
pub mod hardware;

#[cfg(test)]
mod test {
    use super::*;

    use bigint::uint::U256;
    use hardware::PpCPU;
    use types::PpCompute;

    #[test]
    fn test_compute_cpu() {
        let height: u64 = 20;
        let nonce: u64 = 10123012301;
        let header_hash: [u8; 32] = [0; 32];
        let pp_cpu = PpCPU::new();
        let (value, mix) = pp_cpu.verify(&header_hash, height, nonce).unwrap();
        assert_eq!(mix, [
            2660197958, 3827903912, 1330767690, 4237192674,
            2755488049, 3332560304, 263437910, 1583650287]);
    }

    #[test]
    #[cfg(feature="gpu")]
    fn test_compute_gpu() {
        use hardware::PpGPU;
        use progpow_gpu::utils::get_gpu_solution;

        let header = [20u8; 32];
        let epoch: i32 = 0;
        let height: u64 = 1;
        let boundary: u64 = 100000000;

        let mut difficulty: U256 = U256::max_value();
        difficulty = difficulty / U256::from(boundary);
        let target = difficulty >> 192;

        let (nonce, mix) = get_gpu_solution(header.clone(), height, epoch, boundary);
        let cpu = PpCPU::new();
        let (value, mix_hash) = cpu.verify(&header, height, nonce).unwrap();

        //let mix_verify: U256 = U256::from_big_endian(&mix_hash);
        //let r: U256 = U256::from_big_endian(&value);

        // va.copy_from_slice(&value[0..8]);
        //let v: [u32; 2] = unsafe { ::std::mem::transmute(&va) };
        
        let mix32: [u32; 8] = unsafe { ::std::mem::transmute(mix) };
        //println!("v: {:?}", (v[0] as u64) << 32 | v[1] as u64);

        assert_eq!(mix32, mix_hash);
        assert!(((value[0] as u64) << 32 | value[1] as u64) < target.low_u64());
    }
}