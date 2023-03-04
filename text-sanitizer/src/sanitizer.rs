#![allow(unused)]
/*
* @author Bodo (Hugo) Barwich
* @version 2023-02-26
* @package text-sanitizer
* @subpackage sanitizer.rs

* This module implements the Text-Sanitizer logic
*
*---------------------------------
* Requirements:
* - serde
*/

extern crate serde;

use std::collections::HashMap;
use std::str;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConversionMap(pub HashMap<String, LanguageMap>);

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageMap(pub HashMap<String, String>);

//==============================================================================
// Structure TextSanitizer Declaration

/// Structure that holds reusable data as the "_ConversionMap_", the vector
/// of Language Replacement Maps to be applied and runtime options like verbosity.

#[derive(Default, Debug)]
pub struct TextSanitizer {
    _oconv_map: Option<ConversionMap>,
    _vrqlangs: Vec<String>,
    _bquiet: bool,
    _bdebug: bool,
    _bprofiling: bool,
}

//==============================================================================
// Structure TextSanitizer Implementation

impl TextSanitizer {
    /*----------------------------------------------------------------------------
     * Constructors
     */

    /// The Default Constructor with default runtime options.
    ///
    /// # Default Options:
    ///
    /// * `bquiet = false` - do print warnings and errors.
    /// * `bdebug = false` - do not print detailed activity messages.
    ///
    /// # Example:
    ///
    /// Create a `TextSanitizer` object with default settings
    /// ```
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.add_request_language(&"en");
    /// ```
    pub fn new() -> TextSanitizer {
        let mut sanitizer = TextSanitizer {
            _oconv_map: None,
            _vrqlangs: Vec::new(),
            _bquiet: false,
            _bdebug: false,
            _bprofiling: false,
        };

        sanitizer.init();

        //Return the New TextSanitizer Object
        sanitizer
    }

    /// This Constructor allows to set a custom `ConversionMap` at buildtime.
    /// This will not create the Default `ConversionMap`.
    ///
    /// # Default Options:
    ///
    /// * `bquiet = false` - do print warnings and errors.
    /// * `bdebug = false` - do not print detailed activity messages.
    ///
    /// # Example:
    ///
    /// Create a `TextSanitizer` object with custom `ConversionMap`
    /// ```
    ///    use text_sanitizer::{TextSanitizer, ConversionMap, LanguageMap};
    ///    use std::collections::HashMap;
    ///
    ///    let mut conv_map = ConversionMap(HashMap::with_capacity(2));
    ///    let mut lang_map = LanguageMap(HashMap::with_capacity(4));
    ///
    ///    lang_map.0.insert("d".to_string(), "".to_string());
    ///    lang_map.0.insert("1b".to_string(), "".to_string());
    ///    lang_map.0.insert("80".to_string(), "EUR".to_string());
    ///    lang_map.0.insert("20ac".to_string(), "EUR".to_string());
    ///
    ///    conv_map.0.insert("custom".to_string(), lang_map);
    ///
    ///    let mut sanitizer = TextSanitizer::new_with_conversion_map(conv_map);
    ///
    ///    sanitizer.add_request_language(&"custom");
    /// ```
    pub fn new_with_conversion_map(conversion_map: ConversionMap) -> TextSanitizer {
        let mut sanitizer = TextSanitizer {
            _oconv_map: Some(conversion_map),
            _vrqlangs: Vec::new(),
            _bquiet: false,
            _bdebug: false,
            _bprofiling: false,
        };

        //Return the New TextSanitizer Object
        sanitizer
    }

    /// This Constructor already sets the most used runtime options.
    ///
    /// # Parameters:
    ///
    /// * `bquiet` - do not print any messages not even errors.
    /// * `bdebug` - do print detailed activity messages.
    /// * `bprofiling` - profile the internal processing activities. (currently not implemented
    /// because it affects performance)
    ///
    /// # Example:
    ///
    /// Create a `TextSanitizer` object with debugging enabled
    /// ```
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);
    ///
    ///    sanitizer.add_request_language(&"en");
    /// ```
    pub fn new_with_options(bquiet: bool, bdebug: bool, bprofiling: bool) -> TextSanitizer {
        let mut sanitizer = TextSanitizer {
            _oconv_map: None,
            _vrqlangs: Vec::new(),
            _bquiet: bquiet,
            _bdebug: bdebug,
            _bprofiling: bprofiling,
        };

        sanitizer.init();

        //Return the New TextSanitizer Object
        sanitizer
    }

    pub fn new_with_options_string(options: &str) -> TextSanitizer {
        //-------------------------------------
        // Parse the Function Options

        let mut sopt;

        let mut bdbg: bool = false;
        let mut bqt: bool = false;

        if (!options.is_empty()) {
            for prm in options.split_whitespace() {
                if prm.starts_with("--") {
                    sopt = prm.split_at(2).1;
                    sopt.to_lowercase();
                } else if prm.starts_with('-') {
                    sopt = prm.split_at(1).1;
                    sopt.to_lowercase();

                    match sopt {
                        "q" | "b" => bqt = true,
                        "d" | "v" => bdbg = true,
                        _ => {}
                    } //match sopt.as_ref()
                }
            } //for mut prm in options.split_whitespace() {
        } //if(!options.is_empty())

        //-------------------------------------
        // Create the TextSanitizer Object

        let mut sanitizer = TextSanitizer {
            _oconv_map: None,
            _vrqlangs: Vec::new(),
            _bquiet: bqt,
            _bdebug: bdbg,
            _bprofiling: false,
        };

        sanitizer.init();

        //Return the New TextSanitizer Object
        sanitizer
    }

    /*----------------------------------------------------------------------------
     * Administration Methods
     */

    /// This enables quiet execution.\
    /// This will not print any warnings only the result data.
    ///
    /// # Parameter:
    ///
    /// * `bquiet` - don't print any warnings.
    ///
    /// # Example:
    ///
    /// Create a `TextSanitizer` object and enable debugging
    /// ```
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.set_quiet(true);
    /// ```
    pub fn set_quiet(&mut self, bquiet: bool) {
        self._bquiet = bquiet;
    }

    /// This method enables very verbose activity reports.\
    /// This is mostly useful to troubleshoot raw data input and character conversions
    ///
    /// # Parameter:
    ///
    /// * `bdebug` - do print detailed activity messages.
    ///
    /// # Example:
    ///
    /// Create a `TextSanitizer` object and enable debugging
    /// ```
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.set_debug(true);
    /// ```
    pub fn set_debug(&mut self, bdebug: bool) {
        self._bdebug = bdebug;
    }

    pub fn set_profiling(&mut self, bprofiling: bool) {
        self._bprofiling = bprofiling;
    }

    /// This method parses the runtime options from their string presentation.\
    /// This is used for backward compatibility with the Procedural Interface.
    ///
    /// # Parameter:
    ///
    /// * `options` - string representation of the runtime options.\
    ///   *  ` q | b ` - do not print warings.
    ///   *  ` d | v ` - print detailed activity information.
    ///
    /// # Example:
    ///
    /// Create a `TextSanitizer` object and enable debugging
    /// ```
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.set_options_from_string(&"-d -q");
    /// ```
    pub fn set_options_from_string(&mut self, options: &str) {
        //-------------------------------------
        // Parse the Function Options

        let mut sopt;

        if (!options.is_empty()) {
            for prm in options.split_whitespace() {
                if prm.starts_with("--") {
                    sopt = prm.split_at(2).1;
                    sopt.to_lowercase();
                } else if prm.starts_with('-') {
                    sopt = prm.split_at(1).1;
                    sopt.to_lowercase();

                    match sopt {
                        "q" | "b" => self._bquiet = true,
                        "d" | "v" => self._bdebug = true,
                        _ => {}
                    } //match sopt.as_ref()
                }
            } //for mut prm in options.split_whitespace() {
        } //if(!options.is_empty())
    }

    /// This method allows to replace the default `ConversionMap` with a custom one.
    ///
    /// # Example:
    ///
    /// Set a custom `ConversionMap`
    /// ```
    ///    use text_sanitizer::{TextSanitizer, ConversionMap, LanguageMap};
    ///    use std::collections::HashMap;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    let mut conv_map = ConversionMap(HashMap::with_capacity(2));
    ///    let mut lang_map = LanguageMap(HashMap::with_capacity(4));
    ///
    ///    lang_map.0.insert("d".to_string(), "".to_string());
    ///    lang_map.0.insert("1b".to_string(), "".to_string());
    ///    lang_map.0.insert("80".to_string(), "EUR".to_string());
    ///    lang_map.0.insert("20ac".to_string(), "EUR".to_string());
    ///
    ///    conv_map.0.insert("custom".to_string(), lang_map);
    ///
    ///    sanitizer.set_conversion_map(conv_map);
    ///
    ///    sanitizer.add_request_language(&"custom");
    /// ```
    pub fn set_conversion_map(&mut self, conversion_map: ConversionMap) {
        self._oconv_map = Some(conversion_map);
    }

    /// This method allows to add or replace a custom `LanguageMap` within the `ConversionMap`.
    ///
    /// # Example:
    ///
    /// Add a custom `LanguageMap`
    /// ```
    ///    use text_sanitizer::{TextSanitizer, LanguageMap};
    ///    use std::collections::HashMap;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    let mut lang_map = LanguageMap(HashMap::with_capacity(4));
    ///
    ///    lang_map.0.insert("d".to_string(), "".to_string());
    ///    lang_map.0.insert("1b".to_string(), "".to_string());
    ///    lang_map.0.insert("80".to_string(), "EUR".to_string());
    ///    lang_map.0.insert("20ac".to_string(), "EUR".to_string());
    ///
    ///    sanitizer.set_language_map(&"custom", lang_map);
    ///
    ///    sanitizer.add_request_language(&"custom");
    /// ```
    pub fn set_language_map(&mut self, language: &str, language_map: LanguageMap) {
        if self._oconv_map.is_none() {
            self._oconv_map = Some(ConversionMap(HashMap::new()));
        }

        if let Some(map) = &mut self._oconv_map {
            map.0.insert(language.to_string(), language_map);
        }
    }

    /// This method allows to delete a `LanguageMap` from the `ConversionMap`.
    ///
    /// # Example:
    ///
    /// Remove support for the "es" `LanguageMap`
    /// ```
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.delete_language_map(&"es");
    /// ```
    pub fn delete_language_map(&mut self, language: &str) {
        if let Some(map) = &mut self._oconv_map {
            map.0.remove(language);
        }
    }

    /// Adds a Language Shortcode to the Vector of applied Language Replacement Maps.\
    /// In the order in which the Language Shortcode is added is the order in which they
    /// are applied to the Input Data.
    ///
    /// # Parameters:
    ///
    /// * `language` - language shortcode. By default only 'en', 'es' and 'de'
    /// are recognized.
    ///
    /// # Examples:
    ///
    /// This example activates the "en" Language Replacement Map.\
    /// This Language Replacement Map holds replacements that are not language specific.\
    /// Like the Unicode Symbol Sparkling Heart "U+1F496"
    /// ```
    ///    //-------------------------------------
    ///    // Activate the "en" Language Replacement Map
    ///
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.add_request_language(&"en");
    /// ```
    pub fn add_request_language(&mut self, language: &str) {
        let slang = String::from(language);

        if !self._vrqlangs.contains(&slang) {
            self._vrqlangs.push(slang);
        }
    }

    /// Clears the Vector of applied Language Replacement Maps.\
    /// This is needed to change the order of applied Language Replacement Maps.
    ///
    /// # Examples:
    ///
    /// This example deactivates all Language Replacement Maps beside "es".\
    /// This can be useful to fine tune the sanitization process and reduce unused overhead.
    /// ```
    ///    //-------------------------------------
    ///    // Deactivate all Language Replacement Maps beside "es"
    ///
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.add_request_language(&"en");
    ///    sanitizer.add_request_language(&"de");
    ///    sanitizer.add_request_language(&"es");
    ///
    ///    sanitizer.clear_request_languages();
    ///
    ///    sanitizer.add_request_language(&"es");
    /// ```
    pub fn clear_request_languages(&mut self) {
        self._vrqlangs.clear();
    }

    #[doc(hidden)]
    fn init(&mut self) {
        self._oconv_map = Some(ConversionMap(HashMap::with_capacity(3)));

        let mut lngrplmap = LanguageMap(HashMap::with_capacity(12));

        lngrplmap.0.insert("d".to_string(), "".to_string());
        lngrplmap.0.insert("1b".to_string(), "".to_string());
        lngrplmap.0.insert("bb".to_string(), "\"".to_string());
        lngrplmap.0.insert("ab".to_string(), "\"".to_string());
        lngrplmap.0.insert("80".to_string(), "EUR".to_string());
        lngrplmap.0.insert("20ac".to_string(), "EUR".to_string());
        lngrplmap.0.insert("25cf".to_string(), "*".to_string());
        lngrplmap.0.insert("251c".to_string(), "|-".to_string());
        lngrplmap.0.insert("2514".to_string(), "|-".to_string());
        lngrplmap.0.insert("2500".to_string(), "-".to_string());
        lngrplmap.0.insert("2764".to_string(), "<3".to_string());
        lngrplmap.0.insert("1f496".to_string(), "<3".to_string());

        if let Some(conv_map) = &mut self._oconv_map {
            conv_map.0.insert("en".to_string(), lngrplmap);
        }

        let mut lngrplmap = LanguageMap(HashMap::with_capacity(5));

        lngrplmap.0.insert("df".to_string(), "ss".to_string());
        lngrplmap.0.insert("dc".to_string(), "Ue".to_string());
        lngrplmap.0.insert("e4".to_string(), "ae".to_string());
        lngrplmap.0.insert("fc".to_string(), "ue".to_string());
        lngrplmap.0.insert("f6".to_string(), "oe".to_string());

        if let Some(conv_map) = &mut self._oconv_map {
            conv_map.0.insert("de".to_string(), lngrplmap);
        }

        let mut lngrplmap = LanguageMap(HashMap::with_capacity(6));

        lngrplmap.0.insert("d3".to_string(), "O".to_string());
        lngrplmap.0.insert("e1".to_string(), "a".to_string());
        lngrplmap.0.insert("e9".to_string(), "e".to_string());
        lngrplmap.0.insert("ed".to_string(), "i".to_string());
        lngrplmap.0.insert("f1".to_string(), "n".to_string());
        lngrplmap.0.insert("f3".to_string(), "o".to_string());

        if let Some(conv_map) = &mut self._oconv_map {
            conv_map.0.insert("es".to_string(), lngrplmap);
        }
    }

    #[doc(hidden)]
    fn from_utf8_lossy(mut input: &[u8]) -> String {
        let mut sanitized = String::with_capacity(input.len());

        loop {
            //        println!(
            //            "sequence (cnt: '{}'): '{:x?}' - parsing ...",
            //            input.len(),
            //            &input
            //        );

            match std::str::from_utf8(input) {
                Ok(valid) => {
                    unsafe {
                        for c in valid.chars() {
                            if c.is_ascii() {
                                sanitized.push(c);
                            } else {
                                //println!("{}", c.escape_unicode());
                                sanitized.push_str(&format!("{}", c.escape_unicode()));
                            }
                        }
                    }
                    break;
                }
                Err(error) => {
                    //println!("err beyond: '{}'", error.valid_up_to());

                    let (valid, after_valid) = input.split_at(error.valid_up_to());

                    unsafe {
                        let spec = std::str::from_utf8_unchecked(valid);

                        for c in spec.chars() {
                            if c.is_ascii() {
                                sanitized.push(c);
                            } else {
                                //println!("{}", c.escape_unicode());
                                sanitized.push_str(&format!("{}", c.escape_unicode()));
                            }
                        }
                    }

                    //println!("{:x?}", &after_valid);

                    if let Some(invalid_sequence_length) = error.error_len() {
                        //println!("ivld chrs cnt: '{}'", invalid_sequence_length);

                        for iu in 0..invalid_sequence_length {
                            sanitized.push_str(&format!("(?{:x?})", &after_valid[iu]));
                        }

                        input = &after_valid[invalid_sequence_length..]
                    } else {
                        break;
                    }
                }
            }
        }

        sanitized
    }

    #[doc(hidden)]
    // Parse byte sequence into unicode sequence strings
    fn build_unicode(&self, sequence: &[u8]) -> Vec<String> {
        let mut build_result: Vec<String> = Vec::new();
        let mut suni: String;
        let mut icstrt = 0;
        let mut icend = sequence.len();
        let mut ivldps = 0;
        let mut bprsgo: bool = true;

        if (self._bdebug && !self._bquiet) {
            println!(
                "; sequence 0 (cnt: '{}', strt: '{}', end: '{}'): '{:x?}' - parsing ...",
                sequence.len(),
                icstrt,
                icend,
                &sequence
            );
        } //if(bdebug && ! bquiet)

        while bprsgo && icstrt < icend {
            if (self._bdebug && !self._bquiet) {
                println!(
                    "; sequence (cnt: '{}', strt: '{}', end: '{}'): '{:x?}' - parsing ...",
                    sequence[icstrt..icend].len(),
                    icstrt,
                    icend,
                    &sequence[icstrt..icend]
                );
            } //if(bdebug && ! bquiet)

            let utf8rs = str::from_utf8(&sequence[icstrt..icend]);

            match utf8rs {
                Ok(s) => {
                    bprsgo = false;
                    build_result.push(s.to_owned());

                    if (self._bdebug && !self._bquiet) {
                        println!("utf8 ok: '{}'", s);
                    }
                }
                Err(e) => {
                    if (self._bdebug && !self._bquiet) {
                        println!("utf8 Err: '{:?}'", e);
                    }

                    ivldps = e.valid_up_to();

                    if (self._bdebug && !self._bquiet) {
                        println!("vld ps: '{}'", ivldps);
                    }

                    ivldps += icstrt;

                    if (self._bdebug && !self._bquiet) {
                        println!("vld idx: '{}'", ivldps);
                    }

                    if ivldps > icstrt {
                        if (self._bdebug && !self._bquiet) {
                            println!("utf8 recovered: '{:x?}'", &sequence[icstrt..ivldps]);
                        }

                        unsafe {
                            build_result.push(
                                std::str::from_utf8_unchecked(&sequence[icstrt..ivldps]).to_owned(),
                            );
                        }

                        icstrt = ivldps;
                    } else {
                        ivldps = icstrt;
                    } //if ivldps > icstrt

                    if let Some(invalid_sequence_length) = e.error_len() {
                        if (self._bdebug && !self._bquiet) {
                            println!("ivld chrs cnt: '{}'", invalid_sequence_length);
                        }

                        for iu in (ivldps)..(ivldps + invalid_sequence_length) {
                            if (self._bdebug && !self._bquiet) {
                                println!("ivld chr: '{:x?}'", &sequence[iu]);
                            }

                            build_result.push(format!("(?{:x?})", &sequence[iu]));
                        } //for iu in (ivldps)..(ivldps + invalid_sequence_length)

                        icstrt = ivldps + invalid_sequence_length;
                    } else {
                        //All Bytes are invalid
                        if (self._bdebug && !self._bquiet) {
                            println!("ivld chrs cnt: all");
                        }

                        for iu in (ivldps)..(icend) {
                            if (self._bdebug && !self._bquiet) {
                                println!("ivld chr: '{:x?}'", &sequence[iu]);
                            }

                            build_result.push(format!("(?{:x?})", &sequence[iu]));
                        } //for iu in (ivldps)..(ivldps + invalid_sequence_length)

                        bprsgo = false;
                    } //if let Some(invalid_sequence_length) = e.error_len()
                }
            }; //match utf8rs
        } //while bprsgo

        build_result
    }

    #[doc(hidden)]
    // A sequence of bytes is parsed into multiple characters or ascii symbols
    fn parse_unicode(&self, sequence: &[u8]) -> Vec<String> {
        let mut parse_result: Vec<String> = Vec::new();

        let vunicode = self.build_unicode(&sequence);

        if (self._bdebug && !self._bquiet) {
            println!("uni res: '{:?}'", vunicode);
        }

        if !vunicode.is_empty() {
            for sv in vunicode {
                if sv.starts_with("(?") && sv.ends_with(')') {
                    parse_result.push(sv[2..(sv.len() - 1)].to_owned());
                } else {
                    for c in sv.chars() {
                        let suni = c.escape_unicode().to_string();

                        parse_result.push(suni[3..(suni.len() - 1)].to_owned());
                    } // for c in sv.chars()
                } //if sv.starts_with("(?") && sv.ends_with(')')
            } //for sv in vunicode
        } //if ! vunicode.is_empty()

        parse_result
    }

    /// Parses the given reference to raw text data as array of bytes `u8` into
    /// a new valid `std::str::String`.
    ///
    /// # Parameters:
    ///
    /// * `text` - raw text data as array of bytes `u8`
    ///
    /// # Examples:
    ///
    /// Test data is the Sparkle Heart from the UTF-8 documentation examples but it is broken.\
    /// According to the Official Standard Library Documentation at:\
    /// [std::string::String::from_utf8()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8)\
    /// this would produce a `FromUtf8Error` or **panic** the application
    /// when used with `unwrap()`
    /// ```
    ///    //-------------------------------------
    ///    // Test data is the Sparkle Heart from the UTF-8 documentation examples but it is broken
    ///
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let vsparkle_heart = vec![240, 159, 119, 150];
    ///
    ///    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);
    ///
    ///    sanitizer.add_request_language(&"en");
    ///
    ///    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);
    ///
    ///    println!("sparkle_heart: '{}'", srsout);
    ///
    ///    assert_eq!(srsout, "(?f0)(?9f)w(?96)");
    /// ```
    pub fn sanitize_u8(&self, text: &[u8]) -> String {
        let mut olngrplmap = None;

        if (self._bdebug && !self._bquiet) {
            println!("vtext 0:'{:?}'", text);
        }

        if let Some(conv_map) = &self._oconv_map {
            let mut srstxt = String::with_capacity(text.len());
            let mut srptchrs = String::new();
            let mut orpl = None;
            let mut ic: usize = 0;
            let mut icstrt: Option<usize> = None;
            let mut icend: Option<usize> = None;

            for uc in text {
                if (self._bdebug && !self._bquiet) {
                    srptchrs.push_str(&format!("; {} - {}:'{}'", ic, uc, char::from(*uc)));
                }

                if (*uc >= 32 as u8 && *uc < 127 as u8) || (*uc == 10 as u8) || (*uc == 9 as u8) {
                    //------------------------
                    //Valid ASCII Character

                    //srptchrs.push_str(" - ascii");

                    if icstrt.is_some() {
                        //------------------------
                        //Pending Non ASCII Characters

                        icend = Some(ic);

                        if (self._bdebug && !self._bquiet) {
                            println!(
                                "pdg spec chars '{} - {}': '{:?}'",
                                icstrt.unwrap(),
                                icend.unwrap(),
                                &text[icstrt.unwrap()..icend.unwrap()]
                            );
                        }

                        //Parse the slice of Non ASCII Characters
                        let vuni = self.parse_unicode(&text[icstrt.unwrap()..icend.unwrap()]);

                        if (self._bdebug && !self._bquiet) {
                            print!("= {:?}", vuni);
                        }

                        for suni in vuni {
                            orpl = None;

                            for slng in &self._vrqlangs {
                                if orpl.is_none() {
                                    olngrplmap = conv_map.0.get(slng.as_str());

                                    if let Some(lngmap) = olngrplmap {
                                        orpl = lngmap.0.get(suni.as_str());
                                    }
                                } //if orpl.is_none()
                            } //for slng in vrqlanguages

                            match orpl {
                                Some(rpl) => {
                                    srstxt.push_str(rpl);

                                    if (self._bdebug && !self._bquiet) {
                                        print!(" -> '{}'", rpl);
                                    }
                                }
                                None => {
                                    srstxt.push_str(&format!("(?{})", &suni));

                                    if (self._bdebug && !self._bquiet) {
                                        print!(" -> '(?{})'", &suni);
                                    }
                                } //Some(rpl)
                            } //match orpl
                        } //for suni in vuni

                        if (self._bdebug && !self._bquiet) {
                            println!("'");
                        } //if(bdbg && ! bqt)
                          /**/

                        icstrt = None;
                    } //if icstrt.is_some()

                    //Add the valid ASCII Character
                    srstxt.push(char::from(*uc));
                } else {
                    //------------------------
                    //Non ASCII Character

                    srptchrs.push_str(&format!(" - non-ascii '{:?}", icstrt));

                    if icstrt.is_none() {
                        icstrt = Some(ic);
                        //if(bdbg && ! bqt) {
                        srptchrs.push_str(&format!(" > {:?} - {:?}'|", icstrt, icend));
                        //}
                    } else {
                        srptchrs.push_str(&format!(" - {:?}'|", icend));
                    } //if (uc[0] >= 32 as u8 && uc[0] < 127 as u8)
                } //if (*uc >= 32 as u8 && *uc < 127 as u8) || ( *uc == 10 as u8 )

                ic += 1;
            } //for uc in text

            if icstrt.is_some() {
                icend = Some(ic);

                if (self._bdebug && !self._bquiet) {
                    print!(
                        "\nrst spec char '{} - {}': '{:?}",
                        icstrt.unwrap(),
                        icend.unwrap(),
                        &text[icstrt.unwrap()..icend.unwrap()]
                    );
                }

                let vuni = self.parse_unicode(&text[icstrt.unwrap()..icend.unwrap()]);

                if (self._bdebug && !self._bquiet) {
                    print!(" | {:?}", vuni);
                }

                for suni in vuni {
                    orpl = None;

                    for slng in &self._vrqlangs {
                        if orpl.is_none() {
                            olngrplmap = conv_map.0.get(slng.as_str());

                            if let Some(lngmap) = olngrplmap {
                                orpl = lngmap.0.get(suni.as_str());
                            }
                        } //if orpl.is_none()
                    } //for slng in vrqlanguages

                    match orpl {
                        Some(rpl) => {
                            srstxt.push_str(rpl);

                            if (self._bdebug && !self._bquiet) {
                                print!(" -> '{}'", rpl);
                            }
                        }
                        None => {
                            srstxt.push_str(&format!("(?{})", &suni));

                            if (self._bdebug && !self._bquiet) {
                                print!(" -> '(?{})'", &suni);
                            }
                        } //Some(rpl)
                    } //match orpl
                } //for suni in vuni

                if (self._bdebug && !self._bquiet) {
                    print!("'");
                } //if(bdbg && ! bqt)

                icstrt = None;
            } //if icstrt.is_some()

            if (self._bdebug && !self._bquiet) {
                srptchrs.push_str(&format!("; chr cnt '{}'", ic));

                println!("; sanitze done.");
                println!("chrs rpt: '{:?}'", &srptchrs);

                let vsttrpt: Vec<char> = String::from_utf8_lossy(text).to_mut().chars().collect();

                println!("stt rpt chrs (count : '{}'):\n{:?}", vsttrpt.len(), vsttrpt);

                println!("stt chrs ascii:");

                for c in &vsttrpt {
                    if !c.is_ascii() {
                        print!("{}|", c.escape_unicode().to_string());
                    } else {
                        print!("{}|", c);
                    }
                } //for c in &vsttrpt

                println!();
            } //if(bdbg && ! bqt)

            //Return the sanitized String
            srstxt
        } else {
            String::from_utf8_lossy(text).into_owned()
        }
    }

    /// Creates from a given string slice a simplified version with ASCII characters.
    ///
    /// # Parameters:
    ///
    /// * `text` - String slice of text to sanitize
    ///
    /// # Examples:
    ///
    /// ```
    ///    //-------------------------------------
    ///    // Test data is the Sparkle Heart from the UTF-8 documentation examples.
    ///    // It will be sanitized into the ASCII Characters " <3 "
    ///
    ///    use text_sanitizer::TextSanitizer;
    ///
    ///    let vsparkle_heart = vec![240, 159, 146, 150];
    ///
    ///    let mut sanitizer = TextSanitizer::new();
    ///
    ///    sanitizer.add_request_language(&"en");
    ///
    ///    let srsout = match std::str::from_utf8(&vsparkle_heart) {
    ///         Ok(s) => sanitizer.sanitize_string(s),
    ///         Err(_) => sanitizer.sanitize_u8(&vsparkle_heart),
    ///    };
    ///
    ///    println!("sparkle_heart: '{}'", srsout);
    ///
    ///    assert_eq!(srsout, "<3");
    /// ```
    pub fn sanitize_string(&self, text: &str) -> String {
        self.sanitize_u8(text.as_bytes())
    }

    /*----------------------------------------------------------------------------
     * Consultation Methods
     */

    pub fn is_quiet(&self) -> bool {
        self._bquiet
    }

    pub fn is_debug(&self) -> bool {
        self._bdebug
    }

    pub fn is_profiling(&self) -> bool {
        self._bprofiling
    }

    pub fn has_request_language(&self, slanguage: &str) -> bool {
        self._vrqlangs.contains(&String::from(slanguage))
    }
}

//==============================================================================
// Procedural Interface

/// Parses the given reference to raw text data as array of bytes `u8` into
/// a new valid `std::str::String`.
///
/// # Parameters:
///
/// * `text` - raw text data as array of bytes `u8`
/// * `vrqlanguages` - Vector of language references. Currently only 'en', 'es' and 'de'
/// are recognized.
/// * `options` - reference to a string. Like command line arguments '-b', '-q' and '-d' and '-v'
/// are recognized.
///
/// # Examples:
///
/// Test data is the Sparkle Heart from the UTF-8 documentation examples but it is broken.\
/// According to the Official Standard Library Documentation at:\
/// [std::string::String::from_utf8()](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8)\
/// this would produce a `FromUtf8Error` or **panic** the application
/// when used with `unwrap()`
/// ```
///    //-------------------------------------
///    // Test data is the Sparkle Heart from the UTF-8 documentation examples but it is broken
///
///    use text_sanitizer::sanitizer::sanitize_u8;
///
///    let vsparkle_heart = vec![240, 159, 119, 150];
///
///    let vrqlngs: Vec<String> = vec![String::from("en")];
///
///    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &"");
///
///    println!("sparkle_heart: '{}'", srsout);
///
///    assert_eq!(srsout, "(?f0)(?9f)w(?96)");
/// ```
pub fn sanitize_u8(text: &[u8], vrqlanguages: &Vec<String>, options: &str) -> String {
    let mut sanitizer = TextSanitizer::new_with_options_string(options);

    for srqlang in vrqlanguages {
        sanitizer.add_request_language(srqlang.as_str());
    }

    sanitizer.sanitize_u8(text)
}

/// Creates from a given string slice a simplified version with ASCII characters.
///
/// # Parameters:
///
/// * `text` - String of text to sanitize
/// * `vrqlanguages` - Vector of language references. Currently only 'en', 'es' and 'de'
/// are recognized.
/// * `options` - reference to a string. Like command line arguments '-b', '-q' and '-d' and '-v'
/// are recognized.
///
/// # Examples:
///
/// ```
///    //-------------------------------------
///    // Test data is the Sparkle Heart from the UTF-8 documentation examples.
///    // It will be sanitized into the ASCII Characters " <3 "
///
///    use text_sanitizer::sanitizer::{sanitize_string, sanitize_u8};
///
///    let vsparkle_heart = vec![240, 159, 146, 150];
///
///    let vrqlngs: Vec<String> = vec![String::from("en")];
///
///    let srsout = match std::str::from_utf8(&vsparkle_heart) {
///         Ok(s) => sanitize_string(s, &vrqlngs, &""),
///         Err(_) => sanitize_u8(&vsparkle_heart, &vrqlngs, &""),
///    };
///
///    println!("sparkle_heart: '{}'", srsout);
///
///    assert_eq!(srsout, "<3");
/// ```
pub fn sanitize_string(text: &str, vrqlanguages: &Vec<String>, options: &str) -> String {
    let mut sanitizer = TextSanitizer::new_with_options_string(options);

    for srqlang in vrqlanguages {
        sanitizer.add_request_language(srqlang.as_str());
    }

    sanitizer.sanitize_u8(text.as_bytes())
}

//==============================================================================
// Unit Tests

/*
Recreating the Test Data:

    $ perl -e 'my @arrchrs = (240, 159, 151, 119, 150, 139); print pack "U*", @arrchrs;' | target/debug/text-sanitizer -i -d

*/

#[test]
fn proc_sparkle_heart() {
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples

    let vsparkle_heart = vec![240, 159, 146, 150];

    let vrqlngs: Vec<String> = vec![String::from("en")];

    let srsout = sanitize_u8(&vsparkle_heart, &vrqlngs, &"-d");

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}

#[test]
fn proc_sparkle_heart_string() {
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples

    let vsparkle_heart = vec![240, 159, 146, 150];

    let vrqlngs: Vec<String> = vec![String::from("en")];

    let srsout = sanitize_string(str::from_utf8(&vsparkle_heart).unwrap(), &vrqlngs, &"-d");

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}

#[test]
fn sanitizer_sparkle_heart() {
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples

    let vsparkle_heart = vec![240, 159, 146, 150];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}

#[test]
fn sanitizer_sparkle_heart_string() {
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples

    let vsparkle_heart = vec![240, 159, 146, 150];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_string(str::from_utf8(&vsparkle_heart).unwrap());

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}

/// Instantiating the `TextSanitizer` with a different `ConversionMap` causes that the german
/// umlaut " _ü_ " is not recognized anymore. Bytes are shown as byte sequence.

#[test]
fn sanitizer_new_conversion_map() {
    let finnish_data = "Minä tahdon kernaasti puhua suomen kielen, [mutta] en minä taida.";
    let german_data = "Deutsch verwendet ähnliche Umlaute, wird aber anders übersetzt.";
    let finnish_res = "Mina tahdon kernaasti puhua suomen kielen, [mutta] en mina taida.";
    let german_res = "Deutsch verwendet ahnliche Umlaute, wird aber anders (?fc)bersetzt.";


    let mut conv_map = ConversionMap(HashMap::with_capacity(2));
    let mut lang_map = LanguageMap(HashMap::with_capacity(1));

    lang_map.0.insert("e4".to_string(), "a".to_string());

    conv_map.0.insert("fi".to_string(), lang_map);

    let mut sanitizer = TextSanitizer::new_with_conversion_map(conv_map);

    sanitizer.add_request_language(&"fi");

    let sanitized = sanitizer.sanitize_string(finnish_data);

    println!("finnish result: '{}'", sanitized);

    assert_eq!(sanitized.as_str(), finnish_res);

    let sanitized = sanitizer.sanitize_string(german_data);

    println!("german result: '{}'", sanitized);

    assert_eq!(sanitized.as_str(), german_res);
}

#[test]
fn sanitizer_set_conversion_map() {
    let finnish_data = "Minä tahdon kernaasti puhua suomen kielen, [mutta] en minä taida.";
    let german_data = "Deutsch verwendet ähnliche Umlaute, wird aber anders übersetzt.";
    let finnish_res = "Mina tahdon kernaasti puhua suomen kielen, [mutta] en mina taida.";
    let german_res = "Deutsch verwendet aehnliche Umlaute, wird aber anders uebersetzt.";

    let mut sanitizer = TextSanitizer::new();

    let mut conv_map = ConversionMap(HashMap::with_capacity(2));
    let mut lang_map = LanguageMap(HashMap::with_capacity(1));

    lang_map.0.insert("e4".to_string(), "a".to_string());

    conv_map.0.insert("fi".to_string(), lang_map);

    let mut lang_map = LanguageMap(HashMap::with_capacity(2));

    lang_map.0.insert("e4".to_string(), "ae".to_string());
    lang_map.0.insert("fc".to_string(), "ue".to_string());

    conv_map.0.insert("de".to_string(), lang_map);

    sanitizer.set_conversion_map(conv_map);

    sanitizer.add_request_language(&"fi");
    sanitizer.add_request_language(&"de");

    let sanitized = sanitizer.sanitize_string(finnish_data);

    println!("finnish result: '{}'", sanitized);

    assert_eq!(sanitized.as_str(), finnish_res);

    sanitizer.clear_request_languages();

    sanitizer.add_request_language(&"de");
    sanitizer.add_request_language(&"fi");

    let sanitized = sanitizer.sanitize_string(german_data);

    println!("german result: '{}'", sanitized);

    assert_eq!(sanitized.as_str(), german_res);
}

#[test]
fn sanitizer_language_map() {
    let finnish_data = "Minä tahdon kernaasti puhua suomen kielen, [mutta] en minä taida.";
    let german_data = "Deutsch verwendet ähnliche Umlaute, wird aber anders übersetzt.";
    let finnish_res = "Mina tahdon kernaasti puhua suomen kielen, [mutta] en mina taida.";
    let german_res = "Deutsch verwendet aehnliche Umlaute, wird aber anders uebersetzt.";

    let mut sanitizer = TextSanitizer::new();

    let mut lang_map = LanguageMap(HashMap::with_capacity(1));

    lang_map.0.insert("e4".to_string(), "a".to_string());

    sanitizer.set_language_map(&"fi", lang_map);

    sanitizer.add_request_language(&"fi");

    let sanitized = sanitizer.sanitize_string(finnish_data);

    println!("finnish result: '{}'", sanitized);

    assert_eq!(sanitized.as_str(), finnish_res);

    sanitizer.clear_request_languages();

    sanitizer.add_request_language(&"de");
    sanitizer.add_request_language(&"fi");

    let sanitized = sanitizer.sanitize_string(german_data);

    println!("german result: '{}'", sanitized);

    assert_eq!(sanitized.as_str(), german_res);
}

#[test]
fn sparkle_heart_broken() {
    //-------------------------------------
    //Test data is the Sparkle Heart from the UTF-8 documentation examples but it is broken

    let vsparkle_heart = vec![240, 159, 119, 150];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "(?f0)(?9f)w(?96)");
}

#[test]
fn two_hearts_center() {
    let vsparkle_heart = vec![240, 159, 146, 150, 119, 250, 240, 159, 146, 150];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3w(?fa)<3");
}

#[test]
fn two_hearts_start() {
    let vsparkle_heart = vec![250, 240, 159, 146, 150, 119, 240, 159, 146, 150];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "(?fa)<3w<3");
}

#[test]
fn two_hearts_end() {
    let vsparkle_heart = vec![240, 159, 146, 150, 119, 240, 159, 146, 150, 250];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3w<3(?fa)");
}

#[test]
fn hearts_game() {
    let vsparkle_heart = vec![
        226, 157, 164, 240, 159, 146, 150, 119, 250, 248, 240, 159, 146, 150, 247, 190,
    ];

    let mut sanitizer = TextSanitizer::new_with_options(false, true, false);

    sanitizer.add_request_language(&"en");

    let srsout = sanitizer.sanitize_u8(&vsparkle_heart);

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3<3w(?fa)(?f8)<3(?f7)(?be)");
}
