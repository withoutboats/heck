# **heck** is a case conversion library

!["I specifically requested the opposite of this."](./no_step_on_snek.png)

This library exists to provide case conversion between common cases like
CamelCase and snake_case. It is intended to be unicode aware, internally
consistent, and reasonably well performing.

## Definition of a word boundary

The definition of a word boundary is based on the
[identifier word boundary](https://www.unicode.org/reports/tr55/#Identifier-Chunks)
in Unicode Technical Standard 55. The rules are as follows:

- The set of characters that can be in a word is
  [`[\p{ID_Continue}\p{ID_Compat_Math_Continue}\p{Unassigned}\p{Private_Use}-[\p{Punctuation}-\p{Other_Punctuation}]]`][1],
  plus U+05F3, U+05F4, and U+0F0B. This notably includes
  alphabetic and numeric characters, accents and other combining marks,
  emoji, a few mathematical symbols, a few non-word-separating punctuation marks,
  unassigned characters, private-use characters, and the asterisk `*`.

- Characters that cannot be in a word separate words.
  For example, `foo_bar` is segmented `foo`|`bar`
  because words cannout contain `_`.
  These characters will be excluded from the output string.

- Words cannot be empty. For example, `_foo__bar_` is segmented `foo`|`bar`,
  and in snake_case becomes `foo_bar`.

- There is a word boundary between a lowercase (or non-Greek titlecase)
  and an uppercase (or titlecase) letter. For example, `fooBar` is segmented
  `foo`|`Bar` because `oB` is a lowercase letter followed by an uppercase letter.

- An uppercase letter followed by a lowercase letter
  has a word boundary before it. For example, `XMLHttpRequest` is segmented
  `XML`|`Http`|`Request`; the `Ht` in `HttpRequest` is an uppercase letter
  followed by a lowercase letter, so there is a word boundary before it.

 - There is always a word boundary before a non-Greek titlecase letter
   (U+01C5 'ǅ', U+01C8 'ǈ', U+01CB 'ǋ', or U+01F2 'ǲ').

 - For the purpose of the preceding three rules, a letter followed
   by some number of nonspacing marks (like accents or other diacritics)
   is treated as if it was the letter alone. For example, `áB` is segmented `á`|`B`.

[1]: https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5B%5Cp%7BID_Continue%7D%5Cp%7BID_Compat_Math_Continue%7D%5Cp%7BUnassigned%7D%5Cp%7BPrivate_Use%7D-%5B%5Cp%7BPunctuation%7D-%5Cp%7BOther_Punctuation%7D%5D%5D&abb=on

## Cases contained in this library:

1. UpperCamelCase
2. lowerCamelCase
3. snake_case
4. kebab-case
5. SHOUTY_SNAKE_CASE
6. Title Case
7. SHOUTY-KEBAB-CASE
8. Train-Case

## MSRV

The minimum supported Rust version for this crate is 1.56.0. This may change in
minor or patch releases, but we probably won't ever require a very recent
version. If you would like to have a stronger guarantee than that, please open
an issue.

## License

heck is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
