#include <cstring>

#include "kde.hpp"

template <unsigned int D>
float kde(float bandwidth,
          float* x,
          float** data,
          float (*kernel)(float* data)) {
    float sum = 0;
    float temp[D];

    for (int i = 0; i < D; ++i) {
        memcpy((void*)temp, (void*)data[i], sizeof(float) * D);
        cblas_saxpy(D, -1.0, x, 1, temp, 1);
		sum += kernel(temp) / bandwidth;
    }

    return sum / (bandwidth * D);
}
