use std::fmt;

use crate::{lowercase, transform};

/// This trait defines a dot case conversion.
///
/// In dot_case, word boundaries are indicated by underscores.
///
/// ## Example:
///
/// ```rust
/// use heck::ToDotCase;
///
/// let sentence = "We carry a new world here, in our hearts.";
/// assert_eq!(sentence.to_dot_case(), "we.carry.a.new.world.here.in.our.hearts");
/// ```
pub trait ToDotCase: ToOwned {
    /// Convert this type to dot case.
    fn to_dot_case(&self) -> Self::Owned;
}

impl ToDotCase for str {
    fn to_dot_case(&self) -> String {
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
/// let sentence = "We carry a new world here, in our hearts.";
/// assert_eq!(format!("{}", AsDotCase(sentence)), "we.carry.a.new.world.here.in.our.hearts");
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
    #[cfg(feature = "unicode")]
    t!(test9: "XΣXΣ baﬄe" => "xσxς.baﬄe");
    t!(test10: "XMLHttpRequest" => "xml.http.request");
    t!(test11: "FIELD_NAME11" => "field.name11");
    t!(test12: "99BOTTLES" => "99bottles");
    t!(test13: "FieldNamE11" => "field.nam.e11");
    t!(test14: "abc123def456" => "abc123def456");
    t!(test16: "abc123DEF456" => "abc123.def456");
    t!(test17: "abc123Def456" => "abc123.def456");
    t!(test18: "abc123DEf456" => "abc123.d.ef456");
    t!(test19: "ABC123def456" => "abc123def456");
    t!(test20: "ABC123DEF456" => "abc123def456");
    t!(test21: "ABC123Def456" => "abc123.def456");
    t!(test22: "ABC123DEf456" => "abc123d.ef456");
    t!(test23: "ABC123dEEf456FOO" => "abc123d.e.ef456.foo");
    t!(test24: "abcDEF" => "abc.def");
    t!(test25: "ABcDE" => "a.bc.de");
    t!(test26: "dot_case" => "dot.case");
    t!(test27: "SHOUTY_DOT_CASE" => "shouty.dot.case");
    t!(test28: "mixed.up. dot.case with some .spaces" => "mixed.up.dot.case.with.some.spaces");
}
