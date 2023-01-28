/*
* @author Bodo (Hugo) Barwich
* @version 2022-12-18
* @package text-sanitizer
* @subpackage lib.rs

* This module provides the Text-Sanitizer library
*
*---------------------------------
* Requirements:
* - The Rust module "sanitizer" must be installed
*/

//! Converts raw text bytes into valid UTF-8 `std::str::String` with simplyfied
//! ASCII Characters
//!
//! For example Unicode Symbol Sparkling Heart "U+1F496" will be converted to " <3 ".
//! [Emoji _Sparkling Heart_](https://codepoints.net/U+1F496)
//!
//! The conversion relies on parsing the bytes into unicode codepoint strings
//! which then are mapped with a conversion map to simplyfied ASCII Characters.
//!
//! The conversion map helps also to rescue unrecognized bytes with custom mappings.
//! So, a wrongly encoded byte like "(?80)" can be mapped to "EUR" which correctly
//! encoded should be "U+20AC"

pub mod sanitizer;

pub use sanitizer::TextSanitizer;
