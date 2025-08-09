use core::{fmt::Display, str::FromStr};

use alloc::{borrow::ToOwned, fmt, string::String};

use crate::{
    AsKebabCase, AsLowerCamelCase, AsShoutyKebabCase, AsShoutySnakeCase, AsSnakeCase, AsTitleCase,
    AsTrainCase, AsUpperCamelCase, ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase,
    ToShoutySnakeCase, ToSnakeCase, ToTitleCase, ToTrainCase, ToUpperCamelCase,
};
/// Error returned when a case is not found
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseNotFound(String);

impl<T: Into<String>> From<T> for CaseNotFound {
    fn from(s: T) -> Self {
        Self(s.into())
    }
}
impl Display for CaseNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid Case: {:?} expected one of {}",
            self.0, EXPTECTED_CASES
        )
    }
}
/// Implements AsRef<str>, Into<String>, and Display for the specified case
/// Creates a static array of all the case names
macro_rules! variants {
    ($($varient:ident => $name:literal),+) => {
        #[doc = "Case variants for the `Case` enum"]
        pub static CASES: &[&str] = &[$($name),+];
        #[doc = "A string of all the expected case names. Formatted as [`lowerCamelCase`, `upperCamelCase`, ...]"]
        static EXPTECTED_CASES: &str = concat!("[", $("`", $name, "`, "),+,"]");
        impl AsRef<str> for Case {
            fn as_ref(&self) -> &str {
                match self {
                    $(Case::$varient => $name),+
                }
            }
        }
        impl From<Case> for String {
            fn from(case: Case) -> Self {
                match case {
                    $(Case::$varient => $name.into()),+
                }
            }
        }
        impl Display for Case {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $(Case::$varient => f.write_str($name)),+
                }
            }
        }
    };
}
#[cfg(feature = "std")]
impl std::error::Error for CaseNotFound {}

/// The case to convert to
///
/// This is a way to specify case conversion "dynamicly"
/// # Example:
/// ```
///
/// use heck::Case;
/// use heck::ToCase;
/// let original = "We are going to inherit the earth";
/// let cases = vec![
///     ("camelCase", "weAreGoingToInheritTheEarth"),
///     ("UpperCamelCase", "WeAreGoingToInheritTheEarth"),
///     ("PascalCase", "WeAreGoingToInheritTheEarth"),
///     ("snake_case", "we_are_going_to_inherit_the_earth"),
///     ("UPPER_SNAKE_CASE", "WE_ARE_GOING_TO_INHERIT_THE_EARTH"),
///     ("kebab-case", "we-are-going-to-inherit-the-earth"),
///     ("UPPER-KEBAB-CASE", "WE-ARE-GOING-TO-INHERIT-THE-EARTH"),
///     ("Title Case", "We Are Going To Inherit The Earth"),
///     ("Train-Case", "We-Are-Going-To-Inherit-The-Earth"),
///     ("UPPERCASE", "WE ARE GOING TO INHERIT THE EARTH"),
///     ("lowercase", "we are going to inherit the earth"),
/// ];
/// for (case, expected) in cases {
///    let case = case.parse().expect("Failed to parse case");
///    assert_eq!(original.to_case(case), expected);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Case {
    /// `camelCase` is primary name
    ///
    /// Other accepted names are `lowerCamelCase`
    ///
    /// [See Also](ToLowerCamelCase)
    LowerCamelCase,
    /// `UpperCamelCase` is primary name
    ///
    /// [See Also](ToUpperCamelCase)
    UpperCamelCase,
    /// `PascalCase` is primary name
    ///
    /// [See Also](ToPascalCase)
    PascalCase,
    /// `snake_case` is primary name
    ///
    /// Other accepted names are `lower_snake_case`
    ///
    /// [See Also](ToSnakeCase)
    SnakeCase,
    /// `UPPER_SNAKE_CASE` is primary name
    ///
    /// Other accepted names are `SCREAMING_SNAKE_CASE`
    ///
    /// [See Also](ToShoutySnakeCase)
    ScreamingSnakeCase,
    /// `kebab-case` is primary name
    ///
    /// Other accepted names are `lower-kebab-case`
    ///
    /// [See Also](ToKebabCase)
    KebabCase,
    /// `SCREAMING-KEBAB-CASE` is primary name
    ///
    /// Other accepted names are `UPPER-KEBAB-CASE`
    ///
    /// [See Also](ToShoutyKebabCase)
    ScreamingKebabCase,
    /// `Title Case` is the primary name
    ///
    /// Other accepted names are `TitleCase`
    ///
    /// [See Also](ToTitleCase)
    TitleCase,
    /// `Train-Case` is the primary name
    ///
    /// [See Also](ToTrainCase)
    TrainCase,
    /// `UPPERCASE` is the primary name
    ///
    /// This corresponds to the to_uppercase method in [String]
    UpperCase,
    /// `lowercase` is the primary name
    ///
    /// This corresponds to the to_lowercase method in [String]
    LowerCase,
}
variants!(
    LowerCamelCase => "lowerCamelCase",
    UpperCamelCase => "UpperCamelCase",
    PascalCase => "PascalCase",
    SnakeCase => "snake_case",
    ScreamingSnakeCase => "UPPER_SNAKE_CASE",
    KebabCase => "kebab-case",
    ScreamingKebabCase => "UPPER-KEBAB-CASE",
    TitleCase => "Title Case",
    TrainCase => "Train-Case",
    UpperCase => "UPPERCASE",
    LowerCase => "lowercase"
);

impl Case {
    /// Creates a [AsCase] wrapper for the specified case
    pub fn as_case<T: AsRef<str>>(&self, value: T) -> AsCase<T> {
        match self {
            Case::LowerCamelCase => AsCase::LowerCamelCase(AsLowerCamelCase(value)),
            Case::UpperCamelCase => AsCase::UpperCamelCase(AsUpperCamelCase(value)),
            Case::PascalCase => AsCase::PascalCase(AsUpperCamelCase(value)),
            Case::SnakeCase => AsCase::SnakeCase(AsSnakeCase(value)),
            Case::ScreamingSnakeCase => AsCase::ShoutySnakeCase(AsShoutySnakeCase(value)),
            Case::KebabCase => AsCase::KebabCase(AsKebabCase(value)),
            Case::ScreamingKebabCase => AsCase::ShoutyKebabCase(AsShoutyKebabCase(value)),
            Case::TitleCase => AsCase::TitleCase(AsTitleCase(value)),
            Case::TrainCase => AsCase::TrainCase(AsTrainCase(value)),
            Case::UpperCase => AsCase::UpperCase(value),
            Case::LowerCase => AsCase::LowerCase(value),
        }
    }
}

impl FromStr for Case {
    type Err = CaseNotFound;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "camelCase" | "lowerCamelCase" => Ok(Case::LowerCamelCase),
            "UpperCamelCase" => Ok(Case::UpperCamelCase),
            "PascalCase" => Ok(Case::PascalCase),
            "lower_snake_case" | "snake_case" | "snek_case" => Ok(Case::SnakeCase),
            "UPPER_SNAKE_CASE" | "SCREAMING_SNAKE_CASE" | "SHOUTY_SNEK_CASE" => {
                Ok(Case::ScreamingSnakeCase)
            }
            "lower-kebab-case" | "kebab-case" => Ok(Case::KebabCase),
            "UPPER-KEBAB-CASE" | "SCREAMING-KEBAB-CASE" => Ok(Case::ScreamingKebabCase),
            "TitleCase" | "Title Case" => Ok(Case::TitleCase),
            "Train-Case" => Ok(Case::TrainCase),
            "UPPERCASE" => Ok(Case::UpperCase),
            "lowercase" => Ok(Case::LowerCase),
            _ => Result::Err(s.into()),
        }
    }
}

impl TryFrom<String> for Case {
    type Error = CaseNotFound;

    #[inline(always)]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}
impl TryFrom<&str> for Case {
    type Error = CaseNotFound;

    #[inline(always)]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

/// The Trait that defines case conversion
/// This wrapper performs  case conversion in [`fmt::Display`].
///
/// ## Example:
/// ```
/// use heck::ToCase;
/// use heck::Case;
/// let original = "We are going to inherit the earth";
/// let cases = vec![
///   (Case::LowerCamelCase, "weAreGoingToInheritTheEarth"),
///   (Case::UpperCamelCase, "WeAreGoingToInheritTheEarth"),
///   (Case::PascalCase, "WeAreGoingToInheritTheEarth"),
///   (Case::SnakeCase, "we_are_going_to_inherit_the_earth"),
///   (Case::ScreamingSnakeCase, "WE_ARE_GOING_TO_INHERIT_THE_EARTH"),
///   (Case::KebabCase, "we-are-going-to-inherit-the-earth"),
///   (Case::ScreamingKebabCase, "WE-ARE-GOING-TO-INHERIT-THE-EARTH"),
///   (Case::TitleCase, "We Are Going To Inherit The Earth"),
///   (Case::TrainCase, "We-Are-Going-To-Inherit-The-Earth"),
///   (Case::UpperCase, "WE ARE GOING TO INHERIT THE EARTH"),
///   (Case::LowerCase, "we are going to inherit the earth"),
/// ];
///
/// for (case, expected) in cases {
///   assert_eq!(original.to_case(case), expected);
/// }
/// ```
pub trait ToCase: ToOwned {
    /// Converts the case of the string to the specified case

    fn to_case(&self, case: Case) -> Self::Owned;
    /// Converts the case of the type to the specified case
    ///
    /// If the case is None, it will return an owned version of the type
    fn to_optional_case(&self, case: Option<Case>) -> Self::Owned {
        match case {
            Some(case) => self.to_case(case),
            None => self.to_owned(),
        }
    }
}

impl ToCase for str {
    fn to_case(&self, case: Case) -> Self::Owned {
        match case {
            Case::LowerCamelCase => self.to_lower_camel_case(),
            Case::UpperCamelCase => self.to_upper_camel_case(),
            Case::PascalCase => self.to_pascal_case(),
            Case::SnakeCase => self.to_snake_case(),
            Case::ScreamingSnakeCase => self.to_shouty_snake_case(),
            Case::KebabCase => self.to_kebab_case(),
            Case::ScreamingKebabCase => self.to_shouty_kebab_case(),
            Case::TitleCase => self.to_title_case(),
            Case::TrainCase => self.to_train_case(),
            Case::UpperCase => self.to_uppercase(),
            Case::LowerCase => self.to_lowercase(),
        }
    }
}
/// This wrapper performs  case conversion in [`fmt::Display`].
///
/// ## Example:
/// ```
/// use heck::AsCase;
/// use heck::Case;
/// let original = "We are going to inherit the earth";
/// let cases = vec![
///   (Case::LowerCamelCase, "weAreGoingToInheritTheEarth"),
///   (Case::UpperCamelCase, "WeAreGoingToInheritTheEarth"),
///   (Case::PascalCase, "WeAreGoingToInheritTheEarth"),
///   (Case::SnakeCase, "we_are_going_to_inherit_the_earth"),
///   (Case::ScreamingSnakeCase, "WE_ARE_GOING_TO_INHERIT_THE_EARTH"),
///   (Case::KebabCase, "we-are-going-to-inherit-the-earth"),
///   (Case::ScreamingKebabCase, "WE-ARE-GOING-TO-INHERIT-THE-EARTH"),
///   (Case::TitleCase, "We Are Going To Inherit The Earth"),
///   (Case::TrainCase, "We-Are-Going-To-Inherit-The-Earth"),
///   (Case::UpperCase, "WE ARE GOING TO INHERIT THE EARTH"),
///   (Case::LowerCase, "we are going to inherit the earth"),
/// ];
///
/// for (case, expected) in cases {
///   assert_eq!(format!("{}", AsCase::from((original, case))), expected);
/// }
/// ```
#[non_exhaustive]
pub enum AsCase<T: AsRef<str>> {
    /// Wrapper Around [AsLowerCamelCase]
    LowerCamelCase(AsLowerCamelCase<T>),
    /// Wrapper Around [AsUpperCamelCase]
    UpperCamelCase(AsUpperCamelCase<T>),
    /// Wrapper Around [AsUpperCamelCase]
    PascalCase(AsUpperCamelCase<T>),
    /// Wrapper Around [AsSnakeCase]
    SnakeCase(AsSnakeCase<T>),
    /// Wrapper Around [AsShoutySnekCase]
    ShoutySnakeCase(AsShoutySnakeCase<T>),
    /// Wrapper Around [AsKebabCase]
    KebabCase(AsKebabCase<T>),
    /// Wrapper Around [AsShoutyKebabCase]
    ShoutyKebabCase(AsShoutyKebabCase<T>),
    /// Wrapper Around [AsTitleCase]
    TitleCase(AsTitleCase<T>),
    /// Wrapper Around [AsTrainCase]
    TrainCase(AsTrainCase<T>),
    /// Just calls to_uppercase in [str]
    UpperCase(T),
    /// Just calls to_lowercase in [str]
    LowerCase(T),
}
impl<T: AsRef<str>> From<(T, Case)> for AsCase<T> {
    fn from((s, case): (T, Case)) -> Self {
        case.as_case(s)
    }
}
/// Converts the case from the name and creates an AsCase wrapper
///
/// # Arguments
/// - First argument is the string to convert
/// - Second argument is the name of the case
impl<T: AsRef<str>> TryFrom<(T, &str)> for AsCase<T> {
    type Error = CaseNotFound;

    fn try_from((s, case): (T, &str)) -> Result<Self, Self::Error> {
        Case::from_str(case).map(|v| v.as_case(s))
    }
}

impl<T: AsRef<str>> fmt::Display for AsCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Currently, UpperCase and LowerCase do not have a write to the formatter method
        // So we will just convert them to a string and write that
        // This is not ideal, but it works
        // This could be changed in the future.
        match self {
            AsCase::LowerCamelCase(s) => s.fmt(f),
            AsCase::UpperCamelCase(s) => s.fmt(f),
            AsCase::PascalCase(s) => s.fmt(f),
            AsCase::SnakeCase(s) => s.fmt(f),
            AsCase::ShoutySnakeCase(s) => s.fmt(f),
            AsCase::KebabCase(s) => s.fmt(f),
            AsCase::ShoutyKebabCase(s) => s.fmt(f),
            AsCase::TitleCase(s) => s.fmt(f),
            AsCase::TrainCase(s) => s.fmt(f),
            AsCase::UpperCase(s) => write!(f, "{}", s.as_ref().to_uppercase()),
            AsCase::LowerCase(s) => write!(f, "{}", s.as_ref().to_lowercase()),
        }
    }
}
