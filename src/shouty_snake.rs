/// This trait defines a shouty snake case conversion.
///
/// In SHOUTY_SNAKE_CASE, word boundaries are indicated by underscores and all
/// words are in uppercase.
///
/// ## Example:
///
/// ```rust
/// extern crate heck;
/// fn main() {
///     
///     use heck::ShoutySnakeCase;
///
///     let sentence = "That world is growing in this minute.";
///     assert_eq!(sentence.to_shouty_snake_case(), "THAT_WORLD_IS_GROWING_IN_THIS_MINUTE");
/// }
/// ```
pub trait ShoutySnakeCase: ToOwned {
    /// Convert this type to shouty snake case.
    fn to_shouty_snake_case(&self) -> Self::Owned;
}

/// Oh heck, ShoutySnekCase is an alias for ShoutySnakeCase. See ShoutySnakeCase for
/// more documentation.
pub trait ShoutySnekCase: ToOwned {
    /// CONVERT THIS TYPE TO SNEK CASE.
    #[allow(non_snake_case)]
    fn TO_SHOUTY_SNEK_CASE(&self) -> Self::Owned;
}

impl<T: ShoutySnakeCase> ShoutySnekCase for T {
    fn TO_SHOUTY_SNEK_CASE(&self) -> Self::Owned {
        self.to_shouty_snake_case()
    }
}


impl ShoutySnakeCase for str {
    fn to_shouty_snake_case(&self) -> Self::Owned {
        ::transform(self, |c, s| {
            s.push('_');
            s.extend(c.to_uppercase())
        }, |c, s| s.extend(c.to_uppercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::ShoutySnakeCase;

    macro_rules! t {
        ($t:ident : $s1:expr => $s2:expr) => {
            #[test]
            fn $t() {
                assert_eq!($s1.to_shouty_snake_case(), $s2)
            }
        }
    }

    t!(test1: "CamelCase" => "CAMEL_CASE");
    t!(test2: "This is Human case." => "THIS_IS_HUMAN_CASE");
    t!(test3: "MixedUp CamelCase, with some Spaces" => "MIXED_UP_CAMEL_CASE_WITH_SOME_SPACES");
    t!(test4: "mixed_up snake_case with some _spaces" => "MIXED_UP_SNAKE_CASE_WITH_SOME_SPACES");
    t!(test5: "kebab-case" => "KEBAB_CASE");
    t!(test6: "SHOUTY_SNAKE_CASE" => "SHOUTY_SNAKE_CASE");
    t!(test7: "snake_case" => "SNAKE_CASE");
    t!(test8: "this-contains_ ALLkinds OfWord_Boundaries" => "THIS_CONTAINS_ALL_KINDS_OF_WORD_BOUNDARIES");
    // TODO unicode tests
}
