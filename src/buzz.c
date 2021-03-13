#include <ctype.h>
#include <stdlib.h>
#include <unistd.h>

#include "buzz.h"

void print_tag_pattern(TagPattern tag_pattern) {
    printf("TagPattern: %s\n", tag_pattern.pattern);
    printf("\tlength: %d\n", tag_pattern.length);
    printf("\tcapture_from: %d\n", tag_pattern.capture_from);
    printf("\tboundary:\n");
    printf("\t\tcharacter: %c\n", tag_pattern.boundary.character);
    printf("\t\tskip: %d\n", tag_pattern.boundary.skip);
    printf("\t\tresume: %d\n", tag_pattern.boundary.resume);
    printf("\tparse_status: %d\n", tag_pattern.parse_status);
};

void process_boundary(TagPattern* tag_pattern, int start) {
    CaptureBoundary boundary;
    boundary.character = '\0';
    boundary.skip = 0;
    boundary.resume = -1;
    // Parse skip
    int current_index = start;
    if (tag_pattern->pattern[current_index++] != BUGOUT_BUZZ_BOUNDARY_START_CHAR) {
        tag_pattern->parse_status = PARSE_INVALID;
        return;
    }
    while (isdigit(tag_pattern->pattern[current_index])) {
        int digit_as_int = tag_pattern->pattern[current_index] - '0';
        boundary.skip = 10*boundary.skip + digit_as_int;
        current_index++;
    }
    if (tag_pattern->pattern[current_index++] != BUGOUT_BUZZ_BOUNDARY_END_CHAR) {
        tag_pattern->parse_status = PARSE_INVALID;
        return;
    }
    boundary.character = tag_pattern->pattern[current_index];
    if (boundary.character != '\0') {
        boundary.resume = current_index;
    } else {
        boundary.skip = 0;
    }
    tag_pattern->length = current_index;
    tag_pattern->boundary = boundary;
};

TagPattern read_pattern(char* raw_pattern) {
    TagPattern tag_pattern;
    tag_pattern.boundary.character = '\0';
    tag_pattern.boundary.resume = -1;
    tag_pattern.boundary.skip = -1;
    tag_pattern.pattern = raw_pattern;
    tag_pattern.length = 0;
    tag_pattern.capture_from = -1;
    tag_pattern.parse_status = PARSE_VALID;

    while (raw_pattern[tag_pattern.length] != '\0') {
        if isspace(raw_pattern[tag_pattern.length]) {
            tag_pattern.parse_status = PARSE_INVALID;
            break;
        }
        if (raw_pattern[tag_pattern.length] == BUGOUT_BUZZ_CAPTURE_CHAR) {
            if (tag_pattern.capture_from > 0) {
                tag_pattern.parse_status = PARSE_INVALID;
                break;
            } else {
                tag_pattern.capture_from = tag_pattern.length;
                process_boundary(&tag_pattern, tag_pattern.length+1);
                if (tag_pattern.parse_status == PARSE_INVALID || raw_pattern[tag_pattern.length] == '\0') {
                    break;
                }
            }
        }
        tag_pattern.length++;
    }
    return tag_pattern;
};