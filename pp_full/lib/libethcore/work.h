#pragma once

#include <libethash/internal.h>

using namespace dev;
using namespace eth;

class Work {
public:
    h256 *header;
    uint64_t height;
    int epoch;
    uint64_t startNonce = 0;
    uint64_t target;

    Work(uint64_t height, uint64_t startNonce, int epoch, uint64_t target) {
        this->header = new h256{0};
        this->height = height;
        this->target = target;
        this->startNonce = startNonce;
        this->epoch = epoch;
    };
};