#ifndef _BUGOUT_BUZZ_H
#define _BUGOUT_BUZZ_H

#include <stdio.h>

#define BUGOUT_BUZZ_WILDCARD_CHAR '*'
#define BUGOUT_BUZZ_CAPTURE_CHAR '#'
#define BUGOUT_BUZZ_BOUNDARY_START_CHAR '<'
#define BUGOUT_BUZZ_BOUNDARY_END_CHAR '>'

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
 */
TagPattern read_pattern(char* raw_pattern);

/**
 * Reads a list of TagPattern objects from a file.
 */
TagPattern* read_patterns(FILE* patterns_file_pointer);

#endif // _BUGOUT_BUZZ_H
