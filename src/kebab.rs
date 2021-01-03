use crate::{lowercase, transform, ConvertCaseOpt, Case};
use crate::convert_case::convert_case;

/// This trait defines a kebab case conversion.
///
/// In kebab-case, word boundaries are indicated by hyphens.
///
/// ## Example:
///
/// ```rust
/// use heck::ToKebabCase;
///
/// let sentence = "We are going to inherit the earth.";
/// assert_eq!(sentence.to_kebab_case(), "we-are-going-to-inherit-the-earth");
/// ```
pub trait ToKebabCase: ToOwned {
    /// Convert this type to kebab case.
    fn to_kebab_case(&self) -> Self::Owned;
}

pub fn to_kebab(s: &str, number_starts_word: bool) -> String {
    transform(s, number_starts_word, lowercase, |s| s.push('-'))
}

impl ToKebabCase for str {
    fn to_kebab_case(&self) -> Self::Owned {
        convert_case(&self, ConvertCaseOpt {case: Case::Kebab, number_starts_word: false})
    }
}

#[cfg(test)]
mod tests {
    use super::ToKebabCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_kebab_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "camel-case");
    t!(test2: "This is Human case." => "this-is-human-case");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixed-up-camel-case-with-some-spaces");
    t!(test4: "mixed_up_ snake_case with some _spaces" => "mixed-up-snake-case-with-some-spaces");
    t!(test5: "kebab-case" => "kebab-case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shouty-snake-case");
    t!(test7: "snake_case" => "snake-case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "this-contains-all-kinds-of-word-boundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxς-baﬄe");
    t!(test10: "XMLHttpRequest" => "xml-http-request");
}
