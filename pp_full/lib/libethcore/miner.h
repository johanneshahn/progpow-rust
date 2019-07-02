#pragma once

#include <libethash/internal.h>

class Miner{
public:
    virtual void compute(const void* header, size_t header_size, uint64_t height, int epoch, uint64_t boundary) = 0;
    virtual bool get_solutions(void* data) = 0;
};
