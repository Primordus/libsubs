#include <stdio.h>
#include <stdint.h>
#include "movie_hash.h"

// Algorithm found at:
// http://trac.opensubtitles.org/projects/opensubtitles/wiki/HashSourceCodes

#define MAX(x,y) ((x > y) ? (x) : (y))

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

HashResult calc_hash(const char* const episode_name)
{
    FILE* handle = fopen(episode_name, "rb");
    if (!handle)
    {
        fclose(handle);
        HashResult result = { .type = ERROR, .hash = 0 };
        return result;
    }

    uint64_t hash = compute_hash(handle);
    fclose(handle);
    HashResult result = { .type = OK, .hash = hash };
    return result;
}

