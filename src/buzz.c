#include <ctype.h>
#include <stdlib.h>
#include <unistd.h>

#include "buzz.h"

void print_tag_pattern(TagPattern tag_pattern) {
    printf("TagPattern: %s\n", tag_pattern.pattern);
    printf("\tlength: %d\n", tag_pattern.length);
    printf("\tcapture_from: %d\n", tag_pattern.capture_from);
    printf("\tcapture_until: %d\n", tag_pattern.capture_until);
    printf("\tparse_status: %d\n", tag_pattern.parse_status);
};

TagPattern read_pattern(char* raw_pattern) {
    int length;
    int capture_from = -1;
    char capture_until = -1;
    char previous_char = '\0';
    char* current_char = raw_pattern;
    int parse_status = PARSE_VALID;
    for (length = 0; *current_char != '\0'; length++) {
        if isspace(*current_char) {
            parse_status += PARSE_CATASTROPHIC;
            break;
        }
        if (previous_char == BUGOUT_BUZZ_CAPTURE_CHAR && *current_char != BUGOUT_BUZZ_CAPTURE_CHAR) {
            capture_until = length;
        }
        if (*current_char == BUGOUT_BUZZ_CAPTURE_CHAR) {
            if (capture_from > 0) {
                parse_status += PARSE_WARN;
            } else {
                capture_from = length;
            }
        }
        previous_char = *current_char++;
    }
    TagPattern tag_pattern;
    tag_pattern.length = length;
    tag_pattern.pattern = raw_pattern;
    tag_pattern.capture_from = capture_from;
    tag_pattern.capture_until = capture_until;
    tag_pattern.parse_status = parse_status;
    return tag_pattern;
};
