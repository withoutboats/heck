use std::fmt;

use crate::{transform, uppercase};

/// This trait defines a shouty dot case conversion.
///
/// In SHOUTY_dot_CASE, word boundaries are indicated by underscores and all
/// words are in uppercase.
///
/// ## Example:
///
/// ```rust
/// use heck::ToShoutyDotCase;
///
/// let sentence = "That world is growing in this minute.";
/// assert_eq!(sentence.to_shouty_dot_case(), "THAT.WORLD.IS.GROWING.IN.THIS.MINUTE");
/// ```
pub trait ToShoutyDotCase: ToOwned {
    /// Convert this type to shouty dot case.
    fn to_shouty_dot_case(&self) -> Self::Owned;
}

impl ToShoutyDotCase for str {
    fn to_shouty_dot_case(&self) -> Self::Owned {
        AsShoutyDotCase(self).to_string()
    }
}

/// This wrapper performs a shouty dot  case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsShoutyDotCase;
///
/// let sentence = "That world is growing in this minute.";
/// assert_eq!(format!("{}", AsShoutyDotCase(sentence)), "THAT.WORLD.IS.GROWING.IN.THIS.MINUTE");
/// ```
pub struct AsShoutyDotCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsShoutyDotCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), uppercase, |f| write!(f, "."), f)
    }
}

#[cfg(test)]
mod tests {
    use super::ToShoutyDotCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_shouty_dot_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "CAMEL.CASE");
    t!(test2: "This is Human case." => "THIS.IS.HUMAN.CASE");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "MIXED.UP.CAMEL.CASE.WITH.SOME.SPACES");
    t!(test4: "mixed_up_snake_case with some _spaces" => "MIXED.UP.SNAKE.CASE.WITH.SOME.SPACES");
    t!(test5: "kebab-case" => "KEBAB.CASE");
    t!(test6: "SHOUTY_SNAKE_CASE" => "SHOUTY.SNAKE.CASE");
    t!(test7: "snake_case" => "SNAKE.CASE");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "THIS.CONTAINS.ALL.KINDS.OF.WORD.BOUNDARIES");
    #[cfg(feature = "unicode")]
    t!(test9: "XΣXΣ baﬄe" => "XΣXΣ.BAFFLE");
    t!(test10: "XMLHttpRequest" => "XML.HTTP.REQUEST");
    t!(test11: "SHOUTY.DOT.CASE" => "SHOUTY.DOT.CASE");
    t!(test12: "dot.case" => "DOT.CASE");
    t!(test13: "mixed.up. dot.case with some .spaces" => "MIXED.UP.DOT.CASE.WITH.SOME.SPACES");
}
