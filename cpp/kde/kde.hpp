#include <cblas.h>

#include <cstdint>

#include "gaussian.hpp"

float kde(float bandwidth,
          uint32_t n,
          float* x,
          float** data,
          float (*kernel)(float* data));
