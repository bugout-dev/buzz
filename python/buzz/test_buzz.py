from typing import Union, List

from buzz import (
    read_pattern,
    read_tag,
    load_pattern,
    Boundary,
    TagPattern,
    MatchingResult,
)


def test_read_pattern(
    raw_pattern: str,
    expected_pattern: str,
    expected_length: int,
    expected_capture_from: int,
    expected_boundary_character: str,
    expected_boundary_skip: int,
    expected_boundary_resume: int,
    expected_parse_status: int,
):
    result = 0
    tag_pattern: TagPattern = read_pattern(raw_pattern)

    if tag_pattern.valid != expected_parse_status:
        print(
            "\t- Incorrect parse_status: {}. Expected: {}.\n".format(
                tag_pattern.valid, expected_parse_status
            )
        )
        result = 1

    if len(tag_pattern.raw) != expected_length:
        print(
            "\t- Incorrect length: {}. Expected: {}.\n".format(
                len(tag_pattern.raw), expected_length
            )
        )
        result = 1

    if tag_pattern.patern_catch_sign_position != expected_capture_from:
        print(
            "\t- Incorrect capture_from: {}. Expected: {}.\n".format(
                tag_pattern.patern_catch_sign_position, expected_capture_from
            )
        )
        result = 1

    if tag_pattern.boundary.character != expected_boundary_character:
        print(
            "\t- Incorrect boundary character: '{}'. Expected: '{}'.\n".format(
                tag_pattern.boundary.character, expected_boundary_character
            )
        )
        result = 1

    if tag_pattern.boundary.skip != expected_boundary_skip:
        print(
            "\t- Incorrect boundary skip: {}. Expected: {}.\n".format(
                tag_pattern.boundary.skip, expected_boundary_skip
            )
        )
        result = 1

    if tag_pattern.boundary.resume != expected_boundary_resume:
        print(
            "\t- Incorrect boundary resume: {}. Expected: {}.\n".format(
                tag_pattern.boundary.resume, expected_boundary_resume
            )
        )
        result = 1

    if tag_pattern.raw != expected_pattern:
        print(
            '\t- Incorrect pattern: "{}". Expected: "{}".\n'.format(
                tag_pattern.raw, expected_pattern
            )
        )
        result = 1

    if result == 0:
        print("\t- SUCCESS!\n")
    else:
        print("\t- FAILURE!\n")

    return result


def test_load_pattern():
    patterns = []
    pattern1: str = "os:Windows"
    pattern2: str = "python:#<1>."
    load_pattern(patterns, pattern1)
    load_pattern(patterns, pattern2)
    result = 0

    if not patterns:
        print("\t- Patterns list is empty\n")
        result = 1

    num_vertices = len(patterns)

    if num_vertices != 2:
        print("\t- Expected {} vertices, actual: {}\n", 2, num_vertices)
        result = 1

    if patterns[1].raw != pattern2:
        print(
            "\t- Unexpected pattern string in first pattern: expected - {}, actual - {}\n",
            pattern2,
            patterns[1].raw,
        )
        result = 1

    if patterns[0].raw != pattern1:
        print(
            "\t- Unexpected pattern string in first pattern: expected - {}, actual - {}\n",
            pattern1,
            patterns[0].raw,
        )
        result = 1

    if result == 0:
        print("\t- SUCCESS!\n")
    else:
        print("\t- FAILURE!\n")

    return result


def test_process_tag(raw_pattern: str, tag: str, expected_match: bool):
    tag_pattern: TagPattern = read_pattern(raw_pattern)
    result = 0

    buzz_result: MatchingResult = read_tag(tag, tag_pattern)

    if buzz_result.match != expected_match:
        print(
            "\t- Unexpected match value: actual - {}, expected - {}\n",
            buzz_result.match,
            expected_match,
        )
        result = 1

    if result == 0:
        print("\t- SUCCESS!\n")
    else:
        print("\t- FAILURE!\n")

    return result


def main():
    result = 0

    print("Testing simple pattern...\n")
    result += test_read_pattern("<a>", "<a>", 3, -1, False, -1, -1, True)

    print("Testing pattern with capture at end...\n")
    result += test_read_pattern("os:#<0>", "os:#<0>", 7, 3, False, 0, -1, True)

    print("Testing pattern with capture in middle...\n")
    result += test_read_pattern("python:#<1>.", "python:#<1>.", 12, 7, ".", 1, 11, True)

    print("Testing pattern with syntactically incorrect capture...\n")
    result += test_read_pattern(
        "python:#<a>.", "python:#<a>.", 12, 7, False, -1, -1, False
    )

    print("Testing pattern with simple capture...\n")
    result += test_read_pattern("python:#.", "python:#.", 9, 7, ".", 0, 8, True)

    print("Testing pattern with capture at the end...\n")
    result += test_read_pattern("python:#", "python:#", 8, 7, False, 0, -1, True)

    print("Testing pattern with multiple captures...\n")
    result += test_read_pattern(
        "omg#<0>*wtf#<0>*bbq", "omg#<0>*wtf#<0>*bbq", 19, 3, "*", 0, 7, False
    )

    print("Testing pattern with whitespace in it...\n")
    result += test_read_pattern(
        "omg wtf bbq", "omg wtf bbq", 11, -1, False, -1, -1, False
    )

    print("Testing pattern with capture after wildcard...\n")
    result += test_read_pattern("omg*#", "omg*#", 5, -1, False, -1, -1, False)

    print("Testing pattern with wildcard after wildcard...\n")
    result += test_read_pattern("omg**", "omg**", 5, -1, False, -1, -1, False)

    print(
        "Testing load_patterns with 2 patterns loaded into an empty patterns list...\n"
    )
    result += test_load_pattern()

    print("Testing matching tag against simple pattern...\n")
    result += test_process_tag("os:Linux", "os:Linux", True)

    print("Testing non-matching tag against simple pattern...\n")
    result += test_process_tag("os:Linux", "os:Windows", False)

    print("Testing matching tag against pattern with ending wildcard...\n")
    result += test_process_tag("os:*", "os:Windows", True)

    print(
        "Testing processing non-matching tag against pattern with ending wildcard...\n"
    )
    result += test_process_tag("os:*", "python:3", False)

    print("Testing matching tag against pattern with non-trailing wildcard...\n")
    result += test_process_tag("os:*x", "os:Linux", True)

    print("Testing non-matching tag against pattern with non-trailing wildcard...\n")
    result += test_process_tag(
        "os:*x",
        "os:Windows",
        False,
    )

    print("Testing matching tag against pattern with multiple wildcards...\n")
    result += test_process_tag("os:*u*", "os:Linux", True)

    print(
        "Testing matching tag against pattern with multiple wildcards and empty wildcard matches...\n"
    )
    result += test_process_tag("os:*n*u*x", "os:Linux", True)

    print("Testing nonmatching tag against pattern with multiple wildcards...\n")
    result += test_process_tag("os:*u*", "os:Windows", False)

    print("Testing matching tag against pattern with trailing capture...\n")
    result += test_process_tag("python:#", "python:3", True)

    print("Testing matching tag against pattern with trailing long capture...\n")
    result += test_process_tag("python:#", "python:38", True)

    print(
        "Testing matching tag against pattern with trailing explicit long capture...\n"
    )
    result += test_process_tag("python:#<5>", "python:38", True)

    print(
        "Testing matching tag against pattern with non-trailing capture, stop character, and trailing wildcard...\n"
    )
    result += test_process_tag("python:#.*", "python:3.8.5", True)

    print(
        "Testing non-matching tag against pattern with non-trailing capture, stop character, and trailing wildcard...\n"
    )
    result += test_process_tag("python:#.*", "python:3,8,5", False)

    print(
        "Testing matching tag against pattern with non-trailing capture, stop character, and skips...\n"
    )
    result += test_process_tag("python:#<1>.*", "python:3.8.5", True)

    print(
        "Testing extravagantly non-matching tag against pattern with non-trailing capture, stop character, and skips...\n"
    )
    result += test_process_tag("python:#<5>.", "python:3.8.5", False)

    print(
        "Testing barely non-matching tag against pattern with non-trailing capture, stop character, and skips...\n"
    )
    result += test_process_tag("python:#<2>.", "python:3.8.5", False)

    print(
        "Testing matching tag against pattern with wildcard then simple capture then wildcard...\n"
    )
    result += test_process_tag("*:#.*", "python:3.8.5", True)

    print("FAILURES: {}\n", result)

    return result


main()
