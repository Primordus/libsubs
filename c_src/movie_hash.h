#pragma once
#include <stdbool.h>

typedef struct HashResult HashResult;

/*
 * Calculates the movie hash for a certain episode name.
 * Returns pointer to a dynamically allocated hash result struct 
 * which may contain a valid hash if no errors occurred 
 * (be sure to check validness of the hash before actually retrieving it).
 */
HashResult* calculate_hash(const char* const episode_name);

/*
 * Frees the memory occupied by the hash result.
 * This has to be called after every time a new HashResult has been calculated
 * and the struct isn't used anymore..
 */
void destroy_hash(HashResult* hash);

/*
 * Function which checks if the hash was successfully computed.
 * The hash should be a previously allocated HashResult struct!
 * Returns true if a valid hash, otherwise false.
 */
bool is_valid_hash(const HashResult* const hash);

/*
 * Returns the hash contained in the result.
 * If hash was invalid, this returns 0.
 */
uint64_t get_hash(const HashResult* const hash);

