#ifndef ID_MAP
#define ID_MAP

#include <stdint.h>
struct id_map;

struct id_map* id_map_new();
void id_map_free(struct id_map* id_map);

// 0 means not found
uint32_t id_map_get(const struct id_map* map, const char* word);
uint32_t id_map_register(struct id_map* map, const char* word);
uint32_t id_map_get_or_register(struct id_map* map, const char* word);

#endif
