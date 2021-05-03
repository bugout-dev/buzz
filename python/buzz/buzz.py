from typing import Union, List

BUGOUT_BUZZ_WILDCARD_CHAR = "*"
BUGOUT_BUZZ_CAPTURE_CHAR = "#"


def read_pattern_from_files(patterns_file_path: str):
    patterns_list = []
    with open(patterns_file_path) as pattern_file:
        for line in pattern_file.readlines():
            pattern = read_pattern(line)
            if pattern:
                patterns_list.append(pattern)
    return patterns_list


class Boundary:
    character: Union[bool, str]  # resume character
    skip: int  # amount of skip numbers
    resume: int  # skip position


class TagPattern:
    raw: str  # original raw patern
    boundary: Boundary  # Skip numbers rule
    patern_catch_sign_position: int  # catch symbol position after what go rule
    valid: bool  # it's valid pattern or no


class MatchingResult:
    tag: str
    tag_pattern: TagPattern
    match: bool


def load_pattern(patterns: List[TagPattern], raw_pattern: str):

    tag_pattern: TagPattern = read_pattern(raw_pattern)

    if tag_pattern.valid:
        patterns.append(tag_pattern)


def read_pattern(raw_pattern: str):

    pattern = TagPattern()

    pattern.boundary = Boundary()

    pattern_proccesing: List[str] = []

    pattern.boundary.skip = -1
    pattern.boundary.character = False
    pattern.boundary.resume = -1
    pattern.patern_catch_sign_position = -1

    catch = False
    pattern.raw = raw_pattern

    if "*#" in raw_pattern or len(raw_pattern) == 0 or "**" in raw_pattern:
        pattern.valid = False
        pattern.raw = raw_pattern
        return pattern

    for index, i in enumerate(raw_pattern):

        if catch:
            catch = False
            if i == "<":
                # catch int
                try:
                    pattern.boundary.skip = int(
                        raw_pattern[raw_pattern.index("<") + 1 : raw_pattern.index(">")]
                    )
                except:
                    print(raw_pattern[raw_pattern.index("<") : raw_pattern.index(">")])
                    pattern.valid = False
                    pattern.boundary.skip = -1
                    return pattern

                if raw_pattern.index(">") + 1 != len(raw_pattern):
                    pattern.boundary.resume = raw_pattern.index(">") + 1
                    pattern.boundary.character = raw_pattern[raw_pattern.index(">") + 1]
                else:
                    pattern.boundary.character = False
                    pattern.boundary.skip = 0
            else:
                pattern.boundary.resume = index
                pattern.boundary.character = i

        if i == "#":
            if pattern.patern_catch_sign_position != -1:
                pattern.valid = False
                pattern.raw = "".join(raw_pattern)
                return pattern
            pattern.patern_catch_sign_position = index
            catch = True
            pattern.boundary.skip = 0

        if i == " ":
            pattern.valid = False
            pattern.raw = raw_pattern
            return pattern

        pattern_proccesing.append(i)

    pattern.raw = "".join(pattern_proccesing)
    pattern.valid = True

    return pattern


def read_tag(tag: str, pattern: TagPattern):

    tag_index = 0
    pattern_index = 0

    result = MatchingResult()
    result.match = True

    while result.match:
        if tag_index == len(tag):
            break

        if pattern_index == len(pattern.raw):
            break

        pattern_current = pattern.raw[pattern_index]

        if pattern_current == BUGOUT_BUZZ_WILDCARD_CHAR:
            pattern_next: Union[bool, str] = False

            if pattern_index + 1 < len(pattern.raw):
                pattern_next = pattern.raw[pattern_index + 1]

            pattern_index += 1

            if not pattern_next:
                break

            while tag_index + 1 != len(tag) and tag[tag_index] != pattern_next:
                tag_index += 1

            if len(tag) == tag_index:
                result.match = False
        elif pattern_current == BUGOUT_BUZZ_CAPTURE_CHAR:

            pattern_index = pattern.boundary.resume

            if pattern_index == -1:
                pattern_index = len(pattern.raw)
            capture_start = tag_index
            num_skipchar_encounters = -1
            while (
                tag_index != len(tag)
                and num_skipchar_encounters < pattern.boundary.skip
            ):
                if tag[tag_index] == pattern.boundary.character:
                    num_skipchar_encounters += 1

                tag_index += 1

            if not pattern.boundary.character == False:
                tag_index -= 1

        else:
            result.match = tag[tag_index] == pattern.raw[pattern_index]
            tag_index += 1
            pattern_index += 1
    return result
