/// This trait defines a camel case conversion.
///
/// In CamelCase, word boundaries are indicated by capital letters, including
/// the first word.
///
/// ## Example:
///
/// ```rust
/// extern crate heck;
/// fn main() {
///     
///     use heck::CamelCase;
///
///     let sentence = "We are not in the least afraid of ruins.";
///     assert_eq!(sentence.to_camel_case(), "WeAreNotInTheLeastAfraidOfRuins");
/// }
/// ```
pub trait CamelCase: ToOwned {
    /// Convert this type to camel case.
    fn to_camel_case(&self) -> Self::Owned;
}

impl CamelCase for str {
    fn to_camel_case(&self) -> String {
        ::transform(self, |c, s| s.extend(c.to_uppercase()), |c, s| {
            if s.len() == 0 {
                s.extend(c.to_uppercase())
            } else {
                s.extend(c.to_lowercase())
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::CamelCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_camel_case(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "CamelCase");
    t!(test2: "This is Human case." => "ThisIsHumanCase");
    t!(test3: "MixedUp CamelCase, with some Spaces" => "MixedUpCamelCaseWithSomeSpaces");
    t!(test4: "mixed_up snake_case, with some _spaces" => "MixedUpSnakeCaseWithSomeSpaces");
    t!(test5: "kebab-case" => "KebabCase");
    t!(test6: "SHOUTY_SNAKE_CASE" => "ShoutySnakeCase");
    t!(test7: "snake_case" => "SnakeCase");
    t!(test8: "this-contains_ ALLkinds OfWord_Boundaries" => "ThisContainsAllKindsOfWordBoundaries");
    // TODO unicode tests
}
