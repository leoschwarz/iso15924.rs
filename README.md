[travis-badge]: https://img.shields.io/travis/zeyla/iso15924.rs.svg?style=flat-square
[travis]: https://travis-ci.org/zeyla/iso15924.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license]: https://opensource.org/licenses/ISC

[![travis-badge][]][travis] [![license-badge][]][license]

# iso15924.rs

Rust crate for ISO 15924 data, retrieved from unicode.org.

Data in the crate is updated regularly (every week or so) from [the table] on
<unicode.org>.


### What is ISO 15924

> ISO 15924, Codes for the representation of names of scripts, defines two sets
> of codes for a number of writing systems (scripts). Each script is given both
> a four-letter code and a numeric one.[1] Script is defined as "set of graphic
> characters used for the written form of one or more languages".
>
> -- [Wikipedia](https://en.wikipedia.org/wiki/ISO_15924)


### Installation

Add the following dependency to your Cargo.toml:

```rust
iso15924 = "^0.1"
```

And include it in your project:

```rust
extern crate iso15924;
```

### Examples

Retrieve a `Vec` of all `ScriptCode` definitions:

```rust
let scripts = iso15924::all();
```


A full list of examples can be found in the [examples folder] or in the
[documentation].


### License

License info in [LICENSE.md]. Long story short, ISC.

[LICENSE.md]: https://gitlab.com/kalasi/iso15924.rs/blob/master/LICENSE.md
[documentation]: https://docs.austinhellyer.me/iso15924
[examples folder]: https://gitlab.com/zeyla/iso15924.rs/tree/master/examples
[the table]: http://unicode.org/iso15924/iso15924-codes.html
