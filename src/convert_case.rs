use crate::kebab::to_kebab;
use crate::lower_camel::to_lower_camel_case;
use crate::shouty_kebab::to_shouty_kebab_case;
use crate::shouty_snake::to_shouty_snake_case;
use crate::snake::to_snake_case;
use crate::title::to_title_case;
use crate::upper_camel::to_upper_camel_case;

/// This trait defines a wrapper function for other case-conversion functions
/// in that this can tweak the behaviour of those functions based on customization options
///
///
/// ## Example:
/// ```rust
///
/// use heck::{ConvertCase, ConvertCaseOpt, Case};
///
/// let sentence = "Aes128";
/// assert_eq!(sentence.convert_case(ConvertCaseOpt { case: Case::ShoutyKebab, number_starts_word: true }),
/// "AES-128");
///

pub trait ConvertCase: ToOwned {
    /// Convert this type to supported cases with options
    fn convert_case(&self, opt: ConvertCaseOpt) -> String;
}

/// Options to tweak how convert_case will behave
pub struct ConvertCaseOpt {
    /// supported case
    pub case: Case,
    /// whether numbers should start a new word
    pub number_starts_word: bool,
}

/// supported cases
pub enum Case {
    /// kebab-case
    Kebab,
    /// lowerCamelCase
    LowerCamel,
    /// SHOUT-KEBAB-CASE
    ShoutyKebab,
    /// SHOUTY_SNAKE_CASE
    ShoutySnake,
    /// snake_case
    Snake,
    /// Title Case
    Title,
    /// UpperCamelCase
    UpperCamel,
}

pub fn convert_case(s: &str, opt: ConvertCaseOpt) -> String {
    match opt.case {
        Case::Kebab => to_kebab(s, opt.number_starts_word),
        Case::LowerCamel => to_lower_camel_case(s, opt.number_starts_word),
        Case::ShoutyKebab => to_shouty_kebab_case(s, opt.number_starts_word),
        Case::ShoutySnake => to_shouty_snake_case(s, opt.number_starts_word),
        Case::Snake => to_snake_case(s, opt.number_starts_word),
        Case::Title => to_title_case(s, opt.number_starts_word),
        Case::UpperCamel => to_upper_camel_case(s, opt.number_starts_word),
    }
}
impl ConvertCase for str {
    fn convert_case(&self, opt: ConvertCaseOpt) -> Self::Owned {
        convert_case(self, opt)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Case, ConvertCase, ConvertCaseOpt};

    macro_rules! t {
        ($t:ident: $s1:expr, $c:ident,  $n:ident  => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!(
                    $s1.convert_case(ConvertCaseOpt {
                        case: Case::$c,
                        number_starts_word: $n
                    }),
                    $s2
                )
            }
        };
    }

    t!(test1: "AES 128 bit key", LowerCamel, false => "aes128BitKey");
    t!(test2: "AES 128 bit key", LowerCamel, true => "aes128BitKey");
    t!(test3: "AES 128 bit key", Kebab, true => "aes-128-bit-key");
    t!(test4: "99BOTTLES", Snake, false => "99bottles");
    t!(test5: "99BOTTLES", Snake, true => "99_bottles");
    t!(test6: "ABC123dEEf456FOO", Snake, false => "abc123d_e_ef456_foo");
    t!(test7: "ABC123dEEf456FOO", Snake, true => "abc_123_d_e_ef_456_foo");
    t!(test8: "XMLHttpRequest404", Title, false => "Xml Http Request404");
    t!(test9: "XMLHttpRequest404", Title, true => "Xml Http Request 404");
    t!(test10: "this-contains_ ALLKinds OfWord_Boundaries_1Also", Kebab, false => "this-contains-all-kinds-of-word-boundaries-1also");
    t!(test11: "this-contains_ ALLKinds OfWord_Boundaries_1Also", Kebab, true => "this-contains-all-kinds-of-word-boundaries-1-also");
}
