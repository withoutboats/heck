/// This trait defines a kebab case conversion.
///
/// In kebab-case, word boundaries are indicated by hyphens.
///
/// ## Example:
///
/// ```rust
/// extern crate heck;
/// fn main() {
///     
///     use heck::KebabCase;
///
///     let sentence = "We are going to inherit the earth.";
///     assert_eq!(sentence.to_kebab_case(), "we-are-going-to-inherit-the-earth");
/// }
/// ```
pub trait KebabCase: ToOwned {
    /// Convert this type to kebab case.
    fn to_kebab_case(&self) -> Self::Owned;
}

impl KebabCase for str {
    fn to_kebab_case(&self) -> Self::Owned {
        ::transform(self, |c, s| {
            s.push('-');
            s.extend(c.to_lowercase())
        }, |c, s| s.extend(c.to_lowercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::KebabCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_kebab_case(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "camel-case");
    t!(test2: "This is Human case." => "this-is-human-case");
    t!(test3: "MixedUp CamelCase, with some Spaces" => "mixed-up-camel-case-with-some-spaces");
    t!(test4: "mixed_up snake_case with some _spaces" => "mixed-up-snake-case-with-some-spaces");
    t!(test5: "kebab-case" => "kebab-case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shouty-snake-case");
    t!(test7: "snake_case" => "snake-case");
    t!(test8: "this-contains_ ALLkinds OfWord_Boundaries" => "this-contains-all-kinds-of-word-boundaries");
}
