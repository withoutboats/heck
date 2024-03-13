use core::fmt;

use alloc::{borrow::ToOwned, string::ToString};

use crate::{lowercase, transform};

/// This trait defines a dot case conversion.
///
/// In dot-case, word boundaries are indicated by hyphens.
///
/// ## Example:
///
/// ```rust
/// use heck::ToDotCase;
///
/// let sentence = "We are going to inherit the earth.";
/// assert_eq!(sentence.to_dot_case(), "we.are.going.to.inherit.the.earth");
/// ```
pub trait ToDotCase: ToOwned {
    /// Convert this type to dot case.
    fn to_dot_case(&self) -> Self::Owned;
}

impl ToDotCase for str {
    fn to_dot_case(&self) -> Self::Owned {
        AsDotCase(self).to_string()
    }
}

/// This wrapper performs a dot case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsDotCase;
///
/// let sentence = "We are going to inherit the earth.";
/// assert_eq!(format!("{}", AsDotCase(sentence)), "we.are.going.to.inherit.the.earth");
/// ```
pub struct AsDotCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsDotCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), lowercase, |f| write!(f, "."), f)
    }
}

#[cfg(test)]
mod tests {
    use super::ToDotCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_dot_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "camel.case");
    t!(test2: "This is Human case." => "this.is.human.case");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixed.up.camel.case.with.some.spaces");
    t!(test4: "mixed_up_ snake_case with some _spaces" => "mixed.up.snake.case.with.some.spaces");
    t!(test5: "kebab-case" => "kebab.case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shouty.snake.case");
    t!(test7: "snake_case" => "snake.case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "this.contains.all.kinds.of.word.boundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxς.baﬄe");
    t!(test10: "XMLHttpRequest" => "xml.http.request");
    // Japanese and Chinese do not have word separation.
    t!(test12: "ファイルを読み込み" => "ファイルを読み込み");
    t!(test13: "祝你一天过得愉快" => "祝你一天过得愉快");
}
