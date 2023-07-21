#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "hashmap/hashmap.h"
#include <stdlib.h>

struct id_entry {
    const char* word;
    uint32_t id;
};

struct id_map {
    struct hashmap* inner;
    uint32_t next_id;
};

static int id_entry_compare(const void* a, const void* b, void* udata) {
    const struct id_entry* ia = a;
    const struct id_entry* ib = b;
    return strcmp(ia->word, ib->word);
}

static uint64_t id_entry_hash(const void* item, uint64_t seed0, uint64_t seed1) {
    const struct id_entry* id_entry = item;
    return hashmap_sip(id_entry->word, strlen(id_entry->word), seed0, seed1);
}

static void id_entry_free(void* item) {
    struct id_entry* id_entry = item;
    free((char*) id_entry->word);
}

struct id_map* id_map_new() {
    struct id_map* id_map = malloc(sizeof(struct id_map));

    id_map->inner = hashmap_new(sizeof(struct id_entry), 0, 0, 0, id_entry_hash, id_entry_compare, id_entry_free, NULL);
    id_map->next_id = 1;
    
    return id_map;
}

void id_map_free(struct id_map* id_map) {
    hashmap_free(id_map->inner);
    free(id_map);
}

// 0 means not found
uint32_t id_map_get(const struct id_map* map, const char* word) {
    const struct id_entry* entry = hashmap_get(map->inner, &(struct id_entry) {.word = word});
    
    return entry ? entry->id : 0;
}

uint32_t id_map_register(struct id_map* map, const char* word) {
    size_t word_len = strlen(word);
    // printf("%td\n", word_len);
    char* owned_word = malloc(word_len + 1);
    strncpy(owned_word, word, word_len + 1);
    uint32_t id = map->next_id;
    map->next_id++;
    hashmap_set(map->inner, &(struct id_entry) {.word = owned_word, .id = id});
    return id;
}

uint32_t id_map_get_or_register(struct id_map* map, const char* word) {
    uint32_t id = id_map_get(map, word);
    if (!id) {
        id = id_map_register(map, word);
    }

    return id;
}
