#include "hashmap/hashmap.h"
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

struct entry {
    char* key;
    uint32_t value;
};

uint64_t hash(const void* e, uint64_t seed0, uint64_t seed1) {
    const struct entry* entry = e;
    return hashmap_sip(entry->key, strlen(entry->key), seed0, seed1);
}

int compare(const void* e1, const void* e2, void* data) {

    const struct entry* entry1 = e1;
    const struct entry* entry2 = e2;

    return strcmp(entry1->key, entry2->key);
}

void elfree(void* e) {
    const struct entry* entry = e;
    free(entry->key);
}

int main(int argc, char** argv) {
    if (argc < 2) {
        fprintf(stderr, "hashmap <count>\n");
        return 1;
    }
    uint32_t count;
    int r = sscanf(argv[1], "%u\n", &count);
    if (r == 0 || r == EOF) {
        fprintf(stderr, "hashmap <count>\n");
        return 1;
    }

    struct hashmap* map = hashmap_new(sizeof(struct entry), 0, 0, 0, hash, compare, elfree, NULL);

    for (uint32_t i = 0; i < count; i++) {
        int size = i == 0 ? 2 : (int)(ceil(log10(i))+1) ;
        char* key = malloc(size);
        snprintf(key, size, "%d", i);

        hashmap_set(map, &(struct entry) { .key = key, .value = i});
    }

    hashmap_free(map);
}
