import pytest

from .buzz import (
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

    assert result == 0


@pytest.mark.parametrize(
    "raw_pattern,tag,expected_match",
    [
        ("os:Linux", "os:Linux", True),
        ("os:Linux", "os:Windows", False),
        ("os:*", "os:Windows", True),
        ("os:*", "python:3", False),
        ("os:*x", "os:Linux", True),
        ("os:*x", "os:Windows", False),
        ("os:*u*", "os:Linux", True),
        ("os:*n*u*x", "os:Linux", True),
        ("os:*u*", "os:Windows", False),
        ("python:#", "python:3", True),
        ("python:#", "python:38", True),
        ("python:#<5>", "python:38", True),
        ("python:#.*", "python:3.8.5", True),
        ("python:#.*", "python:3,8,5", False),
        ("python:#<1>.*", "python:3.8.5", True),
        ("python:#<5>.", "python:3.8.5", False),
        ("python:#<2>.", "python:3.8.5", False),
        ("*:#.*", "python:3.8.5", True),
    ],
)
def test_process_tag(raw_pattern: str, tag: str, expected_match: bool):
    tag_pattern: TagPattern = read_pattern(raw_pattern)
    result = 0

    buzz_result: MatchingResult = read_tag(tag, tag_pattern)

    assert buzz_result.match == expected_match


def test_read_pattern_1():
    assert 0 == _test_read_pattern("<a>", "<a>", 3, -1, False, -1, -1, True)


def test_read_pattern_2():
    assert 0 == _test_read_pattern("os:#<0>", "os:#<0>", 7, 3, False, 0, -1, True)


def test_read_pattern_3():
    assert 0 == _test_read_pattern(
        "python:#<1>.", "python:#<1>.", 12, 7, ".", 1, 11, True
    )


def test_read_pattern_4():
    assert 0 == _test_read_pattern(
        "python:#<a>.", "python:#<a>.", 12, 7, False, -1, -1, False
    )


def test_read_pattern_5():
    assert 0 == _test_read_pattern("python:#.", "python:#.", 9, 7, ".", 0, 8, True)


def test_read_pattern_6():
    assert 0 == _test_read_pattern(
        "omg#<0>*wtf#<0>*bbq", "omg#<0>*wtf#<0>*bbq", 19, 3, "*", 0, 7, False
    )


def test_read_pattern_7():
    assert 0 == _test_read_pattern(
        "omg wtf bbq", "omg wtf bbq", 11, -1, False, -1, -1, False
    )


def test_read_pattern_8():
    assert 0 == _test_read_pattern("omg*#", "omg*#", 5, -1, False, -1, -1, False)


def test_read_pattern_9():
    assert 0 == _test_read_pattern("omg**", "omg**", 5, -1, False, -1, -1, False)
