#pragma once

#include <libethash/internal.h>

using namespace dev;
using namespace eth;

class Work {
public:
    h256 header;
    h256 boundary;
    uint64_t height;
    int epoch;
    uint64_t startNonce = 0;

    Work(uint64_t height, h256 boundary, uint64_t startNonce, int epoch) {
        this->height = height;
        this->boundary = boundary;
        this->startNonce = startNonce;
        this->epoch = epoch;
    };
};