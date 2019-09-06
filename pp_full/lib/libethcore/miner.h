#pragma once

#include <libethash/internal.h>

class Miner{
public:
    virtual void compute(const void* header, uint64_t height, int epoch, uint64_t target, uint64_t startNonce) = 0;
    virtual bool get_solutions(void* data) = 0;
};
