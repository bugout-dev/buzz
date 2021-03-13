#include "../src/buzz.h"

int test_read_pattern(char* raw_pattern, char* expected_pattern, int expected_length, int expected_capture_from, char expected_boundary_character, int expected_boundary_skip, int expected_boundary_resume, int expected_parse_status) {
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
    if (tag_pattern.boundary.character != expected_boundary_character) {
        printf("\t- Incorrect boundary character: %c. Expected: %c.\n", tag_pattern.boundary.character, expected_boundary_character);
        result = 1;
    }
    if (tag_pattern.boundary.skip != expected_boundary_skip) {
        printf("\t- Incorrect boundary skip: %d. Expected: %d.\n", tag_pattern.boundary.skip, expected_boundary_skip);
        result = 1;
    }
    if (tag_pattern.boundary.resume != expected_boundary_resume) {
        printf("\t- Incorrect boundary resume: %d. Expected: %d.\n", tag_pattern.boundary.resume, expected_boundary_resume);
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
    result += test_read_pattern("<a>", "<a>", 3, -1, '\0', -1, -1, PARSE_VALID);

    printf("Testing pattern with capture at end...\n");
    result += test_read_pattern("os:#<0>", "os:#<0>", 7, 3, '\0', 0, -1, PARSE_VALID);

    printf("Testing pattern with capture in middle...\n");
    result += test_read_pattern("python:#<1>.", "python:#<1>.", 12, 7, '.', 1, 11, PARSE_VALID);

    printf("Testing pattern with syntactically incorrect capture...\n");
    result += test_read_pattern("python:#.", "python:#.", 7, 7, '\0', -1, -1, PARSE_INVALID);

    printf("Testing pattern with multiple captures...\n");
    result += test_read_pattern("omg#<0>.wtf#<0>.bbq", "omg#<0>.wtf#<0>.bbq", 11, 3, '.', 0, 7, PARSE_INVALID);

    printf("Testing pattern with whitespace in it...\n");
    result += test_read_pattern("omg wtf bbq", "omg wtf bbq", 3, -1, '\0', -1, -1, PARSE_INVALID);

    return result;
}
