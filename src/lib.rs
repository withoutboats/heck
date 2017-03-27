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
//! 2. A single uppercase letter (followed by no letters or by lowercase letters)
//! is considered to be just after a word boundary.
//! 3. Multiple consecutive uppercase letters are considered to be between two
//! word boundaries.
//! 
//! That is, "HelloWorld" is segmented "Hello World" whereas "HELLOworld" is
//! segmented "HELLO world."
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

fn transform<F, G>(s: &str, word_boundary: F, not_word_boundary: G) -> String
where
    F: Fn(char, &mut String),
    G: Fn(char, &mut String),
{
    let mut out = String::new();
    let mut after_word_boundary = false;

    for word in s.unicode_words() {
        if out.len() != 0 { after_word_boundary = true; }
        let mut last_c_was_uppercase = false;
        let mut multiple_uppercase = false;

        for c in word.chars() {
            if c == '_' {
                after_word_boundary = true;
                continue
            }

            if c.is_uppercase() {
                if out.len() != 0 && !last_c_was_uppercase {
                    after_word_boundary = true;
                }

                if last_c_was_uppercase {
                    multiple_uppercase = true;
                }
                last_c_was_uppercase = true;
            } else {
                if multiple_uppercase && !after_word_boundary {
                    after_word_boundary = true;
                }

                multiple_uppercase = false;
                last_c_was_uppercase = false;
            }
            if after_word_boundary {
                word_boundary(c, &mut out);
            } else {
                not_word_boundary(c, &mut out);
            }
            after_word_boundary = false;
        }
    }

    out
}
