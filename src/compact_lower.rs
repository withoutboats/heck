use alloc::{
    borrow::ToOwned,
    fmt,
    string::{String, ToString},
};

use crate::{lowercase, transform};

/// This trait defines a compact lower case conversion.
///
/// In compactlowercase, word boundaries are omitted.
///
/// ## Example:
///
/// ```rust
/// use heck::ToCompactLowercase;
///
/// let sentence = "We carry a new world here, in our hearts.";
/// assert_eq!(sentence.to_compact_lowercase(), "wecarryanewworldhereinourhearts");
/// ```
pub trait ToCompactLowercase: ToOwned {
    /// Convert this type to compact lowercase.
    fn to_compact_lowercase(&self) -> Self::Owned;
}

impl ToCompactLowercase for str {
    fn to_compact_lowercase(&self) -> String {
        AsCompactLowercase(self).to_string()
    }
}

/// This wrapper performs a compact lowercase conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsCompactLowercase;
///
/// let sentence = "We carry a new world here, in our hearts.";
/// assert_eq!(format!("{}", AsCompactLowercase(sentence)), "wecarryanewworldhereinourhearts");
/// ```
pub struct AsCompactLowercase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsCompactLowercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), lowercase, |_f| Ok(()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::ToCompactLowercase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_compact_lowercase(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "camelcase");
    t!(test2: "This is Human case." => "thisishumancase");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixedupcamelcasewithsomespaces");
    t!(test4: "mixed_up_ snake_case with some _spaces" => "mixedupsnakecasewithsomespaces");
    t!(test5: "kebab-case" => "kebabcase");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shoutysnakecase");
    t!(test7: "snake_case" => "snakecase");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "thiscontainsallkindsofwordboundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxςbaﬄe");
    t!(test10: "XMLHttpRequest" => "xmlhttprequest");
    t!(test11: "FIELD_NAME11" => "fieldname11");
    t!(test12: "99BOTTLES" => "99bottles");
    t!(test13: "FieldNamE11" => "fieldname11");
    t!(test14: "abc123def456" => "abc123def456");
    t!(test16: "abc123DEF456" => "abc123def456");
    t!(test17: "abc123Def456" => "abc123def456");
    t!(test18: "abc123DEf456" => "abc123def456");
    t!(test19: "ABC123def456" => "abc123def456");
    t!(test20: "ABC123DEF456" => "abc123def456");
    t!(test21: "ABC123Def456" => "abc123def456");
    t!(test22: "ABC123DEf456" => "abc123def456");
    t!(test23: "ABC123dEEf456FOO" => "abc123deef456foo");
    t!(test24: "abcDEF" => "abcdef");
    t!(test25: "ABcDE" => "abcde");
}
