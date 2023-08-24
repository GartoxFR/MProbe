#include "hashmap/hashmap.h"
#include "id_map.h"
#include "utf8proc/utf8proc.h"
#include <math.h>
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct file_score {
    const char* filename;
    double score;
};

struct file_term_count_entry {
    const char* filename;
    struct hashmap* term_count;
};

static void file_term_count_entry_free(void* item) {
    struct file_term_count_entry* entry = item;
    hashmap_free(entry->term_count);
}

static int file_term_count_entry_cmp(const void* a, const void* b,
                                     void* udata) {
    const struct file_term_count_entry* ca = a;
    const struct file_term_count_entry* cb = b;
    return ca->filename - cb->filename;
}

static uint64_t file_term_count_entry_hash(const void* item, uint64_t seed0,
                                           uint64_t seed1) {
    const struct file_term_count_entry* entry = item;
    return hashmap_sip(entry->filename, strlen(entry->filename), seed0, seed1);
}

struct count_entry {
    uint32_t id;
    uint32_t count;
};

static int count_entry_cmp(const void* a, const void* b, void* udata) {
    const struct count_entry* ca = a;
    const struct count_entry* cb = b;
    return ca->id - cb->id;
}

static uint64_t count_entry_hash(const void* item, uint64_t seed0,
                                 uint64_t seed1) {
    const struct count_entry* entry = item;
    return hashmap_sip(&entry->id, sizeof(entry->id), seed0, seed1);
}

static void process_line(const char* line, struct hashmap* term_count,
                         struct id_map* ids) {
    size_t line_len = strlen(line);
    utf8proc_int32_t* unicode_buffer =
        malloc((line_len + 1) * sizeof(utf8proc_int32_t));

    utf8proc_ssize_t buffer_len = utf8proc_decompose(
        (utf8proc_uint8_t*)line, line_len, unicode_buffer, line_len + 1, 0);

    if (buffer_len < 0) {
        fprintf(stderr, "Warning: Encountered invalid UTF-8....skipped a line");
        return;
    }

    size_t word_begin = 0;
    size_t word_end = 0;

    while (word_begin < buffer_len) {
        bool alpha = utf8proc_islower(unicode_buffer[word_end]) ||
                     utf8proc_isupper(unicode_buffer[word_end]);
        utf8proc_category_t category =
            utf8proc_category(unicode_buffer[word_end]);
        bool num = category == UTF8PROC_CATEGORY_ND ||
                   category == UTF8PROC_CATEGORY_NL ||
                   category == UTF8PROC_CATEGORY_NO;

        if ((alpha || num) && word_end < buffer_len) {
            // Not a separator so just go next chararcter
            word_end++;
        } else {
            // we found a separator so we can process the word
            size_t word_len = word_end - word_begin;
            if (word_len > 0) {
                // We lower all the chars
                for (size_t i = word_begin; i < word_end; i++) {
                    unicode_buffer[i] = utf8proc_tolower(unicode_buffer[i]);
                }

                utf8proc_ssize_t ret =
                    utf8proc_reencode(unicode_buffer + word_begin, word_len, 0);
                if (ret > 0) {
                    // printf("%s\n", (char*)(unicode_buffer + word_begin));
                    uint32_t id = id_map_get_or_register(
                        ids, (char*)(unicode_buffer + word_begin));

                    struct count_entry new_entry;
                    const struct count_entry* old_entry = hashmap_get(
                        term_count, &(struct count_entry){.id = id});
                    if (old_entry) {
                        new_entry = (struct count_entry){
                            .id = id, .count = old_entry->count + 1};
                    } else {
                        new_entry = (struct count_entry){.id = id, .count = 1};
                    }

                    hashmap_set(term_count, &new_entry);
                } else {
                    fprintf(stderr,
                            "Warning: Unable to reencode word...skipped");
                }
            }

            word_end++;
            word_begin = word_end;
        }
    }

    free(unicode_buffer);
}
static struct hashmap* parse_file(const char* filename, struct id_map* ids) {
    FILE* file = fopen(filename, "r");
    if (!file) {
        return NULL;
    }

    struct hashmap* term_count =
        hashmap_new(sizeof(struct count_entry), 0, 0, 0, count_entry_hash,
                    count_entry_cmp, NULL, NULL);

    const int chunk_size = 128;
    char chunk[chunk_size];

    size_t line_cap = chunk_size;
    size_t line_len = 0;
    char* line = malloc(line_cap);
    line[0] = '\0';

    while (fgets(chunk, sizeof(chunk), file) != NULL) {
        size_t chunk_len = strlen(chunk);
        if (line_cap - line_len - 1 < chunk_len) {
            line_cap *= 2;
            line = realloc(line, line_cap);
        }

        strncpy(line + line_len, chunk, line_cap - line_len);
        line_len += chunk_len;

        if (line[line_len - 1] == '\n') {
            line[line_len - 1] = '\0';

            process_line(line, term_count, ids);

            line[0] = '\0';
            line_len = 0;
        }
    }

    free(line);

    return term_count;
}

double compute_score(const char* query, struct hashmap* term_count,
                     struct hashmap* document_term_count, size_t total_document,
                     const struct id_map* ids) {
    double score = 0.0;
    uint32_t total_term_count = 0;
    size_t iter = 0;
    struct count_entry* entry;
    while (hashmap_iter(term_count, &iter, (void*)&entry)) {
        total_term_count += entry->count;
    }

    size_t query_len = strlen(query);
    utf8proc_int32_t* unicode_buffer =
        malloc((query_len + 1) * sizeof(utf8proc_int32_t));

    utf8proc_ssize_t buffer_len = utf8proc_decompose(
        (utf8proc_uint8_t*)query, query_len, unicode_buffer, query_len + 1, 0);

    if (buffer_len < 0) {
        fprintf(stderr, "Warning: Encountered invalid UTF-8 while parsing "
                        "query....skipped file");
        return score;
    }

    size_t word_begin = 0;
    size_t word_end = 0;

    while (word_begin < buffer_len) {
        bool alpha = utf8proc_islower(unicode_buffer[word_end]) ||
                     utf8proc_isupper(unicode_buffer[word_end]);
        utf8proc_category_t category =
            utf8proc_category(unicode_buffer[word_end]);
        bool num = category == UTF8PROC_CATEGORY_ND ||
                   category == UTF8PROC_CATEGORY_NL ||
                   category == UTF8PROC_CATEGORY_NO;

        if ((alpha || num) && word_end < buffer_len) {
            // Not a separator so just go next chararcter
            word_end++;
        } else {
            // we found a separator so we can process the word
            size_t word_len = word_end - word_begin;
            if (word_len > 0) {
                // We lower all the chars
                for (size_t i = word_begin; i < word_end; i++) {
                    unicode_buffer[i] = utf8proc_tolower(unicode_buffer[i]);
                }

                utf8proc_ssize_t ret =
                    utf8proc_reencode(unicode_buffer + word_begin, word_len, 0);
                if (ret > 0) {
                    uint32_t id =
                        id_map_get(ids, (char*)(unicode_buffer + word_begin));

                    if (id) {

                        const struct count_entry* term_count_entry =
                            hashmap_get(term_count,
                                        &(struct count_entry){.id = id});
                        const struct count_entry* document_term_count_entry =
                            hashmap_get(document_term_count,
                                        &(struct count_entry){.id = id});
                        // printf("%d %p %p\n", id, (void*) term_count_entry, (void*) document_term_count_entry);
                        if (term_count_entry && document_term_count_entry) {
                            double tf = term_count_entry->count /
                                        (double)total_term_count;
                            double idf =
                                log2(total_document /
                                     (double)document_term_count_entry->count);
                            score += tf * idf;
                        }
                    }
                } else {
                    fprintf(stderr,
                            "Warning: Unable to reencode word...skipped");
                }
            }

            word_end++;
            word_begin = word_end;
        }
    }

    free(unicode_buffer);
    return score;
}

int score_cmp(const void* a, const void* b) {
    const struct file_score* sa = a;
    const struct file_score* sb = b;
    if (sa->score > sb->score) {
        return -1;
    } else if (sa->score < sb->score) {
        return 1;
    } else {
        return 0;
    }
}
int main(int argc, char** argv) {

    if (argc < 3) {
        fprintf(stderr, "tf_idf <query> <FILES...>");
        exit(1);
    }

    struct id_map* ids = id_map_new();

    struct hashmap* document_term_count =
        hashmap_new(sizeof(struct count_entry), 0, 0, 0, count_entry_hash,
                    count_entry_cmp, NULL, NULL);

    struct hashmap* files_term_count =
        hashmap_new(sizeof(struct file_term_count_entry), 0, 0, 0,
                    file_term_count_entry_hash, file_term_count_entry_cmp,
                    file_term_count_entry_free, NULL);

    for (int i = 2; i < argc; i++) {
        struct hashmap* term_count = parse_file(argv[i], ids);
        if (!term_count) {
            continue;
        }

        size_t iter = 0;
        struct count_entry* entry;
        while (hashmap_iter(term_count, &iter, (void*)&entry)) {
            struct count_entry new_entry;

            const struct count_entry* old_entry = hashmap_get(
                document_term_count, &(struct count_entry){.id = entry->id});
            if (old_entry) {
                new_entry = (struct count_entry){.id = entry->id,
                                                 .count = old_entry->count + 1};
            } else {
                new_entry = (struct count_entry){.id = entry->id, .count = 1};
            }

            hashmap_set(document_term_count, &new_entry);
        }

        hashmap_set(files_term_count,
                    &(struct file_term_count_entry){.filename = argv[i],
                                                    .term_count = term_count});
    }

    size_t iter = 0;
    struct file_term_count_entry* entry;
    size_t score_len = hashmap_count(files_term_count);
    struct file_score* scores = malloc(sizeof(struct file_score) * score_len);
    size_t i = 0;
    while (hashmap_iter(files_term_count, &iter, (void*)&entry)) {
        double score = compute_score(argv[1], entry->term_count,
                                     document_term_count, score_len, ids);
        scores[i++] =
            (struct file_score){.filename = entry->filename, .score = score};
    }

    qsort(scores, score_len, sizeof(struct file_score), score_cmp);

    size_t len = score_len < 20 ? score_len : 20; 
    for (int i = 0; i < len; i++) {
        printf("%s: %lf\n", scores[i].filename, scores[i].score);
    }

    free(scores);
    id_map_free(ids);
    hashmap_free(document_term_count);
    hashmap_free(files_term_count);
    return 0;
}
