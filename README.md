# **heck** is a case conversion library

!["I specifically requested the opposite of this."](https://github.com/withoutboats/heck/blob/master/no_step_on_snek.png)

This library exists to provide case conversion between common cases like
CamelCase and snake_case. It is intended to be unicode aware, internally
consistent, and reasonably well performing.

## Definition of a word boundary

Word boundaries are defined as the "unicode words" defined in the
`unicode_segmentation` library, as well as within those words in this manner:

1. All underscore characters are considered word boundaries.
2. A single uppercase letter (followed by no letters or by lowercase letters)
is considered to be just after a word boundary.
3. Multiple consecutive uppercase letters are considered to be between two
word boundaries.

That is, "HelloWorld" is segmented "Hello World" whereas "HELLOworld" is
segmented "HELLO world."

Characters not within words (such as spaces, punctuations, and underscores)
are not included in the output string except as they are a part of the case
being converted to. Multiple adjacent word boundaries (such as a series of
underscores) are folded into one. ("hello__world" in snake case is therefore
"hello_world", not the exact same string). Leading or trailing word boundary
indicators are dropped, except insofar as CamelCase capitalizes the first word.

### Cases contained in this library:

1. CamelCase
2. snake_case
3. kebab-case
4. SHOUTY_SNAKE_CASE
5. mixedCase
6. Title Case


### Contributing

PRs of additional well-established cases welcome.

This library is a little bit opinionated (dropping punctuation, for example).
If that doesn't fit your use case, I hope there is another crate that does. I
would prefer **not** to receive PRs to make this behavior more configurable.

Bug reports & fixes always welcome. :-)

### License

heck is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See LICENSE-APACHE and LICENSE-MIT for details.
