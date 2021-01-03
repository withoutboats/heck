use crate::{capitalize, lowercase, transform, ConvertCaseOpt, Case};
use crate::convert_case::convert_case;

/// This trait defines a lower camel case conversion.
///
/// In lowerCamelCase, word boundaries are indicated by capital letters,
/// excepting the first word.
///
/// ## Example:
///
/// ```rust
/// use heck::ToLowerCamelCase;
///
/// let sentence = "It is we who built these palaces and cities.";
/// assert_eq!(sentence.to_lower_camel_case(), "itIsWeWhoBuiltThesePalacesAndCities");
/// ```
pub trait ToLowerCamelCase: ToOwned {
    /// Convert this type to lower camel case.
    fn to_lower_camel_case(&self) -> Self::Owned;
}

pub fn to_lower_camel_case(s: &str, number_starts_word: bool) -> String {
    transform(
        s, number_starts_word,
        |s, out| {
            if out.is_empty() {
                lowercase(s, out);
            } else {
                capitalize(s, out)
            }
        },
        |_| {},
    )
}

impl ToLowerCamelCase for str {
    fn to_lower_camel_case(&self) -> Self::Owned {
        convert_case(&self, ConvertCaseOpt {case: Case::LowerCamel, number_starts_word: false})
    }
}

#[cfg(test)]
mod tests {
    use super::ToLowerCamelCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_lower_camel_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "camelCase");
    t!(test2: "This is Human case." => "thisIsHumanCase");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixedUpCamelCaseWithSomeSpaces");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "mixedUpSnakeCaseWithSomeSpaces");
    t!(test5: "kebab-case" => "kebabCase");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shoutySnakeCase");
    t!(test7: "snake_case" => "snakeCase");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "thisContainsAllKindsOfWordBoundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxςBaﬄe");
    t!(test10: "XMLHttpRequest" => "xmlHttpRequest");
    // TODO unicode tests
}
