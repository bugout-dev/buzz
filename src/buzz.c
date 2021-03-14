#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#include "buzz.h"

void destroy(TagPatternList* tag_pattern_list) {
    TagPatternList* current_item = tag_pattern_list;
    TagPatternList* next_item = NULL;
    while (current_item != NULL) {
        next_item = current_item->next;
        free(current_item->tag_pattern.pattern);
        free(current_item);
        current_item = next_item;
    }
};

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
    bool done = false;
    // Parse skip
    int current_index = start;
    if (tag_pattern->pattern[current_index] != BUGOUT_BUZZ_BOUNDARY_START_CHAR) {
        boundary.character = tag_pattern->pattern[current_index];
        boundary.resume = current_index;
        done = true;
    }
    if (tag_pattern->pattern[current_index] == '\0') {
        boundary.resume = -1;
        done = true;
    }
    if (!done) {
        current_index++;
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
        done = true;
    }
    tag_pattern->length = current_index;
    tag_pattern->boundary = boundary;
};

TagPattern read_pattern(char* raw_pattern) {
    TagPattern tag_pattern;
    tag_pattern.boundary.character = '\0';
    tag_pattern.boundary.resume = -1;
    tag_pattern.boundary.skip = -1;
    tag_pattern.length = 0;
    tag_pattern.capture_from = -1;
    tag_pattern.parse_status = PARSE_VALID;

    bool post_wildcard = false;

    int raw_pattern_length = 0;
    while (raw_pattern[raw_pattern_length++] != '\0') {
        if (raw_pattern_length >= BUGOUT_BUZZ_MAX_PATTERN_LENGTH) {
            tag_pattern.parse_status = PARSE_INVALID;
            break;
        }
    }
    char* pattern_string = (char*) malloc(raw_pattern_length+1);
    for (int i = 0; i <= raw_pattern_length; i++) {
        pattern_string[i] = raw_pattern[i];
    }
    pattern_string[raw_pattern_length] = '\0';
    tag_pattern.pattern = pattern_string;

    while (raw_pattern[tag_pattern.length] != '\0') {
        if isspace(raw_pattern[tag_pattern.length]) {
            tag_pattern.parse_status = PARSE_INVALID;
            break;
        } else if (raw_pattern[tag_pattern.length] == BUGOUT_BUZZ_CAPTURE_CHAR) {
            if (tag_pattern.capture_from > 0) {
                tag_pattern.parse_status = PARSE_INVALID;
                break;
            } else if (post_wildcard) {
                tag_pattern.parse_status = PARSE_NO_CAPTURE_AFTER_WILDCARD;
                break;
            } else {
                tag_pattern.capture_from = tag_pattern.length;
                process_boundary(&tag_pattern, tag_pattern.length+1);
                if (tag_pattern.parse_status != PARSE_VALID || tag_pattern.boundary.resume == -1) {
                    break;
                }
            }
        }
        if (raw_pattern[tag_pattern.length] == BUGOUT_BUZZ_WILDCARD_CHAR) {
            post_wildcard = true;
        } else {
            post_wildcard = false;
        }
        tag_pattern.length++;
    }
    tag_pattern.pattern = pattern_string;
    return tag_pattern;
};

TagPatternList* load_pattern(TagPatternList* pattern_list, char* raw_pattern) {
    TagPattern tag_pattern = read_pattern(raw_pattern);
    if (tag_pattern.parse_status != PARSE_VALID) {
        // Do not load invalid patterns
        return pattern_list;
    }

    TagPatternList* head = (TagPatternList*) malloc(sizeof(TagPatternList));
    head->tag_pattern = tag_pattern;
    head->next = pattern_list;
    return head;
};

TagPatternList* load_patterns_from_file(FILE* ifp) {
    TagPatternList* patterns = NULL;

    char c = getc(ifp);
    char buffer[BUGOUT_BUZZ_MAX_PATTERN_LENGTH + 1];
    int current_index = 0;
    bool keep_processing = true;
    int i = 0;

    while (c != EOF) {
        if (current_index >= BUGOUT_BUZZ_MAX_PATTERN_LENGTH) {
            keep_processing = false;
        }
        if (isspace(c)) {
            if (keep_processing && current_index > 0) {
                buffer[current_index] = '\0';
                patterns = load_pattern(patterns, buffer);
            }
            current_index = 0;
        } else if (keep_processing) {
            buffer[current_index++] = c;
        }
        c = getc(ifp);
    }

    return patterns;
};

BuzzResult process_tag(char* tag, TagPattern* tag_pattern) {
    int tag_index = 0;
    int pattern_index = 0;

    BuzzResult result;
    result.tag = tag;
    result.tag_pattern = tag_pattern;
    result.match = true;
    result.capture_start = -1;
    result.capture_end = -1;

    bool inside_nontrailing_wildcard = false;

    while (result.match) {
        if (tag[tag_index] == '\0') {
            break;
        }

        if (pattern_index >= tag_pattern->length) {
            break;
        }

        char pattern_current = tag_pattern->pattern[pattern_index];
        if (pattern_current == BUGOUT_BUZZ_WILDCARD_CHAR) {
            char pattern_next = '\0';
            if (pattern_index + 1 < tag_pattern->length) {
                pattern_next = tag_pattern->pattern[pattern_index+1];
            }

            if (pattern_next != '\0') {
                inside_nontrailing_wildcard = true;
            }

            if (tag[tag_index] == pattern_next) {
                inside_nontrailing_wildcard = false;
                pattern_index++;
            }

            tag_index++;
        } else if (pattern_current == BUGOUT_BUZZ_CAPTURE_CHAR) {
            // Skip pattern ahead to where capture definition ends.
            pattern_index = tag_pattern->boundary.resume;
            if (pattern_index == -1) {
                pattern_index = tag_pattern->length;
            }
            result.capture_start = tag_index;
            int num_skipchar_encounters = 0;
            while (tag[tag_index] != '\0') {
                if (tag[tag_index] == tag_pattern->boundary.character) {
                    if (++num_skipchar_encounters >= tag_pattern->boundary.skip) {
                        break;
                    } else {
                        tag_index++;
                    }
                } else {
                    tag_index++;
                }
            }
            result.capture_end = tag_index - 1;
        } else {
            result.match = (tag[tag_index++] == tag_pattern->pattern[pattern_index++]);
        }
    }

    if (inside_nontrailing_wildcard) {
        result.match = false;
    }

    return result;
};
