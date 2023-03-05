//
//  dot.c
//  convolutions_videoanim
//
//  Created by Jake Landers on 3/4/23.
//

#include "dot.h"

double dot_product(double* a, double* b, int length) {
    double result = 0.0;
    for (int i = 0; i < length; i++) {
        result += a[i] * b[i];
    }
    return result;
}
