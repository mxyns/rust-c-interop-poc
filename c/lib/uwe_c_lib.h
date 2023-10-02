// SPDX-License-Identifier: MIT

#ifndef UWE_C_LIB_H
#define UWE_C_LIB_H

#include <stdint.h>

struct CoolNumber {
    uint8_t value;
    float random;
};

struct CoolNumber my_cool_number(uint8_t value);

#endif /* UWE_C_LIB_H */
