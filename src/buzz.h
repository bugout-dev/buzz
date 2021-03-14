#ifndef _BUGOUT_BUZZ_H
#define _BUGOUT_BUZZ_H

#include <stdio.h>

#define BUGOUT_BUZZ_WILDCARD_CHAR '*'
#define BUGOUT_BUZZ_CAPTURE_CHAR '#'
#define BUGOUT_BUZZ_BOUNDARY_START_CHAR '<'
#define BUGOUT_BUZZ_BOUNDARY_END_CHAR '>'
#define BUGOUT_BUZZ_MAX_PATTERN_LENGTH 512

enum _parse_status {
    PARSE_VALID = 0,
    PARSE_INVALID = 1,
};

typedef struct CaptureBoundary {
    char character;
    int skip;
    int resume;
} CaptureBoundary;

typedef struct TagPattern {
    int length;
    char* pattern;
    int capture_from;
    CaptureBoundary boundary;
    int parse_status;
} TagPattern;

void print_tag_pattern(TagPattern tag_pattern);

/**
 * Reads a single TagPattern object from a string.
 * The input string is copied and it is the caller's responsibility to free this memory when they
 * are done with it.
 */
TagPattern read_pattern(char* raw_pattern);

#endif // _BUGOUT_BUZZ_H
