#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main(int argc, char **argv) {
    if (argc < 3) {
        fprintf(stderr, "axwy <alloc MB> <write MB>\n");
        return 1;
    }
    size_t alloc;
    size_t write;

    int r = sscanf(argv[1], "%ld", &alloc);
    if (r == 0 || r == EOF) {
        fprintf(stderr, "axwy <alloc MB> <write MB>\n");
        return 1;
    }

    r = sscanf(argv[2], "%ld", &write);

    if (r == 0 || r == EOF) {
        fprintf(stderr, "axwy <alloc MB> <write MB>\n");
        return 1;
    }

    if (write > alloc) {
        fprintf(stderr, "Error:  write should be less or equal than alloc\n");
        return 1;
    }

    alloc *= 1024 * 1024;
    write *= 1024 * 1024;

    // volatile here to prevent compiler optimization for benchmark purposes
    volatile uint8_t* arr = (uint8_t*) malloc(alloc);
    if (arr == NULL) {
        fprintf(stderr, "Error:  Failed to malloc\n");
        return 1;
    }
    for (int i = 0; i < write ; i++) {
        arr[i] = 42;
    }

    free((uint8_t*) arr);

    return 0;
}
