# 0.4.0 (unreleased)

Breaking changes:

* Rename all traits from `SomeCase` to `ToSomeCase`, matching `std`s convention
  of beginning trait names with a verb (`ToOwned`, `AsRef`, …)
* Rename `ToMixedCase` to `ToLowerCamelCase`
* Rename `ToCamelCase` to `ToUpperCamelCase`
* Add `ToPascalCase` as an alias to `ToUpperCamelCase`
