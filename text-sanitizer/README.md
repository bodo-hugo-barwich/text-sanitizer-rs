[![Automated Tests](https://github.com/bodo-hugo-barwich/text-sanitizer-rs/actions/workflows/testing.yml/badge.svg)](https://github.com/bodo-hugo-barwich/text-sanitizer-rs/actions/workflows/testing.yml)


# Text-Sanitizer
_Rust_ Crate to convert raw text bytes into valid `std::str::String` with plain ASCII encoding

## Features
* Very low Dependencies\
  This leads to:
  * High Compability (compiles even with old _Rust_ Compilers)
  * Very fast Startup Time (Execution Time about **3 - 5 ms**)
* Robust Code (does not use risky `unwrap()` Methods)\
  Developed with the _DevOps_ Mentalitity: "_can fail but will live to tell_"


## Motivation
Most _Rust_ parsing libraries will bail out when fed with raw data that is not UTF-8 encoded like `ISO-8859-15 Windows` encoding
and others or mixed-up encodings. \
Using `Str::from_utf8_lossy()` will break those data and includes linear back and forth parsing on byte level
which introduces performance penality on bigger data.\
`text-sanitizer` does not depend on proper encoding detection and relies only on an internal customizable convertion map.

## Usage
### Object Oriented Method
When several sanitization operations are executed the Conversion Map can be reused.\
This is more resource saving and results also in a Micro Optimization.
For example Web Server can benefit from this.
```rust
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples

    use text_sanitizer::TextSanitizer;

    let vsparkle_heart = vec![240, 159, 146, 150];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
```

### Procedural Method
The `sanitizer::sanitize_u8()` function takes the raw data and creates a new valid UTF-8 `std::str::String` from it.
```rust
use text_sanitizer::sanitizer;

fn sparkle_heart() {
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples
    // which will be converted to " <3 ".

    let vsparkle_heart = vec![240, 159, 146, 150];
    let vrqlngs: Vec<String> = vec![String::from("en")];

    let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &"");

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}
```
Considering this example where the data in the center is corrupted somehow:
This data cannot be parsed by normal _Rust_ libraries and the containing valid information would be lost.
```rust
use text_sanitizer::sanitizer;

fn two_hearts_center() {
    //-------------------------------------
    // Test data contains 2 Sparkle Hearts but is corrupted in the center
    // According to the Official Standard Library Documentation at:
    // https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8
    // this would produce a FromUtf8Error or panic the application
    // when used with unwrap()

    let vsparkle_heart = vec![240, 159, 146, 150, 119, 250, 240, 159, 146, 150];
    let vrqlngs: Vec<String> = vec![String::from("en")];

    let srsout = sanitizer::sanitize_u8(&vsparkle_heart, &vrqlngs, &" -d");

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3w(?fa)<3");
}
```
