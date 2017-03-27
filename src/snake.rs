/// This trait defines a camel case conversion.
///
/// In snake_case, word boundaries are indicated by underscores.
///
/// ## Example:
///
/// ```rust
/// extern crate heck;
/// fn main() {
///     
///     use heck::SnakeCase;
///
///     let sentence = "We carry a new world here, in our hearts.";
///     assert_eq!(sentence.to_snake_case(), "we_carry_a_new_world_here_in_our_hearts");
/// }
/// ```
pub trait SnakeCase: ToOwned {
    /// Convert this type to snake case.
    fn to_snake_case(&self) -> Self::Owned;
}

/// Oh heck, SnekCase is an alias for SnakeCase. See SnakeCase for
/// more documentation.
pub trait SnekCase: ToOwned {
    /// Convert this type to snek case.
    fn to_snek_case(&self) -> Self::Owned;
}

impl<T: SnakeCase> SnekCase for T {
    fn to_snek_case(&self) -> Self::Owned {
        self.to_snake_case()
    }
}

impl SnakeCase for str {
    fn to_snake_case(&self) -> String {
        ::transform(self, ::lowercase, |s| s.push('_'))
    }
}

#[cfg(test)]
mod tests {
    use super::SnakeCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_snake_case(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "camel_case");
    t!(test2: "This is Human case." => "this_is_human_case");
    t!(test3: "MixedUP CamelCase, with some Spaces" => "mixed_up_camel_case_with_some_spaces");
    t!(test4: "mixed_up snake_case with some _spaces" => "mixed_up_snake_case_with_some_spaces");
    t!(test5: "kebab-case" => "kebab_case");
    t!(test6: "SHOUTY_SNAKE_CASE" => "shouty_snake_case");
    t!(test7: "snake_case" => "snake_case");
    t!(test8: "this-contains_ ALLKinds OfWord_Boundaries" => "this_contains_all_kinds_of_word_boundaries");
    t!(test9: "XΣXΣ baﬄe" => "xσxς_baﬄe");
    t!(test10: "XMLHttpRequest" => "xml_http_request");
}
