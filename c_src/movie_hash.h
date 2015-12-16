#pragma once

typedef enum
{
    OK,
    FILE_NOT_FOUND
} ResultType;

typedef struct
{
    ResultType type;
    uint64_t hash;  // the actual hash.
} HashResult;

/*
 * Calculates the movie hash for a certain episode name.
 * Returns the hash for that episode or 0xffffffff on error (e.g. wrong episode name)
 */
HashResult calc_hash(const char* const episode_name);
