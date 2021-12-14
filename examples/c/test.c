#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <stdint.h>

int main() {
  char *error;
  void *handle; 
  uint64_t (*solve)(uint64_t, uint64_t(*)[], uint64_t);

  handle = dlopen("./libsolve.so", RTLD_LAZY);
  if (!handle) {
    fprintf(stderr, "unable to load `libsolve.so`: %s\n", dlerror());
    return 1;
  }

  dlerror();
  solve = dlsym(handle, "solve");
  error = dlerror();

  if (error != NULL) {
    fprintf(stderr, "`solve` not found: %s\n", error);
    return 1;
  }


  uint64_t factor_count = 2;
  uint64_t factors[] = {3, 5};
  uint64_t upper_bound = 10;
  if (solve(factor_count, &factors, upper_bound) != 3 + 5 + 6 + 9) {
    fprintf(stderr, "TEST0 was not correct value");
  }
  dlclose(handle);

  return 0;
}
