//! **heck** is a case conversion library.
//!
//! This library exists to provide case conversion between common cases like
//! CamelCase and snake_case. It is intended to be unicode aware, internally,
//! consistent, and reasonably well performing.
//! 
//! ## Definition of a word boundary
//! 
//! Word boundaries are defined as the "unicode words" defined in the
//! `unicode_segmentation` library, as well as within those words in this manner:
//! 
//! 1. All underscore characters are considered word boundaries.
//! 2. If an uppercase character is followed by lowercase letters, a word boundary
//! is considered to be just prior to that uppercase character.
//! 3. If multiple uppercase characters are consecutive, they are considered to be
//! within a single word, except that the last will be part of the next word if it
//! is followed by lowercase characters (see rule 2).
//! 
//! That is, "HelloWorld" is segmented `Hello|World` whereas "XMLHttpRequest" is
//! segmented `XML|Http|Request`.
//! 
//! Characters not within words (such as spaces, punctuations, and underscores)
//! are not included in the output string except as they are a part of the case
//! being converted to. Multiple adjacent word boundaries (such as a series of
//! underscores) are folded into one. ("hello__world" in snake case is therefore
//! "hello_world", not the exact same string). Leading or trailing word boundary
//! indicators are dropped, except insofar as CamelCase capitalizes the first word.
//! 
//! ### Cases contained in this library:
//! 
//! 1. CamelCase
//! 2. snake_case
//! 3. kebab-case
//! 4. SHOUTY_SNAKE_CASE
//! 5. mixedCase
//! 6. Title Case
#![deny(missing_docs)]
extern crate unicode_segmentation;

mod camel;
mod kebab;
mod mixed;
mod shouty_snake;
mod snake;
mod title;

pub use camel::CamelCase;
pub use kebab::KebabCase;
pub use mixed::MixedCase;
pub use shouty_snake::{ShoutySnakeCase, ShoutySnekCase};
pub use snake::{SnakeCase, SnekCase};
pub use title::TitleCase;

use unicode_segmentation::UnicodeSegmentation;

fn transform<F, G>(s: &str, with_word: F, boundary: G) -> String
where
    F: Fn(&str, &mut String),
    G: Fn(&mut String)
{
    macro_rules! apply {
        ($s:ident [ $init:ident .. $next:ident ], $out:ident, $boundary:ident, $with_word:ident, $first_word:ident) => {
            if !$first_word {
                $boundary(&mut $out);
            }
            $with_word(&$s[$init..$next], &mut $out);
            $init = $next_i;
        };
    }
    
    let mut out = String::new();
    let mut first_word = true;

    for word in s.unicode_words() {
        let mut char_indices = word.char_indices().peekable();
        let mut init = 0;
        let mut previous_is_uppercase = false;

        while let Some((i, c)) = char_indices.next() {
            // Skip underscore characters
            if c == '_' {
                if init == i { init += 1; }
                continue
            }

            match char_indices.peek() {
                Some(&(next_i, next)) if next == '_' => {
                    if !first_word { boundary(&mut out); }
                    with_word(&word[init..next_i], &mut out);
                    first_word = false;
                    init = next_i;
                    previous_is_uppercase = c.is_uppercase();
                }

                Some(&(_, next)) if c.is_uppercase() => {
                    if next.is_lowercase() && previous_is_uppercase {
                        if !first_word { boundary(&mut out); }
                        with_word(&word[init..i], &mut out);
                        first_word = false;
                        init = i;
                    }
                    previous_is_uppercase = true;
                }

                Some(&(next_i, next)) => {
                    if next.is_uppercase() {
                        if !first_word { boundary(&mut out); }
                        with_word(&word[init..next_i], &mut out);
                        first_word = false;
                        init = next_i;
                    }
                    previous_is_uppercase = false;
                }

                None => {
                    if !first_word { boundary(&mut out); }
                    with_word(&word[init..], &mut out);
                    first_word = false;
                    break;
                }
            }
        }
    }

    out
}

fn lowercase(s: &str, out: &mut String) {
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == 'Σ' && chars.peek().is_none() {
            out.push('ς');
        } else {
            out.extend(c.to_lowercase());
        }
    }
}

fn uppercase(s: &str, out: &mut String ) {
    for c in s.chars() {
        out.extend(c.to_uppercase())
    }
}

fn capitalize(s: &str, out: &mut String) {
    let mut char_indices = s.char_indices();
    if let Some((_, c)) = char_indices.next() {
        out.extend(c.to_uppercase());
        if let Some((i, _)) = char_indices.next() {
            lowercase(&s[i..], out);
        }
    }
}
