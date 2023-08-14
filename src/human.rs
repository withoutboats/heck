use core::fmt;

use alloc::{
    borrow::ToOwned,
    string::ToString,
};

use crate::{capitalize, lowercase, transform};

/// This trait defines a human case conversion.
///
/// In human-case, word boundaries are indicated by hyphens.
///
/// ## Example:
///
/// ```rust
/// use heck::ToHumanCase;
///
/// let sentence = "we-are-going-to-inherit-the-earth.";
/// assert_eq!(sentence.to_human_case(), "We are going to inherit the earth");
/// ```
pub trait ToHumanCase: ToOwned {
    /// Convert this type to human case.
    fn to_human_case(&self) -> Self::Owned;
}

impl ToHumanCase for str {
    fn to_human_case(&self) -> Self::Owned {
        AsHumanCase(self).to_string()
    }
}

/// This wrapper performs an human case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsHumanCase;
///
/// let sentence = "we-are-going-to-inherit-the-earth.";
/// assert_eq!(format!("{}", AsHumanCase(sentence)), "We are going to inherit the earth");
/// ```
pub struct AsHumanCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsHumanCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first = true;
        transform(
            self.0.as_ref(),
            |s, f| {
                if first {
                    first = false;
                    capitalize(s, f)
                } else {
                    lowercase(s, f)
                }
            },
            |f| write!(f, " "),
            f,
        )
    }
}


#[cfg(test)]
mod tests {
    use super::ToHumanCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_human_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "Camel case");
    t!(test2: "This is Human case." => "This is human case");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "Mixed up camel case with some spaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "Mixed up snake case with some spaces");
    t!(test5: "kebab-case" => "Kebab case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "Shouty snake case");
    t!(test7: "snake_case" => "Snake case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "This contains all kinds of word boundaries");
    t!(test9: "XΣXΣ baﬄe" => "Xσxς baﬄe");
    t!(test10: "XMLHttpRequest" => "Xml http request");
}