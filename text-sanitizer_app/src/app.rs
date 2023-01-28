#![allow(unused)]
/*
* @author Bodo (Hugo) Barwich
* @version 2022-11-04
* @package text-sanitizer_app
* @subpackage app.rs

* This module is the application to run the Text-Sanitizer library
*
*---------------------------------
* Requirements:
* - The Rust crate "text-sanitizer" must be installed
*/

extern crate text_sanitizer;

use text_sanitizer::TextSanitizer;

use std::io::{self, Read};

/*
extern crate serde; // 1.0.149;
extern crate serde_yaml; // 0.8.26;

use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
*/

/*
#[derive(Debug, Deserialize, Serialize)]
struct ConversionMap(HashMap<String, LanguageMap>);

#[derive(Debug, Deserialize, Serialize)]
struct LanguageMap(HashMap<String, String>);

#[allow(unused_variables)]
fn main() {
    let conv_map_yaml = "---
en:
  'd': ''
  '1b': ''
  '20ac': 'EUR'
de:
  'fc': 'ue'
  'f6': 'oe'
";
     // Deserialize it back to a Rust type.
    let conv_map: ConversionMap = serde_yaml::from_str(&conv_map_yaml).unwrap();
   println!("conv mp 0 dmp: {:?}", conv_map);
}

*/

//==============================================================================
// Structure RunTextSanitizer Declaration

#[derive(Debug)]
pub struct RunTextSanitizer {
    _sanitizer: TextSanitizer,
    _vinput: Vec<u8>,
    _srsout: String,
    _vrqlangs: Vec<String>,
    _bimport: bool,
    _bquiet: bool,
    _bdebug: bool,
    _bprofiling: bool,
    _ierr: i32,
}

//==============================================================================
// Structure RunTextSanitizer Implementation

impl Default for RunTextSanitizer {
    /*----------------------------------------------------------------------------
     * Default Constructor
     */

    fn default() -> Self {
        RunTextSanitizer::new()
    }
}

#[allow(dead_code)]
impl RunTextSanitizer {
    /*----------------------------------------------------------------------------
     * Constructors
     */

    pub fn new() -> RunTextSanitizer {
        let mut app = RunTextSanitizer {
            _sanitizer: TextSanitizer::new(),
            _vinput: Vec::new(),
            _srsout: String::new(),
            _vrqlangs: Vec::new(),
            _bimport: false,
            _bquiet: false,
            _bdebug: false,
            _bprofiling: false,
            _ierr: 0,
        };

        app.init();

        //Return the New Application Object
        app
    }

    pub fn new_with_options(
        bimport: bool,
        bquiet: bool,
        bdebug: bool,
        bprofiling: bool,
    ) -> RunTextSanitizer {
        let mut app = RunTextSanitizer {
            _sanitizer: TextSanitizer::new(),
            _vinput: Vec::new(),
            _srsout: String::new(),
            _vrqlangs: Vec::new(),
            _bimport: bimport,
            _bquiet: bquiet,
            _bdebug: bdebug,
            _bprofiling: bprofiling,
            _ierr: 0,
        };

        app.init();

        //Return the New Application Object
        app
    }

    /*----------------------------------------------------------------------------
     * Administration Methods
     */

    pub fn set_import(&mut self, bimport: bool) {
        self._bimport = bimport;
    }

    pub fn set_quiet(&mut self, bquiet: bool) {
        self._bquiet = bquiet;

        self._sanitizer.set_quiet(bquiet);
    }

    pub fn set_debug(&mut self, bdebug: bool) {
        self._bdebug = bdebug;

        self._sanitizer.set_debug(bdebug);
    }

    pub fn set_profiling(&mut self, bprofiling: bool) {
        self._bprofiling = bprofiling;

        self._sanitizer.set_profiling(bprofiling);
    }

    pub fn add_request_language(&mut self, slanguage: &str) {
        let slang = String::from(slanguage);

        if !self._vrqlangs.contains(&slang) {
            self._vrqlangs.push(slang);
        }

        self._sanitizer.add_request_language(slanguage);
    }

    fn init(&mut self) {
        self.add_request_language(&"en");
    }

    fn set_input(&mut self, mut vinput: Vec<u8>) {
        self._vinput = vinput.drain(0..).collect();
    }

    pub fn input_from_stdin(&mut self) -> i32 {
        //-------------------------------------
        //Read the Input Data from STDIN

        //    let duration_input = SystemTime::now();
        let mut stdin = io::stdin();

        //    if self._bprofiling {
        //      match duration_input.duration_since(SystemTime::UNIX_EPOCH) {
        //        Ok(n) => eprintln!("1: {:?}", n)
        //        , Err(e) => eprintln!("1: SystemTime::now() failed!\nErr: '{:?}'", e)
        //      }
        //    } //if bprfg

        match stdin.read_to_end(&mut self._vinput) {
            Ok(_) => {}
            Err(e) => {
                if !self._bquiet {
                    eprintln!("{}", &format!("msg: '{:?}'", e));
                }

                //Clear the invalid Input
                self._vinput.clear();
                //Set Execution Error
                self._ierr = 1;
            }
        } //match stdin.read_to_end(&mut self._vinput)

        self._ierr
    }

    fn do_sanitze(&mut self) -> i32 {
        //-------------------------------------
        //Parse the Input Data

        //  let duration_parse = SystemTime::now();

        //  if self._bprofiling {
        //    match duration_parse.duration_since(SystemTime::UNIX_EPOCH) {
        //      Ok(tn) => eprintln!("3: {:?}", tn)
        //      , Err(e) => eprintln!("3: SystemTime::now() failed!\nErr: '{:?}'", e)
        //    }
        //  }  //if self._bprofiling

        self._srsout = self._sanitizer.sanitize_u8(&self._vinput);

        //  if self._bprofiling {
        //    match duration_parse.elapsed() {
        //      Ok(tp) => eprintln!("Parse time: {:?}", tp)
        //      , Err(e) => eprintln!("Parse time - elapsed() failed!\nErr: '{:?}'", e)
        //    }

        if (self._bdebug && !self._bquiet) {
            println!(
                "rs rpt chrs (count : '{}'):\n{:?}",
                self._srsout.len(),
                self._srsout
            );
        }

        self._ierr
    }

    pub fn do_run(&mut self) -> i32 {
        if self._bimport {
            self.input_from_stdin();
        }

        self.do_sanitze();

        //  if(bdbg && ! bqt) {
        //    println!("sleeping 30 secs: go ...");

        // we sleep for 30 seconds
        //    sleep(Duration::new(30, 0));

        //    println!("sleeping 30 secs: done.");
        //  }  //if(bdbg && ! bqt)

        self.output_to_stdout();

        self._ierr
    }

    /*----------------------------------------------------------------------------
     * Consultation Methods
     */

    pub fn is_import(&self) -> bool {
        self._bimport
    }

    pub fn is_quiet(&self) -> bool {
        self._bquiet
    }

    pub fn is_debug(&self) -> bool {
        self._bdebug
    }

    pub fn is_profiling(&self) -> bool {
        self._bprofiling
    }

    pub fn get_output(&self) -> &str {
        self._srsout.as_str()
    }

    fn output_to_stdout(&self) -> i32 {
        //        let data = self._importer.export_accounts_str();

        print!("{}", &self._srsout);

        self._ierr
    }

    pub fn get_error_code(&self) -> i32 {
        self._ierr
    }
}

//==============================================================================
// Auxiliary Functions

fn remove_match<T: PartialEq>(vvector: &mut Vec<T>, search: &T) -> Option<usize> {
    let mut iter = vvector.iter_mut();
    let mut oitem = iter.next();
    let mut iitempos = 0;
    let mut oipos = None;

    while oitem.is_some() && oipos.is_none() {
        if let Some(item) = oitem {
            if item == search {
                oipos = Some(iitempos);
            } else {
                iitempos += 1;
            }

            oitem = iter.next();
        }
    } //while oitem.is_some() && oipos.is_none()

    if let Some(ipos) = oipos {
        vvector.remove(ipos);
    }

    //eprintln!("remove_match rs: '{:?}'\n", oipos);

    oipos
}

//==============================================================================
// Unit Tests

#[test]
fn app_sparkle_heart() {
    //-------------------------------------
    // Test data is the Sparkle Heart from the UTF-8 documentation examples

    let vsparkle_heart = vec![240, 159, 146, 150];

    let mut app = RunTextSanitizer::new_with_options(false, false, true, false);

    app.add_request_language(&"en");
    app.set_input(vsparkle_heart);

    app.do_sanitze();

    let srsout = app.get_output();

    println!("sparkle_heart: '{}'", srsout);

    assert_eq!(srsout, "<3");
}
