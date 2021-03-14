#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

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
        printf("\t- Incorrect boundary character: '%c'. Expected: '%c'.\n", tag_pattern.boundary.character, expected_boundary_character);
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
    if (strcmp(tag_pattern.pattern, expected_pattern)) {
        printf("\t- Incorrect pattern: \"%s\". Expected: \"%s\".\n", tag_pattern.pattern, expected_pattern);
        result = 1;
    }

    if (result == 0) {
        printf("\t- SUCCESS!\n");
    } else {
        printf("\t- FAILURE!\n");
    }

    free(tag_pattern.pattern);

    return result;
}

int test_load_pattern() {
    TagPatternList* patterns = NULL;
    char* pattern1 = "os:Windows";
    char* pattern2 = "python:#<1>.";
    patterns = load_pattern(patterns, pattern1);
    patterns = load_pattern(patterns, pattern2);
    int result = 0;

    TagPatternList* head = patterns;
    if (head == NULL) {
        printf("\t- Patterns list is empty\n");
        result = 1;
    }

    int num_vertices = 0;
    while (patterns != NULL) {
        num_vertices++;
        patterns = patterns->next;
    }
    if (num_vertices != 2) {
        printf("\t- Expected %d vertices, actual: %d\n", 2, num_vertices);
        result = 1;
    }

    patterns = head;
    if (strcmp(patterns->tag_pattern.pattern, pattern2)) {
        printf("\t- Unexpected pattern string in first pattern: expected - %s, actual - %s\n", pattern2, patterns->tag_pattern.pattern);
        result = 1;
    }
    patterns = patterns->next;
    if (strcmp(patterns->tag_pattern.pattern, pattern1)) {
        printf("\t- Unexpected pattern string in first pattern: expected - %s, actual - %s\n", pattern1, patterns->tag_pattern.pattern);
        result = 1;
    }

    if (result == 0) {
        printf("\t- SUCCESS!\n");
    } else {
        printf("\t- FAILURE!\n");
    }

    destroy(head);
    return result;
}

int test_process_tag(char* raw_pattern, char* tag, bool expected_match, int expected_capture_start, int expected_capture_end) {
    TagPattern tag_pattern = read_pattern(raw_pattern);
    int result = 0;

    BuzzResult buzz_result = process_tag(tag, &tag_pattern);

    if (buzz_result.match != expected_match) {
        printf("\t- Unexpected match value: actual - %d, expected - %d\n", buzz_result.match, expected_match);
        result = 1;
    }

    if (buzz_result.capture_start != expected_capture_start) {
        printf("\t- Unexpected capture_start value: actual - %d, expected - %d\n", buzz_result.capture_start, expected_capture_start);
        result = 1;
    }

    if (buzz_result.capture_end != expected_capture_end) {
        printf("\t- Unexpected capture_end value: actual - %d, expected - %d\n", buzz_result.capture_end, expected_capture_end);
        result = 1;
    }

    if (result == 0) {
        printf("\t- SUCCESS!\n");
    } else {
        printf("\t- FAILURE!\n");
    }

    free(tag_pattern.pattern);
    return result;
};

int main(int argc, char* argv[]) {
    int result = 0;

    printf("Testing simple pattern...\n");
    result += test_read_pattern("<a>", "<a>", 3, -1, '\0', -1, -1, PARSE_VALID);

    printf("Testing pattern with capture at end...\n");
    result += test_read_pattern("os:#<0>", "os:#<0>", 7, 3, '\0', 0, -1, PARSE_VALID);

    printf("Testing pattern with capture in middle...\n");
    result += test_read_pattern("python:#<1>.", "python:#<1>.", 12, 7, '.', 1, 11, PARSE_VALID);

    printf("Testing pattern with syntactically incorrect capture...\n");
    result += test_read_pattern("python:#<a>.", "python:#<a>.", 7, 7, '\0', -1, -1, PARSE_INVALID);

    printf("Testing pattern with simple capture...\n");
    result += test_read_pattern("python:#.", "python:#.", 9, 7, '.', 0, 8, PARSE_VALID);

    printf("Testing pattern with capture at the end...\n");
    result += test_read_pattern("python:#", "python:#", 8, 7, '\0', 0, -1, PARSE_VALID);

    printf("Testing pattern with multiple captures...\n");
    result += test_read_pattern("omg#<0>*wtf#<0>*bbq", "omg#<0>*wtf#<0>*bbq", 11, 3, '*', 0, 7, PARSE_INVALID);

    printf("Testing pattern with whitespace in it...\n");
    result += test_read_pattern("omg wtf bbq", "omg wtf bbq", 3, -1, '\0', -1, -1, PARSE_INVALID);

    printf("Testing pattern with capture after wildcard...\n");
    result += test_read_pattern("omg*#", "omg*#", 4, -1, '\0', -1, -1, PARSE_NO_CAPTURE_AFTER_WILDCARD);

    printf("Testing pattern with wildcard after wildcard...\n");
    result += test_read_pattern("omg**", "omg**", 4, -1, '\0', -1, -1, PARSE_NO_WILDCARD_AFTER_WILDCARD);

    printf("Testing load_patterns with 2 patterns loaded into an empty patterns list...\n");
    result += test_load_pattern();

    printf("Testing matching tag against simple pattern...\n");
    result += test_process_tag("os:Linux", "os:Linux", true, -1, -1);

    printf("Testing non-matching tag against simple pattern...\n");
    result += test_process_tag("os:Linux", "os:Windows", false, -1, -1);

    printf("Testing matching tag against pattern with ending wildcard...\n");
    result += test_process_tag("os:*", "os:Windows", true, -1, -1);

    printf("Testing processing non-matching tag against pattern with ending wildcard...\n");
    result += test_process_tag("os:*", "python:3", false, -1, -1);

    printf("Testing matching tag against pattern with non-trailing wildcard...\n");
    result += test_process_tag("os:*x", "os:Linux", true, -1, -1);

    printf("Testing non-matching tag against pattern with non-trailing wildcard...\n");
    result += test_process_tag("os:*x", "os:Windows", false, -1, -1);

    printf("Testing matching tag against pattern with multiple wildcards...\n");
    result += test_process_tag("os:*u*", "os:Linux", true, -1, -1);

    printf("Testing matching tag against pattern with multiple wildcards and empty wildcard matches...\n");
    result += test_process_tag("os:*n*u*x", "os:Linux", true, -1, -1);

    printf("Testing nonmatching tag against pattern with multiple wildcards...\n");
    result += test_process_tag("os:*u*", "os:Windows", false, -1, -1);

    printf("Testing matching tag against pattern with trailing capture...\n");
    result += test_process_tag("python:#", "python:3", true, 7, -1);

    printf("Testing matching tag against pattern with trailing long capture...\n");
    result += test_process_tag("python:#", "python:38", true, 7, -1);

    printf("Testing matching tag against pattern with trailing explicit long capture...\n");
    result += test_process_tag("python:#<5>", "python:38", true, 7, -1);

    printf("Testing matching tag against pattern with non-trailing capture, stop character, and trailing wildcard...\n");
    result += test_process_tag("python:#.*", "python:3.8.5", true, 7, 8);

    printf("Testing non-matching tag against pattern with non-trailing capture, stop character, and trailing wildcard...\n");
    result += test_process_tag("python:#.*", "python:3,8,5", false, 7, 11);

    printf("Testing matching tag against pattern with non-trailing capture, stop character, and skips...\n");
    result += test_process_tag("python:#<1>.*", "python:3.8.5", true, 7, 10);

    printf("Testing extravagantly non-matching tag against pattern with non-trailing capture, stop character, and skips...\n");
    result += test_process_tag("python:#<5>.", "python:3.8.5", false, 7, 11);

    printf("Testing barely non-matching tag against pattern with non-trailing capture, stop character, and skips...\n");
    result += test_process_tag("python:#<2>.", "python:3.8.5", false, 7, 11);

    printf("Testing matching tag against pattern with wildcard then simple capture then wildcard...\n");
    result += test_process_tag("*:#.*", "python:3.8.5", true, 7, 8);

    printf("FAILURES: %d\n", result);

    return result;
}
