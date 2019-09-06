#pragma once 

#include <libethash/internal.h>
#include <libethcore/miner.h>

#ifdef ETH_ETHASHCL
#include <libethash-cl/CLMiner.h>
#endif

#ifdef ETH_ETHASHCUDA
#include <libethash-cuda/CUDAMiner.h>
#endif

#if defined(__cplusplus)
extern "C" {
#endif
    bool progpow_destroy(void* miner);
    void* progpow_gpu_init(unsigned device, unsigned driver);
    void progpow_gpu_configure(uint32_t devicesCount);
    void progpow_gpu_compute(void* miner, const void* header, uint64_t height, int epoch, uint64_t boundary, uint64_t startNonce);
    bool progpow_gpu_get_solutions(void* miner, void* data);
#if defined(__cplusplus)
}
#endif