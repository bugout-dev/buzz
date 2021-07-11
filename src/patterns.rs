use std::collections::VecDeque;

const CAPTURE_CHAR: char = '#';
const WILDCARD_CHAR: char = '*';
const SKIP_START_CHAR: char = '<';
const SKIP_END_CHAR: char = '>';

/// Represents a wildcard (denoted by '*') in a buzz pattern.
#[derive(Debug, Clone)]
pub struct Wildcard {
    start: usize,
    resume: char,
}

/// Represents a capture group (denoted by '#') in a buzz pattern.
/// Currently, a buzz pattern may have at most one capture group.
#[derive(Debug)]
pub struct Capture {
    start: usize,
    skip: usize,
    resume: char,
}

/// Structural representation of a buzz pattern.
#[derive(Debug)]
pub struct Pattern {
    pattern: String,
    capture: Option<Capture>,
    wildcards: VecDeque<Wildcard>,
}

/// Different reasons that the buzz parser could fail to parse a pattern from its string representation.
#[derive(Debug)]
pub enum PatternError {
    // TODO(zomglings): In the future, we may allow multiple captures. For now, only one capture group
    // per pattern.
    CaptureAfterCaptureNotAllowed,
    CaptureImmediatelyAfterCaptureNotAllowed,
    CaptureImmediatelyAfterWildcardNotAllowed,
    WildcardImmediatelyAfterWildcardNotAllowed,
    TrailingSkipNotAllowed,
    NonNumericCharacterInSkip,
    SkipAfterNonCaptureNotAllowed,
}

/// Structural representation of the results of processing a tag against a buzz pattern.
#[derive(Debug)]
pub struct PatternMatch {
    pattern: String,
    tag: String,
    matches: bool,
    capture_start: Option<usize>,
    /// The capture is represented by the slice tag[capture_start.unwrap()..capture_end.unwrap()]
    capture_end: Option<usize>,
}

/// Errors that can occur when processing a tag against a given pattern.
#[derive(Debug)]
pub enum PatternMatchError {
    EmptyPatternNotAllowed,
    EmptyTagNotAllowed,
    UnexpectedWildcardInPattern,
    UnexpectedCaptureInPattern,
    UnexpectedWildcardResume,
}

// python:#
// python:2.7.1 -> 2.7.1
// python:3.8.6 -> 3.8.6
// python:2.6.3 -> 2.6.3

// python:#.*
// python:2.7.1 -> 2
// python:3.8.6 -> 3
// python:2.6.3 -> 2

// python:#<1>.
// python:2.7.1 -> 2.7
// python:3.8.6 -> 3.8
// python:2.6.3 -> 2.6

// python:*.*.*
// python:2.6.3 -> matches
// python:2.6 -> does not match

// python:#.*.*
// python:2.6.3 -> matches with capture "2"
// python:2.6 -> does not match

impl Pattern {
    /// Parses a raw pattern string into a Pattern object. If the pattern string has invalid syntax,
    /// returns a PatternError.
    /// Pattern syntax:
    /// Special characters are '#', '*', '<', and '>'. Any other character, outside of "<...>" context
    /// is treated as specifying an exact character match.
    /// # (capture group) - This specifies that we should capture the slice until the next match. Slices are
    /// captured into a capture variable.
    /// * (wildcard) - This specifies that we should match any character.
    /// <n> (skip) - Should come immediately after a capture group and before a match which terminates
    /// the capture group. Specifies that n matches should be ignored when building the capture variable.
    /// n should be a non-negative integer.
    // TODO(zomglings): For now, we make the assumption that tags do not containg the characters
    // "*", "#", "<", and ">". We should revisit this assumption and fix it later.
    pub fn from(raw_pattern: &String) -> Result<Self, PatternError> {
        let pattern: String = raw_pattern.clone();
        let mut capture: Option<Capture> = None;
        let wildcards_mut: &mut VecDeque<Wildcard> = &mut VecDeque::new();
        let mut skip: Option<usize> = None;

        let mut prev_wildcard: bool = false;
        let mut prev_capture: bool = false;
        let mut prev_skip: bool = false;
        let mut in_skip: bool = false;

        for (current_index, current_character) in pattern.chars().enumerate() {
            match current_character {
                CAPTURE_CHAR => {
                    if prev_wildcard {
                        return Err(PatternError::CaptureImmediatelyAfterWildcardNotAllowed);
                    }
                    if prev_capture {
                        return Err(PatternError::CaptureImmediatelyAfterCaptureNotAllowed);
                    }
                    if capture.is_some() {
                        return Err(PatternError::CaptureAfterCaptureNotAllowed);
                    }

                    prev_capture = true;
                    prev_wildcard = false;
                    prev_skip = false;

                    // If the resume: '#' survives until the end, it will signify that we capture the
                    // entire tail of each matched tag.
                    capture = Some(Capture {
                        start: current_index,
                        skip: 0,
                        resume: CAPTURE_CHAR,
                    });
                }
                WILDCARD_CHAR => {
                    if prev_wildcard {
                        return Err(PatternError::WildcardImmediatelyAfterWildcardNotAllowed);
                    }

                    if prev_capture {
                        match capture {
                            Some(inner_capture) => {
                                capture = Some(Capture {
                                    start: inner_capture.start,
                                    skip: inner_capture.skip,
                                    resume: WILDCARD_CHAR,
                                });
                            }
                            None => {}
                        }
                    }

                    wildcards_mut.push_back(Wildcard {
                        start: current_index,
                        resume: WILDCARD_CHAR,
                    });

                    prev_capture = false;
                    prev_wildcard = true;
                    prev_skip = false;
                }
                SKIP_START_CHAR => {
                    if !prev_capture {
                        return Err(PatternError::SkipAfterNonCaptureNotAllowed);
                    }
                    in_skip = true;
                    skip = Some(0);
                    prev_skip = true;
                }
                SKIP_END_CHAR => {
                    in_skip = false;
                    match capture {
                        Some(inner_capture) => {
                            capture = Some(Capture {
                                start: inner_capture.start,
                                skip: skip.unwrap(),
                                resume: inner_capture.resume,
                            });
                        }
                        None => {}
                    }
                }
                _ => {
                    if in_skip {
                        let added: usize = match current_character.to_digit(10) {
                            Some(digit) => (digit as usize),
                            None => {
                                return Err(PatternError::NonNumericCharacterInSkip);
                            }
                        };
                        skip = match skip {
                            Some(accumulator) => Some(10 * accumulator + added),
                            None => Some(added),
                        };
                    } else {
                        if prev_capture {
                            match capture {
                                Some(inner_capture) => {
                                    capture = Some(Capture {
                                        start: inner_capture.start,
                                        skip: inner_capture.skip,
                                        resume: current_character,
                                    })
                                }
                                None => {}
                            }
                        } else if prev_wildcard {
                            match wildcards_mut.back_mut() {
                                Some(last_wildcard) => {
                                    last_wildcard.resume = current_character;
                                }
                                None => {}
                            }
                        }
                        prev_capture = false;
                        prev_wildcard = false;
                        prev_skip = false;
                    }
                }
            }
        }

        if prev_skip {
            return Err(PatternError::TrailingSkipNotAllowed);
        }

        return Ok(Pattern {
            pattern: pattern,
            capture: capture,
            wildcards: (*wildcards_mut).clone(),
        });
    }

    pub fn process(&self, tag: &String) -> Result<PatternMatch, PatternMatchError> {
        let pattern = self.pattern.clone();
        let target = tag.clone();
        let mut matches = true;
        let mut capture_start: Option<usize> = None;
        let mut capture_end: Option<usize> = None;

        let mut pattern_iterator = pattern.chars();
        let mut target_iterator = target.chars();

        let mut pattern_char: char = match pattern_iterator.next() {
            Some(c) => c,
            None => return Err(PatternMatchError::EmptyPatternNotAllowed),
        };

        let mut target_char: char = match target_iterator.next() {
            Some(c) => c,
            None => return Err(PatternMatchError::EmptyTagNotAllowed),
        };
        let mut target_index: usize = 0;

        let mut wildcard_iterator = self.wildcards.iter();

        while matches {
            if pattern_char == WILDCARD_CHAR {
                let current_wildcard = match wildcard_iterator.next() {
                    Some(wildcard) => wildcard,
                    None => return Err(PatternMatchError::UnexpectedWildcardInPattern),
                };

                // Iterate through target until:
                // 0. If the wildcard resume character is WILDCARD_CHAR, we are done.
                // 1. We hit the wildcard resume character, which signifies we are done with the
                //    wildcard match.
                // 2. We exhaust all the characters in the target, which means that we failed to
                //    match. Note that, since we got past step 0, the wildcard resume character is
                //    guaranteed to be a non-special pattern character.
                if current_wildcard.resume == WILDCARD_CHAR {
                    break;
                } else if target_char == current_wildcard.resume {
                    break;
                } else {
                    loop {
                        match target_iterator.next() {
                            Some(c) => {
                                target_index += 1;
                                if c == current_wildcard.resume {
                                    break;
                                }
                            }
                            None => {
                                matches = false;
                                break;
                            }
                        }
                    }
                }

                // Now we have to fast forward the pattern iterator once
                match pattern_iterator.next() {
                    Some(c) => {
                        if c != current_wildcard.resume {
                            return Err(PatternMatchError::UnexpectedWildcardResume);
                        }
                    }
                    None => {
                        return Err(PatternMatchError::UnexpectedWildcardResume);
                    }
                }
            } else if pattern_char == CAPTURE_CHAR {
                let capture = match &self.capture {
                    Some(inner_capture) => inner_capture,
                    None => return Err(PatternMatchError::UnexpectedCaptureInPattern),
                };

                // Iterate through target until:
                // 0. If the capture is meant to go to the end of the tag, return the starting index
                //    as capture_start and the string_length as capture_end.
                // 1. If the capture is meant to stop at a non-CAPTURE_CHAR resume character, keep
                //    iterating through the target until we hit the character more than capture.skip
                //    times and mark that position as capture_end.
                // 2. If we never hit the resume character more than capture.skip times, match fails.
                capture_start = Some(target_index);
                let mut skip_count: usize = 0;
                if capture.resume == CAPTURE_CHAR {
                    break;
                }
                loop {
                    target_index += 1;
                    match target_iterator.next() {
                        Some(c) => {
                            target_char = c;
                            if capture.resume != CAPTURE_CHAR && c == capture.resume {
                                skip_count += 1;
                                if skip_count > capture.skip {
                                    break;
                                }
                            }
                        }
                        None => {
                            matches = false;
                            break;
                        }
                    }
                }
                capture_end = Some(target_index);

                // Now we have to fast forward the pattern iterator until we hit the resume character.
                loop {
                    match pattern_iterator.next() {
                        Some(c) => {
                            if c == capture.resume {
                                break;
                            }
                        }
                        None => {
                            matches = false;
                            break;
                        }
                    }
                }
            } else {
                matches = pattern_char == target_char;
            }

            let mut pattern_ended: bool = false;
            let mut tag_ended: bool = false;
            match pattern_iterator.next() {
                Some(c) => {
                    pattern_char = c;
                }
                None => {
                    pattern_ended = true;
                }
            };
            match target_iterator.next() {
                Some(c) => {
                    target_index += 1;
                    target_char = c;
                }
                None => {
                    tag_ended = true;
                }
            };

            if pattern_ended && tag_ended {
                break;
            } else if pattern_ended != tag_ended {
                if pattern_ended {
                    matches = false;
                } else if tag_ended {
                    matches = false;
                }
            }
        }

        return Ok(PatternMatch {
            pattern: pattern,
            tag: target,
            matches: matches,
            capture_start: capture_start,
            capture_end: capture_end,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{Capture, Pattern, PatternError, Wildcard, CAPTURE_CHAR, WILDCARD_CHAR};

    use std::collections::VecDeque;

    #[test]
    fn read_valid_pattern_simple() {
        let raw_pattern = String::from("os:Linux");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let expected_wildcards_length = 0;

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );
        assert!(
            pattern.capture.is_none(),
            "Capture -- Expected: No capture, Actual: {:?}",
            pattern.capture
        );
        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
    }

    #[test]
    fn read_valid_pattern_capture() {
        let raw_pattern = String::from("os:#");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let expected_capture_start = 3;
        let expected_capture_skip = 0;
        let expected_capture_resume = CAPTURE_CHAR;
        let expected_wildcards_length = 0;

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );
        assert!(
            pattern.capture.is_some(),
            "Capture -- Expected: some capture, Actual: None"
        );
        let capture = pattern.capture.unwrap();
        assert_eq!(
            capture.start, expected_capture_start,
            "Capture start -- Expected: {}, Actual: {}",
            expected_capture_start, capture.start
        );
        assert_eq!(
            capture.skip, expected_capture_skip,
            "Capture skip -- Expected: {}, Actual: {}",
            expected_capture_skip, capture.skip
        );
        assert_eq!(
            capture.resume, expected_capture_resume,
            "Capture resume -- Expected: {}, Actual: {}",
            expected_capture_resume, capture.resume
        );
        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
    }

    #[test]
    fn read_valid_pattern_major_version() {
        let raw_pattern = String::from("version:#.*");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let expected_capture_start = 8;
        let expected_capture_skip = 0;
        let expected_capture_resume = '.';
        let expected_wildcards_length = 1;

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );

        assert!(
            pattern.capture.is_some(),
            "Capture -- Expected: some capture, Actual: None"
        );
        let capture = pattern.capture.unwrap();
        assert_eq!(
            capture.start, expected_capture_start,
            "Capture start -- Expected: {}, Actual: {}",
            expected_capture_start, capture.start
        );
        assert_eq!(
            capture.skip, expected_capture_skip,
            "Capture skip -- Expected: {}, Actual: {}",
            expected_capture_skip, capture.skip
        );
        assert_eq!(
            capture.resume, expected_capture_resume,
            "Capture resume -- Expected: {}, Actual: {}",
            expected_capture_resume, capture.resume
        );

        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
        let wildcard = pattern.wildcards.front().unwrap();
        let expected_wildcard_start = 10;
        let expected_wildcard_resume = WILDCARD_CHAR;
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );
    }

    #[test]
    fn read_valid_pattern_capture_major_minor_version() {
        let raw_pattern = String::from("version:#<1>.*");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let expected_capture_start = 8;
        let expected_capture_skip = 1;
        let expected_capture_resume = '.';
        let expected_wildcards_length = 1;

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );

        assert!(
            pattern.capture.is_some(),
            "Capture -- Expected: some capture, Actual: None"
        );
        let capture = pattern.capture.unwrap();
        assert_eq!(
            capture.start, expected_capture_start,
            "Capture start -- Expected: {}, Actual: {}",
            expected_capture_start, capture.start
        );
        assert_eq!(
            capture.skip, expected_capture_skip,
            "Capture skip -- Expected: {}, Actual: {}",
            expected_capture_skip, capture.skip
        );
        assert_eq!(
            capture.resume, expected_capture_resume,
            "Capture resume -- Expected: {}, Actual: {}",
            expected_capture_resume, capture.resume
        );

        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
        let wildcard = pattern.wildcards.front().unwrap();
        let expected_wildcard_start = 13;
        let expected_wildcard_resume = WILDCARD_CHAR;
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );
    }

    #[test]
    fn read_valid_pattern_capture_5_chars() {
        let raw_pattern = String::from("version:#<5>*");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let expected_capture_start = 8;
        let expected_capture_skip = 5;
        let expected_capture_resume = WILDCARD_CHAR;
        let expected_wildcards_length = 1;

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );

        assert!(
            pattern.capture.is_some(),
            "Capture -- Expected: some capture, Actual: None"
        );
        let capture = pattern.capture.unwrap();
        assert_eq!(
            capture.start, expected_capture_start,
            "Capture start -- Expected: {}, Actual: {}",
            expected_capture_start, capture.start
        );
        assert_eq!(
            capture.skip, expected_capture_skip,
            "Capture skip -- Expected: {}, Actual: {}",
            expected_capture_skip, capture.skip
        );
        assert_eq!(
            capture.resume, expected_capture_resume,
            "Capture resume -- Expected: {}, Actual: {}",
            expected_capture_resume, capture.resume
        );

        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
        let wildcard = pattern.wildcards.front().unwrap();
        let expected_wildcard_start = 12;
        let expected_wildcard_resume = WILDCARD_CHAR;
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );
    }

    #[test]
    fn read_valid_pattern_semantic_dev_version() {
        let raw_pattern = String::from("version:*-dev");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );

        assert!(
            pattern.capture.is_none(),
            "Capture -- Expected: None, Actual: {:?}",
            pattern.capture
        );

        let expected_wildcards_length = 1;
        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
        let wildcard = pattern.wildcards.front().unwrap();
        let expected_wildcard_start = 8;
        let expected_wildcard_resume = '-';
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );
    }

    #[test]
    fn read_valid_pattern_wildcard_at_start() {
        let raw_pattern = String::from("*suffix");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );

        assert!(
            pattern.capture.is_none(),
            "Capture -- Expected: None, Actual: {:?}",
            pattern.capture
        );

        let expected_wildcards_length = 1;
        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
        let wildcard = pattern.wildcards.front().unwrap();
        let expected_wildcard_start = 0;
        let expected_wildcard_resume = 's';
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );
    }

    #[test]
    fn read_valid_pattern_wildcard_at_start_and_end() {
        let raw_pattern = String::from("*middle*");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let pattern = result.unwrap();
        assert_eq!(
            pattern.pattern, raw_pattern,
            "Pattern -- Expected: {}, Actual: {}",
            raw_pattern, pattern.pattern
        );

        assert!(
            pattern.capture.is_none(),
            "Capture -- Expected: None, Actual: {:?}",
            pattern.capture
        );

        let expected_wildcards_length = 2;
        assert_eq!(
            pattern.wildcards.len(),
            expected_wildcards_length,
            "Wildcards -- Expected length: {}, Actual: {:?}",
            expected_wildcards_length,
            pattern.wildcards
        );
        let mut wildcard_iterator = pattern.wildcards.iter();

        let mut wrapped_wildcard = wildcard_iterator.next();
        let mut wildcard = wrapped_wildcard.unwrap();
        let expected_wildcard_start = 0;
        let expected_wildcard_resume = 'm';
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );

        wrapped_wildcard = wildcard_iterator.next();
        wildcard = wrapped_wildcard.unwrap();
        let expected_wildcard_start = 7;
        let expected_wildcard_resume = WILDCARD_CHAR;
        assert_eq!(
            wildcard.start, expected_wildcard_start,
            "Wildcard start -- Expected: {}, Actual: {}",
            expected_wildcard_start, wildcard.start
        );
        assert_eq!(
            wildcard.resume, expected_wildcard_resume,
            "Wildcard resume -- Expected: {}, Actual: {}",
            expected_wildcard_resume, wildcard.resume
        );
    }

    #[test]
    fn read_invalid_pattern_capture_immediately_after_capture() {
        let raw_pattern = String::from("os:##");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::CaptureImmediatelyAfterCaptureNotAllowed => true,
                _ => false,
            },
            "Error -- Expected: CaptureImmediatelyAfterCaptureNotAllowed, Actual: {:?}",
            err
        );
    }

    #[test]
    fn read_invalid_pattern_capture_after_capture() {
        let raw_pattern = String::from("python:#.#.*");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::CaptureAfterCaptureNotAllowed => true,
                _ => false,
            },
            "Error -- Expected: CaptureAfterCaptureNotAllowed, Actual: {:?}",
            err
        );
    }

    #[test]
    fn read_invalid_pattern_wildcard_immediately_after_wildcard() {
        let raw_pattern = String::from("os:**");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::WildcardImmediatelyAfterWildcardNotAllowed => true,
                _ => false,
            },
            "Error -- Expected: WildcardImmediatelyAfterWildcardNotAllowed, Actual: {:?}",
            err
        );
    }

    #[test]
    fn read_invalid_pattern_capture_immediately_after_wildcard() {
        let raw_pattern = String::from("os:*#");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::CaptureImmediatelyAfterWildcardNotAllowed => true,
                _ => false,
            },
            "Error -- Expected: CaptureImmediatelyAfterWildcardNotAllowed, Actual: {:?}",
            err
        );
    }

    #[test]
    fn read_invalid_pattern_trailing_skip() {
        let raw_pattern = String::from("os:#<5>");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::TrailingSkipNotAllowed => true,
                _ => false,
            },
            "Error -- Expected: TrailingSkipNotAllowed, Actual: {:?}",
            err
        );
    }

    #[test]
    fn read_invalid_pattern_skip_after_noncapture() {
        let raw_pattern = String::from("os:<5>*");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::SkipAfterNonCaptureNotAllowed => true,
                _ => false,
            },
            "Error -- Expected: SkipAfterNonCaptureNotAllowed, Actual: {:?}",
            err
        );
    }

    #[test]
    fn read_invalid_pattern_nonumeric_character_in_skip() {
        let raw_pattern = String::from("version:#<-1>.");
        let result = Pattern::from(&raw_pattern);
        assert!(result.is_err(), "Expected error. Actual: {:?}", result);
        let err = result.unwrap_err();
        assert!(
            match err {
                PatternError::NonNumericCharacterInSkip => true,
                _ => false,
            },
            "Error -- Expected: NonNumericCharacterInSkip, Actual: {:?}",
            err
        );
    }

    #[test]
    fn process_matching_tag() {
        let wildcards: VecDeque<Wildcard> = VecDeque::new();
        let pattern = Pattern {
            pattern: String::from("matching_tag"),
            capture: None,
            wildcards: wildcards,
        };

        let tag = String::from("matching_tag");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            processed.matches,
            "Process matches -- Expected: true, Actual processed: {:?}",
            processed
        );
    }

    #[test]
    fn process_nonmatching_tag() {
        let wildcards: VecDeque<Wildcard> = VecDeque::new();
        let pattern = Pattern {
            pattern: String::from("matching_tag"),
            capture: None,
            wildcards: wildcards,
        };

        let tag = String::from("matching_not_lol_tag");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            !processed.matches,
            "Process matches -- Expected: false, Actual processed: {:?}",
            processed
        );
    }

    #[test]
    fn process_nonmatching_tag_due_to_suffix() {
        let wildcards: VecDeque<Wildcard> = VecDeque::new();
        let pattern = Pattern {
            pattern: String::from("matching_tag"),
            capture: None,
            wildcards: wildcards,
        };

        let tag = String::from("matching_tag_not!");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            !processed.matches,
            "Process matches -- Expected: false, Actual processed: {:?}",
            processed
        );
    }

    #[test]
    fn process_match_multiple_wildcards() {
        let mut wildcards: VecDeque<Wildcard> = VecDeque::new();
        wildcards.push_back(Wildcard {
            start: 8,
            resume: '.',
        });
        wildcards.push_back(Wildcard {
            start: 10,
            resume: '*',
        });
        let pattern = Pattern {
            pattern: String::from("version:*.*"),
            capture: None,
            wildcards: wildcards,
        };

        let tag = String::from("version:1.2");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            processed.matches,
            "Process matches -- Expected: true, Actual processed: {:?}",
            processed
        );
    }

    #[test]
    fn process_nonmatch_multiple_wildcards() {
        let mut wildcards: VecDeque<Wildcard> = VecDeque::new();
        wildcards.push_back(Wildcard {
            start: 8,
            resume: '.',
        });
        wildcards.push_back(Wildcard {
            start: 10,
            resume: '*',
        });
        let pattern = Pattern {
            pattern: String::from("version:*.*"),
            capture: None,
            wildcards: wildcards,
        };

        let tag = String::from("version:1-2");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            !processed.matches,
            "Process matches -- Expected: false, Actual processed: {:?}",
            processed
        );
    }


    #[test]
    fn process_python_major_version() {
        let mut wildcards: VecDeque<Wildcard> = VecDeque::new();
        wildcards.push_back(Wildcard {
            start: 9,
            resume: '*',
        });
        let pattern = Pattern {
            pattern: String::from("python:#.*"),
            capture: Some(Capture {
                start: 7,
                skip: 0,
                resume: '.',
            }),
            wildcards: wildcards,
        };

        let tag = String::from("python:3.8.5");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            processed.matches,
            "Process matches -- Expected: true, Actual processed: {:?}",
            processed
        );
        assert!(
            processed.capture_start.is_some(),
            "Process capture_start -- Expected: Some, Actual: None"
        );
        assert!(
            processed.capture_end.is_some(),
            "Process capture_end -- Expected: Some, Actual: None"
        );

        let expected_capture_start: usize = 7;
        let expected_capture_end: usize = 8;
        assert_eq!(processed.capture_start.unwrap(), expected_capture_start);
        assert_eq!(processed.capture_end.unwrap(), expected_capture_end);
    }

    #[test]
    fn process_python_major_minor_version() {
        let mut wildcards: VecDeque<Wildcard> = VecDeque::new();
        wildcards.push_back(Wildcard {
            start: 12,
            resume: '*',
        });
        let pattern = Pattern {
            pattern: String::from("python:#<1>.*"),
            capture: Some(Capture {
                start: 7,
                skip: 1,
                resume: '.',
            }),
            wildcards: wildcards,
        };

        let tag = String::from("python:3.8.5");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            processed.matches,
            "Process matches -- Expected: true, Actual processed: {:?}",
            processed
        );
        assert!(
            processed.capture_start.is_some(),
            "Process capture_start -- Expected: Some, Actual: None"
        );
        assert!(
            processed.capture_end.is_some(),
            "Process capture_end -- Expected: Some, Actual: None"
        );

        let expected_capture_start: usize = 7;
        let expected_capture_end: usize = 10;
        assert_eq!(processed.capture_start.unwrap(), expected_capture_start);
        assert_eq!(processed.capture_end.unwrap(), expected_capture_end);
    }

    #[test]
    fn process_python_version() {
        let wildcards: VecDeque<Wildcard> = VecDeque::new();
        let pattern = Pattern {
            pattern: String::from("python:#"),
            capture: Some(Capture {
                start: 7,
                skip: 0,
                resume: '#',
            }),
            wildcards: wildcards,
        };

        let tag = String::from("python:3.8.5");
        let result = pattern.process(&tag);

        assert!(
            result.is_ok(),
            "Process result -- Expected: No error, Actual: {:?}",
            result
        );

        let processed = result.unwrap();
        assert_eq!(
            processed.pattern, pattern.pattern,
            "Process pattern -- Expected: {}, Actual: {}",
            pattern.pattern, processed.pattern
        );
        assert_eq!(
            processed.tag, tag,
            "Process tag -- Expected: {}, Actual: {}",
            tag, processed.tag
        );
        assert!(
            processed.matches,
            "Process matches -- Expected: true, Actual processed: {:?}",
            processed
        );
        assert!(
            processed.capture_start.is_some(),
            "Process capture_start -- Expected: Some, Actual: None"
        );
        assert!(
            processed.capture_end.is_none(),
            "Process capture_end -- Expected: None, Actual processed: {:?}",
            processed
        );

        let expected_capture_start: usize = 7;
        assert_eq!(processed.capture_start.unwrap(), expected_capture_start);
    }
}
