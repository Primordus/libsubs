#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include "movie_hash.h"

// Algorithm found at:
// http://trac.opensubtitles.org/projects/opensubtitles/wiki/HashSourceCodes

#define MAX(x,y) ((x > y) ? (x) : (y))

typedef enum
{
    OK,
    ERROR
} ResultType;

struct HashResult
{
    ResultType type;
    uint64_t hash;  // the actual hash.
};

/*
 * Helper function that does the actual computation.
 * The file handle should be a valid file handle to an episode!
 */
static uint64_t compute_hash(FILE* const handle)
{
    // TODO assert for validness of handle?
    uint64_t hash, file_size, tmp, i;

    // Calculate file_size:
    fseek(handle, 0, SEEK_END);
    file_size = ftell(handle);
    hash = file_size;

    // Calculate checksums:
    fseek(handle, 0, SEEK_SET);
    for(tmp = 0, i = 0; i < 65536 / sizeof(tmp) 
                     && fread((char*) &tmp, sizeof(tmp), 1, handle); 
        hash += tmp, ++i);

    fseek(handle, (long) MAX(0, file_size - 65536), SEEK_SET);
    for(tmp = 0, i = 0; i < 65536 / sizeof(tmp) 
                     && fread((char*) &tmp, sizeof(tmp), 1, handle);
        hash += tmp, ++i);

    return hash;
}

static HashResult* create_hash(const ResultType type,
                               const uint64_t value)
{
    HashResult* result = (HashResult*) malloc(sizeof(HashResult));
    if (!result)
    {
        return NULL;
    }

    result->type = type;
    result->hash = value;
    return result;
}

HashResult* calculate_hash(const char* const episode_name)
{
    FILE* handle = fopen(episode_name, "rb");
    if (!handle)
    {
        fclose(handle);
        return create_hash(ERROR, 0);
    }

    uint64_t hash = compute_hash(handle);
    fclose(handle);
    return create_hash(OK, hash);
}

void destroy_hash(HashResult* hash)
{
    free(hash);
}

bool is_valid_hash(const HashResult* const hash)
{
    if (hash == NULL || hash->type != OK)
    {
        return false;
    }

    return true;
}

uint64_t get_hash(const HashResult* const hash)
{
    if (is_valid_hash(hash)) return hash->hash;
    return 0;
}

