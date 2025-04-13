#ifndef _SPECQSORTH_
#define _SPECQSORTH_

// #include <stddef.h>
#include <zkvm.h>

#ifdef __cplusplus
extern "C"
#endif
void spec_qsort(void *array, size_t nitems, size_t size, int (*cmp)(const void*,const void*));
#endif
