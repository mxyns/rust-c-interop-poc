#include "uwe_rs_lib.h"
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <stdlib.h>
#include <arpa/inet.h>

#include "../lib/uwe_c_lib.h"

void show_StructWithOpt(StructWithOpt opt) {
  if (opt.ptr) {
    printf("%p -> %u\n", opt.ptr, opt.ptr->x);
  } else {
    printf("%p\n", opt.ptr);
  }
}

int main() {

  void* ptr = NULL; // malloc(69);
  printf("%p\n", ptr);

  StructWithOpt with = make_opt(true);
  // StructWithOpt without = make_opt(false);

  show_StructWithOpt(with);
  // show_StructWithOpt(without);




  return 0;
    if (is_uwe_the_goat()) {
        printf("Yes Uwe is the GOAT\n");
    }

    if (get_random_value() < 0.5) {
        printf("And he is double GOAT too\n");
    }

    const size_t buffer_len = 20;
    char buffer[buffer_len];
    memset(buffer, 0, buffer_len);

    /* using htonl because Rust lib expects NetworkEndian */
    *(uint16_t *) buffer = htonl(1); /* set msg type */
    *(uint32_t *) (buffer + sizeof(uint16_t)) = htonl(sizeof(buffer) - sizeof(uint16_t) - sizeof(uint32_t)); /* set msg length */

    for (int i = 0; i < buffer_len; i++) { printf("%d ", buffer[i]); }
    printf("\n");

    TransparentHeader *hdr = read_transparent_buffer(buffer, buffer_len);
    printf("Type is %d, length is %d\n", hdr->pkt_type, hdr->pkt_len);

    if (get_random_value() < 0.5) {
        printf("Random chose to use C free; it works\n");
        free(hdr);
    } else {
        free_transparent_header(hdr);
        printf("Random chose to use Rust free; it works\n");
    }

    /* remove this call is you want a perfect valgrind leak-check report
     * Rust's println! macro does not free some memory
     * but this memory size does not scale with anything (call count, format size and whatnot) */
    print_my_number(69);

    return 0;
}