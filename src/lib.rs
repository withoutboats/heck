//! **heck** is a case conversion library.
//!
//! This library exists to provide case conversion between common cases like
//! CamelCase and snake_case. It is intended to be unicode aware, internally
//! consistent, and reasonably well performing.
//!
//! ## Definition of a word boundary
//!
//! Word boundaries are defined by the specification of
//! [identifier chunks](https://www.unicode.org/reports/tr55/#Identifier-Chunks)
//! in Unicode Technical Standard 55.
//!
//! ### Cases contained in this library:
//!
//! 1. UpperCamelCase
//! 2. lowerCamelCase
//! 3. snake_case
//! 4. kebab-case
//! 5. SHOUTY_SNAKE_CASE
//! 6. Title Case
//! 7. SHOUTY-KEBAB-CASE
//! 8. Train-Case
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![no_std]

extern crate alloc;

mod kebab;
mod lower_camel;
mod shouty_kebab;
mod shouty_snake;
mod snake;
mod tables;
mod title;
mod train;
mod upper_camel;

pub use kebab::{AsKebabCase, ToKebabCase};
pub use lower_camel::{AsLowerCamelCase, ToLowerCamelCase};
pub use shouty_kebab::{AsShoutyKebabCase, ToShoutyKebabCase};
pub use shouty_snake::{
    AsShoutySnakeCase, AsShoutySnakeCase as AsShoutySnekCase, ToShoutySnakeCase, ToShoutySnekCase,
};
pub use snake::{AsSnakeCase, AsSnakeCase as AsSnekCase, ToSnakeCase, ToSnekCase};
pub use tables::UNICODE_VERSION;
pub use title::{AsTitleCase, ToTitleCase};
pub use train::{AsTrainCase, ToTrainCase};
pub use upper_camel::{
    AsUpperCamelCase, AsUpperCamelCase as AsPascalCase, ToPascalCase, ToUpperCamelCase,
};

use core::fmt;

use tables::{is_non_greek_titlecase, CasedLetterKind};

fn transform<F, G>(
    s: &str,
    mut with_word: F,
    mut boundary: G,
    f: &mut fmt::Formatter,
) -> fmt::Result
where
    F: FnMut(&str, &mut fmt::Formatter) -> fmt::Result,
    G: FnMut(&mut fmt::Formatter) -> fmt::Result,
{
    let mut first_word = true;

    for word in s.split(|c: char| !tables::allowed_in_word(c)) {
        let mut start_of_word_idx = 0;
        // Whether the previous character seen, ignoring nonspacing marks,
        // was lowercase or non-Greek titlecase.
        // Used for determining CamelBoundaries.
        let mut prev_was_lowercase_or_non_greek_titlecase = false;
        // If the previous character seen, ignoring nonspacing marks,
        // was uppercase or titlecase, then this stores that character's index.
        // Otherwise, it stores `None`.
        // Used for determining HATBoundaries.
        let mut index_of_preceding_uppercase_or_titlecase_letter: Option<usize> = None;

        for (i, c) in word.char_indices() {
            match tables::letter_casing(c) {
                None => {
                    // Nonspacing marks are ignored for the purpose of determining boundaries.
                    if !tables::is_nonspacing_mark(c) {
                        prev_was_lowercase_or_non_greek_titlecase = false;
                        index_of_preceding_uppercase_or_titlecase_letter = None;
                    }
                }
                Some(CasedLetterKind::Lowercase) => {
                    prev_was_lowercase_or_non_greek_titlecase = true;
                    // There is a HATBoundary before an uppercase or titlecase letter followed by a lowercase letter
                    if let Some(preceding_idx) = index_of_preceding_uppercase_or_titlecase_letter {
                        index_of_preceding_uppercase_or_titlecase_letter = None;
                        if preceding_idx != start_of_word_idx {
                            if !first_word {
                                boundary(f)?;
                            } else {
                                first_word = false;
                            }
                            with_word(&word[start_of_word_idx..preceding_idx], f)?;
                            start_of_word_idx = preceding_idx;
                        }
                    }
                }
                Some(CasedLetterKind::Uppercase) => {
                    index_of_preceding_uppercase_or_titlecase_letter = Some(i);
                    // There is a CamelBoundary before an uppercase letter
                    // that is preceded by a lowercase or non-Greek titlecase letter
                    if prev_was_lowercase_or_non_greek_titlecase {
                        prev_was_lowercase_or_non_greek_titlecase = false;
                        if !first_word {
                            boundary(f)?;
                        } else {
                            first_word = false;
                        }
                        with_word(&word[start_of_word_idx..i], f)?;
                        start_of_word_idx = i;
                    }
                }
                Some(CasedLetterKind::Titlecase) => {
                    index_of_preceding_uppercase_or_titlecase_letter = Some(i);
                    // There is always a HATBoundary before a non-Greek titlecase letter
                    if is_non_greek_titlecase(c) {
                        prev_was_lowercase_or_non_greek_titlecase = true;
                        if i != start_of_word_idx {
                            if !first_word {
                                boundary(f)?;
                            } else {
                                first_word = false;
                            }
                            with_word(&word[start_of_word_idx..i], f)?;
                            start_of_word_idx = i;
                        }
                    } else {
                        // There is a CamelBoundary before a titlecase letter
                        // that is preceded by a lowercase or non-Greek titlecase letter
                        if prev_was_lowercase_or_non_greek_titlecase {
                            prev_was_lowercase_or_non_greek_titlecase = false;
                            if !first_word {
                                boundary(f)?;
                            } else {
                                first_word = false;
                            }
                            with_word(&word[start_of_word_idx..i], f)?;
                            start_of_word_idx = i;
                        }
                    }
                }
            }
        }

        if start_of_word_idx != word.len() {
            // Collect trailing characters as a word
            if !first_word {
                boundary(f)?;
            } else {
                first_word = false;
            }
            with_word(&word[start_of_word_idx..], f)?;
        }
    }

    Ok(())
}

fn lowercase(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == 'Σ' && chars.peek().is_none() {
            write!(f, "ς")?;
        } else {
            write!(f, "{}", c.to_lowercase())?;
        }
    }

    Ok(())
}

fn uppercase(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    for c in s.chars() {
        write!(f, "{}", c.to_uppercase())?;
    }

    Ok(())
}

fn capitalize(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    let mut char_indices = s.char_indices();
    if let Some((_, c)) = char_indices.next() {
        write!(f, "{}", c.to_uppercase())?;
        if let Some((i, _)) = char_indices.next() {
            lowercase(&s[i..], f)?;
        }
    }

    Ok(())
}
