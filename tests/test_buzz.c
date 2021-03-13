#include "../src/buzz.h"

int test_read_pattern(char* raw_pattern, char* expected_pattern, int expected_length, int expected_capture_from, int expected_capture_until, int expected_parse_status) {
    int result = 0;
    TagPattern tag_pattern = read_pattern(raw_pattern);

    if (tag_pattern.parse_status != expected_parse_status) {
        printf("\t- Incorrect parse_status: %d. Expected: %d.\n", tag_pattern.parse_status, expected_parse_status);
        result = 1;
    }
    if (tag_pattern.length != expected_length) {
        printf("\t- Incorrect length: %d. Expected: %d.\n", tag_pattern.length, expected_length);
        result = 1;
    }
    if (tag_pattern.capture_from != expected_capture_from) {
        printf("\t- Incorrect capture_from: %d. Expected: %d.\n", tag_pattern.capture_from, expected_capture_from);
        result = 1;
    }
    if (tag_pattern.capture_until != expected_capture_until) {
        printf("\t- Incorrect capture_until: %d. Expected: %d.\n", tag_pattern.capture_until, expected_capture_until);
        result = 1;
    }
    if (tag_pattern.pattern != expected_pattern) {
        printf("\t- Incorrect pattern: \"%s\". Expected: \"%s\".\n", tag_pattern.pattern, expected_pattern);
        result = 1;
    }

    if (result == 0) {
        printf("\t- SUCCESS!\n");
    } else {
        printf("\t- FAILURE!\n");
    }

    return result;
}

int main(int argc, char* argv[]) {
    int result = 0;

    printf("Testing simple pattern...\n");
    result += test_read_pattern("<a>", "<a>", 3, -1, -1, PARSE_VALID);

    printf("Testing pattern with capture at end...\n");
    result += test_read_pattern("os:#", "os:#", 4, 3, -1, PARSE_VALID);

    printf("Testing pattern with capture in middle...\n");
    result += test_read_pattern("os:#inux", "os:#inux", 8, 3, 4, PARSE_VALID);

    printf("Testing pattern with multiple captures...\n");
    result += test_read_pattern("omg#wtf#bbq", "omg#wtf#bbq", 11, 3, 8, PARSE_WARN);

    printf("Testing pattern with whitespace in it...\n");
    result += test_read_pattern("omg wtf bbq", "omg wtf bbq", 3, -1, -1, PARSE_CATASTROPHIC);

    return result;
}
