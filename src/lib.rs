//! **heck** is a case conversion library.
//!
//! This library exists to provide case conversion between common cases like
//! CamelCase and snake_case. It is intended to be unicode aware, internally
//! consistent, and reasonably well performing.
//!
//! ## Definition of a word boundary
//!
//! The definition of a word boundary is based on the
//! [identifier word boundary](https://www.unicode.org/reports/tr55/#Identifier-Chunks)
//! in Unicode Technical Standard 55. The rules are as follows:
//!
//! - The set of characters that can be in a word is
//!   [`[\p{ID_Continue}\p{ID_Compat_Math_Continue}\p{Unassigned}\p{Private_Use}-[\p{Punctuation}-\p{Other_Punctuation}]]`][1],
//!   plus U+05F3, U+05F4, and U+0F0B. This notably includes
//!   alphabetic and numeric characters, accents and other combining marks,
//!   emoji, a few mathematical symbols, a few non-word-separating punctuation marks,
//!   unassigned characters, and private-use characters.
//!
//! - Characters that cannot be in a word separate words.
//!   For example, `foo_bar` is segmented `foo`|`bar`
//!   because words cannout contain `_`.
//!   These characters will be excluded from the output string.
//!
//! - Words cannot be empty. For example, `_foo__bar_` is segmented `foo`|`bar`,
//!   and in snake_case becomes `foo_bar`.
//!
//! - There is a word boundary between a lowercase (or non-Greek titlecase)
//!   and an uppercase (or titlecase) letter. For example, `fooBar` is segmented
//!   `foo`|`Bar` because `oB` is a lowercase letter followed by an uppercase letter.
//!
//! - An uppercase letter followed by a lowercase letter
//!   has a word boundary before it. For example, `XMLHttpRequest` is segmented
//!   `XML`|`Http`|`Request`; the `Ht` in `HttpRequest` is an uppercase letter
//!   followed by a lowercase letter, so there is a word boundary before it.
//!
//!  - There is always a word boundary before a non-Greek titlecase letter
//!    (U+01C5 'ǅ', U+01C8 'ǈ', U+01CB 'ǋ', or U+01F2 'ǲ').
//!
//!  - For the purpose of the preceding three rules, a letter followed
//!    by some number of nonspacing marks (like accents or other diacritics)
//!    is treated as if it was the letter alone. For example, `áB` is segmented `á`|`B`.
//!
//! [1]: https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5Cp%7BID_Continue%7D%5Cp%7BID_Compat_Math_Continue%7D%5Cp%7BUnassigned%7D%5Cp%7BPrivate_Use%7D-%5B%5Cp%7BPunctuation%7D-%5Cp%7BOther_Punctuation%7D%5D%5D&abb=on
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

fn titlecase(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
    // Find the first cased character
    if let Some(titlecase_idx) =
        s.find(|c| tables::letter_casing(c).is_some() || c.is_lowercase() || c.is_uppercase())
    {
        // Everything before the first cased character is passed through unchanged.
        f.write_str(&s[..titlecase_idx])?;

        let rem = &s[titlecase_idx..];
        let mut char_indices = rem.char_indices();
        if let Some((_, c)) = char_indices.next() {
            write!(f, "{}", tables::to_titlecase(c))?;
            if let Some((i, _)) = char_indices.next() {
                lowercase(&rem[i..], f)?;
            }
        }
    } else {
        // If there are no cased characters, pass through the string unchanged
        write!(f, "{}", s)?;
    }

    Ok(())
}
