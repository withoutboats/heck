use crate::{lowercase, transform};

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

impl ToKebabCase for str {
    fn to_kebab_case(&self) -> Self::Owned {
        transform(self, lowercase, |s| s.push('-'))
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
    t!(test11: "ファイルを読み込み" => "ファイルを読み込み");
    t!(test12: "お前はもう死んでいる！何？" => "お前はもう死んでいる何");
    t!(test13: "石室诗士施氏，嗜狮，誓食十狮。" => "石室诗士施氏嗜狮誓食十狮");
    t!(test14: "石室詩士施氏，嗜獅，誓食十獅。" => "石室詩士施氏嗜獅誓食十獅");
    t!(test15: "ㄕˊㄕˋㄕㄕˋㄕㄕˋ，ㄕˋㄕ，ㄕˋㄕˊㄕˊㄕ。" => "ㄕˊㄕˋㄕㄕˋㄕㄕˋㄕˋㄕㄕˋㄕˊㄕˊㄕ");
    t!(test16: "shí shì shī shì shī shì ， shì shī ， shì shí shí shī 。" => "shí-shì-shī-shì-shī-shì-shì-shī-shì-shí-shí-shī");
    t!(test17: "sek6 sat1 si1 si6 si1 si6 ，si3 si1 ，sai6 sik6 sap6 si1 。" => "sek6-sat1-si1-si6-si1-si6-si3-si1-sai6-sik6-sap6-si1");
    t!(test18: "唱K" => "唱-k"); // this is incorrect but it doesn't matter
    t!(test19: "YouTube" => "you-tube"); // this is incorrect but it doesn't matter
}
