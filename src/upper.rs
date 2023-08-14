use core::fmt;

use alloc::{
    borrow::ToOwned, 
    string::ToString
};

use crate::{uppercase, transform};

/// This trait defines a upper case conversion.
///
/// In upper-case, word boundaries are indicated by hyphens.
///
/// ## Example:
///
/// ```rust
/// use heck::ToUpperCase;
///
/// let sentence = "we-are-going-to-inherit-the-earth.";
/// assert_eq!(sentence.to_upper_case(), "WE ARE GOING TO INHERIT THE EARTH");
/// ```
pub trait ToUpperCase: ToOwned {
    /// Convert this type to upper case.
    fn to_upper_case(&self) -> Self::Owned;
}

impl ToUpperCase for str {
    fn to_upper_case(&self) -> Self::Owned {
        AsUpperCase(self).to_string()
    }
}

/// This wrapper performs an upper case conversion in [`fmt::Display`].
///
/// ## Example:
///
/// ```
/// use heck::AsUpperCase;
///
/// let sentence = "we-are-going-to-inherit-the-earth.";
/// assert_eq!(format!("{}", AsUpperCase(sentence)), "WE ARE GOING TO INHERIT THE EARTH");
/// ```
pub struct AsUpperCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsUpperCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        transform(self.0.as_ref(), uppercase, |f| write!(f, " "), f)
    }
}


#[cfg(test)]
mod tests {
    use super::ToUpperCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_upper_case(), $s2)
            }
        };
    }

    t!(test1: "CamelCase" => "CAMEL CASE");
    t!(test2: "This is upper case." => "THIS IS UPPER CASE");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "MIXED UP CAMEL CASE WITH SOME SPACES");
    t!(test4: "mixed_up_ snake_case, with some _spaces" => "MIXED UP SNAKE CASE WITH SOME SPACES");
    t!(test5: "kebab-case" => "KEBAB CASE");
    t!(test6: "SHOUTY_SNAKE_CASE" => "SHOUTY SNAKE CASE");
    t!(test7: "snake_case" => "SNAKE CASE");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "THIS CONTAINS ALL KINDS OF WORD BOUNDARIES");
    t!(test9: "XΣXΣ baﬄe" => "XΣXΣ BAFFLE");
    t!(test10: "XMLHttpRequest" => "XML HTTP REQUEST");
}