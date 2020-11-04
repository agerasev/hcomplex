# hcomplex

[![Crates.io][crates_badge]][crates]
[![Docs.rs][docs_badge]][docs]
[![Travis CI][travis_badge]][travis]
[![Appveyor][appveyor_badge]][appveyor]
[![Codecov.io][codecov_badge]][codecov]
[![License][license_badge]][license]

[crates_badge]: https://img.shields.io/crates/v/hcomplex.svg
[docs_badge]: https://docs.rs/hcomplex/badge.svg
[travis_badge]: https://api.travis-ci.org/nthend/hcomplex.svg
[appveyor_badge]: https://ci.appveyor.com/api/projects/status/github/nthend/hcomplex?branch=master&svg=true
[codecov_badge]: https://codecov.io/gh/nthend/hcomplex/graphs/badge.svg
[license_badge]: https://img.shields.io/crates/l/hcomplex.svg

[crates]: https://crates.io/crates/hcomplex
[docs]: https://docs.rs/hcomplex
[travis]: https://travis-ci.org/nthend/hcomplex
[appveyor]: https://ci.appveyor.com/project/nthend/hcomplex
[codecov]: https://codecov.io/gh/nthend/hcomplex
[license]: #license

Lightweight complex and hypercomplex algebra library.

## Contents

+ Complex and hypercomplex numbers:
  + `Complex<T>` - 2-dimensional commutative and associative algebra
  + `Quaternion<T>` - 4-dimensional associative but non-commutative algebra
  + `Octonion<T>` - 8-dimensional non-commutative and non-associative algebra
  + `Sedenion<T>` - 16-dimensional non-commutative and non-associative algebra with nontrivial zero divisors
  + and following algebras created by Cayley-Dickson construction `Construct<T, A<T>>` where `A<T>` is previous algebra
+ Transformations:
  + `transform::Moebius<T, A<T>>` - Moebius transform, where `A<T>` is some algebra

## Crate Features

Crate could be used in `no_std` mode.

+ `std` - Use `std`. Enabled by default.
+ `random` - Hypercomplex number random generator.
+ `approx` - Approximate comparison of hypercomplex numbers.

## Testing

```bash
cargo test --features random,approx
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
