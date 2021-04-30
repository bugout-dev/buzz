import pytest

from buzz import (
    read_pattern,
    read_tag,
    load_pattern,
    TagPattern,
    MatchingResult,
)


def _test_read_pattern(
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


@pytest.mark.parametrize(
    "raw_pattern,tag,expected_match",
    [
        ("*:#.*", "python:3.8.5", True),
        ("python:#<2>.", "python:3.8.5", False)
    ],
)
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


def test_read_pattern_1():
    _test_read_pattern("<a>", "<a>", 3, -1, False, -1, -1, True)
