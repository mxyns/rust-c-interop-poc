//
// Created by max on 10/2/2023.
//

#include "uwe_c_lib.h"
#include "uwe_rs_lib.h"
#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdbool.h>


struct CoolNumber my_cool_number(uint8_t value) {
    struct CoolNumber cool = {
        .value =  value,
        .random = get_random_value()
    };

    printf("C made a cool number of value %d\n", cool.value);

    return cool;
}