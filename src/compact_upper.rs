use alloc::{
    borrow::ToOwned,
    fmt,
    string::{String, ToString},
};

use crate::{transform, uppercase};

/// This trait defines a compact upper case conversion.
///
/// In COMPACTUPPERCASE, word boundaries are omitted.
///
/// ## Example:
///
/// ```rust
/// use heck::ToCompactUppercase;
///
/// let sentence = "We carry a new world here, in our hearts.";
/// assert_eq!(sentence.to_compact_uppercase(), "WECARRYANEWWORLDHEREINOURHEARTS");
/// ```
pub trait ToCompactUppercase: ToOwned {
    /// Convert this type to compact lowercase.
    fn to_compact_uppercase(&self) -> Self::Owned;
}

impl ToCompactUppercase for str {
    fn to_compact_uppercase(&self) -> String {
        AsCompactUppercase(self).to_string()
    }
}

/// This wrapper performs a compact uppercase conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsCompactUppercase;
///
/// let sentence = "We carry a new world here, in our hearts.";
/// assert_eq!(format!("{}", AsCompactUppercase(sentence)), "WECARRYANEWWORLDHEREINOURHEARTS");
/// ```
pub struct AsCompactUppercase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsCompactUppercase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), uppercase, |_f| Ok(()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::ToCompactUppercase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_compact_uppercase(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "CAMELCASE");
    t!(test2: "This is Human case." => "THISISHUMANCASE");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "MIXEDUPCAMELCASEWITHSOMESPACES");
    t!(test4: "mixed_up_ snake_case with some _spaces" => "MIXEDUPSNAKECASEWITHSOMESPACES");
    t!(test5: "kebab-case" => "KEBABCASE");
    t!(test6: "SHOUTY_SNAKE_CASE" => "SHOUTYSNAKECASE");
    t!(test7: "snake_case" => "SNAKECASE");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "THISCONTAINSALLKINDSOFWORDBOUNDARIES");
    t!(test9: "XΣXΣ baﬄe" => "XΣXΣBAFFLE");
    t!(test10: "XMLHttpRequest" => "XMLHTTPREQUEST");
    t!(test11: "FIELD_NAME11" => "FIELDNAME11");
    t!(test12: "99BOTTLES" => "99BOTTLES");
    t!(test13: "FieldNamE11" => "FIELDNAME11");
    t!(test14: "abc123def456" => "ABC123DEF456");
    t!(test16: "abc123DEF456" => "ABC123DEF456");
    t!(test17: "abc123Def456" => "ABC123DEF456");
    t!(test18: "abc123DEf456" => "ABC123DEF456");
    t!(test19: "ABC123def456" => "ABC123DEF456");
    t!(test20: "ABC123DEF456" => "ABC123DEF456");
    t!(test21: "ABC123Def456" => "ABC123DEF456");
    t!(test22: "ABC123DEf456" => "ABC123DEF456");
    t!(test23: "ABC123dEEf456FOO" => "ABC123DEEF456FOO");
    t!(test24: "abcDEF" => "ABCDEF");
    t!(test25: "ABcDE" => "ABCDE");
}
