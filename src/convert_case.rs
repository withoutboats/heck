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

    #[test]
    fn number_starts_word_kebab_simple() {
        assert_eq!(
            "aes128".convert_case(ConvertCaseOpt {
                case: Case::Kebab,
                number_starts_word: true
            }),
            "aes-128"
        );
    }

    #[test]
    fn number_starts_word_kebab_complex() {
        assert_eq!(
            "aes128Key".convert_case(ConvertCaseOpt {
                case: Case::Kebab,
                number_starts_word: true
            }),
            "aes-128-key"
        );
    }

    #[test]
    fn number_starts_word_kebab_complex_underscore() {
        assert_eq!(
            "aes128 Key".convert_case(ConvertCaseOpt {
                case: Case::Kebab,
                number_starts_word: true
            }),
            "aes-128-key"
        );
    }

    #[test]
    fn number_starts_word_false_kebab_complex_underscore() {
        assert_eq!(
            "aes128 Key".convert_case(ConvertCaseOpt {
                case: Case::Kebab,
                number_starts_word: false
            }),
            "aes128-key"
        );
    }

    #[test]
    fn number_starts_word_true_title_case() {
        assert_eq!(
            "AES128BitKey".convert_case(ConvertCaseOpt {
                case: Case::Title,
                number_starts_word: true
            }),
            "Aes 128 Bit Key"
        );
    }

    #[test]
    fn number_starts_word_true_snake_case() {
        assert_eq!(
            "99BOTTLES".convert_case(ConvertCaseOpt {
                case: Case::Snake,
                number_starts_word: true
            }),
            "99_bottles"
        );
    }

    #[test]
    fn number_starts_word_true_snake_case_2() {
        assert_eq!(
            "abc123DEF456".convert_case(ConvertCaseOpt {
                case: Case::Snake,
                number_starts_word: true
            }),
            "abc_123_def_456"
        );
    }

    #[test]
    fn number_starts_word_true_snake_case_3() {
        assert_eq!(
            "ABC123dEEf456FOO".convert_case(ConvertCaseOpt {
                case: Case::Snake,
                number_starts_word: true
            }),
            "abc_123_d_e_ef_456_foo"
        );
    }
}
