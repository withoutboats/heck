use core::fmt;

use alloc::{borrow::ToOwned, string::ToString};

use crate::{titlecase, transform};

/// This trait defines a train case conversion.
///
/// In Train-Case, word boundaries are indicated by hyphens and words start
/// with Capital Letters.
///
/// ## Example:
///
/// ```rust
/// use heck::ToTrainCase;
///
/// let sentence = "We are going to inherit the earth.";
/// assert_eq!(sentence.to_train_case(), "We-Are-Going-To-Inherit-The-Earth");
/// ```
pub trait ToTrainCase: ToOwned {
    /// Convert this type to Train-Case.
    fn to_train_case(&self) -> Self::Owned;
}

impl ToTrainCase for str {
    fn to_train_case(&self) -> Self::Owned {
        AsTrainCase(self).to_string()
    }
}

/// This wrapper performs a train case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsTrainCase;
///
/// let sentence = "We are going to inherit the earth.";
/// assert_eq!(format!("{}", AsTrainCase(sentence)), "We-Are-Going-To-Inherit-The-Earth");
/// ```
pub struct AsTrainCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsTrainCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), titlecase, |f| write!(f, "-"), f)
    }
}

#[cfg(test)]
mod tests {
    use super::ToTrainCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_train_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "Camel-Case");
    t!(test2: "This is Human case." => "This-Is-Human-Case");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "Mixed-Up-Camel-Case-With-Some-Spaces");
    t!(test4: "mixed_up_ snake_case with some _spaces" => "Mixed-Up-Snake-Case-With-Some-Spaces");
    t!(test5: "kebab-case" => "Kebab-Case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "Shouty-Snake-Case");
    t!(test7: "snake_case" => "Snake-Case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "This-Contains-All-Kinds-Of-Word-Boundaries");
    t!(test9: "XΣXΣ baﬄe" => "Xσxς-Baﬄe");
    t!(test10: "XMLHttpRequest" => "Xml-Http-Request");
    t!(test11: "FIELD_NAME11" => "Field-Name11");
    t!(test12: "99BOTTLES" => "99Bottles");
    t!(test13: "FieldNamE11" => "Field-Nam-E11");
    t!(test14: "abc123def456" => "Abc123def456");
    t!(test16: "abc123DEF456" => "Abc123def456");
    t!(test17: "abc123Def456" => "Abc123-Def456");
    t!(test18: "abc123DEf456" => "Abc123d-Ef456");
    t!(test19: "ABC123def456" => "Abc123def456");
    t!(test20: "ABC123DEF456" => "Abc123def456");
    t!(test21: "ABC123Def456" => "Abc123-Def456");
    t!(test22: "ABC123DEf456" => "Abc123d-Ef456");
    t!(test23: "ABC123dEEf456FOO" => "Abc123d-E-Ef456foo");
    t!(test24: "abcDEF" => "Abc-Def");
    t!(test25: "ABcDE" => "A-Bc-De");
    t!(test26: "ǄO" => "ǅo");
    t!(test27: "ǆO" => "ǅ-O");
    t!(test28: "ǆo" => "ǅo");
    t!(test29: "∇𝐀" => "∇𝐀");
    t!(test30: "∇𝔞" => "∇𝔞");
    t!(test31: "𝔞" => "𝔞");
    t!(test32: "🐈‍⬛🐈" => "\u{200d}");
    t!(test33: "🐈‍⬛🐈a" => "\u{200d}-A");
    t!(test34: "A🐈‍⬛🐈a" => "A-\u{200D}-A");
    t!(test35: "☕" => "");
    t!(test36: "a*️⃣b" => "A-\u{fe0f}-B");
    t!(test37: "a*b" => "A-B");
    t!(test38: "\u{0301}a" => "\u{0301}A");
    t!(test39: "a\u{0301}B" => "A\u{0301}-B");
    t!(test40: "ﬄololo" => "Fflololo");

    t!(uts55_test1: "TypeII" => "Type-Ii");
    t!(uts55_test2: "OCaml" => "O-Caml");
    t!(uts55_test3: "HTTPЗапрос" => "Http-Запрос");
    t!(uts55_test4: "UAX9ClauseHL4" => "Uax9-Clause-Hl4");
    t!(uts55_test5: "LOUD_SNAKE" => "Loud-Snake");

    t!(uts55_test6: "Fancy_Snake" => "Fancy-Snake");
    t!(uts55_test7: "snake-kebab" => "Snake-Kebab");
    t!(uts55_test8: "Paral·lel" => "Paral·lel");
    t!(uts55_test9: "microB" => "Micro-B");
    t!(uts55_test10: "microᖯ" => "Microᖯ");
    t!(uts55_test11: "HTTPसर्वर" => "Httpसर्वर");
    t!(uts55_test12: "dromedaryCamel" => "Dromedary-Camel");
    t!(uts55_test13: "snakeELEPHANTSnake" => "Snake-Elephant-Snake");
}
