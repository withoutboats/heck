use core::fmt;

use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
};

use crate::{lowercase, transform};

/// This trait defines a mid sentence case conversion.
///
/// In mid sentence case, word boundaries are indicated by spaces, and every word is
/// lowercased.
///
/// ## Example:
///
/// ```rust
/// use heck::ToMidSentenceCase;
///
/// let sentence = "We have always lived in slums and holes in the wall.";
/// assert_eq!(sentence.to_mid_sentence_case(), "we have always lived in slums and holes in the wall");
/// ```
pub trait ToMidSentenceCase: ToOwned {
    /// Convert this type to title case.
    fn to_mid_sentence_case(&self) -> Self::Owned;
}

impl ToMidSentenceCase for str {
    fn to_mid_sentence_case(&self) -> String {
        AsMidSentenceCase(self).to_string()
    }
}

/// This wrapper performs a title case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsMidSentenceCase;
///
/// let sentence = "We have always lived in slums and holes in the wall.";
/// assert_eq!(format!("{}", AsMidSentenceCase(sentence)), "we have always lived in slums and holes in the wall");
/// ```
pub struct AsMidSentenceCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsMidSentenceCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), lowercase, |f| write!(f, " "), f)
    }
}

#[cfg(test)]
mod tests {
    use super::ToMidSentenceCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_mid_sentence_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "camel case");
    t!(test2: "This is Human case." => "this is human case");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixed up camel case with some spaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "mixed up snake case with some spaces");
    t!(test5: "kebab-case" => "kebab case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shouty snake case");
    t!(test7: "snake_case" => "snake case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "this contains all kinds of word boundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxς baﬄe");
    t!(test10: "XMLHttpRequest" => "xml http request");
}
