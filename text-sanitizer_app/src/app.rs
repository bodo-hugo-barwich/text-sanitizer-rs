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

use text_sanitizer::sanitizer;

use std::io::{self, Read};

//==============================================================================
// Structure RunTextSanitizer Declaration

#[derive(Debug)]
pub struct RunTextSanitizer {
    //_sanitizer: MovementImporter::new(),
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
            //_sanitizer: TextSanitizer::new(),
            _vinput: Vec::new(),
            _srsout: String::new(),
            _vrqlangs: Vec::new(),
            _bimport: false,
            _bquiet: false,
            _bdebug: false,
            _bprofiling: false,
            _ierr: 0,
        };

        app._init();

        //Return the New RunTextSanitizer Object
        app
    }

    /*
    #----------------------------------------------------------------------------
    #Administration Methods
    */

    pub fn set_import(&mut self, bimport: bool) {
        self._bimport = bimport;
    }

    pub fn set_quiet(&mut self, bquiet: bool) {
        self._bquiet = bquiet;

        // self._sanitizer.set_quiet(bquiet);
    }

    pub fn set_debug(&mut self, bdebug: bool) {
        self._bdebug = bdebug;

        // self._sanitizer.set_debug(bdebug);
    }

    pub fn set_profiling(&mut self, bprofiling: bool) {
        self._bprofiling = bprofiling;

        // self._sanitizer.set_profiling(bprofiling);
    }

    pub fn add_request_language(&mut self, slanguage: &str) {
        let slang = String::from(slanguage);

        if !self._vrqlangs.contains(&slang) {
            self._vrqlangs.push(slang);
        }
    }

    fn _init(&mut self) {
        self._vrqlangs.push(String::from("en"));
    }

    fn input_from_stdin(&mut self) -> i32 {
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

        let mut sopt = String::new();

        if self._bquiet {
            sopt.push_str(" -q");
        }

        if self._bdebug {
            sopt.push_str(" -d");
        }

        //  let duration_parse = SystemTime::now();

        //  if self._bprofiling {
        //    match duration_parse.duration_since(SystemTime::UNIX_EPOCH) {
        //      Ok(tn) => eprintln!("3: {:?}", tn)
        //      , Err(e) => eprintln!("3: SystemTime::now() failed!\nErr: '{:?}'", e)
        //    }
        //  }  //if self._bprofiling

        self._srsout = sanitizer::sanitize_u8(&self._vinput, &self._vrqlangs, &sopt);

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
