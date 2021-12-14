#include <stdint.h>
#include <stdio.h>

uint64_t solve(
    uint64_t factor_count, 
    uint64_t (*factors)[factor_count], 
    uint64_t upper_bound
) {
  uint64_t sum = 0;
  uint64_t multiple;
  for (multiple = 1; multiple < upper_bound ; multiple++) {
    uint64_t i;
    for (i = 0; i < factor_count ; i++) {
      if (multiple % (*factors)[i] == 0) {
        sum = sum + multiple;
        break;
      }
    }
  }
  return sum;
}

int main() {
  uint64_t factor_count = 2;
  uint64_t factors[] = {3, 5};
  uint64_t upper_bound = 10;
  printf("%ld", solve(factor_count, &factors, upper_bound));

  return 0;
}
