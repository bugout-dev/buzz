use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Clone)]
pub struct Wildcard {
    start: usize,
    resume: char,
}

#[derive(Debug)]
pub struct Capture {
    start: usize,
    skip: usize,
    resume: char,
}

#[derive(Debug)]
pub struct Pattern {
    pattern: String,
    capture: Option<Capture>,
    wildcards: VecDeque<Wildcard>,
}

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
    /// Special characters are "#", "*", "<", and ">". Any other character, outside of "<...>" context
    /// is treated as specifying an exact character match.
    /// # (capture group) - This specifies that we should capture the slice until the next match. Slices are
    /// captured into a capture variable.
    /// * (wildcard) - This specifies that we should match any character.
    /// <n> (skip) - Should come immediately after a capture group and before a match which terminates
    /// the capture group. Specifies that n matches should be ignored when building the capture variable.
    /// n should be a non-negative integer.
    // TODO(zomglings): For now, we make the assumption that tags do not containg the characters
    // "*", "#", "<", and ">". We should revisit this assumption and fix it later.
    pub fn from(raw_pattern: &String) -> Result<Pattern, PatternError> {
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
                '#' => {
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
                        resume: '#',
                    });
                }
                '*' => {
                    if prev_wildcard {
                        return Err(PatternError::WildcardImmediatelyAfterWildcardNotAllowed);
                    }

                    if prev_capture {
                        match capture {
                            Some(inner_capture) => {
                                capture = Some(Capture {
                                    start: inner_capture.start,
                                    skip: inner_capture.skip,
                                    resume: '*',
                                });
                            }
                            None => {}
                        }
                    }

                    wildcards_mut.push_back(Wildcard{start: current_index, resume: '*'});

                    prev_capture = false;
                    prev_wildcard = true;
                    prev_skip = false;
                }
                '<' => {
                    if !prev_capture {
                        return Err(PatternError::SkipAfterNonCaptureNotAllowed);
                    }
                    in_skip = true;
                    skip = Some(0);
                    prev_skip = true;
                }
                '>' => {
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
                                },
                                None => {},
                            }
                        }
                    }
                    prev_capture = false;
                    prev_wildcard = false;
                    prev_skip = false;
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
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    #[test]
    fn read_valid_pattern_simple() {
        let raw_pattern = String::from("os:Linux");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let pattern = result.unwrap();
        assert_eq!(pattern.pattern, raw_pattern, "Expected: {}, Actual: {}", raw_pattern, pattern.pattern);
        assert!(pattern.capture.is_none(), "Expected: No capture, Actual: {:?}", pattern.capture);
        assert_eq!(pattern.wildcards.len(), 0, "Expected: No wildcards, Actual: {:?}", pattern.wildcards);
    }

    #[test]
    fn read_valid_pattern_capture() {
        let raw_pattern = String::from("os:#");
        let result = Pattern::from(&raw_pattern);
        assert!(!result.is_err(), "Unexpected error: {:?}", result);

        let pattern = result.unwrap();
        assert_eq!(pattern.pattern, raw_pattern, "Expected: {}, Actual: {}", raw_pattern, pattern.pattern);
        assert!(pattern.capture.is_some(), "Pattern capture -- Expected: some capture, Actual: None");
        let capture = pattern.capture.unwrap();
        assert_eq!(capture.start, 3, "Capture start -- Expected: 3, Actual: {}", capture.start);
        assert_eq!(capture.skip, 0, "Capture skip -- Expected: 0, Actual: {}", capture.skip);
        assert_eq!(capture.resume, '#', "Capture skip -- Expected: #, Actual: {}", capture.resume);
        assert_eq!(pattern.wildcards.len(), 0, "Expected: No wildcards, Actual: {:?}", pattern.wildcards);
    }
}
